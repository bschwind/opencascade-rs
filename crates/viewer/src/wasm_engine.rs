use anyhow::Result;
use core::sync::atomic::{AtomicBool, Ordering};
use glam::DVec3;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use opencascade::primitives as occ;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use wasmtime::{
    component::{Component, Linker, Resource, ResourceTable, ResourceTableError},
    Config, Engine, Store,
};

wasmtime::component::bindgen!({
    path: "../model-api/wit",
    // This is where we map the types we declared in the WIT file
    // to their corresponding host type, which will be managed as
    // a wasmtime "Resource".
    with: {
        "wire-builder": occ::WireBuilder,
        "edge-iterator": occ::EdgeIterator,
        "face-iterator": occ::FaceIterator,
        "chamfer-maker": occ::ChamferMaker,
        "edge": occ::Edge,
        "wire": occ::Wire,
        "face": occ::Face,
        "shell": occ::Shell,
        "solid": occ::Solid,
        "compound": occ::Compound,
        "shape": occ::Shape,
    },
});

impl From<Point3> for DVec3 {
    fn from(p: Point3) -> Self {
        Self { x: p.x, y: p.y, z: p.z }
    }
}

impl From<DVec3> for Point3 {
    fn from(p: DVec3) -> Self {
        Self { x: p.x, y: p.y, z: p.z }
    }
}

struct TypedResourceTable<T> {
    resource_table: ResourceTable,
    _marker: std::marker::PhantomData<T>,
}

impl<T: std::marker::Send + 'static> TypedResourceTable<T> {
    fn new() -> Self {
        Self { resource_table: ResourceTable::new(), _marker: std::marker::PhantomData }
    }

    fn push(&mut self, val: T) -> Result<Resource<T>, ResourceTableError> {
        self.resource_table.push(val)
    }

    fn get(&self, key: &Resource<T>) -> Result<&T, ResourceTableError> {
        self.resource_table.get(key)
    }

    fn get_mut(&mut self, key: &Resource<T>) -> Result<&mut T, ResourceTableError> {
        self.resource_table.get_mut(key)
    }

    fn delete(&mut self, resource: Resource<T>) -> Result<T, ResourceTableError> {
        self.resource_table.delete(resource)
    }
}

struct ModelHost {
    wire_builders: TypedResourceTable<occ::WireBuilder>,
    edge_iterators: TypedResourceTable<occ::EdgeIterator>,
    face_iterators: TypedResourceTable<occ::FaceIterator>,
    chamfer_makers: TypedResourceTable<occ::ChamferMaker>,
    edges: TypedResourceTable<occ::Edge>,
    wires: TypedResourceTable<occ::Wire>,
    faces: TypedResourceTable<occ::Face>,
    shells: TypedResourceTable<occ::Shell>,
    solids: TypedResourceTable<occ::Solid>,
    compounds: TypedResourceTable<occ::Compound>,
    shapes: TypedResourceTable<occ::Shape>,
}

impl ModelHost {
    fn new() -> Self {
        Self {
            wire_builders: TypedResourceTable::new(),
            edge_iterators: TypedResourceTable::new(),
            face_iterators: TypedResourceTable::new(),
            chamfer_makers: TypedResourceTable::new(),
            edges: TypedResourceTable::new(),
            wires: TypedResourceTable::new(),
            faces: TypedResourceTable::new(),
            shells: TypedResourceTable::new(),
            solids: TypedResourceTable::new(),
            compounds: TypedResourceTable::new(),
            shapes: TypedResourceTable::new(),
        }
    }
}

impl HostEdgeIterator for ModelHost {
    fn new(
        &mut self,
        face_resource: Resource<occ::Face>,
    ) -> Result<Resource<occ::EdgeIterator>, anyhow::Error> {
        let face = self.faces.get(&face_resource)?;

        Ok(self.edge_iterators.push(face.edges())?)
    }

