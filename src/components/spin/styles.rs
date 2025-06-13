//! Spin 组件样式定义

use css_in_rust::*;
use crate::theme::Theme as AntTheme;
use super::types::*;

/// 生成Spin组件的基础样式
pub fn spin_base_styles(theme: &AntTheme) -> String {
    css! {
        ".ant-spin" {
            box_sizing: "border-box";
            margin: 0;
            padding: 0;
            color: theme.color_text;
            font_size: px(theme.font_size);
            line_height: theme.line_height;
            list_style: "none";
            font_family: theme.font_family;
            position: "absolute";
            display: "none";
            color: theme.color_primary;
            text_align: "center";
            vertical_align: "middle";
            opacity: 0;
            transition: "transform 0.3s cubic-bezier(0.78, 0.14, 0.15, 0.86)";
        }

        ".ant-spin-spinning" {
            position: "static";
            display: "inline-block";
            opacity: 1;
        }

        ".ant-spin-container" {
            position: "relative";
            transition: "opacity 0.3s";
        }

        ".ant-spin-container > .ant-spin" {
            position: "absolute";
            top: 0;
            left: 0;
            z_index: 4;
            display: "block";
            width: percent(100);
            height: percent(100);
            max_height: px(400);
        }

        ".ant-spin-container > .ant-spin .ant-spin-dot" {
            position: "absolute";
            top: percent(50);
            left: percent(50);
            margin: px(-10);
        }

        ".ant-spin-container > .ant-spin .ant-spin-text" {
            position: "absolute";
            top: percent(50);
            width: percent(100);
            padding_top: px(5);
            text_shadow: format!("0 1px 2px {}", theme.color_bg_container);
        }

        ".ant-spin-container > .ant-spin.ant-spin-show-text .ant-spin-dot" {
            margin_top: px(-20);
        }

        ".ant-spin-blur" {
            clear: "both";
            overflow: "hidden";
            opacity: 0.5;
            user_select: "none";
            pointer_events: "none";
        }

        ".ant-spin-blur::after" {
            opacity: 0.4;
            pointer_events: "none";
        }

        ".ant-spin-dot-holder" {
            position: "relative";
            display: "block";
            font_size: px(20);
        }

        ".ant-spin-dot" {
            position: "relative";
            display: "inline-block";
            font_size: px(20);
            width: em(1);
            height: em(1);
        }

        ".ant-spin-dot-item" {
            position: "absolute";
            display: "block";
            width: px(9);
            height: px(9);
            background_color: theme.color_primary;
            border_radius: percent(100);
            transform: "scale(0.75)";
            transform_origin: "50% 50%";
            opacity: 0.3;
            animation: "antSpinMove 1s infinite linear alternate";
        }

        ".ant-spin-dot-item:nth-child(1)" {
            top: 0;
            left: 0;
        }

        ".ant-spin-dot-item:nth-child(2)" {
            top: 0;
            right: 0;
            animation_delay: "0.4s";
        }

        ".ant-spin-dot-item:nth-child(3)" {
            right: 0;
            bottom: 0;
            animation_delay: "0.8s";
        }

        ".ant-spin-dot-item:nth-child(4)" {
            bottom: 0;
            left: 0;
            animation_delay: "1.2s";
        }

        ".ant-spin-dot-spin" {
            transform: "rotate(45deg)";
            animation: "antRotate 1.2s infinite linear";
        }

        ".ant-spin-text" {
            position: "relative";
            top: px(5);
            color: theme.color_primary;
            font_size: px(theme.font_size);
            text_shadow: format!("0 1px 2px {}", theme.color_bg_container);
        }

        ".ant-spin-show-text .ant-spin-dot" {
            margin_top: px(-10);
        }

        // 尺寸变体
        ".ant-spin-sm .ant-spin-dot" {
            font_size: px(14);
        }

        ".ant-spin-sm .ant-spin-dot-item" {
            width: px(6);
            height: px(6);
        }

        ".ant-spin-lg .ant-spin-dot" {
            font_size: px(32);
        }

        ".ant-spin-lg .ant-spin-dot-item" {
            width: px(14);
            height: px(14);
        }

        // 全屏模式
        ".ant-spin-fullscreen" {
            position: "fixed";
            top: 0;
            left: 0;
            width: percent(100);
            height: percent(100);
            z_index: 9999;
            background_color: rgba(255, 255, 255, 0.8);
            display: "flex";
            align_items: "center";
            justify_content: "center";
            flex_direction: "column";
        }

        // 进度显示
        ".ant-spin-progress" {
            margin_top: px(8);
            font_size: px(12);
            color: theme.color_text_secondary;
        }

        // 动画定义
        "@keyframes antSpinMove" {
            "to" {
                opacity: 1;
            }
        }

        "@keyframes antRotate" {
            "to" {
                transform: "rotate(405deg)";
            }
        }

        // RTL 支持
        ".ant-spin-rtl" {
            direction: "rtl";
        }

        // 嵌套模式
        ".ant-spin-nested-loading" {
            position: "relative";
        }

        ".ant-spin-nested-loading > div > .ant-spin" {
            position: "absolute";
            top: 0;
            left: 0;
            z_index: 4;
            display: "block";
            width: percent(100);
            height: percent(100);
            max_height: px(400);
        }

        ".ant-spin-nested-loading > div > .ant-spin .ant-spin-dot" {
            position: "absolute";
            top: percent(50);
            left: percent(50);
            margin: px(-10);
        }

        ".ant-spin-nested-loading > div > .ant-spin .ant-spin-text" {
            position: "absolute";
            top: percent(50);
            width: percent(100);
            padding_top: px(5);
            text_shadow: format!("0 1px 2px {}", theme.color_bg_container);
        }

        ".ant-spin-nested-loading > div > .ant-spin.ant-spin-show-text .ant-spin-dot" {
            margin_top: px(-20);
        }
    }
}

/// 生成暗色主题的Spin样式
pub fn spin_dark_styles(theme: &Theme) -> String {
    css! {
        ".ant-spin" {
            color: theme.color_primary;
        }

        ".ant-spin-dot-item" {
            background_color: theme.color_primary;
        }

        ".ant-spin-text" {
            color: theme.color_primary;
            text_shadow: format!("0 1px 2px {}", theme.color_bg_elevated);
        }

        ".ant-spin-container > .ant-spin .ant-spin-text" {
            text_shadow: format!("0 1px 2px {}", theme.color_bg_elevated);
        }

        ".ant-spin-fullscreen" {
            background_color: rgba(0, 0, 0, 0.8);
        }

        ".ant-spin-blur" {
            opacity: 0.3;
        }
    }
}

/// 生成Spin组件的完整样式
pub fn generate_spin_styles() -> String {
    // 返回基础CSS类名，实际样式通过CSS文件提供
    String::new()
}