/// Radio 组件样式生成器
///
/// 用于生成 Radio 组件的样式
use crate::theme::AliasToken;
use css_in_rust::css;

/// Radio 尺寸
#[derive(Debug, Clone, PartialEq)]
pub enum RadioSize {
    /// 小尺寸
    Small,
    /// 中等尺寸（默认）
    Middle,
    /// 大尺寸
    Large,
}

impl Default for RadioSize {
    fn default() -> Self {
        Self::Middle
    }
}

impl RadioSize {
    /// 获取尺寸对应的CSS类名
    pub fn to_class(&self) -> &'static str {
        match self {
            RadioSize::Small => "ant-radio-group-small",
            RadioSize::Middle => "",
            RadioSize::Large => "ant-radio-group-large",
        }
    }
}

/// Radio 组样式（Button风格）
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RadioButtonStyle {
    /// 描边样式（默认）
    Outline,
    /// 填充样式
    Solid,
}

impl Default for RadioButtonStyle {
    fn default() -> Self {
        Self::Outline
    }
}

/// Radio 样式生成器
pub struct RadioStyleGenerator {
    /// 是否选中
    pub checked: bool,
    /// 是否禁用
    pub disabled: bool,
    /// Radio尺寸
    pub size: RadioSize,
    /// 是否为按钮样式
    pub is_button: bool,
    /// 按钮风格
    pub button_style: RadioButtonStyle,
    /// 主题令牌
    pub token: AliasToken,
}

impl Default for RadioStyleGenerator {
    fn default() -> Self {
        Self {
            checked: false,
            disabled: false,
            size: RadioSize::default(),
            is_button: false,
            button_style: RadioButtonStyle::default(),
            token: AliasToken::default(),
        }
    }
}

impl RadioStyleGenerator {
    /// 创建新的样式生成器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置是否选中
    pub fn with_checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// 设置是否禁用
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 设置Radio尺寸
    pub fn with_size(mut self, size: RadioSize) -> Self {
        self.size = size;
        self
    }

    /// 设置是否为按钮样式
    pub fn with_button(mut self, is_button: bool) -> Self {
        self.is_button = is_button;
        self
    }

    /// 设置按钮风格
    pub fn with_button_style(mut self, button_style: RadioButtonStyle) -> Self {
        self.button_style = button_style;
        self
    }

    /// 设置主题令牌
    pub fn with_token(mut self, token: AliasToken) -> Self {
        self.token = token;
        self
    }

    /// 生成样式类名
    pub fn generate(&self) -> String {
        if self.is_button {
            self.generate_button_class()
        } else {
            self.generate_radio_class()
        }
    }

    /// 生成标准Radio类名
    fn generate_radio_class(&self) -> String {
        let mut classes = vec!["ant-radio-wrapper"];

        if self.disabled {
            classes.push("ant-radio-wrapper-disabled");
        }

        if self.checked {
            classes.push("ant-radio-wrapper-checked");
        }

        classes.join(" ")
    }

    /// 生成Radio内部类名
    pub fn generate_inner_class(&self) -> String {
        if self.is_button {
            if self.checked {
                if self.disabled {
                    "ant-radio-button ant-radio-button-checked ant-radio-button-disabled"
                } else {
                    "ant-radio-button ant-radio-button-checked"
                }
            } else if self.disabled {
                "ant-radio-button ant-radio-button-disabled"
            } else {
                "ant-radio-button"
            }
        } else {
            if self.checked {
                if self.disabled {
                    "ant-radio ant-radio-checked ant-radio-disabled"
                } else {
                    "ant-radio ant-radio-checked"
                }
            } else if self.disabled {
                "ant-radio ant-radio-disabled"
            } else {
                "ant-radio"
            }
        }
        .to_string()
    }

    /// 生成按钮样式Radio类名
    fn generate_button_class(&self) -> String {
        let mut classes = vec!["ant-radio-button-wrapper"];

        if self.disabled {
            classes.push("ant-radio-button-wrapper-disabled");
        }

        if self.checked {
            classes.push("ant-radio-button-wrapper-checked");
        }

        match self.size {
            RadioSize::Small => classes.push("ant-radio-button-wrapper-sm"),
            RadioSize::Middle => {}
            RadioSize::Large => classes.push("ant-radio-button-wrapper-lg"),
        }

        match self.button_style {
            RadioButtonStyle::Outline => {}
            RadioButtonStyle::Solid => {
                if self.checked {
                    classes.push("ant-radio-button-wrapper-solid");
                }
            }
        }

        classes.join(" ")
    }

