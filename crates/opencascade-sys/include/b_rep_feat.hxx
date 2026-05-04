#include <BRepFeat_MakeCylindricalHole.hxx>
#include <BRepFeat_MakeDPrism.hxx>
#include <bindings_common.hxx>

inline std::unique_ptr<BRepFeat_MakeCylindricalHole> BRepFeat_MakeCylindricalHole_ctor() {
  return std::unique_ptr<BRepFeat_MakeCylindricalHole>(new BRepFeat_MakeCylindricalHole());
}
