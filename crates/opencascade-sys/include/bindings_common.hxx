#pragma once
#include "rust/cxx.h"
#include <NCollection_List.hxx>
#include <memory>

// Generic template constructor
template <typename T, typename... Args> std::unique_ptr<T> construct_unique(Args... args) {
  return std::unique_ptr<T>(new T(args...));
}

// Type casting
template <typename T, typename U> inline U upcast(T src) { return src; }
template <typename T, typename U> inline const U &upcast_ref(const T &src) { return src; }

// Generic List
template <typename T> std::unique_ptr<std::vector<T>> list_to_vector(const NCollection_List<T> &list) {
  return std::unique_ptr<std::vector<T>>(new std::vector<T>(list.begin(), list.end()));
}

template <typename T> const T &handle_try_deref(const opencascade::handle<T> &handle) {
  if (handle.IsNull()) {
    throw std::runtime_error("null handle dereference");
  }
  return *handle;
}

#include <Standard_Failure.hxx>
#include <TopoDS_Shape.hxx>
#include <stdexcept>

// OCCT's exceptions don't derive from std::exception, the only kind cxx can catch, so a
// failed operation used to terminate the process. Shape() builds the result if needed and
// throws when that fails: rethrow as std::runtime_error so cxx can return an Err.
template <typename T> const TopoDS_Shape &checked_shape(T &builder) {
  try {
    return builder.Shape();
  } catch (const Standard_Failure &e) {
    const char *msg = e.GetMessageString();
    throw std::runtime_error(msg && *msg ? msg : e.DynamicType()->Name());
  }
}
