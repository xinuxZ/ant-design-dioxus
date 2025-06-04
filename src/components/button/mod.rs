//! Button 按钮组件
//!
//! 按钮用于开始一个即时操作。
//!
//! ## 何时使用
//!
//! 标记了一个（或封装一组）操作命令，响应用户点击行为，触发相应的业务逻辑。
//!
//! 在 Ant Design 中我们提供了五种按钮。
//!
//! - 主按钮：用于主行动点，一个操作区域只能有一个主按钮。
//! - 默认按钮：用于没有主次之分的一组行动点。
//! - 虚线按钮：常用于添加操作。
//! - 文本按钮：用于最次级的行动点。
//! - 链接按钮：一般用于链接，即导航至某位置。
//!
//! 以及四种状态属性与上面配合使用。
//!
//! - 危险：删除/移动/修改权限等危险操作，一般需要二次确认。
//! - 幽灵：用于背景色比较复杂的地方，常用在首页/产品页等展示场景。
//! - 禁用：行动点不可用的时候，一般需要文案解释。
//! - 加载中：用于异步操作等待反馈的时候，也可以避免多次提交。

// 导出按钮组模块
mod button_group;
mod styles;
pub use button_group::*;

use dioxus::prelude::*;

use self::styles::{ButtonHtmlType, ButtonShape, ButtonSize, ButtonStyleGenerator, ButtonType};

/// 按钮属性
#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    /// 按钮类型
    #[props(default)]
    pub button_type: ButtonType,

    /// 按钮尺寸
    #[props(default)]
    pub size: ButtonSize,

    /// 按钮形状
    #[props(default)]
    pub shape: ButtonShape,

    /// HTML 按钮类型
    #[props(default)]
    pub html_type: ButtonHtmlType,

    /// 是否为危险按钮
    #[props(default = false)]
    pub danger: bool,

    /// 是否为幽灵按钮
    #[props(default = false)]
    pub ghost: bool,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 是否加载中
    #[props(default = false)]
    pub loading: bool,

    /// 是否为块级按钮
    #[props(default = false)]
    pub block: bool,

    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 点击事件处理器
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,

    /// 子元素
    children: Element,
}

/// 按钮组件
///
/// # 参数
///
/// * `props` - 按钮属性
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::Button;
///
/// fn app() -> Element {
///     rsx! {
///         Button {
///             button_type: ButtonType::Primary,
///             onclick: move |_| {
///                 println!("Button clicked!");
///             },
///             "Primary Button"
///         }
///     }
/// }
/// ```
#[component]
pub fn Button(props: ButtonProps) -> Element {
    // 使用 css! 宏生成样式类名
    let button_class = get_button_css_class(&props);
    let custom_style = props.style.clone().unwrap_or_default();

    let btnclass = format!(
        "{} {}",
        button_class,
        props.class.clone().unwrap_or_default()
    );

    rsx! {
        button {
            class: btnclass,
            style: custom_style,
            r#type: get_html_type(&props.html_type),
            disabled: props.disabled || props.loading,
            onclick: move |evt| {
                if !props.disabled && !props.loading {
                    if let Some(handler) = &props.onclick {
                        handler.call(evt);
                    }
                }
            },

            // 加载图标
            if props.loading {
                span {
                    class: "ant-btn-loading-icon",
                    // TODO: 添加加载图标
                }
            }

            // 按钮内容
            span {
                class: "ant-btn-content",
                {props.children}
            }
        }
    }
}

/// 获取按钮的 CSS 类名
///
/// 根据按钮的属性生成对应的 CSS 类名
fn get_button_css_class(props: &ButtonProps) -> String {
    // 转换枚举类型
    let style_type = props.button_type.clone();

    let style_size = props.size.clone();

    let style_shape = props.shape.clone();

    // 使用样式生成器
    ButtonStyleGenerator::new()
        .with_type(style_type)
        .with_size(style_size)
        .with_shape(style_shape)
        .with_danger(props.danger)
        .with_ghost(props.ghost)
        .with_disabled(props.disabled || props.loading)
        .with_loading(props.loading)
        .with_block(props.block)
        .generate()
}

/// 获取 HTML 按钮类型字符串
///
/// # 参数
///
/// * `html_type` - HTML 按钮类型枚举
///
/// # 返回值
///
/// 返回对应的 HTML type 属性值
fn get_html_type(html_type: &ButtonHtmlType) -> &'static str {
    match html_type {
        ButtonHtmlType::Submit => "submit",
        ButtonHtmlType::Reset => "reset",
        ButtonHtmlType::Button => "button",
    }
}

// 重新导出主要类型（避免名称冲突，不使用通配符导出）
