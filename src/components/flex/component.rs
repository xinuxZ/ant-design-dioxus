//! Flex 组件的主要实现
//!
//! 提供 Flex 组件及相关高阶组件的实现。

use crate::components::flex::styles::*;
use crate::components::flex::types::*;
use crate::components::flex::utils::*;
use dioxus::prelude::*;
use std::collections::HashMap;

/// Flex 组件
///
/// 基于 CSS Flexbox 的现代布局容器，提供强大的对齐和分布能力。
#[component]
pub fn Flex(props: FlexProps) -> Element {
    // 验证 Props
    if let Err(error) = validate_flex_props(&props) {
        log::warn!("Flex component validation error: {}", error);
    }

    // 创建配置
    let config = create_flex_config(&props);

    // 获取主题
    let theme = use_flex_theme();

    // 生成样式
    let container_styles = generate_flex_container_styles(&config, &theme);
    let css_class = get_flex_container_class(&config);

    // 合并自定义类名
    let final_class = if let Some(ref custom_class) = props.class {
        format!("{} {}", css_class, custom_class)
    } else {
        css_class
    };

    // 合并自定义样式
    let final_style = if let Some(ref custom_style) = props.style {
        format!("{} {}", container_styles, custom_style)
    } else {
        container_styles
    };

    // 渲染组件
    match props.component {
        "div" => rsx! {
            div {
                class: "{final_class}",
                style: "{final_style}",
                {props.children}
            }
        },
        "section" => rsx! {
            section {
                class: "{final_class}",
                style: "{final_style}",
                {props.children}
            }
        },
        "article" => rsx! {
            article {
                class: "{final_class}",
                style: "{final_style}",
                {props.children}
            }
        },
        "aside" => rsx! {
            aside {
                class: "{final_class}",
                style: "{final_style}",
                {props.children}
            }
        },
        "header" => rsx! {
            header {
                class: "{final_class}",
                style: "{final_style}",
                {props.children}
            }
        },
        "footer" => rsx! {
            footer {
                class: "{final_class}",
                style: "{final_style}",
                {props.children}
            }
        },
        "main" => rsx! {
            main {
                class: "{final_class}",
                style: "{final_style}",
                {props.children}
            }
        },
        "nav" => rsx! {
            nav {
                class: "{final_class}",
                style: "{final_style}",
                {props.children}
            }
        },
        _ => rsx! {
            div {
                class: "{final_class}",
                style: "{final_style}",
                {props.children}
            }
        },
    }
}

/// Flex 项目组件实现
#[component]
pub fn FlexItem(props: FlexItemProps) -> Element {
    let config = FlexItemConfig {
        flex: props.flex.clone(),
        flex_grow: props.flex_grow,
        flex_shrink: props.flex_shrink,
        flex_basis: props.flex_basis.clone(),
        align_self: props.align_self.clone(),
        order: props.order,
    };

    let theme = use_flex_theme();
    let item_styles = generate_flex_item_styles(&config, &theme);
    let css_class = get_flex_item_class(&config);

    let final_class = if let Some(ref custom_class) = props.class {
        format!("{} {}", css_class, custom_class)
    } else {
        css_class
    };

    let final_style = if let Some(ref custom_style) = props.style {
        format!("{} {}", item_styles, custom_style)
    } else {
        item_styles
    };

    rsx! {
        div {
            class: "{final_class}",
            style: "{final_style}",
            {props.children}
        }
    }
}

#[component]
pub fn FlexContainer(props: FlexContainerProps) -> Element {
    let theme = use_flex_theme();
    let container_styles = generate_flex_container_styles(&props.config, &theme);
    let css_class = get_flex_container_class(&props.config);

    let final_class = if let Some(ref custom_class) = props.class {
        format!("{} {}", css_class, custom_class)
    } else {
        css_class
    };

    let final_style = if let Some(ref custom_style) = props.style {
        format!("{} {}", container_styles, custom_style)
    } else {
        container_styles
    };

    rsx! {
        div {
            class: "{final_class}",
            style: "{final_style}",
            {props.children}
        }
    }
}

#[component]
pub fn FlexGrid(props: FlexGridProps) -> Element {
    let theme = use_flex_theme();
    let grid_styles = generate_grid_styles(props.cols, &props.gap, &theme);

    let final_class = if let Some(ref custom_class) = props.class {
        format!("ant-flex-grid {}", custom_class)
    } else {
        "ant-flex-grid".to_string()
    };

    let final_style = if let Some(ref custom_style) = props.style {
        format!("{} {}", grid_styles, custom_style)
    } else {
        grid_styles
    };

    rsx! {
        div {
            class: "{final_class}",
            style: "{final_style}",
            {props.children}
        }
    }
}

#[component]
pub fn FlexLayout(props: FlexLayoutProps) -> Element {
    let theme = use_flex_theme();
    let area_refs: Vec<&str> = props.areas.iter().map(|s| s.as_str()).collect();
    let layout_styles = generate_layout_styles(&area_refs, &theme);

    let final_class = if let Some(ref custom_class) = props.class {
        format!("ant-flex-layout {}", custom_class)
    } else {
        "ant-flex-layout".to_string()
    };

    let final_style = if let Some(ref custom_style) = props.style {
        format!("{} {}", layout_styles, custom_style)
    } else {
        layout_styles
    };

    rsx! {
        div {
            class: "{final_class}",
            style: "{final_style}",
            {props.children}
        }
    }
}

