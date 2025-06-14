//! Empty 空状态组件
//!
//! 空状态时的展示占位图。

use dioxus::prelude::*;

const EMPTY_STYLE: &str = include_str!("./style.css");

/// Empty 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct EmptyProps {
    /// 自定义描述内容
    #[props(default)]
    pub description: Option<String>,

    /// 自定义图片，为 string 时表示自定义图片地址
    #[props(default)]
    pub image: Option<String>,

    /// 图片样式
    #[props(default)]
    pub image_style: Option<String>,

    /// 自定义 CSS 类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 子元素（通常是操作按钮）
    children: Element,
}

/// Empty 空状态组件
///
/// # Props
/// - `description`: 自定义描述内容
/// - `image`: 自定义图片地址
/// - `image_style`: 图片样式
/// - `class`: 自定义 CSS 类名
/// - `style`: 自定义样式
/// - `children`: 子元素（通常是操作按钮）
///
/// # Example
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::Empty;
///
/// #[component]
/// fn App() -> Element {
///     rsx! {
///         Empty {
///             description: "暂无数据",
///             Button { "创建" }
///         }
///     }
/// }
/// ```
#[component]
pub fn Empty(props: EmptyProps) -> Element {
    // 构建CSS类名
    let mut class_names = vec!["ant-empty".to_string()];

    // 添加自定义类名
    if let Some(custom_class) = &props.class {
        class_names.push(custom_class.clone());
    }

    let class_str = class_names.join(" ");

    let description = props.description.unwrap_or_else(|| "暂无数据".to_string());

    let image_url = props.image.unwrap_or_else(|| {
        // 使用内置的空状态图片（SVG格式）
        "data:image/svg+xml;base64,PHN2ZyB3aWR0aD1cIjY0XCIgaGVpZ2h0PVwiNDFcIiB2aWV3Qm94PVwiMCAwIDY0IDQxXCIgeG1sbnM9XCJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2Z1wiPjxnIHRyYW5zZm9ybT1cInRyYW5zbGF0ZSgwIDEpXCIgZmlsbD1cIm5vbmVcIiBmaWxsLXJ1bGU9XCJldmVub2RkXCI+PGVsbGlwc2UgZmlsbD1cIiNmNWY1ZjVcIiBjeD1cIjMyXCIgY3k9XCIzM1wiIHJ4PVwiMzJcIiByeT1cIjdcIi8+PGcgZmlsbC1ydWxlPVwibm9uemVyb1wiIHN0cm9rZT1cIiNkOWQ5ZDlcIj48cGF0aCBkPVwiTTU1IDEyLjc2TDQ0Ljg1NCAxLjI1OEM0NC4zNjcuNDc0IDQzLjY1NiAwIDQyLjkwNyAwSDIxLjA5M2MtLjc0OSAwLTEuNDYuNDc0LTEuOTQ3IDEuMjU3TDkgMTIuNzYxVjIyaDQ2di05LjI0elwiLz48cGF0aCBkPVwiTTQxLjYxMyAxNS45MzFjMC0xLjYwNS45OTQtMi45MyAyLjIyNy0yLjkzSDU1djE4LjEzN0M1NSAzMy4yNiA1My42OCAzNSA1Mi4wNSAzNWgtNDAuMUM5LjMyIDM1IDggMzMuMjU5IDggMzEuMTM3VjEzaDExLjE2YzEuMjMzIDAgMi4yMjcgMS4zMjMgMi4yMjcgMi45Mjh2LjAyMmMwIDEuNjA1IDEuMDA1IDIuOTAxIDIuMjM3IDIuOTAxaDEzLjc1MmMxLjIzMiAwIDIuMjM3LTEuMzA4IDIuMjM3LTIuOTEzdi0uMDA3elwiIGZpbGw9XCIjZmFmYWZhXCIvPjwvZz48L2c+PC9zdmc+".to_string()
    });

    rsx! {
        style { {EMPTY_STYLE} }

        div {
            class: class_str.clone(),
            style: props.style.clone().unwrap_or_default(),

            div {
                class: "ant-empty-image",
                style: props.image_style.clone().unwrap_or_default(),
                img {
                    src: image_url.clone(),
                    alt: "empty",
                    style: "width: 64px; height: 41px;"
                }
            }

            if !description.is_empty() {
                div {
                    class: "ant-empty-description",
                    {description.clone()}
                }
            }

            div {
                class: "ant-empty-footer",
                {props.children}
            }
        }
    }
}

// 导出组件
pub use Empty as EmptyComponent;
