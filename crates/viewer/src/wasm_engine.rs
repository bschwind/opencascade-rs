use glam::dvec3;
use opencascade::primitives::Vertex;
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

        Self { engine, linker }
    }
}

impl WasmEngine {
    pub fn execute_model_wasm(&mut self, wasm_path: impl AsRef<Path>) {
        let module =
            Module::from_file(&self.engine, wasm_path).expect("Couldn't load the WASM module");
        let mut store = Store::<EngineData>::new(&self.engine, EngineData::default());

        let instance =
            self.linker.instantiate(&mut store, &module).expect("Failed to create WASM instance");
        let model = instance
            .get_typed_func::<(), ()>(&mut store, "model")
            .expect("Failed to get 'model' function");

        model.call(&mut store, ()).expect("Failed to call the WASM model() function");

        store.data().debug_print();
    }
}

#[derive(Default)]
struct EngineData {
    vertices: Vec<Vertex>,
}

impl EngineData {
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
}
