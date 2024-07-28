fn main() {
    println!("cargo:rustc-link-lib=dylib=c++");
    println!("cargo:rustc-link-search=native=/Users/baiyue/arch/auto_action/pkg/ncnn/arm64/lib");
    println!("cargo:rustc-link-lib=static=ncnn");
    // println!("cargo:rustc-link-lib=static=GenericCodeGen");
    // println!("cargo:rustc-link-lib=static=glslang");
    println!("cargo:rustc-link-lib=static=MachineIndependent"); // 必须
                                                                // println!("cargo:rustc-link-lib=static=OGLCompiler"); // 静态库
    println!("cargo:rustc-link-lib=static=OSDependent"); // 必须
    println!("cargo:rustc-link-lib=static=SPIRV"); // 必须
}