    /// 生成 CSS 样式
    pub fn generate_css(&self) -> String {
        css!(
            r#"
            .ant-radio-wrapper {
                box-sizing: border-box;
                margin: 0;
                padding: 0;
                color: ${color_text};
                font-size: ${font_size}px;
                font-variant: tabular-nums;
                line-height: ${line_height};
                list-style: none;
                font-feature-settings: 'tnum';
                position: relative;
                display: inline-flex;
                align-items: baseline;
                margin-right: ${margin_xs}px;
                cursor: pointer;
            }

            .ant-radio {
                box-sizing: border-box;
                margin: 0;
                padding: 0;
                color: ${color_text};
                font-size: ${font_size}px;
                font-variant: tabular-nums;
                line-height: ${line_height};
                list-style: none;
                font-feature-settings: 'tnum';
                position: relative;
                top: 0.2em;
                display: inline-block;
                outline: none;
                cursor: pointer;
            }

            .ant-radio-wrapper:hover .ant-radio,
            .ant-radio:hover .ant-radio-inner,
            .ant-radio-input:focus + .ant-radio-inner {
                border-color: ${color_primary};
            }

            .ant-radio-input:focus + .ant-radio-inner {
                box-shadow: 0 0 0 3px ${color_primary_hover_outline};
            }

            .ant-radio-checked::after {
                position: absolute;
                top: 0;
                left: 0;
                width: 100%;
                height: 100%;
                border: 1px solid ${color_primary};
                border-radius: 50%;
                visibility: hidden;
                animation: antRadioEffect 0.36s ease-in-out;
                animation-fill-mode: both;
                content: '';
            }

            .ant-radio:hover::after,
            .ant-radio-wrapper:hover .ant-radio::after {
                visibility: visible;
            }

            .ant-radio-inner {
                position: relative;
                top: 0;
                left: 0;
                display: block;
                width: ${radio_size}px;
                height: ${radio_size}px;
                background-color: ${color_bg_container};
                border-color: ${color_border};
                border-style: solid;
                border-width: 1px;
                border-radius: 50%;
                transition: all 0.3s;
            }

            .ant-radio-inner::after {
                position: absolute;
                top: ${radio_dot_position}px;
                left: ${radio_dot_position}px;
                display: block;
                width: ${radio_dot_size}px;
                height: ${radio_dot_size}px;
                background-color: ${color_primary};
                border-top: 0;
                border-left: 0;
                border-radius: ${radio_dot_size}px;
                transform: scale(0);
                opacity: 0;
                transition: all 0.3s cubic-bezier(0.78, 0.14, 0.15, 0.86);
                content: ' ';
            }

            .ant-radio-input {
                position: absolute;
                top: 0;
                right: 0;
                bottom: 0;
                left: 0;
                z-index: 1;
                cursor: pointer;
                opacity: 0;
            }

            .ant-radio-checked .ant-radio-inner {
                border-color: ${color_primary};
            }

            .ant-radio-checked .ant-radio-inner::after {
                transform: scale(1);
                opacity: 1;
                transition: all 0.3s cubic-bezier(0.78, 0.14, 0.15, 0.86);
            }

            .ant-radio-disabled {
                cursor: not-allowed;
            }

            .ant-radio-disabled .ant-radio-inner {
                background-color: ${color_bg_container_disabled};
                border-color: ${color_border} !important;
                cursor: not-allowed;
            }

            .ant-radio-disabled .ant-radio-inner::after {
                background-color: rgba(0, 0, 0, 0.2);
            }

            .ant-radio-disabled .ant-radio-input {
                cursor: not-allowed;
            }

            .ant-radio-disabled + span {
                color: ${color_text_disabled};
                cursor: not-allowed;
            }

            span.ant-radio + * {
                padding-right: ${padding_xs}px;
                padding-left: ${padding_xs}px;
            }

            .ant-radio-button-wrapper {
                position: relative;
                display: inline-block;
                height: ${height_base}px;
                margin: 0;
                padding: 0 ${padding_md}px;
                color: ${color_text};
                font-size: ${font_size}px;
                line-height: ${height_base - 2}px;
                background: ${color_bg_container};
                border: 1px solid ${color_border};
                border-top-width: 1.02px;
                border-left-width: 0;
                cursor: pointer;
                transition: color 0.3s, background 0.3s, border-color 0.3s, box-shadow 0.3s;
            }

            .ant-radio-button-wrapper a {
                color: ${color_text};
            }

            .ant-radio-button-wrapper > .ant-radio-button {
                position: absolute;
                top: 0;
                left: 0;
                z-index: -1;
                width: 100%;
                height: 100%;
            }

            .ant-radio-group-large .ant-radio-button-wrapper {
                height: ${height_lg}px;
                font-size: ${font_size_lg}px;
                line-height: ${height_lg - 2}px;
            }

            .ant-radio-group-small .ant-radio-button-wrapper {
                height: ${height_sm}px;
                padding: 0 ${padding_sm}px;
                font-size: ${font_size_sm}px;
                line-height: ${height_sm - 2}px;
            }

            .ant-radio-button-wrapper:not(:first-child)::before {
                position: absolute;
                top: -1px;
                left: -1px;
                display: block;
                box-sizing: content-box;
                width: 1px;
                height: 100%;
                padding: 1px 0;
                background-color: ${color_border};
                transition: background-color 0.3s;
                content: '';
            }

            .ant-radio-button-wrapper:first-child {
                border-left: 1px solid ${color_border};
                border-radius: ${border_radius}px 0 0 ${border_radius}px;
            }

            .ant-radio-button-wrapper:last-child {
                border-radius: 0 ${border_radius}px ${border_radius}px 0;
            }

            .ant-radio-button-wrapper:hover {
                position: relative;
                color: ${color_primary};
            }

            .ant-radio-button-wrapper:focus-within {
                box-shadow: 0 0 0 3px ${color_primary_hover_outline};
            }

            .ant-radio-button-wrapper .ant-radio-inner,
            .ant-radio-button-wrapper input[type='checkbox'],
            .ant-radio-button-wrapper input[type='radio'] {
                width: 0;
                height: 0;
                opacity: 0;
                pointer-events: none;
            }

            .ant-radio-button-wrapper-checked:not(.ant-radio-button-wrapper-disabled) {
                z-index: 1;
                color: ${color_primary};
                background: ${color_bg_container};
                border-color: ${color_primary};
            }

            .ant-radio-button-wrapper-checked:not(.ant-radio-button-wrapper-disabled)::before {
                background-color: ${color_primary};
            }

            .ant-radio-button-wrapper-checked:not(.ant-radio-button-wrapper-disabled):first-child {
                border-color: ${color_primary};
            }

            .ant-radio-button-wrapper-checked:not(.ant-radio-button-wrapper-disabled):hover {
                color: ${color_primary_hover};
                border-color: ${color_primary_hover};
            }

            .ant-radio-button-wrapper-checked:not(.ant-radio-button-wrapper-disabled):hover::before {
                background-color: ${color_primary_hover};
            }

            .ant-radio-button-wrapper-checked:not(.ant-radio-button-wrapper-disabled):active {
                color: ${color_primary_active};
                border-color: ${color_primary_active};
            }

            .ant-radio-button-wrapper-checked:not(.ant-radio-button-wrapper-disabled):active::before {
                background-color: ${color_primary_active};
            }

            .ant-radio-button-wrapper-checked:not(.ant-radio-button-wrapper-disabled):focus-within {
                box-shadow: 0 0 0 3px ${color_primary_hover_outline};
            }

            .ant-radio-group-solid .ant-radio-button-wrapper-checked:not(.ant-radio-button-wrapper-disabled) {
                color: ${color_text_light};
                background: ${color_primary};
                border-color: ${color_primary};
            }

            .ant-radio-group-solid .ant-radio-button-wrapper-checked:not(.ant-radio-button-wrapper-disabled):hover {
                color: ${color_text_light};
                background: ${color_primary_hover};
                border-color: ${color_primary_hover};
            }

            .ant-radio-group-solid .ant-radio-button-wrapper-checked:not(.ant-radio-button-wrapper-disabled):active {
                color: ${color_text_light};
                background: ${color_primary_active};
                border-color: ${color_primary_active};
            }

            .ant-radio-button-wrapper-disabled {
                color: ${color_text_disabled};
                background: ${color_bg_container_disabled};
                border-color: ${color_border};
                cursor: not-allowed;
            }

            .ant-radio-button-wrapper-disabled:first-child,
            .ant-radio-button-wrapper-disabled:hover {
                color: ${color_text_disabled};
                background: ${color_bg_container_disabled};
                border-color: ${color_border};
            }

            .ant-radio-button-wrapper-disabled:first-child {
                border-left-color: ${color_border};
            }

            .ant-radio-button-wrapper-disabled.ant-radio-button-wrapper-checked {
                color: ${color_text_light};
                background: ${color_disabled};
                border-color: ${color_border};
                box-shadow: none;
            }

            @keyframes antRadioEffect {
                0% {
                    transform: scale(1);
                    opacity: 0.5;
                }
                100% {
                    transform: scale(1.6);
                    opacity: 0;
                }
            }

            @media (prefers-color-scheme: dark) {
                .ant-radio-inner {
                    background-color: transparent;
                }
            }
            "#,
            color_text = self.token.color_text,
            font_size = self.token.font_size,
            line_height = self.token.line_height,
            margin_xs = self.token.margin_xs,
            color_primary = self.token.color_primary,
            color_primary_hover_outline = self.token.color_primary_hover_outline,
            radio_size = 16,
            color_bg_container = self.token.color_bg_container,
            color_border = self.token.color_border,
            radio_dot_position = 4,
            radio_dot_size = 8,
            color_bg_container_disabled = self.token.color_bg_container_disabled,
            color_text_disabled = self.token.color_text_disabled,
            padding_xs = self.token.padding_xs,
            height_base = self.token.height_base,
            padding_md = self.token.padding_md,
            height_lg = self.token.height_lg,
            font_size_lg = self.token.font_size_lg,
            height_sm = self.token.height_sm,
            padding_sm = self.token.padding_sm,
            font_size_sm = self.token.font_size_sm,
            border_radius = self.token.border_radius,
            color_primary_hover = self.token.color_primary_hover,
            color_primary_active = self.token.color_primary_active,
            color_text_light = self.token.color_text_light,
            color_disabled = self.token.color_disabled
        ).to_string()
    }
}

