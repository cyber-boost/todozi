#[cfg(feature = "c")]
use std::env;
#[cfg(feature = "c")]
use cbindgen;

fn main() {
    #[cfg(feature = "c")]
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    #[cfg(feature = "c")]
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(cbindgen::Config::from_file("cbindgen.toml").unwrap())
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("bindings/todozi_ffi.h");
}