pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/b_rep.hxx");

        type TopoDS_Builder = crate::topo_ds::TopoDS_Builder;
        type gp_Pnt = crate::gp::gp_Pnt;
        type TopoDS_Face = crate::topo_ds::TopoDS_Face;
        type TopoDS_Edge = crate::topo_ds::TopoDS_Edge;
        type TopoDS_Vertex = crate::topo_ds::TopoDS_Vertex;
        type Handle_Geom_Surface = crate::geom::Handle_Geom_Surface;
        type Handle_Geom_Curve = crate::geom::Handle_Geom_Curve;
        type Handle_Poly_Triangulation = crate::poly::Handle_Poly_Triangulation;
        type TopLoc_Location = crate::top_loc::TopLoc_Location;

        type BRep_Builder;
        #[cxx_name = "construct_unique"]
        pub fn BRep_Builder_new() -> UniquePtr<BRep_Builder>;
        pub fn BRep_Builder_upcast_to_topods_builder(builder: &BRep_Builder) -> &TopoDS_Builder;

        type BRep_Tool;
        pub fn BRep_Tool_Surface(face: &TopoDS_Face) -> UniquePtr<Handle_Geom_Surface>;
        pub fn BRep_Tool_Curve(
            edge: &TopoDS_Edge,
            first: &mut f64,
            last: &mut f64,
        ) -> UniquePtr<Handle_Geom_Curve>;
        pub fn BRep_Tool_Pnt(vertex: &TopoDS_Vertex) -> UniquePtr<gp_Pnt>;
        pub fn BRep_Tool_Triangulation(
            face: &TopoDS_Face,
            location: Pin<&mut TopLoc_Location>,
        ) -> UniquePtr<Handle_Poly_Triangulation>;
    }
}