/// Radio 组样式生成器
pub struct RadioGroupStyleGenerator {
    /// Radio组尺寸
    pub size: RadioSize,
    /// 是否使用按钮样式
    pub button_style: bool,
    /// 是否纵向排列
    pub vertical: bool,
    /// 主题令牌
    pub token: AliasToken,
}

impl Default for RadioGroupStyleGenerator {
    fn default() -> Self {
        Self {
            size: RadioSize::default(),
            button_style: false,
            vertical: false,
            token: AliasToken::default(),
        }
    }
}

impl RadioGroupStyleGenerator {
    /// 创建新的样式生成器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置Radio组尺寸
    pub fn with_size(mut self, size: RadioSize) -> Self {
        self.size = size;
        self
    }

    /// 设置是否使用按钮样式
    pub fn with_button_style(mut self, button_style: bool) -> Self {
        self.button_style = button_style;
        self
    }

    /// 设置是否纵向排列
    pub fn with_vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        self
    }

    /// 设置主题令牌
    pub fn with_token(mut self, token: AliasToken) -> Self {
        self.token = token;
        self
    }

    /// 生成样式类名
    pub fn generate(&self) -> String {
        let mut classes = vec!["ant-radio-group"];

        // 添加尺寸相关的类名
        let size_class = self.size.to_class();
        if !size_class.is_empty() {
            classes.push(size_class);
        }

        // 添加按钮样式类名
        if self.button_style {
            classes.push("ant-radio-group-button");
        }

        // 添加垂直排列类名
        if self.vertical {
            classes.push("ant-radio-group-vertical");
        }

        classes.join(" ")
    }

    /// 生成 CSS 样式
    pub fn generate_css(&self) -> String {
        css!(
            r#"
            .ant-radio-group {
                box-sizing: border-box;
                margin: 0;
                padding: 0;
                color: ${color_text};
                font-size: ${font_size}px;
                font-variant: tabular-nums;
                line-height: ${line_height};
                list-style: none;
                font-feature-settings: 'tnum';
                display: inline-block;
            }

            .ant-radio-group .ant-badge-count {
                z-index: 1;
            }

            .ant-radio-group > .ant-badge:not(:first-child) > .ant-radio-button-wrapper {
                border-left: none;
            }

            .ant-radio-wrapper {
                margin-right: ${margin_xs}px;
            }

            .ant-radio-wrapper:last-child {
                margin-right: 0;
            }

            .ant-radio-group-vertical .ant-radio-wrapper {
                display: flex;
                align-items: baseline;
                margin-right: 0;
                margin-bottom: ${margin_xs}px;
            }

            .ant-radio-group-vertical .ant-radio-wrapper:last-child {
                margin-bottom: 0;
            }

            .ant-radio-group-solid .ant-radio-button-wrapper:not(:first-child)::before {
                background-color: ${color_text_light};
            }

            .ant-radio-button-wrapper {
                margin-right: -1px;
                margin-bottom: 0;
            }

            .ant-radio-button-wrapper:not(:first-child)::before {
                background-color: ${color_border};
            }

            .ant-radio-button-wrapper:last-child {
                margin-right: 0;
            }
            "#,
            color_text = self.token.color_text,
            font_size = self.token.font_size,
            line_height = self.token.line_height,
            margin_xs = self.token.margin_xs,
            color_text_light = self.token.color_text_light,
            color_border = self.token.color_border
        )
        .to_string()
    }
}

