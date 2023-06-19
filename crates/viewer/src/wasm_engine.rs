use glam::dvec3;
use opencascade::primitives::{Edge, Vertex};
use std::path::Path;
use wasmtime::{Caller, Engine, Linker, Module, Store};

#[derive(Default)]
pub struct WasmEngine {
    engine: Engine,
    linker: Linker<EngineData>,
}

impl WasmEngine {
    pub fn new() -> Self {
        let engine = Engine::default();
        let mut linker = Linker::new(&engine);

        linker
            .func_wrap(
                "env",
                "new_vertex",
                |mut caller: Caller<'_, EngineData>, x: f64, y: f64, z: f64| -> u32 {
                    let data = caller.data_mut();
                    data.new_vertex(x, y, z)
                },
            )
            .unwrap();

        linker
            .func_wrap(
                "env",
                "new_segment",
                |mut caller: Caller<'_, EngineData>,
                 x1: f64,
                 y1: f64,
                 z1: f64,
                 x2: f64,
                 y2: f64,
                 z2: f64|
                 -> u32 {
                    let data = caller.data_mut();
                    data.new_segment(x1, y1, z1, x2, y2, z2)
                },
            )
            .unwrap();

        Self { engine, linker }
    }
}

impl WasmEngine {
    pub fn execute_model_wasm(&mut self, wasm_path: impl AsRef<Path>) -> EngineData {
        let module =
            Module::from_file(&self.engine, wasm_path).expect("Couldn't load the WASM module");
        let mut store = Store::<EngineData>::new(&self.engine, EngineData::default());

        let instance =
            self.linker.instantiate(&mut store, &module).expect("Failed to create WASM instance");
        let model = instance
            .get_typed_func::<(), ()>(&mut store, "model")
            .expect("Failed to get 'model' function");

        model.call(&mut store, ()).expect("Failed to call the WASM model() function");

        store.into_data()
    }
}

#[derive(Default)]
pub struct EngineData {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

impl EngineData {
    #[allow(unused)]
    fn debug_print(&self) {
        for _v in &self.vertices {
            println!("got a vertex");
        }
    }

    fn new_vertex(&mut self, x: f64, y: f64, z: f64) -> u32 {
        let vertex_id = self.vertices.len();
        self.vertices.push(Vertex::new(dvec3(x, y, z)));

        vertex_id as u32
    }

    fn new_segment(&mut self, x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> u32 {
        let edge_id = self.vertices.len();
        self.edges.push(Edge::segment(dvec3(x1, y1, z1), dvec3(x2, y2, z2)));

        edge_id as u32
    }

    pub fn edges(&self) -> impl Iterator<Item = &Edge> {
        self.edges.iter()
    }
}
