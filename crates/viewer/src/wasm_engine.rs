use glam::DVec3;
use opencascade::primitives::{Edge, Shape, Wire, WireBuilder};
use wasmtime::{
    component::{bindgen, Component, Linker},
    Config, Engine, Store,
};

bindgen!("model-world" in "../model-api/wit");

impl From<Point3> for DVec3 {
    fn from(p: Point3) -> Self {
        Self { x: p.x, y: p.y, z: p.z }
    }
}

struct ModelHost {
    edges: Vec<Edge>,
    wires: Vec<Wire>,
    wire_builders: Vec<WireBuilder>,
}

impl ModelHost {
    fn new() -> Self {
        Self { edges: Vec::new(), wires: Vec::new(), wire_builders: Vec::new() }
    }
}

// Imports into the world, like the `name` import for this world, are
// satisfied through traits.
impl ModelWorldImports for ModelHost {
    fn print(&mut self, _msg: String) -> Result<(), wasmtime::Error> {
        Ok(())
    }

    fn new_line_segment(&mut self, p1: Point3, p2: Point3) -> Result<u64, anyhow::Error> {
        let edge_id = self.edges.len();
        self.edges.push(Edge::segment(p1.into(), p2.into()));

        Ok(edge_id as u64)
    }

    fn new_wire_builder(&mut self) -> Result<u64, anyhow::Error> {
        let builder_id = self.wire_builders.len();
        self.wire_builders.push(WireBuilder::new());

        Ok(builder_id as u64)
    }

    fn wire_builder_add_edge(
        &mut self,
        builder_id: u64,
        edge_id: u64,
    ) -> Result<(), anyhow::Error> {
        let builder = &mut self.wire_builders[builder_id as usize];
        builder.add_edge(&self.edges[edge_id as usize]);
        Ok(())
    }

    fn wire_builder_build(&mut self, builder_id: u64) -> Result<u64, anyhow::Error> {
        let builder = self.wire_builders.remove(builder_id as usize);
        let wire = builder.build();

        let wire_id = self.wires.len();
        self.wires.push(wire);
        Ok(wire_id as u64)
    }
}

pub struct WasmEngine {
    engine: Engine,
    linker: Linker<ModelHost>,
    component: Component,
}

impl WasmEngine {
    pub fn new() -> Self {
        let mut config = Config::new();
        config.wasm_component_model(true);
        let engine = Engine::new(&config).unwrap();

        let component = Component::from_file(&engine, "./my-component.wasm").unwrap();

        let mut linker = Linker::new(&engine);
        ModelWorld::add_to_linker(&mut linker, |state: &mut ModelHost| state).unwrap();

        Self { engine, linker, component }
    }

    pub fn shape(&self) -> Shape {
        let mut store = Store::new(&self.engine, ModelHost::new());

        let (bindings, _) =
            ModelWorld::instantiate(&mut store, &self.component, &self.linker).unwrap();

        bindings.call_init_model(&mut store).unwrap();
        bindings.call_run(&mut store).unwrap();

        let mut data = store.into_data();
        let last_wire = data.wires.remove(data.wires.len() - 1);

        last_wire.to_face().into()
    }
}