    fn next(
        &mut self,
        resource: Resource<occ::EdgeIterator>,
    ) -> Result<Option<Resource<occ::Edge>>, anyhow::Error> {
        let iter = self.edge_iterators.get_mut(&resource)?;
        let next_item = iter.next().map(|edge| self.edges.push(edge).unwrap());

        Ok(next_item)
    }

    fn drop(&mut self, resource: Resource<occ::EdgeIterator>) -> Result<(), anyhow::Error> {
        let _ = self.edge_iterators.delete(resource);
        Ok(())
    }
}

impl HostFaceIterator for ModelHost {
    fn new(
        &mut self,
        shape_resource: Resource<occ::Shape>,
    ) -> Result<Resource<occ::FaceIterator>, anyhow::Error> {
        let shape = self.shapes.get(&shape_resource)?;

        Ok(self.face_iterators.push(shape.faces())?)
    }

    fn next(
        &mut self,
        resource: Resource<occ::FaceIterator>,
    ) -> Result<Option<Resource<occ::Face>>, anyhow::Error> {
        let iter = self.face_iterators.get_mut(&resource)?;
        let next_item = iter.next().map(|face| self.faces.push(face).unwrap());

        Ok(next_item)
    }

    fn drop(&mut self, resource: Resource<occ::FaceIterator>) -> Result<(), anyhow::Error> {
        let _ = self.face_iterators.delete(resource);
        Ok(())
    }
}

impl HostChamferMaker for ModelHost {
    fn new(
        &mut self,
        shape_resource: Resource<occ::Shape>,
    ) -> Result<Resource<occ::ChamferMaker>, anyhow::Error> {
        let shape = self.shapes.get(&shape_resource)?;

        Ok(self.chamfer_makers.push(occ::ChamferMaker::new(shape))?)
    }

    fn add_edge(
        &mut self,
        chamfer_maker_resource: Resource<occ::ChamferMaker>,
        distance: f64,
        edge_resource: Resource<occ::Edge>,
    ) -> Result<(), anyhow::Error> {
        let chamfer_maker = &mut self.chamfer_makers.get_mut(&chamfer_maker_resource)?;
        let edge = self.edges.get(&edge_resource)?;
        chamfer_maker.add_edge(distance, edge);

        Ok(())
    }

    fn build(
        &mut self,
        resource: Resource<occ::ChamferMaker>,
    ) -> Result<Resource<occ::Shape>, anyhow::Error> {
        let shape = self.chamfer_makers.delete(resource)?.build();
        let shape_resource = self.shapes.push(shape)?;

        Ok(shape_resource)
    }

    fn drop(&mut self, resource: Resource<occ::ChamferMaker>) -> Result<(), anyhow::Error> {
        let _ = self.chamfer_makers.delete(resource);
        Ok(())
    }
}

impl HostEdge for ModelHost {
    fn segment(&mut self, p1: Point3, p2: Point3) -> Result<Resource<occ::Edge>, anyhow::Error> {
        Ok(self.edges.push(occ::Edge::segment(p1.into(), p2.into()))?)
    }

    fn drop(&mut self, resource: Resource<occ::Edge>) -> Result<(), anyhow::Error> {
        let _ = self.edges.delete(resource);
        Ok(())
    }
}

impl HostWireBuilder for ModelHost {
    fn new(&mut self) -> Result<Resource<occ::WireBuilder>, anyhow::Error> {
        Ok(self.wire_builders.push(occ::WireBuilder::new())?)
    }

    fn add_edge(
        &mut self,
        builder_resource: Resource<occ::WireBuilder>,
        edge_resource: Resource<occ::Edge>,
    ) -> Result<(), anyhow::Error> {
        let builder = &mut self.wire_builders.get_mut(&builder_resource)?;
        let edge = self.edges.get(&edge_resource)?;
        builder.add_edge(edge);

        Ok(())
    }

    fn build(
        &mut self,
        resource: Resource<occ::WireBuilder>,
    ) -> Result<Resource<occ::Wire>, anyhow::Error> {
        let wire = self.wire_builders.delete(resource)?.build();

        let new_wire = self.wires.push(wire)?;

        Ok(new_wire)
    }

