use crate::theme::AliasToken;
/// Checkbox 组件样式生成器
///
/// 用于生成 Checkbox 组件的样式
use css_in_rust::css;

/// Checkbox 尺寸
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CheckboxSize {
    /// 大尺寸
    Large,
    /// 中等尺寸（默认）
    Middle,
    /// 小尺寸
    Small,
}

impl Default for CheckboxSize {
    fn default() -> Self {
        Self::Middle
    }
}

impl CheckboxSize {
    /// 获取尺寸对应的CSS类名
    pub fn to_class(&self) -> String {
        match self {
            CheckboxSize::Large => "ant-checkbox-lg".to_string(),
            CheckboxSize::Middle => "".to_string(),
            CheckboxSize::Small => "ant-checkbox-sm".to_string(),
        }
    }

    /// 获取尺寸对应的大小数值
    pub fn get_size_value(&self) -> u8 {
        match self {
            CheckboxSize::Large => 18,
            CheckboxSize::Middle => 16,
            CheckboxSize::Small => 14,
        }
    }
}

/// Checkbox 样式生成器
pub struct CheckboxStyleGenerator {
    /// 是否选中
    pub checked: bool,
    /// 是否禁用
    pub disabled: bool,
    /// 是否为不确定状态
    pub indeterminate: bool,
    /// 复选框尺寸
    pub size: CheckboxSize,
    /// 主题令牌
    pub token: AliasToken,
}

impl Default for CheckboxStyleGenerator {
    fn default() -> Self {
        Self {
            checked: false,
            disabled: false,
            indeterminate: false,
            size: CheckboxSize::default(),
            token: AliasToken::default(),
        }
    }
}

impl CheckboxStyleGenerator {
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

    /// 设置是否为不确定状态
    pub fn with_indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
        self
    }

    /// 设置复选框尺寸
    pub fn with_size(mut self, size: CheckboxSize) -> Self {
        self.size = size;
        self
    }

    /// 设置主题令牌
    pub fn with_token(mut self, token: AliasToken) -> Self {
        self.token = token;
        self
    }

    /// 生成样式类名
    pub fn generate(&self) -> String {
        let mut classes = vec!["ant-checkbox-wrapper".to_string()];
        let mut checkbox_classes = vec!["ant-checkbox".to_string()];

        // 添加尺寸类名
        let size_class = self.size.to_class();
        if !size_class.is_empty() {
            classes.push(size_class);
        }

        // 条件类名
        if self.disabled {
            classes.push("ant-checkbox-wrapper-disabled".to_string());
            checkbox_classes.push("ant-checkbox-disabled".to_string());
        }

        if self.checked {
            checkbox_classes.push("ant-checkbox-checked".to_string());
        }

        if self.indeterminate {
            checkbox_classes.push("ant-checkbox-indeterminate".to_string());
        }

        let wrapper_class = classes.join(" ");
        let checkbox_class = checkbox_classes.join(" ");

        format!(
            "{} {} {}",
            wrapper_class,
            "data-checkbox-inner-class".to_string(),
            checkbox_class
        )
    }

    /// 生成 CSS 样式
    pub fn generate_css(&self) -> String {
        css!(
            r#"
            .ant-checkbox-wrapper {
                box-sizing: border-box;
                margin: 0;
                padding: 0;
                color: ${color_text};
                font-size: ${font_size}px;
                line-height: ${line_height};
                list-style: none;
                font-family: ${font_family};
                display: inline-flex;
                align-items: baseline;
                cursor: pointer;
            }

            .ant-checkbox {
                box-sizing: border-box;
                margin: 0;
                padding: 0;
                color: ${color_text};
                font-size: ${font_size}px;
                line-height: 1;
                list-style: none;
                font-family: ${font_family};
                position: relative;
                top: 0.2em;
                display: inline-block;
                margin-right: 8px;
                white-space: nowrap;
                vertical-align: middle;
                outline: none;
                cursor: pointer;
            }

            .ant-checkbox-input {
                position: absolute;
                inset: 0;
                z-index: 1;
                width: 100%;
                height: 100%;
                cursor: pointer;
                opacity: 0;
            }

            .ant-checkbox-inner {
                position: relative;
                top: 0;
                left: 0;
                display: block;
                width: ${checkbox_size}px;
                height: ${checkbox_size}px;
                direction: ltr;
                background-color: ${bg_color_container};
                border: 1px solid ${border_color};
                border-radius: ${border_radius_sm}px;
                border-collapse: separate;
                transition: all 0.3s;
            }

            .ant-checkbox-checked .ant-checkbox-inner {
                background-color: ${color_primary};
                border-color: ${color_primary};
            }

            .ant-checkbox-checked .ant-checkbox-inner::after {
                position: absolute;
                display: table;
                border: 2px solid #fff;
                border-top: 0;
                border-left: 0;
                transform: rotate(45deg) scale(1) translate(-50%, -50%);
                opacity: 1;
                transition: all 0.2s cubic-bezier(0.12, 0.4, 0.29, 1.46) 0.1s;
                content: '';
                top: 50%;
                left: 22%;
                width: 5.71428571px;
                height: 9.14285714px;
            }

            .ant-checkbox-indeterminate .ant-checkbox-inner::after {
                top: 50%;
                left: 50%;
                width: 8px;
                height: 8px;
                background-color: ${color_primary};
                border: 0;
                transform: translate(-50%, -50%) scale(1);
                opacity: 1;
                content: '';
            }

            .ant-checkbox-disabled {
                cursor: not-allowed;
            }

            .ant-checkbox-disabled .ant-checkbox-inner {
                background-color: ${color_bg_container_disabled};
                border-color: ${border_color_disabled} !important;
            }

            .ant-checkbox-disabled.ant-checkbox-checked .ant-checkbox-inner::after {
                border-color: ${color_text_disabled};
            }

            .ant-checkbox-disabled + span {
                color: ${color_text_disabled};
                cursor: not-allowed;
            }

            .ant-checkbox-wrapper-disabled {
                cursor: not-allowed;
            }

            .ant-checkbox-lg .ant-checkbox-inner {
                width: 18px;
                height: 18px;
            }

            .ant-checkbox-sm .ant-checkbox-inner {
                width: 14px;
                height: 14px;
            }
            "#,
            color_text = self.token.color_text,
            font_size = self.token.font_size,
            line_height = self.token.line_height,
            font_family = self.token.font_family,
            checkbox_size = self.size.get_size_value(),
            bg_color_container = self.token.color_bg_container,
            border_color = self.token.color_border,
            border_radius_sm = self.token.border_radius_sm,
            color_primary = self.token.color_primary,
            color_bg_container_disabled = self.token.color_bg_container_disabled,
            border_color_disabled = self.token.color_border_disabled,
            color_text_disabled = self.token.color_text_disabled
        )
        .to_string()
    }
}

