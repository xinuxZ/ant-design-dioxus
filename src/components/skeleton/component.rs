use dioxus::prelude::*;

use css_in_rust::css;

use super::styles::*;
use super::types::*;
use super::utils::*;

/// Skeleton 主组件
/// 在内容加载时显示占位符，提供更好的用户体验
#[component]
pub fn Skeleton(props: SkeletonProps) -> Element {
    let loading = props.loading.unwrap_or(true);
    let active = props.active.unwrap_or(false);
    let round = props.round.unwrap_or(false);
    let theme = merge_theme(props.theme.as_ref());

    // 如果不是加载状态，直接显示子元素
    if !loading {
        return rsx! {
            {props.children}
        };
    }

    // 检查各部分是否显示
    let has_avatar = should_show_avatar(props.avatar.as_ref());
    let has_title = should_show_title(props.title.as_ref());
    let has_paragraph = should_show_paragraph(props.paragraph.as_ref());

    // 计算各部分属性
    let avatar_props = if has_avatar {
        Some(calculate_avatar_props(
            props.avatar.as_ref().unwrap(),
            has_title,
            has_paragraph,
        ))
    } else {
        None
    };

    let title_props = if has_title {
        Some(calculate_title_props(
            props.title.as_ref().unwrap(),
            has_avatar,
            has_paragraph,
        ))
    } else {
        None
    };

    let paragraph_props = if has_paragraph {
        Some(calculate_paragraph_props(
            props.paragraph.as_ref().unwrap(),
            has_avatar,
            has_title,
        ))
    } else {
        None
    };

    // 生成样式
    let container_style =
        generate_skeleton_style(active, round, &theme, props.class_name.as_deref());

    let class_name = get_skeleton_class_name(loading, active, round, props.class_name.as_deref());

    rsx! {
        div {
            class: "{class_name}",
            style: "{container_style} {props.style.unwrap_or_default()}",

            // 头像和内容区域
            if has_avatar {
                div {
                    class: "ant-skeleton-header",
                    SkeletonAvatar {
                        props: avatar_props.unwrap(),
                        active,
                        theme: theme.clone(),
                    }
                }
            }

            div {
                class: "ant-skeleton-content",

                // 标题
                if has_title {
                    let title_props = title_props.unwrap();
                    SkeletonTitle {
                        width: title_props.width,
                        active,
                        round,
                        theme: theme.clone(),
                    }
                }

                // 段落
                if has_paragraph {
                    let paragraph_props = paragraph_props.unwrap();
                    SkeletonParagraph {
                        rows: paragraph_props.rows,
                        width: paragraph_props.width,
                        active,
                        round,
                        theme: theme.clone(),
                    }
                }
            }
        }
    }
}

/// Skeleton Avatar 子组件
#[component]
fn SkeletonAvatar(props: SkeletonAvatarProps, active: bool, theme: SkeletonTheme) -> Element {
    let shape = props.shape.unwrap_or(AvatarShape::Circle);
    let size = props.size.unwrap_or(AvatarSize::Large);
    let is_active = props.active.unwrap_or(active);

    let style = generate_skeleton_avatar_style(&shape, &size, is_active, &theme);
    let class_name = get_skeleton_avatar_class_name(&shape, &size, is_active, None);

    rsx! {
        span {
            class: "{class_name}",
            style: "{style}",
        }
    }
}

/// Skeleton Title 子组件
#[component]
fn SkeletonTitle(
    width: Option<SkeletonWidth>,
    active: bool,
    round: bool,
    theme: SkeletonTheme,
) -> Element {
    let style = generate_skeleton_title_style(width.as_ref(), active, round, &theme);

    rsx! {
        h3 {
            class: "ant-skeleton-title",
            style: "{style}",
        }
    }
}

/// Skeleton Paragraph 子组件
#[component]
fn SkeletonParagraph(
    rows: Option<usize>,
    width: Option<SkeletonWidthConfig>,
    active: bool,
    round: bool,
    theme: SkeletonTheme,
) -> Element {
    let rows = rows.unwrap_or(2);
    let widths = generate_paragraph_widths(rows, width.as_ref());

    let container_style = generate_skeleton_paragraph_style(rows, &widths, active, round, &theme);

    rsx! {
        ul {
            class: "ant-skeleton-paragraph",
            style: "{container_style}",

            for (index, width) in widths.iter().enumerate() {
                li {
                    key: "{index}",
                    style: "width: {width.to_css()};",
                }
            }
        }
    }
}