// 便捷构造函数

/// 创建水平 Flex 布局
pub fn flex_horizontal(gap: Option<FlexGap>, children: Element) -> Element {
    rsx! {
        Flex {
            vertical: false,
            gap: gap,
            {children}
        }
    }
}

/// 创建垂直 Flex 布局
pub fn flex_vertical(gap: Option<FlexGap>, children: Element) -> Element {
    rsx! {
        Flex {
            vertical: true,
            gap: gap,
            {children}
        }
    }
}

/// 创建居中对齐的 Flex 布局
pub fn flex_center(children: Element) -> Element {
    rsx! {
        Flex {
            justify: FlexJustify::Center,
            align: FlexAlign::Center,
            {children}
        }
    }
}

/// 创建两端对齐的 Flex 布局
pub fn flex_between(children: Element) -> Element {
    rsx! {
        Flex {
            justify: FlexJustify::SpaceBetween,
            align: FlexAlign::Center,
            {children}
        }
    }
}

/// 创建环绕对齐的 Flex 布局
pub fn flex_around(children: Element) -> Element {
    rsx! {
        Flex {
            justify: FlexJustify::SpaceAround,
            align: FlexAlign::Center,
            {children}
        }
    }
}

/// 创建均匀对齐的 Flex 布局
pub fn flex_evenly(children: Element) -> Element {
    rsx! {
        Flex {
            justify: FlexJustify::SpaceEvenly,
            align: FlexAlign::Center,
            {children}
        }
    }
}

/// 创建可换行的 Flex 布局
pub fn flex_wrap(gap: Option<FlexGap>, children: Element) -> Element {
    rsx! {
        Flex {
            wrap: FlexWrap::Wrap,
            gap: gap,
            {children}
        }
    }
}

/// 创建列布局 (垂直)
pub fn flex_column(gap: Option<FlexGap>, children: Element) -> Element {
    flex_vertical(gap, children)
}

/// 创建行布局 (水平)
pub fn flex_row(gap: Option<FlexGap>, children: Element) -> Element {
    flex_horizontal(gap, children)
}

// 主题相关

/// Flex 主题提供者
#[derive(Props, Clone, PartialEq)]
pub struct FlexThemeProviderProps {
    /// 主题配置
    pub theme: FlexTheme,

    /// 子元素
    pub children: Element,
}

#[component]
pub fn FlexThemeProvider(props: FlexThemeProviderProps) -> Element {
    use_context_provider(|| props.theme.clone());

    rsx! {
        {props.children}
    }
}

/// 使用 Flex 主题的 Hook
pub fn use_flex_theme() -> FlexTheme {
    use_context::<FlexTheme>()
}

/// 设置全局默认 Flex 主题
pub fn set_default_flex_theme(theme: FlexTheme) {
    crate::components::flex::types::set_default_flex_theme(theme);
}

/// 获取全局默认 Flex 主题
pub fn get_default_flex_theme() -> FlexTheme {
    crate::components::flex::types::get_default_flex_theme()
}

// Hooks

/// 使用 Flex 布局的 Hook
#[derive(Clone, PartialEq)]
pub struct FlexLayoutState {
    pub config: FlexConfig,
    pub theme: FlexTheme,
    pub responsive_config: Option<ResponsiveConfig>,
}

pub fn use_flex_layout(initial_config: FlexConfig) -> Signal<FlexLayoutState> {
    let mut state = use_signal(|| FlexLayoutState {
        config: initial_config,
        theme: get_default_flex_theme(),
        responsive_config: None,
    });

    state
}

/// 使用响应式 Flex 的 Hook
pub fn use_responsive_flex(breakpoints: HashMap<String, FlexConfig>) -> Signal<FlexConfig> {
    let mut current_config = use_signal(|| FlexConfig::default());

    // 在实际应用中，这里应该监听窗口大小变化
    // 并根据当前断点更新配置

    current_config
}

// 性能优化相关

/// 使用性能优化的 Flex Hook
pub fn use_optimized_flex(item_count: usize) -> PerformanceConfig {
    let config = FlexConfig::default();
    optimize_flex_performance(&config, item_count)
}

/// 使用虚拟化 Flex Hook
pub fn use_virtualized_flex(items: Vec<Element>, container_height: f32) -> Vec<Element> {
    // 简化的虚拟化实现
    // 在实际应用中应该根据滚动位置和容器高度计算可见项目
    let visible_count = (container_height / 50.0) as usize + 2; // 假设每个项目高度 50px

    items.into_iter().take(visible_count).collect()
}

/// 使用懒加载 Flex Hook
pub fn use_lazy_flex(items: Vec<Element>, threshold: usize) -> (Vec<Element>, bool) {
    let loaded_count = threshold.min(items.len());
    let has_more = items.len() > threshold;

    let loaded_items = items.into_iter().take(loaded_count).collect();

    (loaded_items, has_more)
}
