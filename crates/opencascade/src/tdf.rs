//! Bindings for OCCT's TDF (Topological Data Framework) and TNaming packages.
//!
//! [`TdfData`] is the top-level datastructure containing a tree of [`TdfLabel`]s
//!
//! The use of these labels within the tree is the OCCT way of capturing moments in the topological story of a
//! "document", and the tree describes how these moments come together.
//!
//! [`TdfTransaction`]s are used to scope changes in a document, and committed changes are captured
//! by [`TdfDelta`]. Feeding a [`TdfDelta`] into a document via [`TdfData::undo`] enables a
//! reversion of a transaction, with said [`TdfData::undo`] emitting an inverted of the delta to
//! allow for a "backwards-undo", i.e. "redo"
//!
//! That leaves us with [`TnamingBuilder`]: It is the write interface for TNaming: it records the
//! provenance of topological shapes into the document, establishing the
//! relationships that allow stable shape references to survive model rebuilds.
//!
//! # Example
//!
//! ```
//! use opencascade::tdf::{TdfData, TnamingBuilder};
//! use opencascade::primitives::Shape;
//! use glam::dvec3;
//!
//! // A document and two shapes: one before, and after a translation
//! let original = Shape::box_centered(10.0, 10.0, 10.0);
//! let mut translated = Shape::box_centered(10.0, 10.0, 10.0);
//! translated.set_global_translation(dvec3(5.0, 0.0, 0.0));
//!
//! let mut doc = TdfData::new();
//!
//! // Label hierarchy mirrors the operation tree
//! let mut tx = doc.transaction();
//! tx.open();
//! let label = doc.root().new_child();
//!
//! // Record the translation as a shape evolution.
//! // The builder borrows the transaction and must be dropped before commit.
//! // Future API iterations will enforce this structurally via closure arguments.
//! {
//!     let mut builder = TnamingBuilder::new(&tx, &label);
//!     builder.modify(
//!         &original,
//!         &translated,
//!     );
//! }
//!
//! // Commit captures the delta; undo reverses it
//! let delta = tx.commit();
//! let redo_delta = doc.undo(delta);
//!
//! // Redo reapplies
//! let _ = doc.undo(redo_delta);
//! ```
use cxx::UniquePtr;
use opencascade_sys::ffi;
use std::marker::PhantomData;

use crate::primitives::Shape;

/// ```compile_fail
/// use opencascade::tdf::TdfData;
/// let root = {
///     let doc = TdfData::new();
///     doc.root()
/// };
/// ```
/// ```compile_fail
/// use opencascade::tdf::TdfData;
/// let tx = {
///     let doc = TdfData::new();
///     doc.transaction()
/// };
/// ```
pub struct TdfData {
    inner: UniquePtr<ffi::HandleTdfData>,
}
/// Owned record of what changed in a committed transaction. Passed back
/// into [`TdfData::undo`] to reverse those changes. The inverse delta
/// returned by undo enables redo, and deltas can be retained externally
/// for history traversal or version management.
pub struct TdfDelta {
    pub(crate) inner: UniquePtr<ffi::HandleTdfDelta>,
}

impl TdfData {
    pub fn new() -> Self {
        Self { inner: ffi::TDF_Data_new() }
    }

    pub fn root(&self) -> TdfLabel<'_> {
        TdfLabel { inner: ffi::TDF_Data_root(self.inner.as_ref().unwrap()), _phantom: PhantomData }
    }
    pub fn transaction(&self) -> TdfTransaction<'_> {
        TdfTransaction {
            inner: ffi::TDF_Transaction_new(self.inner.as_ref().unwrap()),
            _phantom: PhantomData,
        }
    }
    /// Consumes the passed in `delta`, reverting the [`TdfTransaction::commit`] (and friends) that
    /// constructed it.
    /// ```
    /// # use opencascade::tdf::{TdfData, TnamingBuilder, TnamingNamedShape};
    /// # use opencascade::primitives::Shape;
    /// # use glam::{ dvec3, DVec3 };
    /// let original = Shape::box_centered(10.0, 10.0, 10.0);
    /// let mut translated = Shape::box_centered(10.0, 10.0, 10.0);
    /// translated.set_global_translation(dvec3(5.0, 0.0, 0.0));
    /// let mut doc = TdfData::new();
    /// let mut tx = doc.transaction();
    /// tx.open();
    /// let label = doc.root().new_child();
    /// // Record the translation as a shape evolution.
    /// // The builder borrows the transaction and must be dropped before commit.
    /// // Future API iterations will enforce this structurally via closure arguments.
    /// let named_shape;
    /// {
    ///     let mut builder = TnamingBuilder::new(&tx, &label);
    ///     builder.modify(
    ///         &original,
    ///         &translated,
    ///     );
    ///     named_shape = builder.named_shape();
    /// }
    ///
    /// let delta = tx.commit();
    /// // after commit: active shape is the translated one
    /// assert_ne!(named_shape.get().translation(), DVec3::ZERO);
    /// let redo_delta = doc.undo(delta);
    /// // after undo: active shape reverts
    /// assert_eq!(named_shape.get().translation(), DVec3::ZERO);
    /// let _ = doc.undo(redo_delta);
    /// // after redo: translated again
    /// assert_ne!(named_shape.get().translation(), DVec3::ZERO);
    /// ```
    #[must_use]
    pub fn undo(&mut self, delta: TdfDelta) -> TdfDelta {
        // unwrap: A [`TdfDelta`] not being a valid handle implies it was not constructed through
        // the use of [`TdfTransaction::commit`] at time of writing.
        TdfDelta { inner: ffi::TDF_Data_undo(self.inner.pin_mut(), delta.inner.as_ref().unwrap()) }
    }
}

