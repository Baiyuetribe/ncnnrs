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

### 通用示例

```rust
use ncnnrs::{Mat, Net, Option};

fn main() {
    let mut opt = Option::new();
    opt.set_num_threads(4);
    opt.set_vulkan_compute(true);

    let mut net = Net::new();
    net.set_option(&opt); // 设置参数
                          // 加载模型
    net.load_param("xxx.param");
    net.load_model("xxx.bin");

    // 推理
    let mut in0 = Mat::new_3d(224, 224, 3, None);
    let mut out = Mat::new();
    let mut ex = net.create_extractor();
    ex.input("in0", &mut in0);
    ex.extract("out0", &mut out);
    println!("{:?}", out);
}
```

更多演示，可复用`tpoisonooo/rust-ncnn`的相关案例。

### 跨端开发

`ncnnrs`在编译阶段仅关联 ncnn 库的头文件，不依赖 ncnn 的 lib。因此跨端时，在 build.rs 里按场景链接到对应的库即可。

### 仅 CPU 绑定

默认开启 vulkan 加速，如果要单纯使用 cpu 库。只需添加`features = ["cpu"]`。
示例

```toml
# 全局启用
ncnnrs = { version = "*", features = ["cpu"] }
# 或特殊平台启用，比如linux arm64不支持GPU加速时启用
[target.'cfg(all(target_os = "linux",target_arch = "aarch64"))'.dependencies]
ncnnrs = { version = "*", features = ["cpu"] }
```

### 参考

- [tpoisonooo/rust-ncnn](https://github.com/tpoisonooo/rust-ncnn)
