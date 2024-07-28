## ncnnrs

> 现有的 tpoisonooo/rust-ncnn 库，在编译阶段全量下载并编译 ncnn 库，且编译参数已固定，无法适应跨端编译需求。因此本仓库诞生。

### 特性

- 使用 rust 开发 ncnn
- 分离静态库，可满足跨端编译要求

### 使用教程

#### 第一步：设置你已经编译好的 ncnn 头文件路径；或从https://github.com/Tencent/ncnn/releases 下载已编译好的库

```bash
export NCNN_INCLUDE_DIR=/path/to/ncnn/include/ncnn
```

#### 第二步： 在你的个人项目里，build.rs 里添加如下内容

```rust
    println!("cargo:rustc-link-lib=dylib=c++");
    println!("cargo:rustc-link-search=native=/Users/baiyue/arch/auto_action/pkg/ncnn/arm64/lib");
    println!("cargo:rustc-link-lib=static=ncnn");
    // println!("cargo:rustc-link-lib=static=GenericCodeGen");
    // println!("cargo:rustc-link-lib=static=glslang");
    println!("cargo:rustc-link-lib=static=MachineIndependent"); // 必须
    println!("cargo:rustc-link-lib=static=OSDependent"); // 必须
    println!("cargo:rustc-link-lib=static=SPIRV"); // 必须

// 如果要开vulkan，添加vulkan的相关库
// println!("cargo:rustc-link-lib=static=ncnn"); // static代表链接静态库libncnn.a或ncnn.lib
// println!("cargo:rustc-link-lib=dylib=ncnn"); // static代表链接动态库libncnn.dylib或ncnn.dll
```

#### 第三步：正常构建

```bash
cargo add ncnnrs
cargo run .
```

### 演示

```bash
cd demo/get_version
cargo run .
# build size:295.73kb mac arm64
# Out: ncnn version: 1.0.20240727
```

更多演示，可复用`tpoisonooo/rust-ncnn`的相关案例。

### 跨端开发

`ncnnrs`在编译阶段仅关联 ncnn 库的头文件，不依赖 ncnn 的 lib。因此跨端时，在 build.rs 里按场景链接到对应的库即可。

### 参考

- [tpoisonooo/rust-ncnn](https://github.com/tpoisonooo/rust-ncnn)