pub struct TdfLabel<'doc> {
    inner: UniquePtr<ffi::TDF_Label>,
    _phantom: PhantomData<&'doc TdfData>,
}

impl<'doc> TdfLabel<'doc> {
    pub fn new_child(&self) -> TdfLabel<'doc> {
        TdfLabel {
            inner: ffi::TDF_Label_new_child(self.inner.as_ref().unwrap()),
            _phantom: PhantomData,
        }
    }

    pub fn is_null(&self) -> bool {
        ffi::TDF_Label_is_null(self.inner.as_ref().unwrap())
    }
}
/// Brackets a set of mutations to the label tree. The record of what
/// changed is not retained here, as it is transfers to [`TdfDelta`] on commit.
///
/// ```compile_fail
/// use opencascade::tdf::{TdfData, TnamingBuilder};
/// use opencascade::primitives::Shape;
/// let original = Shape::box_centered(10.0, 10.0, 10.0);
/// let mut doc = TdfData::new();
/// let mut tx = doc.transaction();
/// tx.open();
/// let label = doc.root().new_child();
/// let mut builder = TnamingBuilder::new(&tx, &label);
/// let _ = tx.commit();
/// builder.select(&original, &original); // use after commit attempt
/// ```
pub struct TdfTransaction<'doc> {
    inner: UniquePtr<ffi::TDF_Transaction>,
    _phantom: PhantomData<&'doc TdfData>,
}

impl<'doc> TdfTransaction<'doc> {
    // TODO: this i32 return value needs to be new-typed and have an interface that encapsulates
    // its nature. Refer to OCCT support for nested transactions and how this value indexes into
    // them. For now, the nesting is always 1, but that will change...
    pub fn open(&mut self) -> i32 {
        ffi::TDF_Transaction_open(self.inner.pin_mut())
    }

    pub fn is_open(&self) -> bool {
        ffi::TDF_Transaction_is_open(self.inner.as_ref().unwrap())
    }

    /// "Closes the bracket" on this transaction, committing the change to the [`TdfData`]
    /// Dropping the returned [`TdfDelta`] permanently discards the ability
    /// to undo this transaction.
    #[must_use]
    pub fn commit(mut self) -> TdfDelta {
        TdfDelta { inner: ffi::TDF_Transaction_commit(self.inner.pin_mut()) }
    }

    pub fn abort(mut self) {
        ffi::TDF_Transaction_abort(self.inner.pin_mut())
    }
}
/// The write interface for topological naming. Records shape evolution, i.e.
///  the provenance relationships between pre- and post-operation shapes,
///  into a [`TdfLabel`]. These records are what allow stable references
/// to topological entities to survive model rebuilds.
///
/// An open [`TdfTransaction`] is required for the builder's full lifetime
/// because the underlying attribute write must be captured in a delta.
/// The borrow checker enforces this: [`TdfTransaction::commit`] cannot
/// be called while any builder is live.
///
/// # Correctness
/// [`TdfTransaction::open`] must be called before constructing a builder.
/// The borrow checker enforces that the transaction outlives the builder,
/// but cannot enforce that the transaction is open. Attempting to construct a builder
/// on an unopened transaction will panic at the OCCT level.
///
/// Correct usage:
/// ```
/// use opencascade::tdf::{TdfData, TnamingBuilder};
/// let mut doc = TdfData::new();
/// let mut tx = doc.transaction();
/// tx.open(); // must precede builder construction
/// let label = doc.root().new_child();
/// let mut builder = TnamingBuilder::new(&tx, &label);
/// ```
///
/// Future API iterations will enforce this structurally, e.g. via closure arguments,
/// making the open/build/commit sequence impossible to misorder.
pub struct TnamingBuilder<'tx, 'doc: 'tx> {
    inner: UniquePtr<ffi::TNaming_Builder>,
    _phantom: PhantomData<&'tx TdfTransaction<'doc>>,
}