/// 生成 Radio 样式
pub fn generate_radio_style(checked: bool, disabled: bool, is_button: bool) -> String {
    RadioStyleGenerator::new()
        .with_checked(checked)
        .with_disabled(disabled)
        .with_button(is_button)
        .generate()
}

/// 生成 Radio CSS 样式
pub fn generate_radio_css(checked: bool, disabled: bool, is_button: bool) -> String {
    RadioStyleGenerator::new()
        .with_checked(checked)
        .with_disabled(disabled)
        .with_button(is_button)
        .generate_css()
}

/// 生成 Radio 组样式
pub fn generate_radio_group_style(size: RadioSize, button_style: bool, vertical: bool) -> String {
    RadioGroupStyleGenerator::new()
        .with_size(size)
        .with_button_style(button_style)
        .with_vertical(vertical)
        .generate()
}

/// 生成 Radio 组 CSS 样式
pub fn generate_radio_group_css(size: RadioSize, button_style: bool, vertical: bool) -> String {
    RadioGroupStyleGenerator::new()
        .with_size(size)
        .with_button_style(button_style)
        .with_vertical(vertical)
        .generate_css()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radio_size_default() {
        assert_eq!(RadioSize::default(), RadioSize::Middle);
    }

    #[test]
    fn test_radio_size_to_class() {
        assert_eq!(RadioSize::Small.to_class(), "ant-radio-group-small");
        assert_eq!(RadioSize::Middle.to_class(), "");
        assert_eq!(RadioSize::Large.to_class(), "ant-radio-group-large");
    }

    #[test]
    fn test_radio_style_generator() {
        let generator = RadioStyleGenerator::new()
            .with_checked(true)
            .with_disabled(false);

        let style = generator.generate();
        assert!(style.contains("ant-radio-wrapper"));
        assert!(style.contains("ant-radio-wrapper-checked"));
        assert!(!style.contains("ant-radio-wrapper-disabled"));
    }

    #[test]
    fn test_radio_button_style_generator() {
        let generator = RadioStyleGenerator::new()
            .with_checked(true)
            .with_disabled(false)
            .with_button(true)
            .with_size(RadioSize::Small);

        let style = generator.generate();
        assert!(style.contains("ant-radio-button-wrapper"));
        assert!(style.contains("ant-radio-button-wrapper-checked"));
        assert!(style.contains("ant-radio-button-wrapper-sm"));
        assert!(!style.contains("ant-radio-button-wrapper-disabled"));
    }

    #[test]
    fn test_radio_group_style_generator() {
        let generator = RadioGroupStyleGenerator::new()
            .with_size(RadioSize::Large)
            .with_button_style(true)
            .with_vertical(true);

        let style = generator.generate();
        assert!(style.contains("ant-radio-group"));
        assert!(style.contains("ant-radio-group-large"));
        assert!(style.contains("ant-radio-group-button"));
        assert!(style.contains("ant-radio-group-vertical"));
    }
}
