//! Grid 栅格系统演示
//!
//! 展示 24 栅格系统的基本用法

use ant_design_dioxus::prelude::*;

// 引用未使用的依赖以避免编译错误
#[allow(unused_imports)]
use once_cell as _;
#[allow(unused_imports)]
use serde as _;
#[allow(unused_imports)]
use serde_json as _;
#[allow(unused_imports)]
use wasm_bindgen_test as _;
#[allow(unused_imports)]
use web_sys as _;

fn main() {
    dioxus::launch(app);
}

/// 应用主组件
fn app() -> Element {
    rsx! {
        div {
            style: "padding: 20px;",
            h1 { "Grid 栅格系统演示" }

            // 基础栅格
            Row {
                gutter: 16,
                Col {
                    span: 12,
                    div {
                        style: "background: #0092ff; color: white; text-align: center; padding: 16px 0;",
                        "col-12"
                    }
                }
                Col {
                    span: 12,
                    div {
                        style: "background: #096dd9; color: white; text-align: center; padding: 16px 0;",
                        "col-12"
                    }
                }
            }
        }
    }
}