impl<'tx, 'doc: 'tx> TnamingBuilder<'tx, 'doc> {
    /// The `_tx` argument is passed in to bind the lifetime of the [`Self`] to the scope of the
    /// transaction it is used within
    pub fn new(_tx: &'tx TdfTransaction<'doc>, label: &TdfLabel<'doc>) -> Self {
        Self {
            inner: ffi::TNaming_Builder_ctor(label.inner.as_ref().unwrap()),
            _phantom: PhantomData,
        }
    }
    /// Use `old_shape: None` for primitives with no predecessor.
    pub fn generated(&mut self, old_shape: Option<&Shape>, new_shape: &Shape) {
        match old_shape {
            None => ffi::TNaming_Builder_generated(
                self.inner.pin_mut(),
                new_shape.inner.as_ref().unwrap(),
            ),
            Some(old) => ffi::TNaming_Builder_generated_with_old(
                self.inner.pin_mut(),
                old.inner.as_ref().unwrap(),
                new_shape.inner.as_ref().unwrap(),
            ),
        }
    }
    pub fn modify(&mut self, old_shape: &Shape, new_shape: &Shape) {
        ffi::TNaming_Builder_modify(
            self.inner.pin_mut(),
            old_shape.inner.as_ref().unwrap(),
            new_shape.inner.as_ref().unwrap(),
        );
    }

    pub fn delete(&mut self, old_shape: &Shape) {
        ffi::TNaming_Builder_delete(self.inner.pin_mut(), old_shape.inner.as_ref().unwrap());
    }

    pub fn select(&mut self, shape: &Shape, in_shape: &Shape) {
        ffi::TNaming_Builder_select(
            self.inner.pin_mut(),
            shape.inner.as_ref().unwrap(),
            in_shape.inner.as_ref().unwrap(),
        );
    }

    pub fn named_shape(&self) -> TnamingNamedShape {
        TnamingNamedShape { inner: ffi::TNaming_Builder_named_shape(self.inner.as_ref().unwrap()) }
    }
}

/// A handle to the [`TNaming_NamedShape`] attribute written to a [`TdfLabel`]
/// by [`TnamingBuilder`]. Retaining this after the builder is dropped allows
/// querying the recorded shape evolution after commit and across undo/redo
/// boundaries.
pub struct TnamingNamedShape {
    pub(crate) inner: UniquePtr<ffi::Handle_TNaming_NamedShape>,
}

impl TnamingNamedShape {
    pub fn original_shape(&self) -> Shape {
        Shape { inner: ffi::TNaming_Tool_original_shape(self.inner.as_ref().unwrap()) }
    }
    /// The new shape from the recorded evolution pair. Reflects the current undo/redo state
    pub fn get(&self) -> Shape {
        Shape { inner: ffi::TNaming_NamedShape_Get(self.inner.as_ref().unwrap()) }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tdf_data_root_is_not_null() {
        let doc = TdfData::new();
        let root = doc.root();
        assert!(!root.is_null());
    }

    #[test]
    fn test_new_child_with_transaction() {
        let doc = TdfData::new();
        let mut tx = doc.transaction();
        tx.open();
        let root = doc.root();
        let child = root.new_child();
        assert!(!child.is_null());
        let _ = tx.commit();
    }
    #[test]
    fn test_tnaming_builder_constructs() {
        let doc = TdfData::new();
        let mut tx = doc.transaction();
        tx.open();
        let root = doc.root();
        let label = root.new_child();
        let _builder = TnamingBuilder::new(&tx, &label);
    }
    #[test]
    fn test_tnaming_builder_generated() {
        use crate::primitives::Shape;
        let doc = TdfData::new();
        let mut tx = doc.transaction();
        tx.open();
        let root = doc.root();
        let label = root.new_child();
        let mut builder = TnamingBuilder::new(&tx, &label);
        let shape = Shape::sphere(1.0).build();
        builder.generated(None, &shape);
    }
    #[test]
    fn test_tnaming_undo_round_trip() {
        use crate::primitives::Shape;
        use glam::{dvec3, DVec3};

        let original = Shape::box_centered(10.0, 10.0, 10.0);
        let mut translated = Shape::box_centered(10.0, 10.0, 10.0);
        translated.set_global_translation(dvec3(5.0, 0.0, 0.0));

        let mut doc = TdfData::new();
        let mut tx = doc.transaction();
        tx.open();
        let root = doc.root();
        let label = root.new_child();

        let mut builder = TnamingBuilder::new(&tx, &label);
        let named_shape;
        {
            builder.modify(&original, &translated);

            named_shape = builder.named_shape();
        }

        let delta = tx.commit();

        // After commit: active shape should be the translated one
        assert_ne!(named_shape.get().translation(), DVec3::ZERO);

        let _redo = doc.undo(delta);

        // After undo: active shape should be back to original
        assert_eq!(named_shape.get().translation(), DVec3::ZERO);

        // original_shape is always the recorded old shape regardless of undo state
        let recovered = named_shape.original_shape();
        assert_eq!(recovered.translation(), DVec3::ZERO);
    }
}
