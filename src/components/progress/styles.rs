//! Progress 组件样式定义

use css_in_rust::css;

/// 获取 Progress 组件的样式
pub fn get_progress_styles() -> String {
    css!(
        r#"
        /* Progress 进度条组件样式 */

        /* 基础样式 */
        .ant-progress {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
            color: rgba(0, 0, 0, 0.88);
            font-size: 14px;
            line-height: 1.5714285714285714;
            list-style: none;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji';
            display: inline-block;
        }

        /* 线形进度条 */
        .ant-progress-line {
            position: relative;
            width: 100%;
            font-size: 14px;
            display: flex;
            align-items: center;
        }

        .ant-progress-outer {
            display: inline-block;
            width: 100%;
            margin-right: 8px;
            padding-right: 0;
        }

        .ant-progress-inner {
            position: relative;
            display: inline-block;
            width: 100%;
            overflow: hidden;
            vertical-align: middle;
            background-color: #f5f5f5;
            border-radius: 100px;
        }

        .ant-progress-bg {
            position: relative;
            background-color: #1677ff;
            border-radius: 100px;
            transition: all 0.4s cubic-bezier(0.08, 0.82, 0.17, 1) 0s;
        }

        .ant-progress-bg::after {
            content: '';
            background-image: linear-gradient(90deg,
                rgba(255, 255, 255, 0.3) 0%,
                rgba(255, 255, 255, 0.5) 50%,
                rgba(255, 255, 255, 0.3) 100%);
            animation: ant-progress-appear 2s ease-in-out infinite;
        }

        /* 进度文本 */
        .ant-progress-text {
            display: inline-block;
            width: 2em;
            margin-left: 8px;
            color: rgba(0, 0, 0, 0.88);
            font-size: 1em;
            line-height: 1;
            white-space: nowrap;
            text-align: left;
            vertical-align: middle;
            word-wrap: normal;
        }

        /* 小尺寸 */
        .ant-progress-small.ant-progress-line {
            font-size: 12px;
        }

        .ant-progress-small.ant-progress-line .ant-progress-text {
            margin-left: 8px;
            font-size: 12px;
        }

        /* 状态样式 */
        .ant-progress-status-active .ant-progress-bg::before {
            content: '';
            position: absolute;
            top: 0;
            left: 0;
            bottom: 0;
            right: 0;
            background: linear-gradient(to right,
                rgba(255, 255, 255, 0) 0%,
                rgba(255, 255, 255, 0.5) 50%,
                rgba(255, 255, 255, 0) 100%);
            animation: ant-progress-active 2.4s cubic-bezier(0.23, 1, 0.32, 1) infinite;
            border-radius: 10px;
            opacity: 0;
        }

        .ant-progress-status-exception .ant-progress-bg {
            background-color: #ff4d4f;
        }

        .ant-progress-status-exception .ant-progress-text {
            color: #ff4d4f;
        }

        .ant-progress-status-success .ant-progress-bg {
            background-color: #52c41a;
        }

        .ant-progress-status-success .ant-progress-text {
            color: #52c41a;
        }

        /* 圆形进度条 */
        .ant-progress-circle {
            position: relative;
            display: inline-block;
        }

        .ant-progress-circle .ant-progress-inner {
            position: relative;
            line-height: 1;
            background-color: transparent;
        }

        .ant-progress-circle .ant-progress-text {
            position: absolute;
            top: 50%;
            left: 0;
            width: 100%;
            margin: 0;
            padding: 0;
            color: rgba(0, 0, 0, 0.88);
            line-height: 1;
            white-space: normal;
            text-align: center;
            transform: translateY(-50%);
            font-size: 24px;
        }

        .ant-progress-circle.ant-progress-status-exception .ant-progress-text {
            color: #ff4d4f;
        }

        .ant-progress-circle.ant-progress-status-success .ant-progress-text {
            color: #52c41a;
        }

        /* 仪表盘进度条 */
        .ant-progress-dashboard .ant-progress-text {
            font-size: 20px;
        }

        /* 步骤进度条 */
        .ant-progress-steps-outer {
            display: flex;
            flex-direction: row;
            align-items: center;
        }

        .ant-progress-steps-item {
            background-color: #f5f5f5;
            margin-right: 2px;
            transition: all 0.3s;
        }

        .ant-progress-steps-item:last-child {
            margin-right: 0;
        }

        .ant-progress-steps-item-active {
            background-color: #1677ff;
            margin-right: 2px;
            transition: all 0.3s;
        }

        .ant-progress-steps-item-active:last-child {
            margin-right: 0;
        }

        /* 渐变色支持 */
        .ant-progress-gradient .ant-progress-bg {
            background: linear-gradient(to right, #1677ff, #69c0ff);
        }

        /* 成功进度段 */
        .ant-progress-success-bg {
            position: absolute;
            top: 0;
            left: 0;
            background-color: #52c41a;
            border-radius: 100px;
            transition: all 0.4s cubic-bezier(0.08, 0.82, 0.17, 1) 0s;
        }

        /* 多色彩分段 */
        .ant-progress-segment {
            position: absolute;
            top: 0;
            left: 0;
            border-radius: 100px;
            transition: all 0.4s cubic-bezier(0.08, 0.82, 0.17, 1) 0s;
        }

        /* 线条端点样式 */
        .ant-progress-round .ant-progress-bg {
            border-radius: 100px;
        }

        .ant-progress-square .ant-progress-bg {
            border-radius: 0;
        }

        /* 进度条位置 */
        .ant-progress-percent-top {
            flex-direction: column;
        }

        .ant-progress-percent-top .ant-progress-text {
            margin-left: 0;
            margin-bottom: 4px;
            order: -1;
        }

        .ant-progress-percent-bottom {
            flex-direction: column;
        }

        .ant-progress-percent-bottom .ant-progress-text {
            margin-left: 0;
            margin-top: 4px;
        }

        /* 响应式设计 */
        @media (max-width: 575px) {
            .ant-progress-responsive.ant-progress-circle {
                width: 80px !important;
                height: 80px !important;
            }

            .ant-progress-responsive.ant-progress-circle .ant-progress-text {
                font-size: 18px;
            }

            .ant-progress-responsive.ant-progress-dashboard .ant-progress-text {
                font-size: 16px;
            }
        }

        /* 无动画版本 */
        .ant-progress-no-animation .ant-progress-bg {
            transition: none;
        }

        .ant-progress-no-animation .ant-progress-bg::before {
            animation: none;
        }

        .ant-progress-no-animation .ant-progress-bg::after {
            animation: none;
        }

        /* 自定义主题色 */
        .ant-progress-custom-theme .ant-progress-bg {
            background-color: var(--progress-theme-color, #1677ff);
        }

        .ant-progress-custom-theme .ant-progress-text {
            color: var(--progress-theme-color, #1677ff);
        }

        /* RTL 支持 */
        .ant-progress-rtl {
            direction: rtl;
        }

        .ant-progress-rtl.ant-progress-line .ant-progress-outer {
            margin-right: 0;
            margin-left: 8px;
            padding-right: 0;
            padding-left: 0;
        }

        .ant-progress-rtl.ant-progress-line .ant-progress-text {
            margin-right: 8px;
            margin-left: 0;
            text-align: right;
        }

        /* 动画 */
        @keyframes ant-progress-appear {
            0% {
                opacity: 0.1;
                width: 0;
            }

            20% {
                opacity: 0.5;
                width: 0;
            }

            100% {
                opacity: 0;
                width: 100%;
            }
        }

        @keyframes ant-progress-active {
            0% {
                opacity: 0.1;
                width: 0;
            }

            20% {
                opacity: 0.5;
                width: 0;
            }

            100% {
                opacity: 0;
                width: 100%;
            }
        }

        /* 迷你尺寸 */
        .ant-progress-mini {
            font-size: 12px;
        }

        .ant-progress-mini .ant-progress-text {
            font-size: 12px;
            margin-left: 4px;
        }

        .ant-progress-mini .ant-progress-outer {
            margin-right: 4px;
        }

        /* 粗线条样式 */
        .ant-progress-thick .ant-progress-inner {
            height: 12px;
        }

        .ant-progress-thick .ant-progress-bg {
            height: 12px;
        }
        "#
    ).to_string()
}

/// 获取渐变色样式
pub fn get_gradient_style(from: &str, to: &str, direction: Option<f32>) -> String {
    let angle = direction.unwrap_or(90.0);
    format!("linear-gradient({}deg, {}, {})", angle, from, to)
}

/// 获取成功进度段样式
pub fn get_success_segment_style(percent: i32, color: &str, height: i32) -> String {
    format!(
        "position: absolute; left: 0; top: 0; width: {}%; height: {}px; background-color: {}; transition: width 0.3s ease;",
        percent.min(100).max(0),
        height,
        color
    )
}

/// 获取多色彩分段样式
pub fn get_color_segment_style(start: i32, end: i32, color: &str, height: i32) -> String {
    let width = end - start;
    format!(
        "position: absolute; left: {}%; top: 0; width: {}%; height: {}px; background-color: {}; transition: all 0.3s ease;",
        start,
        width.max(0),
        height,
        color
    )
}

/// 获取响应式类名
pub fn get_responsive_class(responsive: bool) -> &'static str {
    if responsive {
        "ant-progress-responsive"
    } else {
        ""
    }
}

/// 获取动画类名
pub fn get_animation_class(no_animation: bool) -> &'static str {
    if no_animation {
        "ant-progress-no-animation"
    } else {
        ""
    }
}

/// 获取主题色类名
pub fn get_theme_class(theme_color: Option<&str>) -> &'static str {
    match theme_color {
        Some("primary") => "ant-progress-theme-primary",
        Some("success") => "ant-progress-theme-success",
        Some("warning") => "ant-progress-theme-warning",
        Some("error") => "ant-progress-theme-error",
        Some("info") => "ant-progress-theme-info",
        _ => "",
    }
}

/// 获取主题色CSS变量
pub fn get_theme_css_vars(theme_color: Option<&str>) -> String {
    match theme_color {
        Some(color) => format!("--ant-progress-theme-color: {};", color),
        None => String::new(),
    }
}