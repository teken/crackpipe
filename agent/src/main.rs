use wasmer::{wat2wasm, Function, Instance, Module, Store};
use wasmer_wasix::WasiEnv;

fn main() {
    let wasm_bytes = wat2wasm(
        br#"
(module
    
    (func $get_inc (import "env" "get_inc") (result i32))
    (memory (export "memory") 1)
    (type $add_one_t (func (param i32) (result i32)))
    (func $add_one_f (type $add_one_t) (param $value i32) (result i32)
        local.get $value
        (call $get_inc)
        i32.add)
    (export "add_one" (func $add_one_f)))
"#,
    )
    .unwrap();

    let mut store = Store::default();

    let module = Module::new(&store, wasm_bytes).unwrap();

    let mut wasi_env = WasiEnv::builder("add_one").finalize(&mut store).unwrap();

    let mut import_object = wasi_env
        .import_object_for_all_wasi_versions(&mut store, &module)
        .unwrap();

    import_object.define(
        "env",
        "get_inc",
        Function::new_typed(&mut store, || -> i32 {
            println!("get_inc called");
            1
        }),
    );

    let instance = Instance::new(&mut store, &module, &import_object).unwrap();

    wasi_env.initialize(&mut store, instance.clone()).unwrap();

    let add_one: wasmer::TypedFunction<i32, i32> = instance
        .exports
        .get_typed_function(&mut store, "add_one")
        .unwrap();
    let result = add_one.call(&mut store, 42).unwrap();
    println!("{} + {} = {}", 42, 1, result);
}