/// Skeleton Button 独立组件
#[component]
pub fn SkeletonButton(props: SkeletonButtonProps) -> Element {
    let active = props.active.unwrap_or(false);
    let block = props.block.unwrap_or(false);
    let shape = props.shape.unwrap_or(ButtonShape::Default);
    let size = props.size.unwrap_or(ButtonSize::Default);
    let theme = SkeletonTheme::default();

    let style = generate_skeleton_button_style(&shape, &size, block, active, &theme);
    let class_name =
        get_skeleton_button_class_name(&shape, &size, block, active, props.class_name.as_deref());

    let style_clone = props.style.clone();

    rsx! {
        span {
            class: "{class_name}",
            style: "{style} {style_clone.as_ref().unwrap()}",
        }
    }
}

/// Skeleton Input 独立组件
#[component]
pub fn SkeletonInput(props: SkeletonInputProps) -> Element {
    let active = props.active.unwrap_or(false);
    let size = props.size.unwrap_or(InputSize::Default);
    let theme = SkeletonTheme::default();

    let style = generate_skeleton_input_style(&size, active, &theme);
    let class_name = get_skeleton_input_class_name(&size, active, props.class_name.as_deref());

    let style_clone = props.style.clone();
    rsx! {
        span {
            class: "{class_name}",
            style: "{style} {style_clone.as_ref().unwrap()}",
        }
    }
}

/// Skeleton Image 独立组件
#[component]
pub fn SkeletonImage(props: SkeletonImageProps) -> Element {
    let active = props.active.unwrap_or(false);
    let theme = SkeletonTheme::default();

    let style =
        generate_skeleton_image_style(props.width.as_ref(), props.height.as_ref(), active, &theme);
    let class_name = get_skeleton_image_class_name(active, props.class_name.as_deref());

    let style_clone = props.style.clone();

    rsx! {
        div {
            class: "{class_name}",
            style: "{style} {style_clone.as_ref().unwrap()}",

            // 图片占位符图标
            svg {
                width: "64",
                height: "41",
                view_box: "0 0 64 41",
                xmlns: "http://www.w3.org/2000/svg",

                g {
                    transform: "translate(0 1)",
                    fill: "none",
                    fill_rule: "evenodd",

                    ellipse {
                        fill: "#f5f5f5",
                        cx: "32",
                        cy: "33",
                        rx: "32",
                        ry: "7",
                    }

                    g {
                        fill_rule: "nonzero",
                        stroke: "#d9d9d9",

                        path {
                            d: "m55 12.76 4 22.8c.58 3.4-2.07 6.44-5.92 6.44H10.92c-3.85 0-6.5-3.05-5.92-6.44l4-22.8C9.58 9.36 12.69 7 16.12 7h31.76c3.43 0 6.54 2.36 7.12 5.76Z",
                        }

                        path {
                            d: "M55 12.76 51 35.56c-.58 3.4-3.69 5.76-7.12 5.76H16.12c-3.43 0-6.54-2.36-7.12-5.76L5 12.76C4.42 9.36 7.07 6.32 10.92 6.32h42.16c3.85 0 6.5 3.05 5.92 6.44Z",
                            fill: "#fafafa",
                        }

                        path {
                            d: "M41 17c0 2.76-2.24 5-5 5s-5-2.24-5-5 2.24-5 5-5 5 2.24 5 5Zm-5-3c-1.66 0-3 1.34-3 3s1.34 3 3 3 3-1.34 3-3-1.34-3-3-3Z",
                            fill: "#d9d9d9",
                        }

                        path {
                            d: "m56 39-7-14-13 20H8l20-20 13 13 15 1Z",
                            fill: "#d9d9d9",
                        }
                    }
                }
            }
        }
    }
}

/// Skeleton Node 通用节点组件
/// 用于创建自定义形状的骨架屏
#[component]
pub fn SkeletonNode(
    active: Option<bool>,
    width: Option<SkeletonWidth>,
    height: Option<SkeletonWidth>,
    class_name: Option<String>,
    style: Option<String>,
    children: Option<Element>,
) -> Element {
    let is_active = active.unwrap_or(false);
    let theme = SkeletonTheme::default();

    let width_css = width
        .as_ref()
        .map(|w| w.to_css())
        .unwrap_or_else(|| "100%".to_string());
    let height_css = height
        .as_ref()
        .map(|h| h.to_css())
        .unwrap_or_else(|| "20px".to_string());

    let node_style = css! {
        display: inline-block;
        width: {width_css};
        height: {height_css};
        border-radius: {theme.block_radius.unwrap_or(4)}px;
    };

    let animation_style = generate_skeleton_animation(is_active, &theme);
    let full_style = format!(
        "{} {} {}",
        node_style,
        animation_style,
        style.unwrap_or_default()
    );

    let mut classes = vec!["ant-skeleton-element"];
    if is_active {
        classes.push("ant-skeleton-element-active");
    }
    if let Some(custom) = class_name.as_deref() {
        classes.push(custom);
    }
    let class_str = classes.join(" ");

    rsx! {
        span {
            class: "{class_str}",
            style: "{full_style}",
            {children}
        }
    }
}
