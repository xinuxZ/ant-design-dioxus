use css_in_rust::css;

/// 按钮基础样式
pub fn base_style() -> String {
    css!(
        r#"
        /* 基础样式 */
        .ant-btn {
            position: relative;
            display: inline-flex;
            align-items: center;
            justify-content: center;
            font-weight: 400;
            white-space: nowrap;
            text-align: center;
            background-image: none;
            background-color: transparent;
            border: 1px solid transparent;
            cursor: pointer;
            transition: all 0.2s cubic-bezier(0.645, 0.045, 0.355, 1);
            user-select: none;
            touch-action: manipulation;
            line-height: 1.5715;
            color: rgba(0, 0, 0, 0.88);
            min-width: 32px;
            min-height: 32px;
            padding: 4px 15px;
            font-size: 14px;
            border-radius: 6px;
            outline: none;
        }

        /* 禁用状态 */
        .ant-btn:disabled,
        .ant-btn[disabled] {
            cursor: not-allowed;
            opacity: 0.65;
            box-shadow: none;
        }

        /* 加载状态 */
        .ant-btn-loading {
            opacity: 0.65;
            cursor: default;
            pointer-events: none;
        }

        /* 块级按钮 */
        .ant-btn-block {
            width: 100%;
        }

        /* 幽灵按钮 */
        .ant-btn-background-ghost {
            color: #fff;
            background: transparent !important;
            border-color: #fff;
        }

        /* 危险按钮 */
        .ant-btn-dangerous {
            color: #ff4d4f;
            border-color: #ff4d4f;
        }

        /* 点击效果 */
        .ant-btn:active {
            outline: 0;
            box-shadow: none;
        }

        /* 焦点样式 */
        .ant-btn:focus-visible {
            outline: 4px solid rgba(24, 144, 255, 0.2);
            outline-offset: 1px;
            transition: outline-offset 0s, outline 0s;
        }

        /* 链接按钮不需要边框和背景 */
        .ant-btn-link {
            color: #1677ff;
            background: transparent;
            border-color: transparent;
            box-shadow: none;
        }

        /* 文本按钮不需要边框和背景 */
        .ant-btn-text {
            color: rgba(0, 0, 0, 0.88);
            background: transparent;
            border-color: transparent;
            box-shadow: none;
        }

        /* 两个中文字符之间自动插入空格 */
        .ant-btn-two-chinese-chars > span {
            letter-spacing: 0.3em;
            margin-right: -0.3em;
        }
    "#
    )
}

/// 按钮类型样式
pub fn type_style() -> String {
    css!(
        r#"
        /* 主要按钮 */
        .ant-btn-primary {
            color: #fff;
            background-color: #1677ff;
            border-color: #1677ff;
            box-shadow: 0 2px 0 rgba(5, 145, 255, 0.1);
        }

        .ant-btn-primary:hover,
        .ant-btn-primary:focus {
            color: #fff;
            background-color: #4096ff;
            border-color: #4096ff;
        }

        .ant-btn-primary:active {
            color: #fff;
            background-color: #0958d9;
            border-color: #0958d9;
        }

        /* 默认按钮 */
        .ant-btn-default {
            background-color: #fff;
            border-color: #d9d9d9;
            box-shadow: 0 2px 0 rgba(0, 0, 0, 0.02);
        }

        .ant-btn-default:hover,
        .ant-btn-default:focus {
            color: #4096ff;
            border-color: #4096ff;
        }

        .ant-btn-default:active {
            color: #0958d9;
            border-color: #0958d9;
        }

        /* 虚线按钮 */
        .ant-btn-dashed {
            background-color: #fff;
            border-color: #d9d9d9;
            border-style: dashed;
        }

        .ant-btn-dashed:hover,
        .ant-btn-dashed:focus {
            color: #4096ff;
            border-color: #4096ff;
        }

        .ant-btn-dashed:active {
            color: #0958d9;
            border-color: #0958d9;
        }

        /* 文本按钮 */
        .ant-btn-text:hover,
        .ant-btn-text:focus {
            background-color: rgba(0, 0, 0, 0.06);
        }

        .ant-btn-text:active {
            background-color: rgba(0, 0, 0, 0.15);
        }

        /* 链接按钮 */
        .ant-btn-link:hover,
        .ant-btn-link:focus {
            color: #4096ff;
        }

        .ant-btn-link:active {
            color: #0958d9;
        }

        /* 危险按钮 - Primary */
        .ant-btn-primary.ant-btn-dangerous {
            background-color: #ff4d4f;
            border-color: #ff4d4f;
            box-shadow: 0 2px 0 rgba(255, 38, 5, 0.06);
        }

        .ant-btn-primary.ant-btn-dangerous:hover,
        .ant-btn-primary.ant-btn-dangerous:focus {
            background-color: #ff7875;
            border-color: #ff7875;
        }

        .ant-btn-primary.ant-btn-dangerous:active {
            background-color: #d9363e;
            border-color: #d9363e;
        }

        /* 危险按钮 - Default */
        .ant-btn-default.ant-btn-dangerous {
            color: #ff4d4f;
            border-color: #ff4d4f;
        }

        .ant-btn-default.ant-btn-dangerous:hover,
        .ant-btn-default.ant-btn-dangerous:focus {
            color: #ff7875;
            border-color: #ff7875;
        }

        .ant-btn-default.ant-btn-dangerous:active {
            color: #d9363e;
            border-color: #d9363e;
        }
    "#
    )
}

