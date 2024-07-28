use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let header_path = env::var("NCNN_INCLUDE_DIR")
        .map(|dir| PathBuf::from(dir).join("c_api.h"))
        .expect(
            "ERROR: please set NCNN_INCLUDE_DIR,e.g. export NCNN_INCLUDE_DIR=/path/to/ncnn/include",
        );
    if !header_path.exists() {
        panic!(
            "ERROR: please set NCNN_INCLUDE_DIR,e.g. export NCNN_INCLUDE_DIR=/path/to/ncnn/include"
        );
    }

    // println!("cargo:rerun-if-env-changed=NCNN_INCLUDE_DIR");
    let bindings = bindgen::Builder::default()
        .header(header_path.to_str().unwrap())
        .allowlist_type("regex")
        .allowlist_function("ncnn.*")
        .allowlist_var("NCNN.*")
        .allowlist_type("ncnn.*")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
