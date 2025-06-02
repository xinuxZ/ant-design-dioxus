//! 构建脚本 - 启用 css-in-rust 编译时优化和热重载功能
//!
//! 该脚本在编译时运行，用于配置 css-in-rust 的各种特性

use std::env;

/// 主构建函数
///
/// 根据编译环境配置不同的 css-in-rust 特性：
/// - 生产环境：启用编译时优化
/// - 开发环境：启用热重载功能
fn main() {
    // 启用 css-in-rust 编译时优化
    // 这将在编译时处理和优化 CSS，提高运行时性能
    println!("cargo:rustc-cfg=css_in_rust_compile_time");

    // 检查编译配置文件，在开发模式下启用热重载
    let profile = env::var("PROFILE").unwrap_or_default();
    if profile == "dev" || profile.contains("dev") {
        println!("cargo:rustc-cfg=css_in_rust_hot_reload");
        println!("cargo:warning=CSS-in-Rust hot reload enabled for development");
    }

    // 输出构建信息
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=PROFILE");
}