/// 按钮大小样式
pub fn size_style() -> String {
    css!(
        r#"
        /* 大型按钮 */
        .ant-btn-large {
            min-height: 40px;
            padding: 6.5px 15px;
            font-size: 16px;
            border-radius: 8px;
        }

        /* 小型按钮 */
        .ant-btn-small {
            min-height: 24px;
            padding: 0 7px;
            font-size: 14px;
            border-radius: 4px;
        }
    "#
    )
}

/// 按钮形状样式
pub fn shape_style() -> String {
    css!(
        r#"
        /* 圆形按钮 */
        .ant-btn-circle {
            min-width: 32px;
            padding: 0;
            text-align: center;
            border-radius: 50%;
        }

        .ant-btn-circle.ant-btn-large {
            min-width: 40px;
            border-radius: 50%;
        }

        .ant-btn-circle.ant-btn-small {
            min-width: 24px;
            border-radius: 50%;
        }

        /* 圆角按钮 */
        .ant-btn-round {
            border-radius: 32px;
            padding: 4px 16px;
        }

        .ant-btn-round.ant-btn-large {
            border-radius: 40px;
            padding: 6.5px 20px;
        }

        .ant-btn-round.ant-btn-small {
            border-radius: 24px;
            padding: 0 12px;
        }
    "#
    )
}

/// 图标按钮样式
pub fn icon_style() -> String {
    css!(
        r#"
        /* 图标样式 */
        .ant-btn > .anticon {
            display: inline-flex;
            align-items: center;
        }

        .ant-btn > .anticon + span,
        .ant-btn > span + .anticon {
            margin-left: 8px;
        }

        /* 纯图标按钮 */
        .ant-btn-icon-only {
            display: inline-flex;
            justify-content: center;
            align-items: center;
            padding: 0;
        }

        .ant-btn-icon-only > .anticon {
            margin: 0;
        }

        .ant-btn-icon-only.ant-btn-large {
            min-width: 40px;
        }

        .ant-btn-icon-only.ant-btn-small {
            min-width: 24px;
        }

        /* 图标位于末尾 */
        .ant-btn-icon-end > span + .anticon {
            margin-left: 8px;
            margin-right: 0;
            order: 1;
        }
    "#
    )
}

/// 加载状态样式
pub fn loading_style() -> String {
    css!(
        r#"
        /* 加载状态 */
        .ant-btn-loading {
            position: relative;
            pointer-events: none;
        }

        .ant-btn-loading .ant-btn-loading-icon {
            display: inline-flex;
            align-items: center;
        }

        .ant-btn-loading .ant-btn-loading-icon .anticon {
            animation: loadingCircle 1s infinite linear;
        }

        .ant-btn-loading .ant-btn-loading-icon + span {
            margin-left: 8px;
        }

        @keyframes loadingCircle {
            100% {
                transform: rotate(360deg);
            }
        }
    "#
    )
}

