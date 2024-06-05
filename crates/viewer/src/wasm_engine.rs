use glam::DVec3;
use opencascade::primitives::{Compound, Edge, Face, Shape, Shell, Solid, Wire, WireBuilder};
use wasmtime::{
    component::{Component, Linker, Resource, ResourceTable},
    Config, Engine, Store,
};

wasmtime::component::bindgen!({
    path: "../model-api/wit",
    with: {
        "wasm-wire-builder": MyWireBuilder,
        "wasm-edge": MyEdge,
        "wasm-wire": MyWire,
        "wasm-face": MyFace,
        "wasm-shell": MyShell,
        "wasm-solid": MySolid,
        "wasm-compound": MyCompound,
        "wasm-shape": MyShape,
    },
});

pub struct MyWireBuilder {
    builder: WireBuilder,
}

pub struct MyEdge {
    edge: Edge,
}

pub struct MyWire {
    wire: Wire,
}

pub struct MyFace {
    face: Face,
}

pub struct MyShell {
    shell: Shell,
}

pub struct MySolid {
    shell: Solid,
}

pub struct MyCompound {
    compound: Compound,
}

pub struct MyShape {
    shape: Shape,
}

impl From<Point3> for DVec3 {
    fn from(p: Point3) -> Self {
        Self { x: p.x, y: p.y, z: p.z }
    }
}

struct ModelHost {
    wire_builders: ResourceTable,
    edges: ResourceTable,
    wires: ResourceTable,
    faces: ResourceTable,
    shells: ResourceTable,
    solids: ResourceTable,
    compounds: ResourceTable,
    shapes: ResourceTable,
}

impl ModelHost {
    fn new() -> Self {
        Self {
            wire_builders: ResourceTable::new(),
            edges: ResourceTable::new(),
            wires: ResourceTable::new(),
            faces: ResourceTable::new(),
            shells: ResourceTable::new(),
            solids: ResourceTable::new(),
            compounds: ResourceTable::new(),
            shapes: ResourceTable::new(),
        }
    }
}

impl HostWasmEdge for ModelHost {
    fn segment(&mut self, p1: Point3, p2: Point3) -> Result<Resource<MyEdge>, anyhow::Error> {
        Ok(self.edges.push(MyEdge { edge: Edge::segment(p1.into(), p2.into()) })?)
    }

    fn drop(&mut self, resource: Resource<MyEdge>) -> Result<(), anyhow::Error> {
        let _ = self.wire_builders.delete(resource);
        Ok(())
    }
}

impl HostWasmWireBuilder for ModelHost {
    fn new(&mut self) -> Result<Resource<MyWireBuilder>, anyhow::Error> {
        Ok(self.wire_builders.push(MyWireBuilder { builder: WireBuilder::new() })?)
    }

    fn add_edge(
        &mut self,
        builder_resource: Resource<MyWireBuilder>,
        edge_resource: Resource<MyEdge>,
    ) -> Result<(), anyhow::Error> {
        let builder = &mut self.wire_builders.get_mut(&builder_resource)?.builder;
        let edge = self.edges.get(&edge_resource)?;
        builder.add_edge(&edge.edge);

        Ok(())
    }

    fn build(
        &mut self,
        resource: Resource<MyWireBuilder>,
    ) -> Result<Resource<MyWire>, anyhow::Error> {
        let wire = self.wire_builders.delete(resource)?.builder.build();

        let new_wire = self.wires.push(MyWire { wire })?;

        Ok(new_wire)
    }

    fn drop(&mut self, resource: Resource<MyWireBuilder>) -> Result<(), anyhow::Error> {
        let _ = self.wire_builders.delete(resource);
        Ok(())
    }
}

impl HostWasmWire for ModelHost {
    fn drop(&mut self, resource: Resource<MyWire>) -> Result<(), anyhow::Error> {
        let _ = self.wires.delete(resource);
        Ok(())
    }
}

impl HostWasmFace for ModelHost {
    fn drop(&mut self, resource: Resource<MyFace>) -> Result<(), anyhow::Error> {
        let _ = self.faces.delete(resource);
        Ok(())
    }
}

impl HostWasmShell for ModelHost {
    fn drop(&mut self, resource: Resource<MyShell>) -> Result<(), anyhow::Error> {
        let _ = self.shells.delete(resource);
        Ok(())
    }
}

impl HostWasmSolid for ModelHost {
    fn drop(&mut self, resource: Resource<MySolid>) -> Result<(), anyhow::Error> {
        let _ = self.solids.delete(resource);
        Ok(())
    }
}

impl HostWasmCompound for ModelHost {
    fn drop(&mut self, resource: Resource<MyCompound>) -> Result<(), anyhow::Error> {
        let _ = self.compounds.delete(resource);
        Ok(())
    }
}

impl HostWasmShape for ModelHost {
    fn drop(&mut self, resource: Resource<MyShape>) -> Result<(), anyhow::Error> {
        let _ = self.wires.delete(resource);
        Ok(())
    }

    fn from_wire(
        &mut self,
        wire_resource: Resource<MyWire>,
    ) -> Result<Resource<MyShape>, anyhow::Error> {
        let wire = self.wires.get(&wire_resource)?;
        let shape = Face::from_wire(&wire.wire).into();

        let new_shape = self.shapes.push(MyShape { shape })?;

        Ok(new_shape)
    }
}

// Imports into the world, like the `name` import for this world, are
// satisfied through traits.
impl ModelWorldImports for ModelHost {
    fn print(&mut self, _msg: String) -> Result<(), wasmtime::Error> {
        Ok(())
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
        let shape = bindings.call_run(&mut store).unwrap();

        let mut data = store.into_data();
        let shape = data.shapes.delete(shape).expect("Should have at least one shape returned");

        shape.shape
    }
}
