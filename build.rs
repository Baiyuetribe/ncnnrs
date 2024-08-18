use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let ncnn_include_dir = env::var("NCNN_INCLUDE_DIR")
        .map(|dir| PathBuf::from(dir))
        .expect(
            "ERROR: please set NCNN_INCLUDE_DIR,e.g. export NCNN_INCLUDE_DIR=/path/to/ncnn/include",
        );
    if !ncnn_include_dir.join("c_api.h").exists() {
        panic!(
            "ERROR: please set NCNN_INCLUDE_DIR,e.g2. export NCNN_INCLUDE_DIR=/path/to/ncnn/include"
        );
    }

    // println!("cargo:rerun-if-env-changed=NCNN_INCLUDE_DIR");
    let bindings = bindgen::Builder::default()
        .header(format!("{}/gpu.h", ncnn_include_dir.display())) // 启用gpu相关的函数；# 对cpu模式，包含gpu.h头文件，所以不做处理
        .header(format!("{}/c_api.h", ncnn_include_dir.display())) // 通用入口
        // .clang_arg(format!("-I{}", ncnn_include_dir.display())) // 无效
        .clang_arg("-x")
        .clang_arg("c++")
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
