//! Avatar 头像组件
//!
//! 用来代表用户或事物，支持图片、图标以及字符展示。

use dioxus::prelude::*;

mod style;
pub use style::*;

const AVATAR_STYLE: &str = include_str!("./style.css");

/// Avatar 形状
#[derive(Debug, Clone, PartialEq)]
pub enum AvatarShape {
    /// 圆形
    Circle,
    /// 方形
    Square,
}

impl Default for AvatarShape {
    fn default() -> Self {
        Self::Circle
    }
}

/// Avatar 尺寸
#[derive(Debug, Clone, PartialEq)]
pub enum AvatarSize {
    /// 大号
    Large,
    /// 默认
    Default,
    /// 小号
    Small,
    /// 自定义尺寸（像素）
    Custom(u32),
}

impl Default for AvatarSize {
    fn default() -> Self {
        Self::Default
    }
}

impl AvatarSize {
    /// 获取尺寸的像素值
    pub fn to_pixels(&self) -> u32 {
        match self {
            AvatarSize::Large => 40,
            AvatarSize::Default => 32,
            AvatarSize::Small => 24,
            AvatarSize::Custom(size) => *size,
        }
    }

    /// 获取字体大小
    pub fn font_size(&self) -> u32 {
        match self {
            AvatarSize::Large => 18,
            AvatarSize::Default => 14,
            AvatarSize::Small => 12,
            AvatarSize::Custom(size) => size / 2,
        }
    }
}

/// Avatar 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct AvatarProps {
    /// 头像的图片地址
    #[props(default)]
    pub src: Option<String>,

    /// 图片无法显示时的替代文本
    #[props(default)]
    pub alt: Option<String>,

    /// 头像的形状
    #[props(default)]
    pub shape: AvatarShape,

    /// 头像的尺寸
    #[props(default)]
    pub size: AvatarSize,

    /// 设置头像的图标类型
    #[props(default)]
    pub icon: Option<String>,

    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 点击事件
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,

    /// 图片加载失败时的回调
    #[props(default)]
    pub on_error: Option<EventHandler<Event<ImageData>>>,

    /// 子元素（文字或图标）
    children: Element,
}

/// Avatar 头像组件
///
/// # Props
/// - `src`: 头像的图片地址
/// - `alt`: 图片无法显示时的替代文本
/// - `shape`: 头像的形状（圆形/方形）
/// - `size`: 头像的尺寸
/// - `icon`: 设置头像的图标类型
/// - `class`: 自定义类名
/// - `style`: 自定义样式
/// - `onclick`: 点击事件
/// - `on_error`: 图片加载失败时的回调
///
/// # Example
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::Avatar;
///
/// fn app() -> Element {
///     rsx! {
///         Avatar {
///             src: "https://example.com/avatar.jpg",
///             alt: "用户头像",
///             "U"
///         }
///     }
/// }
/// ```
#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    // 注入样式
    use_avatar_style();

    let mut image_error = use_signal(|| false);

    // 构建CSS类名
    let mut class_names = vec!["ant-avatar".to_string()];

    // 添加形状类名
    match props.shape {
        AvatarShape::Circle => class_names.push("ant-avatar-circle".to_string()),
        AvatarShape::Square => class_names.push("ant-avatar-square".to_string()),
    }

    // 添加尺寸类名
    match props.size {
        AvatarSize::Large => class_names.push("ant-avatar-lg".to_string()),
        AvatarSize::Small => class_names.push("ant-avatar-sm".to_string()),
        AvatarSize::Default => {}
        AvatarSize::Custom(_) => {}
    }

    // 添加自定义类名
    if let Some(custom_class) = &props.class {
        class_names.push(custom_class.clone());
    }

    let class_name = class_names.join(" ");

    // 构建内联样式
    let mut styles = Vec::new();

    // 自定义尺寸样式
    if let AvatarSize::Custom(_) = props.size {
        let size = props.size.to_pixels();
        let font_size = props.size.font_size();
        styles.push(format!(
            "width: {}px; height: {}px; line-height: {}px; font-size: {}px;",
            size, size, size, font_size
        ));
    }

    // 添加自定义样式
    if let Some(custom_style) = &props.style {
        styles.push(custom_style.clone());
    }

    let style_attr = if styles.is_empty() {
        None
    } else {
        Some(styles.join(" "))
    };

    // 处理图片加载错误
    let handle_image_error = move |evt: Event<ImageData>| {
        image_error.set(true);
        if let Some(on_error) = &props.on_error {
            on_error.call(evt);
        }
    };

    rsx! {
        style { {AVATAR_STYLE} }

        div {
            class: class_name.clone(),
            style: style_attr,
            onclick: move |evt| {
                if let Some(onclick) = &props.onclick {
                    onclick.call(evt);
                }
            },

            // 如果有图片且未出错，显示图片
            if let Some(src) = &props.src {
                if !image_error() {
                    img {
                        class: "ant-avatar-image",
                        src: src.clone(),
                        alt: props.alt.as_deref().unwrap_or(""),
                        onerror: handle_image_error,
                    }
                }
            }

            // 如果没有图片或图片出错，显示图标或文字
            if props.src.is_none() || image_error() {
                if let Some(icon) = &props.icon {
                    span {
                        class: "ant-avatar-icon",
                        {icon.clone()}
                    }
                } else {
                    span {
                        class: "ant-avatar-string",
                        {props.children}
                    }
                }
            }
        }
    }
}
