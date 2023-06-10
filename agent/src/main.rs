use wasmer::{wat2wasm, Module, Store};
use wasmer_wasix::WasiEnv;

fn main() {
    let wasm_bytes = wat2wasm(
        br#"
(module
    (memory (export "memory") 1)
  (type $add_one_t (func (param i32) (result i32)))
  (func $add_one_f (type $add_one_t) (param $value i32) (result i32)
    local.get $value
    i32.const 1
    i32.add)
  (export "add_one" (func $add_one_f)))
"#,
    )
    .unwrap();

    let mut store = Store::default();

    let module = Module::new(&store, wasm_bytes).unwrap();

    let (instance, _) = WasiEnv::builder("add_one")
        .instantiate(module, &mut store)
        .unwrap();

    let add_one: wasmer::TypedFunction<i32, i32> = instance
        .exports
        .get_typed_function(&mut store, "add_one")
        .unwrap();
    let result = add_one.call(&mut store, 42).unwrap();
    println!("{} + {} = {}", 42, 1, result);
}