/// 按钮组样式
pub fn group_style() -> String {
    css!(
        r#"
        /* 按钮组 */
        .ant-btn-group {
            display: inline-flex;
        }

        .ant-btn-group > .ant-btn {
            border-radius: 0;
        }

        .ant-btn-group > .ant-btn:first-child {
            border-top-left-radius: 6px;
            border-bottom-left-radius: 6px;
        }

        .ant-btn-group > .ant-btn:last-child {
            border-top-right-radius: 6px;
            border-bottom-right-radius: 6px;
        }

        .ant-btn-group > .ant-btn:not(:first-child):not(:last-child) {
            border-left: 1px solid rgba(0, 0, 0, 0.06);
            border-right: 1px solid rgba(0, 0, 0, 0.06);
        }

        .ant-btn-group > .ant-btn:not(:first-child) {
            margin-left: -1px;
        }
    "#
    )
}

/// 按钮样式
pub fn button_styles() -> &'static str {
    r#"
    .ant-btn {
        position: relative;
        display: inline-flex;
        align-items: center;
        justify-content: center;
        font-weight: 400;
        white-space: nowrap;
        text-align: center;
        background-image: none;
        background-color: transparent;
        border: 1px solid transparent;
        cursor: pointer;
        transition: all 0.2s cubic-bezier(0.645, 0.045, 0.355, 1);
        user-select: none;
        touch-action: manipulation;
        line-height: 1.5715;
        font-size: 14px;
        height: 32px;
        padding: 4px 15px;
        border-radius: 2px;
        color: rgba(0, 0, 0, 0.88);
        background-color: #ffffff;
        box-shadow: 0 2px 0 rgba(0, 0, 0, 0.02);
    }

    .ant-btn:hover, .ant-btn:focus {
        color: var(--ant-primary-color-hover, #4096ff);
        border-color: var(--ant-primary-color-hover, #4096ff);
    }

    .ant-btn:active {
        color: var(--ant-primary-color-active, #0958d9);
        border-color: var(--ant-primary-color-active, #0958d9);
    }

    .ant-btn[disabled], .ant-btn[disabled]:hover, .ant-btn[disabled]:focus, .ant-btn[disabled]:active {
        color: rgba(0, 0, 0, 0.25);
        border-color: #d9d9d9;
        background-color: rgba(0, 0, 0, 0.04);
        box-shadow: none;
        cursor: not-allowed;
    }

    /* 主要按钮 */
    .ant-btn-primary {
        color: #fff;
        background-color: var(--ant-primary-color, #1677ff);
        box-shadow: 0 2px 0 rgba(5, 145, 255, 0.1);
        border-color: var(--ant-primary-color, #1677ff);
    }

    .ant-btn-primary:hover, .ant-btn-primary:focus {
        color: #fff;
        background-color: var(--ant-primary-color-hover, #4096ff);
        border-color: var(--ant-primary-color-hover, #4096ff);
    }

    .ant-btn-primary:active {
        color: #fff;
        background-color: var(--ant-primary-color-active, #0958d9);
        border-color: var(--ant-primary-color-active, #0958d9);
    }

    .ant-btn-primary[disabled], .ant-btn-primary[disabled]:hover, .ant-btn-primary[disabled]:focus, .ant-btn-primary[disabled]:active {
        color: rgba(0, 0, 0, 0.25);
        border-color: #d9d9d9;
        background-color: rgba(0, 0, 0, 0.04);
        box-shadow: none;
    }

    /* 虚线按钮 */
    .ant-btn-dashed {
        border-style: dashed;
    }

    /* 文本按钮 */
    .ant-btn-text {
        border-color: transparent;
        background-color: transparent;
        box-shadow: none;
        color: rgba(0, 0, 0, 0.88);
    }

    .ant-btn-text:hover, .ant-btn-text:focus {
        color: rgba(0, 0, 0, 0.88);
        background-color: rgba(0, 0, 0, 0.06);
    }

    .ant-btn-text:active {
        color: rgba(0, 0, 0, 0.88);
        background-color: rgba(0, 0, 0, 0.15);
    }

    .ant-btn-text[disabled], .ant-btn-text[disabled]:hover, .ant-btn-text[disabled]:focus, .ant-btn-text[disabled]:active {
        color: rgba(0, 0, 0, 0.25);
        border-color: transparent;
        background-color: transparent;
        box-shadow: none;
    }

    /* 链接按钮 */
    .ant-btn-link {
        color: var(--ant-primary-color, #1677ff);
        border-color: transparent;
        background-color: transparent;
        box-shadow: none;
    }

    .ant-btn-link:hover, .ant-btn-link:focus {
        color: var(--ant-primary-color-hover, #4096ff);
        border-color: transparent;
        background-color: transparent;
    }

    .ant-btn-link:active {
        color: var(--ant-primary-color-active, #0958d9);
        border-color: transparent;
        background-color: transparent;
    }

    .ant-btn-link[disabled], .ant-btn-link[disabled]:hover, .ant-btn-link[disabled]:focus, .ant-btn-link[disabled]:active {
        color: rgba(0, 0, 0, 0.25);
        border-color: transparent;
        background-color: transparent;
        box-shadow: none;
    }

    /* 危险按钮 */
    .ant-btn-dangerous {
        color: var(--ant-error-color, #ff4d4f);
        border-color: var(--ant-error-color, #ff4d4f);
    }

    .ant-btn-dangerous:hover, .ant-btn-dangerous:focus {
        color: var(--ant-error-color-hover, #ff7875);
        border-color: var(--ant-error-color-hover, #ff7875);
    }

    .ant-btn-dangerous:active {
        color: var(--ant-error-color-active, #d9363e);
        border-color: var(--ant-error-color-active, #d9363e);
    }

    .ant-btn-dangerous.ant-btn-primary {
        color: #fff;
        background-color: var(--ant-error-color, #ff4d4f);
        border-color: var(--ant-error-color, #ff4d4f);
    }

    .ant-btn-dangerous.ant-btn-primary:hover, .ant-btn-dangerous.ant-btn-primary:focus {
        color: #fff;
        background-color: var(--ant-error-color-hover, #ff7875);
        border-color: var(--ant-error-color-hover, #ff7875);
    }

    .ant-btn-dangerous.ant-btn-primary:active {
        color: #fff;
        background-color: var(--ant-error-color-active, #d9363e);
        border-color: var(--ant-error-color-active, #d9363e);
    }

    .ant-btn-dangerous.ant-btn-text {
        color: var(--ant-error-color, #ff4d4f);
        border-color: transparent;
        background-color: transparent;
    }

    .ant-btn-dangerous.ant-btn-text:hover, .ant-btn-dangerous.ant-btn-text:focus {
        color: var(--ant-error-color-hover, #ff7875);
        background-color: rgba(255, 77, 79, 0.1);
    }

    .ant-btn-dangerous.ant-btn-text:active {
        color: var(--ant-error-color-active, #d9363e);
        background-color: rgba(255, 77, 79, 0.2);
    }

    .ant-btn-dangerous.ant-btn-link {
        color: var(--ant-error-color, #ff4d4f);
        border-color: transparent;
        background-color: transparent;
    }

    .ant-btn-dangerous.ant-btn-link:hover, .ant-btn-dangerous.ant-btn-link:focus {
        color: var(--ant-error-color-hover, #ff7875);
        border-color: transparent;
        background-color: transparent;
    }

    .ant-btn-dangerous.ant-btn-link:active {
        color: var(--ant-error-color-active, #d9363e);
        border-color: transparent;
        background-color: transparent;
    }

    /* 按钮尺寸 */
    .ant-btn-large {
        height: 40px;
        padding: 6.4px 15px;
        font-size: 16px;
        border-radius: 2px;
    }

    .ant-btn-small {
        height: 24px;
        padding: 0 7px;
        font-size: 14px;
        border-radius: 2px;
    }

    /* 按钮形状 */
    .ant-btn-circle {
        min-width: 32px;
        padding: 0;
        border-radius: 50%;
    }

    .ant-btn-circle.ant-btn-large {
        min-width: 40px;
        border-radius: 50%;
    }

    .ant-btn-circle.ant-btn-small {
        min-width: 24px;
        border-radius: 50%;
    }

    .ant-btn-round {
        border-radius: 32px;
    }

    .ant-btn-round.ant-btn-large {
        border-radius: 40px;
    }

    .ant-btn-round.ant-btn-small {
        border-radius: 24px;
    }

    /* 块级按钮 */
    .ant-btn-block {
        width: 100%;
    }

    /* 幽灵按钮 */
    .ant-btn-background-ghost {
        color: #fff;
        background-color: transparent;
        border-color: #fff;
    }

    .ant-btn-background-ghost.ant-btn-primary {
        color: var(--ant-primary-color, #1677ff);
        background-color: transparent;
        border-color: var(--ant-primary-color, #1677ff);
    }

    .ant-btn-background-ghost.ant-btn-primary:hover, .ant-btn-background-ghost.ant-btn-primary:focus {
        color: var(--ant-primary-color-hover, #4096ff);
        border-color: var(--ant-primary-color-hover, #4096ff);
    }

    .ant-btn-background-ghost.ant-btn-primary:active {
        color: var(--ant-primary-color-active, #0958d9);
        border-color: var(--ant-primary-color-active, #0958d9);
    }

    .ant-btn-background-ghost.ant-btn-dangerous {
        color: var(--ant-error-color, #ff4d4f);
        background-color: transparent;
        border-color: var(--ant-error-color, #ff4d4f);
    }

    /* 加载状态 */
    .ant-btn-loading {
        position: relative;
        pointer-events: none;
    }

    .ant-btn-loading::before {
        content: '';
        position: absolute;
        top: -1px;
        right: -1px;
        bottom: -1px;
        left: -1px;
        z-index: 1;
        background-color: rgba(255, 255, 255, 0.35);
        border-radius: inherit;
        transition: opacity 0.2s;
    }

    .ant-btn-loading-icon {
        margin-right: 8px;
    }

    /* 图标按钮 */
    .ant-btn-icon-only {
        width: 32px;
        height: 32px;
        padding: 0;
        font-size: 16px;
    }

    .ant-btn-icon-only.ant-btn-large {
        width: 40px;
        height: 40px;
        padding: 0;
        font-size: 18px;
    }

    .ant-btn-icon-only.ant-btn-small {
        width: 24px;
        height: 24px;
        padding: 0;
        font-size: 14px;
    }

    /* 图标位置 */
    .ant-btn-icon-end {
        flex-direction: row-reverse;
    }

    .ant-btn-icon-end .ant-btn-loading-icon {
        margin-right: 0;
        margin-left: 8px;
    }

    /* 两个中文字符 */
    .ant-btn-two-chinese-chars > *:not(.ant-btn-icon) {
        margin-right: -0.3em;
        letter-spacing: 0.3em;
    }
    "#
}

/// 按钮组样式
pub fn button_group_styles() -> &'static str {
    r#"
    .ant-btn-group {
        display: inline-flex;
        position: relative;
    }

    .ant-btn-group > .ant-btn {
        position: relative;
        border-radius: 0;
    }

    .ant-btn-group > .ant-btn:first-child {
        border-top-left-radius: 2px;
        border-bottom-left-radius: 2px;
    }

    .ant-btn-group > .ant-btn:last-child {
        border-top-right-radius: 2px;
        border-bottom-right-radius: 2px;
    }

    .ant-btn-group > .ant-btn:not(:first-child) {
        margin-left: -1px;
    }

    .ant-btn-group > .ant-btn:hover,
    .ant-btn-group > .ant-btn:focus,
    .ant-btn-group > .ant-btn:active {
        z-index: 2;
    }

    .ant-btn-group > .ant-btn[disabled] {
        z-index: 0;
    }

    /* 按钮组尺寸 */
    .ant-btn-group-large > .ant-btn {
        height: 40px;
        padding: 6.4px 15px;
        font-size: 16px;
    }

    .ant-btn-group-large > .ant-btn:first-child {
        border-top-left-radius: 2px;
        border-bottom-left-radius: 2px;
    }

    .ant-btn-group-large > .ant-btn:last-child {
        border-top-right-radius: 2px;
        border-bottom-right-radius: 2px;
    }

    .ant-btn-group-small > .ant-btn {
        height: 24px;
        padding: 0 7px;
        font-size: 14px;
    }

    .ant-btn-group-small > .ant-btn:first-child {
        border-top-left-radius: 2px;
        border-bottom-left-radius: 2px;
    }

    .ant-btn-group-small > .ant-btn:last-child {
        border-top-right-radius: 2px;
        border-bottom-right-radius: 2px;
    }
    "#
}
