use wasmer::{Module, Store};
use wasmer_wasix::fs::InodeGuard;
use wasmer_wasix::types::wasi::{Fdflags, Rights};
use wasmer_wasix::{WasiEnv, WasiFs, WasiInodes};

use std::fs::{create_dir, read, write};

// This is currently very broken
fn setup_fs(nodes: &WasiInodes, wasi_fs: &mut WasiFs) -> Result<(), String> {
    wasi_fs.create_fd(
        Rights.all(),
        Rights.all(),
        Fdflags.all(),
        0,
        InodeGuard { "test" },
    )
}

fn main() {
    // Need to open wasm-compiled loader binary?

    let loader_bin = read("loader.wasm").unwrap();
    // only for converting wasm text format
    // let wasm_bytes = wat2wasm(b"").unwrap();

    let mut store = Store::default();

    let module = Module::new(&store, loader_bin).unwrap();

    _ = create_dir("test");
    _ = write("test/file.txt", "crackpipe");

    let preopen_dir = "test".to_string();

    let mut wasi_env_builder = WasiEnv::builder("crackpipe-packager");
    wasi_env_builder.add_map_dir(&"test", preopen_dir).unwrap();

    let _ = wasi_env_builder.run_with_store(module, &mut store).unwrap();

    // let import_object = wasi_env
    //     .import_object_for_all_wasi_versions(&mut store, &module)
    //     .unwrap();
    //
    // let instance = Instance::new(&mut store, &module, &import_object).unwrap();
    //
    // wasi_env.initialize(&mut store, instance.clone()).unwrap();
}
