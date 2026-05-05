#include <TColgp_Array1OfDir.hxx>
#include <TColgp_Array1OfPnt2d.hxx>
#include <TColgp_Array2OfPnt.hxx>
#include <TColgp_HArray1OfPnt.hxx>
#include <bindings_common.hxx>

inline std::unique_ptr<gp_Dir> TColgp_Array1OfDir_Value(const TColgp_Array1OfDir &array, Standard_Integer index) {
  return std::unique_ptr<gp_Dir>(new gp_Dir(array.Value(index)));
}

inline std::unique_ptr<gp_Pnt2d> TColgp_Array1OfPnt2d_Value(const TColgp_Array1OfPnt2d &array, Standard_Integer index) {
  return std::unique_ptr<gp_Pnt2d>(new gp_Pnt2d(array.Value(index)));
}

inline std::unique_ptr<gp_Pnt> TColgp_HArray1OfPnt_Value(const TColgp_HArray1OfPnt &array, Standard_Integer index) {
  return std::unique_ptr<gp_Pnt>(new gp_Pnt(array.Value(index)));
}

inline std::unique_ptr<Handle_TColgp_HArray1OfPnt>
new_HandleTColgpHArray1OfPnt_from_TColgpHArray1OfPnt(std::unique_ptr<TColgp_HArray1OfPnt> array) {
  return std::unique_ptr<Handle_TColgp_HArray1OfPnt>(new Handle_TColgp_HArray1OfPnt(array.release()));
}