use opencascade::geometry::Workspace;

pub fn main() {
    let mut workspace = Workspace::new();

    let extruded_box = workspace.sketch().rect(16.0, 9.0).extrude(2.0);
    workspace.add(extruded_box);

    workspace.write_stl("high_level.stl").unwrap();
}
