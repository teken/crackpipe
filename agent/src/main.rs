use wasmer::{imports, Instance, Module, Store, Value};
use wasmer_wasix::WasiEnv;

fn main() {
    let wasm_bytes = r#"
    (module
    (type $t0 (func (param i32) (result i32)))
    (func $add_one (export "add_one") (type $t0) (param $p0 i32) (result i32)
        get_local $p0
        i32.const 1
        i32.add))
    "#;

    let mut store = Store::default();

    let module = Module::new(&store, wasm_bytes).unwrap();

    // let wasi_env = WasiState::new("test-program")
    //     .args(&["sum", "1", "2"])
    //     .finalize()
    //     .unwrap();

    // let mut wasi_thread = wasi_env.import_object(store, module).new_thread();

    let mut wasi_env = WasiEnv::builder("add_one").finalize(&mut store).unwrap();

    let import_object = wasi_env
        .import_object_for_all_wasi_versions(&mut store, &module)
        .unwrap();
    let instance = Instance::new(&mut store, &module, &import_object).unwrap();

    wasi_env.initialize(&mut store, instance.clone()).unwrap();

    let add_one = instance.exports.get_function("add_one").unwrap();
    let result = add_one.call(&mut store, &[Value::I32(42)]).unwrap();
    println!("{} + {} = {}", 42, 1, result[0].i32().unwrap()); //assert_eq!(, Value::I32(43));
}
