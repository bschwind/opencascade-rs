#include <gp_Pnt.hxx>

// Point stuff
std::unique_ptr<gp_Pnt> new_point(double x, double y, double z);

// Line segment stuff
std::unique_ptr<GC_MakeSegment> new_segment(const gp_Pnt& p1, const gp_Pnt& p2);

// Arc stuff
std::unique_ptr<GC_MakeArcOfCircle> new_arc_of_circle(const gp_Pnt& p1, const gp_Pnt& p2, const gp_Pnt& p3);