    fn drop(&mut self, resource: Resource<occ::WireBuilder>) -> Result<(), anyhow::Error> {
        let _ = self.wire_builders.delete(resource);
        Ok(())
    }
}

impl HostWire for ModelHost {
    fn drop(&mut self, resource: Resource<occ::Wire>) -> Result<(), anyhow::Error> {
        let _ = self.wires.delete(resource);
        Ok(())
    }
}

impl HostFace for ModelHost {
    fn drop(&mut self, resource: Resource<occ::Face>) -> Result<(), anyhow::Error> {
        let _ = self.faces.delete(resource);
        Ok(())
    }

    fn fillet(
        &mut self,
        face_resource: Resource<occ::Face>,
        radius: f64,
    ) -> Result<Resource<occ::Face>, anyhow::Error> {
        let face = self.faces.get(&face_resource)?;
        let new_face = face.fillet(radius);
        Ok(self.faces.push(new_face)?)
    }

    fn extrude(
        &mut self,
        face_resource: Resource<occ::Face>,
        dir: Point3,
    ) -> Result<Resource<occ::Solid>, anyhow::Error> {
        let face = self.faces.get(&face_resource)?;
        let new_solid = face.extrude(dir.into());
        Ok(self.solids.push(new_solid)?)
    }

    fn from_wire(
        &mut self,
        wire_resource: Resource<occ::Wire>,
    ) -> Result<Resource<occ::Face>, anyhow::Error> {
        let wire = self.wires.get(&wire_resource)?;
        let face = Face::from_wire(wire);

        let new_face = self.faces.push(face)?;

        Ok(new_face)
    }

    fn outer_wire(
        &mut self,
        face_resource: Resource<occ::Face>,
    ) -> Result<Resource<occ::Wire>, anyhow::Error> {
        let face = self.faces.get(&face_resource)?;
        let new_wire = face.outer_wire();
        Ok(self.wires.push(new_wire)?)
    }

    fn center_of_mass(
        &mut self,
        face_resource: Resource<occ::Face>,
    ) -> Result<Point3, anyhow::Error> {
        let face = self.faces.get(&face_resource)?;
        Ok(face.center_of_mass().into())
    }
}

impl HostShell for ModelHost {
    fn drop(&mut self, resource: Resource<occ::Shell>) -> Result<(), anyhow::Error> {
        let _ = self.shells.delete(resource);
        Ok(())
    }
}

impl HostSolid for ModelHost {
    fn drop(&mut self, resource: Resource<occ::Solid>) -> Result<(), anyhow::Error> {
        let _ = self.solids.delete(resource);
        Ok(())
    }
}

impl HostCompound for ModelHost {
    fn drop(&mut self, resource: Resource<occ::Compound>) -> Result<(), anyhow::Error> {
        let _ = self.compounds.delete(resource);
        Ok(())
    }
}

impl HostShape for ModelHost {
    fn drop(&mut self, resource: Resource<occ::Shape>) -> Result<(), anyhow::Error> {
        let _ = self.shapes.delete(resource);
        Ok(())
    }

    fn from_edge(
        &mut self,
        edge_resource: Resource<occ::Edge>,
    ) -> Result<Resource<occ::Shape>, anyhow::Error> {
        let edge = self.edges.get(&edge_resource)?;
        let shape = edge.into();

        let new_shape = self.shapes.push(shape)?;

        Ok(new_shape)
    }

    fn from_wire(
        &mut self,
        wire_resource: Resource<occ::Wire>,
    ) -> Result<Resource<occ::Shape>, anyhow::Error> {
        let wire = self.wires.get(&wire_resource)?;
        let shape = wire.into();

        let new_shape = self.shapes.push(shape)?;

        Ok(new_shape)
    }