/// 生成 Checkbox 样式
pub fn generate_checkbox_style(
    checked: bool,
    disabled: bool,
    indeterminate: bool,
    size: CheckboxSize,
) -> String {
    CheckboxStyleGenerator::new()
        .with_checked(checked)
        .with_disabled(disabled)
        .with_indeterminate(indeterminate)
        .with_size(size)
        .generate()
}

/// 生成 Checkbox CSS 样式
pub fn generate_checkbox_css(
    checked: bool,
    disabled: bool,
    indeterminate: bool,
    size: CheckboxSize,
) -> String {
    CheckboxStyleGenerator::new()
        .with_checked(checked)
        .with_disabled(disabled)
        .with_indeterminate(indeterminate)
        .with_size(size)
        .generate_css()
}

/// 默认 Checkbox 样式
pub fn default_checkbox_style() -> String {
    CheckboxStyleGenerator::new().generate()
}

/// 生成 Checkbox 组件的基础 CSS
pub fn checkbox_base_css() -> String {
    css!(
        r#"
        .ant-checkbox-wrapper {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
            color: rgba(0, 0, 0, 0.88);
            font-size: 14px;
            line-height: 1.5714285714285714;
            list-style: none;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif;
            display: inline-flex;
            align-items: baseline;
            cursor: pointer;
        }

        .ant-checkbox {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
            color: rgba(0, 0, 0, 0.88);
            font-size: 14px;
            line-height: 1;
            list-style: none;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif;
            position: relative;
            top: 0.2em;
            display: inline-block;
            margin-right: 8px;
            white-space: nowrap;
            vertical-align: middle;
            outline: none;
            cursor: pointer;
        }

        .ant-checkbox-input {
            position: absolute;
            inset: 0;
            z-index: 1;
            width: 100%;
            height: 100%;
            cursor: pointer;
            opacity: 0;
        }

        .ant-checkbox-inner {
            position: relative;
            top: 0;
            left: 0;
            display: block;
            width: 16px;
            height: 16px;
            direction: ltr;
            background-color: #ffffff;
            border: 1px solid #d9d9d9;
            border-radius: 2px;
            border-collapse: separate;
            transition: all 0.3s;
        }

        .ant-checkbox-inner::after {
            position: absolute;
            top: 50%;
            left: 21.5%;
            display: table;
            width: 5.71428571px;
            height: 9.14285714px;
            border: 2px solid #fff;
            border-top: 0;
            border-left: 0;
            transform: rotate(45deg) scale(0) translate(-50%, -50%);
            opacity: 0;
            transition: all 0.1s cubic-bezier(0.71, -0.46, 0.88, 0.6), opacity 0.1s;
            content: "";
        }

        .ant-checkbox-checked .ant-checkbox-inner {
            background-color: #1677ff;
            border-color: #1677ff;
        }

        .ant-checkbox-checked .ant-checkbox-inner::after {
            position: absolute;
            display: table;
            border: 2px solid #fff;
            border-top: 0;
            border-left: 0;
            transform: rotate(45deg) scale(1) translate(-50%, -50%);
            opacity: 1;
            transition: all 0.2s cubic-bezier(0.12, 0.4, 0.29, 1.46) 0.1s;
            content: "";
        }

        .ant-checkbox-indeterminate .ant-checkbox-inner {
            background-color: #fff;
            border-color: #d9d9d9;
        }

        .ant-checkbox-indeterminate .ant-checkbox-inner::after {
            top: 50%;
            left: 50%;
            width: 8px;
            height: 8px;
            background-color: #1677ff;
            border: 0;
            transform: translate(-50%, -50%) scale(1);
            opacity: 1;
            content: "";
        }

        .ant-checkbox-disabled {
            cursor: not-allowed;
        }

        .ant-checkbox-disabled.ant-checkbox-checked .ant-checkbox-inner {
            background-color: rgba(0, 0, 0, 0.25);
            border-color: #d9d9d9 !important;
        }

        .ant-checkbox-disabled .ant-checkbox-input {
            cursor: not-allowed;
        }

        .ant-checkbox-disabled .ant-checkbox-inner {
            background-color: rgba(0, 0, 0, 0.04);
            border-color: #d9d9d9 !important;
        }

        .ant-checkbox-disabled + span {
            color: rgba(0, 0, 0, 0.25);
            cursor: not-allowed;
        }

        .ant-checkbox-wrapper-disabled {
            cursor: not-allowed;
        }

        .ant-checkbox-label {
            padding-right: 8px;
            padding-left: 8px;
        }

        .ant-checkbox-group {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
            color: rgba(0, 0, 0, 0.88);
            font-size: 14px;
            line-height: 1.5714285714285714;
            list-style: none;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif;
            display: inline-block;
        }

        .ant-checkbox-group-item {
            display: inline-block;
            margin-right: 8px;
        }

        .ant-checkbox-group-item:last-child {
            margin-right: 0;
        }

        .ant-checkbox-lg .ant-checkbox-inner {
            width: 18px;
            height: 18px;
        }

        .ant-checkbox-sm .ant-checkbox-inner {
            width: 14px;
            height: 14px;
        }
    "#
    ).to_string()
}

