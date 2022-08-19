#include <gp_Pnt.hxx>

std::unique_ptr<gp_Pnt> make_gp_Pnt() {
  return std::unique_ptr<gp_Pnt>(new gp_Pnt(1.0, 7.0, -23.0));
}
