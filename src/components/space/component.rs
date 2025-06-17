//! Space 组件实现
//!
//! 提供 Space 和 Space.Compact 组件的完整实现，包括间距控制、对齐方式、换行支持等功能。

use crate::components::space::style_generator::SpaceStyleGenerator;
use crate::components::space::types::*;
use crate::components::space::utils::*;
use crate::config_provider::hooks::use_component_config;
use dioxus::prelude::*;

/// Space 组件 - 设置组件之间的间距
///
/// # Props
/// - `direction`: 间距方向，支持水平和垂直
/// - `size`: 间距大小，支持预设值和自定义值
/// - `align`: 对齐方式
/// - `wrap`: 是否自动换行（仅水平方向有效）
/// - `split`: 设置拆分元素
/// - `theme`: 自定义主题配置
/// - `class`: 自定义 CSS 类名
/// - `style`: 自定义样式
/// - `children`: 子元素
///
/// # Examples
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::{Button, SpaceSizeConfig, SpaceDirection, SpaceSize, Space};
///
/// fn app() -> Element {
///     rsx! {
///         Space {
///             direction: SpaceDirection::Horizontal,
///             size: SpaceSizeConfig::Single(SpaceSize::Middle),
///             Button { "按钮1" }
///             Button { "按钮2" }
///             Button { "按钮3" }
///         }
///     }
/// }
/// ```
#[component]
pub fn Space(props: SpaceProps) -> Element {
    // 获取全局配置
    let component_config = use_component_config();
    let read_config = component_config.read();
    // space组件暂时没有专门的配置，使用通用配置
    let space_config = None::<()>;

    // space组件暂时没有专门的配置，直接使用原始props
    let props = props;

    let SpaceProps {
        direction,
        size,
        align,
        wrap,
        split,
        theme,
        class,
        style,
        onclick,
        children,
    } = props;

    // 合并主题
    let merged_theme = merge_space_theme(theme.as_ref());

    // 验证配置
    if let Err(error) = validate_space_config(
        direction.unwrap_or(SpaceDirection::Horizontal),
        &size
            .clone()
            .unwrap_or(SpaceSizeConfig::Single(SpaceSize::Small)),
        wrap.unwrap_or(false),
        align.unwrap_or(SpaceAlign::Start),
    ) {
        log::warn!("Space 配置验证失败: {}", error);
    }

    // 使用样式生成器生成样式
    let style_generator = SpaceStyleGenerator::new()
        .with_direction(direction.unwrap_or(SpaceDirection::Horizontal))
        .with_size(
            size.clone()
                .unwrap_or(SpaceSizeConfig::Single(SpaceSize::Small)),
        )
        .with_align(align)
        .with_wrap(wrap.unwrap_or(false))
        .with_split(split.is_some());

    let generated_class = style_generator.generate();

    // 组合自定义类名
    let class_name = match &class {
        Some(custom_class) => format!("{} {}", generated_class, custom_class),
        None => generated_class,
    };

    // 组合自定义样式
    let inline_style = match &style {
        Some(custom_style) => custom_style.clone(),
        None => String::new(),
    };

    // 渲染子元素
    let children_elements =
        render_space_children(children, direction.unwrap_or(SpaceDirection::Horizontal), &split, &merged_theme);

    rsx! {
        div {
            class: "{class_name}",
            style: "{inline_style}",
            {children_elements}
        }
    }
}

/// Space.Compact 组件 - 紧凑模式的间距组件
///
/// # Props
/// - `direction`: 排列方向
/// - `size`: 组件尺寸
/// - `block`: 是否为块级元素
/// - `theme`: 自定义主题配置
/// - `class`: 自定义 CSS 类名
/// - `style`: 自定义样式
/// - `children`: 子元素
///
/// # Examples
///
/// ```rust
/// use dioxus::prelude::*;
/// use ant_design_dioxus::{Button, SpaceCompact, CompactSize};
///
/// fn app() -> Element {
///     rsx! {
///         SpaceCompact {
///             size: CompactSize::Middle,
///             Button { "按钮1" }
///             Button { "按钮2" }
///         }
///     }
/// }
/// ```
#[component]
pub fn SpaceCompact(props: SpaceCompactProps) -> Element {
    let SpaceCompactProps {
        direction,
        size,
        block,
        theme,
        class,
        style,
        onclick,
        children,
    } = props;

    // 合并主题
    let merged_theme = merge_space_theme(theme.as_ref());

    // 验证配置
    if let Err(error) =
        validate_space_compact_config(direction.unwrap(), size.unwrap(), block.unwrap())
    {
        log::warn!("Space.Compact 配置验证失败: {}", error);
    }

    // 计算样式
    let container_class = get_space_compact_container_class(
        direction.unwrap(),
        size.unwrap(),
        block.unwrap(),
        &merged_theme,
    );

    // 合并类名
    let final_class = match class {
        Some(custom_class) => format!("{} {}", container_class, custom_class),
        None => container_class,
    };

    // 渲染子元素
    let children_elements =
        render_space_compact_children(children, direction.unwrap(), size.unwrap(), &merged_theme);

    rsx! {
        div {
            class: "{final_class}",
            style: style.unwrap_or_default(),
            {children_elements}
        }
    }
}