/// 生成 Checkbox 组件组的样式
pub fn checkbox_group_style() -> String {
    css!(
        r#"
        .ant-checkbox-group {
            display: inline-flex;
            flex-wrap: wrap;
            gap: 8px;
        }

        .ant-checkbox-group-vertical {
            flex-direction: column;
        }
    "#
    )
    .to_string()
}

/// 在组件中使用此方法来确保样式已注入到DOM
pub fn use_checkbox_style() {
    use dioxus::prelude::*;

    // 在组件首次渲染时注入样式
    use_effect(move || {
        // 样式只会被注入一次
        static STYLE_INJECTED: std::sync::OnceLock<()> = std::sync::OnceLock::new();

        STYLE_INJECTED.get_or_init(|| {
            // 使用css-in-rust自动注入样式
            // 直接调用checkbox_base_css会触发css宏的样式注入逻辑
            let _ = checkbox_base_css();
            let _ = checkbox_group_style();
        });
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checkbox_size_default() {
        assert_eq!(CheckboxSize::default(), CheckboxSize::Middle);
    }

    #[test]
    fn test_checkbox_size_to_class() {
        assert_eq!(CheckboxSize::Large.to_class(), "ant-checkbox-lg");
        assert_eq!(CheckboxSize::Middle.to_class(), "");
        assert_eq!(CheckboxSize::Small.to_class(), "ant-checkbox-sm");
    }

    #[test]
    fn test_checkbox_style_generator() {
        let style_gen = CheckboxStyleGenerator::new()
            .with_checked(true)
            .with_disabled(true)
            .with_indeterminate(false)
            .with_size(CheckboxSize::Large);

        let style = style_gen.generate();
        assert!(style.contains("ant-checkbox-wrapper"));
        assert!(style.contains("ant-checkbox-lg"));
        assert!(style.contains("ant-checkbox-wrapper-disabled"));
        assert!(style.contains("ant-checkbox"));
        assert!(style.contains("ant-checkbox-checked"));
        assert!(style.contains("ant-checkbox-disabled"));
    }
}
