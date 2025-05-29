//! Skeleton 骨架屏组件
//!
//! 在需要等待加载内容的位置提供一个占位图形组合。
//!
//! ## 何时使用
//!
//! - 网络较慢，需要长时间等待加载处理的情况下。
//! - 图文信息内容较多的列表/卡片中。
//! - 只在第一次加载数据的时候使用。
//! - 可以被 Spin 完全代替，但是在可用的场景下可以比 Spin 提供更好的视觉效果和用户体验。

use dioxus::prelude::*;

/// Skeleton 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct SkeletonProps {
    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 是否展示动画效果
    #[props(default = true)]
    pub active: bool,

    /// 是否显示头像占位图
    #[props(default = false)]
    pub avatar: bool,

    /// 头像占位图配置
    #[props(default)]
    pub avatar_props: Option<SkeletonAvatarProps>,

    /// 当为 true 时，显示占位图。当为 false 时，显示子组件
    #[props(default = true)]
    pub loading: bool,

    /// 是否显示段落占位图
    #[props(default = true)]
    pub paragraph: bool,

    /// 段落占位图配置
    #[props(default)]
    pub paragraph_props: Option<SkeletonParagraphProps>,

    /// 是否显示标题占位图
    #[props(default = true)]
    pub title: bool,

    /// 标题占位图配置
    #[props(default)]
    pub title_props: Option<SkeletonTitleProps>,

    /// 为按钮、头像等元素设置圆角
    #[props(default = false)]
    pub round: bool,

    /// 子组件
    #[props(default)]
    pub children: Option<Element>,
}

/// 头像占位图配置
#[derive(Clone, PartialEq, Debug)]
pub struct SkeletonAvatarProps {
    /// 是否展示动画效果
    pub active: Option<bool>,
    /// 设置头像占位图的大小
    pub size: Option<SkeletonAvatarSize>,
    /// 指定头像的形状
    pub shape: Option<SkeletonAvatarShape>,
}

/// 段落占位图配置
#[derive(Clone, PartialEq, Debug)]
pub struct SkeletonParagraphProps {
    /// 设置段落占位图的行数
    pub rows: Option<usize>,
    /// 设置段落占位图的宽度，若为数组时则为对应的每行宽度，反之则是最后一行的宽度
    pub width: Option<SkeletonWidth>,
}

/// 标题占位图配置
#[derive(Clone, PartialEq, Debug)]
pub struct SkeletonTitleProps {
    /// 设置标题占位图的宽度
    pub width: Option<SkeletonWidth>,
}

/// 头像大小
#[derive(Clone, PartialEq, Debug)]
pub enum SkeletonAvatarSize {
    Large,
    Small,
    Default,
    Custom(u32),
}

/// 头像形状
#[derive(Clone, PartialEq, Debug)]
pub enum SkeletonAvatarShape {
    Circle,
    Square,
}

/// 宽度配置
#[derive(Clone, PartialEq, Debug)]
pub enum SkeletonWidth {
    Percentage(u32),
    Pixel(u32),
    Array(Vec<SkeletonWidth>),
}

/// Skeleton 组件
#[component]
pub fn Skeleton(props: SkeletonProps) -> Element {
    let class_name = format!(
        "ant-skeleton{}{}{}",
        if props.active {
            " ant-skeleton-active"
        } else {
            ""
        },
        if props.round {
            " ant-skeleton-round"
        } else {
            ""
        },
        if let Some(class) = &props.class {
            format!(" {}", class)
        } else {
            String::new()
        }
    );

    // 如果不是加载状态，直接显示子组件
    if !props.loading {
        return rsx! {
            {props.children}
        };
    }

    rsx! {
        div {
            class: "{class_name}",
            style: "{props.style.clone().unwrap_or_default()}",

            // 头像占位图
            if props.avatar {
                {render_avatar(&props.avatar_props, props.active, props.round)}
            }

            div {
                class: "ant-skeleton-content",

                // 标题占位图
                if props.title {
                    {render_title(&props.title_props, props.active, props.round)}
                }

                // 段落占位图
                if props.paragraph {
                    {render_paragraph(&props.paragraph_props, props.active, props.round)}
                }
            }
        }
    }
}

