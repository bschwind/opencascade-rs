#include <GProp_GProps.hxx>
#include <bindings_common.hxx>

inline std::unique_ptr<gp_Pnt> GProp_GProps_CentreOfMass(const GProp_GProps &props) {
  return std::unique_ptr<gp_Pnt>(new gp_Pnt(props.CentreOfMass()));
}
