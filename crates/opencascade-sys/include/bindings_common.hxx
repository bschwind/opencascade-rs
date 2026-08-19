#pragma once
#include "rust/cxx.h"
#include <NCollection_List.hxx>
#include <Standard_Handle.hxx>
#include <memory>

class Geom2d_Curve;
class Geom2d_Ellipse;
class Geom2d_TrimmedCurve;
class Geom_BSplineCurve;
class Geom_BezierCurve;
class Geom_BezierSurface;
class Geom_Curve;
class Geom_CylindricalSurface;
class Geom_Plane;
class Geom_Surface;
class Geom_TrimmedCurve;
class Law_Function;
class Poly_Triangulation;
class Standard_Type;
class TColgp_HArray1OfPnt;
class TopTools_HSequenceOfShape;

using RustHandle_Geom2d_Curve = opencascade::handle<Geom2d_Curve>;
using RustHandle_Geom2d_Ellipse = opencascade::handle<Geom2d_Ellipse>;
using RustHandle_Geom2d_TrimmedCurve = opencascade::handle<Geom2d_TrimmedCurve>;
using RustHandle_Geom_BSplineCurve = opencascade::handle<Geom_BSplineCurve>;
using RustHandle_Geom_BezierCurve = opencascade::handle<Geom_BezierCurve>;
using RustHandle_Geom_BezierSurface = opencascade::handle<Geom_BezierSurface>;
using RustHandle_Geom_Curve = opencascade::handle<Geom_Curve>;
using RustHandle_Geom_CylindricalSurface = opencascade::handle<Geom_CylindricalSurface>;
using RustHandle_Geom_Plane = opencascade::handle<Geom_Plane>;
using RustHandle_Geom_Surface = opencascade::handle<Geom_Surface>;
using RustHandle_Geom_TrimmedCurve = opencascade::handle<Geom_TrimmedCurve>;
using RustHandle_Law_Function = opencascade::handle<Law_Function>;
using RustHandle_Poly_Triangulation = opencascade::handle<Poly_Triangulation>;
using RustHandle_Standard_Type = opencascade::handle<Standard_Type>;
using RustHandle_TColgp_HArray1OfPnt = opencascade::handle<TColgp_HArray1OfPnt>;
using RustHandle_TopTools_HSequenceOfShape = opencascade::handle<TopTools_HSequenceOfShape>;

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
