use std::path::Path;

fn main() {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));

    let header_path = crate_root.join("wrapper.h");

    let bindings = bindgen::Builder::default()
        .header(header_path.to_string_lossy())
        .clang_arg(&format!(
            "-I{}",
            crate_root.join("../vendor/ADLX/SDK/Include/").display()
        ))
        .allowlist_item("I?ADLX\\w+")
        .generate()
        .expect("failed to generate adlx bindings");

    bindings
        .write_to_file(crate_root.join("../src/ffi.rs"))
        .expect("Couldn't write bindings!");
}
