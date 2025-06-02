//! Build script for ant-design-dioxus
//!
//! 本构建脚本负责配置CSS-in-Rust的编译时优化和开发时热重载功能。
//! 根据编译环境自动选择最佳配置。

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // 获取编译配置
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let target = env::var("TARGET").unwrap_or_else(|_| "unknown".to_string());

    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=build.rs");

    // 配置CSS-in-Rust特性
    configure_css_in_rust(&profile, &target);

    // 生成样式缓存
    generate_style_cache();

    // 配置WASM优化
    if target.contains("wasm32") {
        configure_wasm_optimization(&profile);
    }

    // 开发环境配置
    if profile == "debug" {
        configure_development_features();
    }

    println!("cargo:rustc-env=BUILD_PROFILE={}", profile);
    println!("cargo:rustc-env=BUILD_TARGET={}", target);
}

/// 配置CSS-in-Rust特性
fn configure_css_in_rust(profile: &str, target: &str) {
    match profile {
        "release" => {
            // 生产环境：启用编译时优化
            println!("cargo:rustc-cfg=css_compile_time_optimization");
            println!("cargo:rustc-cfg=css_minification");
            println!("cargo:rustc-cfg=css_dead_code_elimination");

            // WASM目标额外优化
            if target.contains("wasm32") {
                println!("cargo:rustc-cfg=css_wasm_optimization");
                println!("cargo:rustc-cfg=css_inline_critical");
            }
        }
        _ => {
            // 开发环境：启用热重载和调试功能
            println!("cargo:rustc-cfg=css_hot_reload");
            println!("cargo:rustc-cfg=css_source_maps");
            println!("cargo:rustc-cfg=css_debug_info");
            println!("cargo:rustc-cfg=css_live_editing");
        }
    }

    // 通用特性
    println!("cargo:rustc-cfg=css_autoprefixer");
    println!("cargo:rustc-cfg=css_variables_support");
    println!("cargo:rustc-cfg=css_nesting_support");
}

/// 生成样式缓存
fn generate_style_cache() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let cache_path = Path::new(&out_dir).join("style_cache.rs");

    let cache_content = r#"
// 自动生成的样式缓存文件
// 此文件在构建时生成，请勿手动编辑

use std::collections::HashMap;
use once_cell::sync::Lazy;

/// 全局样式缓存
pub static STYLE_CACHE: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut cache = HashMap::new();

    // 基础组件样式缓存
    cache.insert("button_primary", include_str!("../src/styles/button.rs"));
    cache.insert("input_default", include_str!("../src/styles/input.rs"));
    cache.insert("card_default", include_str!("../src/styles/card.rs"));
    cache.insert("table_default", include_str!("../src/styles/table.rs"));

    // 主题相关缓存
    cache.insert("tokens_light", include_str!("../src/styles/tokens.rs"));
    cache.insert("mixins_common", include_str!("../src/styles/mixins.rs"));

    cache
});

/// 获取缓存的样式
pub fn get_cached_style(key: &str) -> Option<&'static str> {
    STYLE_CACHE.get(key).copied()
}

/// 样式缓存统计信息
pub fn cache_stats() -> (usize, usize) {
    let total_entries = STYLE_CACHE.len();
    let total_size: usize = STYLE_CACHE.values().map(|s| s.len()).sum();
    (total_entries, total_size)
}
"#;

    if let Err(e) = fs::write(&cache_path, cache_content) {
        println!("cargo:warning=Failed to generate style cache: {}", e);
    }
}

/// 配置WASM优化
fn configure_wasm_optimization(profile: &str) {
    if profile == "release" {
        // 启用WASM特定优化
        println!("cargo:rustc-cfg=wasm_size_optimization");
        println!("cargo:rustc-cfg=wasm_speed_optimization");

        // 配置链接器优化
        println!("cargo:rustc-link-arg=--gc-sections");
        println!("cargo:rustc-link-arg=--strip-debug");
    }
}

/// 配置开发环境特性
fn configure_development_features() {
    // 启用开发时有用的特性
    println!("cargo:rustc-cfg=dev_hot_reload");
    println!("cargo:rustc-cfg=dev_style_inspector");
    println!("cargo:rustc-cfg=dev_component_outline");

    // 检查是否有样式文件变更
    check_style_file_changes();
}

/// 检查样式文件变更
fn check_style_file_changes() {
    let style_dirs = ["src/styles", "src/theme", "src/components"];

    for dir in &style_dirs {
        if Path::new(dir).exists() {
            println!("cargo:rerun-if-changed={}", dir);

            // 递归监听目录下的所有文件
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    if let Ok(path) = entry.path().canonicalize() {
                        if path.extension().map_or(false, |ext| ext == "rs") {
                            println!("cargo:rerun-if-changed={}", path.display());
                        }
                    }
                }
            }
        }
    }
}

/// 验证CSS-in-Rust配置
#[allow(dead_code)]
fn validate_css_config() -> Result<(), Box<dyn std::error::Error>> {
    // 检查必要的依赖
    let required_features = ["css-in-rust-macros", "dioxus", "web-sys"];

    for feature in &required_features {
        if env::var(format!(
            "CARGO_FEATURE_{}",
            feature.to_uppercase().replace('-', "_")
        ))
        .is_err()
        {
            println!("cargo:warning=Missing required feature: {}", feature);
        }
    }

    Ok(())
}

/// 生成构建信息
#[allow(dead_code)]
fn generate_build_info() {
    use std::process::Command;

    // 使用系统时间而不是 chrono
    let build_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // 尝试获取 git hash
    let git_hash = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok()
            } else {
                None
            }
        })
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    println!("cargo:rustc-env=BUILD_TIME={}", build_time);
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
}
