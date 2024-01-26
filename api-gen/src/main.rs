fn main() {
    use std::path::PathBuf;

    println!("cargo:rerun-if-changed=api-gen/src/adlx/wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("api-gen/src/adlx/wrapper.h")
        .allowlist_item("I?ADLX\\w+")
        .generate()
        .expect("failed to generate adlx bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from("api-gen/src/adlx/");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
