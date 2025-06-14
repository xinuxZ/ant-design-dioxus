//! Space 组件实现
//!
//! 提供 Space 和 Space.Compact 组件的完整实现，包括间距控制、对齐方式、换行支持等功能。

use crate::components::space::types::*;
use crate::components::space::utils::*;
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
/// use ant_design_dioxus::Space;
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
        direction.unwrap(),
        &size.clone().unwrap(),
        wrap.unwrap(),
        align.unwrap(),
    ) {
        log::warn!("Space 配置验证失败: {}", error);
    }

    // 计算样式
    let container_class = get_space_container_class(
        direction.unwrap(),
        align.unwrap(),
        wrap.unwrap(),
        &merged_theme,
    );

    // 生成间距值
    let gap_value = get_space_gap_value(&size.unwrap(), direction.unwrap(), &merged_theme);

    // 合并类名
    let final_class = match class {
        Some(custom_class) => format!("{} {}", container_class, custom_class),
        None => container_class,
    };

    // 合并样式
    let gap_style = match direction {
        Some(SpaceDirection::Horizontal) => format!("gap: {} 0;", gap_value),
        Some(SpaceDirection::Vertical) => format!("gap: 0 {};", gap_value),
        None => format!(""),
    };

    let final_style = match style {
        Some(custom_style) => format!("{} {}", gap_style, custom_style),
        None => gap_style,
    };

    // 渲染子元素
    let children_elements = match children {
        Ok(children) => {
            // 这里需要处理子元素的包装和分割
            render_space_children(Ok(children), direction.unwrap(), &split, &merged_theme)
        }
        _ => rsx! {},
    };

    rsx! {
        div {
            class: "{final_class}",
            style: "{final_style}",
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
/// use ant_design_dioxus::{SpaceCompact, CompactSize};
///
/// fn app() -> Element {
///     rsx! {
///         SpaceCompact {
///             size: CompactSize::Middle,
///             Button { "按钮1" }
///             Button { "按钮2" }
///             Input { placeholder: "输入框" }
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
    let children_elements = match children {
        Ok(children) => render_space_compact_children(
            Ok(children),
            direction.unwrap(),
            size.unwrap(),
            &merged_theme,
        ),
        _ => rsx! {},
    };

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