/// 渲染头像占位图
fn render_avatar(
    avatar_props: &Option<SkeletonAvatarProps>,
    _active: bool,
    _round: bool,
) -> Element {
    let size_class = if let Some(props) = avatar_props {
        match &props.size {
            Some(SkeletonAvatarSize::Large) => " ant-skeleton-avatar-lg",
            Some(SkeletonAvatarSize::Small) => " ant-skeleton-avatar-sm",
            _ => "",
        }
    } else {
        ""
    };

    let shape_class = if let Some(props) = avatar_props {
        match &props.shape {
            Some(SkeletonAvatarShape::Square) => " ant-skeleton-avatar-square",
            _ => " ant-skeleton-avatar-circle",
        }
    } else {
        " ant-skeleton-avatar-circle"
    };

    let class_name = format!("ant-skeleton-header{}{}", size_class, shape_class);

    rsx! {
        div {
            class: "{class_name}",
            span {
                class: "ant-skeleton-avatar",
            }
        }
    }
}

/// 渲染标题占位图
fn render_title(title_props: &Option<SkeletonTitleProps>, _active: bool, _round: bool) -> Element {
    let width_style = if let Some(props) = title_props {
        if let Some(width) = &props.width {
            format!("width: {};", width_to_string(width))
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    rsx! {
        h3 {
            class: "ant-skeleton-title",
            style: "{width_style}",
        }
    }
}

/// 渲染段落占位图
fn render_paragraph(
    paragraph_props: &Option<SkeletonParagraphProps>,
    _active: bool,
    _round: bool,
) -> Element {
    let rows = if let Some(props) = paragraph_props {
        props.rows.unwrap_or(3)
    } else {
        3
    };

    let width = if let Some(props) = paragraph_props {
        &props.width
    } else {
        &None
    };

    rsx! {
        ul {
            class: "ant-skeleton-paragraph",

            for i in 0..rows {
                li {
                    key: "{i}",
                    style: "{get_paragraph_line_style(width, i, rows)}",
                }
            }
        }
    }
}

/// 获取段落行样式
fn get_paragraph_line_style(width: &Option<SkeletonWidth>, index: usize, total: usize) -> String {
    match width {
        Some(SkeletonWidth::Array(widths)) => {
            if index < widths.len() {
                format!("width: {};", width_to_string(&widths[index]))
            } else {
                String::new()
            }
        }
        Some(width) if index == total - 1 => {
            format!("width: {};", width_to_string(width))
        }
        _ => String::new(),
    }
}

/// 将宽度转换为字符串
fn width_to_string(width: &SkeletonWidth) -> String {
    match width {
        SkeletonWidth::Percentage(p) => format!("{}%", p),
        SkeletonWidth::Pixel(p) => format!("{}px", p),
        SkeletonWidth::Array(_) => String::new(), // 数组类型不应该在这里处理
    }
}

/// 按钮骨架屏
#[derive(Props, Clone, PartialEq)]
pub struct SkeletonButtonProps {
    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 是否展示动画效果
    #[props(default = true)]
    pub active: bool,

    /// 设置按钮的大小
    #[props(default)]
    pub size: Option<SkeletonButtonSize>,

    /// 指定按钮的形状
    #[props(default)]
    pub shape: Option<SkeletonButtonShape>,

    /// 设置按钮的宽度
    #[props(default)]
    pub block: bool,
}

/// 按钮大小
#[derive(Clone, PartialEq, Debug)]
pub enum SkeletonButtonSize {
    Large,
    Small,
    Default,
}

/// 按钮形状
#[derive(Clone, PartialEq, Debug)]
pub enum SkeletonButtonShape {
    Circle,
    Round,
    Default,
}

/// 按钮骨架屏组件
#[component]
pub fn SkeletonButton(props: SkeletonButtonProps) -> Element {
    let size_class = match &props.size {
        Some(SkeletonButtonSize::Large) => " ant-skeleton-button-lg",
        Some(SkeletonButtonSize::Small) => " ant-skeleton-button-sm",
        _ => "",
    };

    let shape_class = match &props.shape {
        Some(SkeletonButtonShape::Circle) => " ant-skeleton-button-circle",
        Some(SkeletonButtonShape::Round) => " ant-skeleton-button-round",
        _ => "",
    };

    let class_name = format!(
        "ant-skeleton ant-skeleton-element{}{}{}{}{}",
        if props.active {
            " ant-skeleton-active"
        } else {
            ""
        },
        size_class,
        shape_class,
        if props.block {
            " ant-skeleton-button-block"
        } else {
            ""
        },
        if let Some(class) = &props.class {
            format!(" {}", class)
        } else {
            String::new()
        }
    );

    rsx! {
        div {
            class: "{class_name}",
            style: "{props.style.clone().unwrap_or_default()}",

            div {
                class: "ant-skeleton-button",
            }
        }
    }
}

/// 输入框骨架屏
#[derive(Props, Clone, PartialEq)]
pub struct SkeletonInputProps {
    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 是否展示动画效果
    #[props(default = true)]
    pub active: bool,

    /// 设置输入框的大小
    #[props(default)]
    pub size: Option<SkeletonInputSize>,
}

/// 输入框大小
#[derive(Clone, PartialEq, Debug)]
pub enum SkeletonInputSize {
    Large,
    Small,
    Default,
}

/// 输入框骨架屏组件
#[component]
pub fn SkeletonInput(props: SkeletonInputProps) -> Element {
    let size_class = match &props.size {
        Some(SkeletonInputSize::Large) => " ant-skeleton-input-lg",
        Some(SkeletonInputSize::Small) => " ant-skeleton-input-sm",
        _ => "",
    };

    let class_name = format!(
        "ant-skeleton ant-skeleton-element{}{}{}",
        if props.active {
            " ant-skeleton-active"
        } else {
            ""
        },
        size_class,
        if let Some(class) = &props.class {
            format!(" {}", class)
        } else {
            String::new()
        }
    );

    rsx! {
        div {
            class: "{class_name}",
            style: "{props.style.clone().unwrap_or_default()}",

            div {
                class: "ant-skeleton-input",
            }
        }
    }
}

/// 图片骨架屏
#[derive(Props, Clone, PartialEq)]
pub struct SkeletonImageProps {
    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 是否展示动画效果
    #[props(default = true)]
    pub active: bool,
}

/// 图片骨架屏组件
#[component]
pub fn SkeletonImage(props: SkeletonImageProps) -> Element {
    let class_name = format!(
        "ant-skeleton ant-skeleton-element{}{}",
        if props.active {
            " ant-skeleton-active"
        } else {
            ""
        },
        if let Some(class) = &props.class {
            format!(" {}", class)
        } else {
            String::new()
        }
    );

    rsx! {
        div {
            class: "{class_name}",
            style: "{props.style.clone().unwrap_or_default()}",

            div {
                class: "ant-skeleton-image",

                svg {
                    view_box: "0 0 1098 1024",
                    xmlns: "http://www.w3.org/2000/svg",
                    class: "ant-skeleton-image-svg",

                    path {
                        d: "M365.714286 329.142857q0 45.714286-32.036571 77.677714t-77.677714 32.036571-77.677714-32.036571-32.036571-77.677714 32.036571-77.677714 77.677714-32.036571 77.677714 32.036571 32.036571 77.677714zM950.857143 548.571429l0 256-804.571429 0 0-109.714286 182.857143-182.857143 91.428571 91.428571 292.571429-292.571429zM1005.714286 146.285714l-914.285714 0q-7.460571 0-12.873143 5.412571t-5.412571 12.873143l0 694.857143q0 7.460571 5.412571 12.873143t12.873143 5.412571l914.285714 0q7.460571 0 12.873143-5.412571t5.412571-12.873143l0-694.857143q0-7.460571-5.412571-12.873143t-12.873143-5.412571zM1097.142857 164.571429l0 694.857143q0 37.741714-26.843429 64.585143t-64.585143 26.843429l-914.285714 0q-37.741714 0-64.585143-26.843429t-26.843429-64.585143l0-694.857143q0-37.741714 26.843429-64.585143t64.585143-26.843429l914.285714 0q37.741714 0 64.585143 26.843429t26.843429 64.585143z",
                    }
                }
            }
        }
    }
}