    fn from_face(
        &mut self,
        face_resource: Resource<occ::Face>,
    ) -> Result<Resource<occ::Shape>, anyhow::Error> {
        let face = self.faces.get(&face_resource)?;
        let shape = face.into();

        let new_shape = self.shapes.push(shape)?;

        Ok(new_shape)
    }

    fn from_shell(
        &mut self,
        shell_resource: Resource<occ::Shell>,
    ) -> Result<Resource<occ::Shape>, anyhow::Error> {
        let shell = self.shells.get(&shell_resource)?;
        let shape = shell.into();

        let new_shape = self.shapes.push(shape)?;

        Ok(new_shape)
    }

    fn from_solid(
        &mut self,
        solid_resource: Resource<occ::Solid>,
    ) -> Result<Resource<occ::Shape>, anyhow::Error> {
        let solid = self.solids.get(&solid_resource)?;
        let shape = solid.into();

        let new_shape = self.shapes.push(shape)?;

        Ok(new_shape)
    }

    fn from_compound(
        &mut self,
        compound_resource: Resource<occ::Compound>,
    ) -> Result<Resource<occ::Shape>, anyhow::Error> {
        let compound = self.compounds.get(&compound_resource)?;
        let shape = compound.into();

        let new_shape = self.shapes.push(shape)?;

        Ok(new_shape)
    }
}

// Imports into the world, like the `name` import for this world, are
// satisfied through traits.
impl ModelWorldImports for ModelHost {
    fn print(&mut self, _msg: String) -> Result<(), wasmtime::Error> {
        Ok(())
    }
}

pub struct WasmEngine {
    engine: Engine,
    linker: Linker<ModelHost>,
    component: Component,
    wasm_path: PathBuf,
    changed: Arc<AtomicBool>,
    _watcher: RecommendedWatcher,
}

fn convert_to_component(path: impl AsRef<Path>) -> Vec<u8> {
    let bytes = &std::fs::read(&path).unwrap();
    wit_component::ComponentEncoder::default().module(bytes).unwrap().encode().unwrap()
}

impl WasmEngine {
    pub fn new(wasm_path: impl AsRef<Path>) -> Self {
        let mut config = Config::new();
        config.wasm_component_model(true);
        let engine = Engine::new(&config).unwrap();

        let component_bytes = convert_to_component(&wasm_path);
        let component = Component::from_binary(&engine, &component_bytes).unwrap();

        let mut linker = Linker::new(&engine);
        ModelWorld::add_to_linker(&mut linker, |state: &mut ModelHost| state).unwrap();

        let changed = Arc::new(AtomicBool::new(false));
        let mut watcher = RecommendedWatcher::new(
            {
                let changed = Arc::clone(&changed);
                move |res| match res {
                    Ok(_) => changed.store(true, Ordering::SeqCst),
                    Err(e) => println!("[warn] Watch error: `{e:?}`."),
                }
            },
            Default::default(),
        )
        .unwrap();

        watcher.watch(wasm_path.as_ref(), RecursiveMode::NonRecursive).unwrap();

        let wasm_path = wasm_path.as_ref();

        Self {
            engine,
            linker,
            component,
            wasm_path: wasm_path.to_owned(),
            changed,
            _watcher: watcher,
        }
    }

    pub fn new_shape_if_wasm_changed(&mut self) -> Option<Result<Shape>> {
        if self.changed.swap(false, Ordering::SeqCst) {
            let component_bytes = convert_to_component(&self.wasm_path);
            self.component = Component::from_binary(&self.engine, &component_bytes).unwrap();

            Some(self.shape())
        } else {
            None
        }
    }

    pub fn shape(&self) -> Result<Shape> {
        let mut store = Store::new(&self.engine, ModelHost::new());

        let (bindings, _) = ModelWorld::instantiate(&mut store, &self.component, &self.linker)?;

        bindings.call_init_model(&mut store)?;
        let shape = bindings.call_run(&mut store)?;

        let mut data = store.into_data();
        Ok(data.shapes.delete(shape)?)
    }
}
