#include <Poly_Connect.hxx>
#include <Poly_Triangulation.hxx>
#include <bindings_common.hxx>

inline std::unique_ptr<Handle_Poly_Triangulation>
Handle_Poly_Triangulation_new(std::unique_ptr<Poly_Triangulation> triangulation) {
  return std::unique_ptr<Handle_Poly_Triangulation>(new Handle_Poly_Triangulation(triangulation.release()));
}

inline std::unique_ptr<gp_Dir> Poly_Triangulation_Normal(const Poly_Triangulation &triangulation,
                                                         const Standard_Integer index) {
  return std::unique_ptr<gp_Dir>(new gp_Dir(triangulation.Normal(index)));
}

inline std::unique_ptr<gp_Pnt> Poly_Triangulation_Node(const Poly_Triangulation &triangulation,
                                                       const Standard_Integer index) {
  return std::unique_ptr<gp_Pnt>(new gp_Pnt(triangulation.Node(index)));
}

inline std::unique_ptr<gp_Pnt2d> Poly_Triangulation_UV(const Poly_Triangulation &triangulation,
                                                       const Standard_Integer index) {
  return std::unique_ptr<gp_Pnt2d>(new gp_Pnt2d(triangulation.UVNode(index)));
}