/// 渲染 Space 子元素
fn render_space_children(
    children: Element,
    direction: SpaceDirection,
    split: &Option<Element>,
    theme: &SpaceTheme,
) -> Element {
    // 直接渲染子元素，Dioxus 会自动处理 Element 类型
    rsx! {
        {children}
    }
}

/// 渲染 Space.Compact 子元素
fn render_space_compact_children(
    children: Element,
    direction: SpaceDirection,
    size: CompactSize,
    theme: &SpaceTheme,
) -> Element {
    // 直接渲染子元素，Dioxus 会自动处理 Element 类型
    rsx! {
        {children}
    }
}

/// 创建小间距的 Space
pub fn space_small() -> SpaceProps {
    SpaceProps {
        size: Some(SpaceSizeConfig::Single(SpaceSize::Small)),
        ..Default::default()
    }
}

/// 创建中等间距的 Space
pub fn space_middle() -> SpaceProps {
    SpaceProps {
        size: Some(SpaceSizeConfig::Single(SpaceSize::Middle)),
        ..Default::default()
    }
}

/// 创建大间距的 Space
pub fn space_large() -> SpaceProps {
    SpaceProps {
        size: Some(SpaceSizeConfig::Single(SpaceSize::Large)),
        ..Default::default()
    }
}

/// 创建自定义间距的 Space
pub fn space_custom(size: u32) -> SpaceProps {
    SpaceProps {
        size: Some(SpaceSizeConfig::Single(SpaceSize::Custom(size))),
        ..Default::default()
    }
}

/// 创建垂直布局的 Space
pub fn space_vertical(size: SpaceSize) -> SpaceProps {
    SpaceProps {
        direction: Some(SpaceDirection::Vertical),
        size: Some(SpaceSizeConfig::Single(size)),
        ..Default::default()
    }
}

/// 创建支持换行的 Space
pub fn space_wrap(size: SpaceSize) -> SpaceProps {
    SpaceProps {
        direction: Some(SpaceDirection::Horizontal),
        size: Some(SpaceSizeConfig::Single(size)),
        wrap: Some(true),
        ..Default::default()
    }
}

/// 创建带分割线的 Space
pub fn space_with_divider(size: SpaceSize, divider: Element) -> SpaceProps {
    SpaceProps {
        size: Some(SpaceSizeConfig::Single(size)),
        split: Some(divider),
        ..Default::default()
    }
}

/// 创建紧凑模式的按钮组
pub fn compact_button_group() -> SpaceCompactProps {
    SpaceCompactProps {
        direction: Some(SpaceDirection::Horizontal),
        size: Some(CompactSize::Middle),
        ..Default::default()
    }
}

/// 创建紧凑模式的输入组
pub fn compact_input_group() -> SpaceCompactProps {
    SpaceCompactProps {
        direction: Some(SpaceDirection::Horizontal),
        size: Some(CompactSize::Middle),
        block: Some(true),
        ..Default::default()
    }
}

/// 创建垂直紧凑布局
pub fn compact_vertical() -> SpaceCompactProps {
    SpaceCompactProps {
        direction: Some(SpaceDirection::Vertical),
        size: Some(CompactSize::Small),
        ..Default::default()
    }
}

/// Space 组件的高阶组件包装器
#[component]
pub fn SpaceWrapper(
    direction: Option<SpaceDirection>,
    size: Option<SpaceSizeConfig>,
    align: Option<SpaceAlign>,
    wrap: Option<bool>,
    children: Element,
) -> Element {
    let props = SpaceProps {
        direction: Some(direction.unwrap_or_default()),
        size: Some(size.unwrap_or_default()),
        align: Some(align.unwrap_or_default()),
        wrap: Some(wrap.unwrap_or_default()),
        children: children,
        ..Default::default()
    };

    rsx! {
        Space { ..props }
    }
}

/// Space.Compact 组件的高阶组件包装器
#[component]
pub fn SpaceCompactWrapper(
    direction: Option<SpaceDirection>,
    size: Option<CompactSize>,
    block: Option<bool>,
    children: Element,
) -> Element {
    let props = SpaceCompactProps {
        direction: Some(direction.unwrap_or_default()),
        size: Some(size.unwrap_or_default()),
        block: Some(block.unwrap_or_default()),
        children: children,
        ..Default::default()
    };

    rsx! {
        SpaceCompact { ..props }
    }
}
