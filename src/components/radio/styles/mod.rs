/// Radio 组件样式生成器
///
/// 用于生成 Radio 组件的样式
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
}

impl Default for RadioStyleGenerator {
    fn default() -> Self {
        Self {
            checked: false,
            disabled: false,
            size: RadioSize::default(),
            is_button: false,
            button_style: RadioButtonStyle::default(),
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

    /// 生成样式
    pub fn generate(&self) -> String {
        if self.is_button {
            self.generate_button_style()
        } else {
            self.generate_radio_style()
        }
    }

    /// 生成标准Radio样式
    fn generate_radio_style(&self) -> String {
        let mut classes = vec!["ant-radio-wrapper"];

        if self.disabled {
            classes.push("ant-radio-wrapper-disabled");
        }

        if self.checked {
            classes.push("ant-radio-wrapper-checked");
        }

        let wrapper_class = classes.join(" ");

        let inner_class = if self.checked {
            if self.disabled {
                "ant-radio ant-radio-checked ant-radio-disabled"
            } else {
                "ant-radio ant-radio-checked"
            }
        } else if self.disabled {
            "ant-radio ant-radio-disabled"
        } else {
            "ant-radio"
        };

        // 使用css-in-rust的css宏来生成CSS样式字符串
        let style = css!(
            r#"
            /* 样式将由全局样式生成器提供 */
        "#
        );

        format!(
            "{} {} {} {}",
            wrapper_class, style, "data-radio-inner-class", inner_class
        )
    }

    /// 生成按钮样式Radio
    fn generate_button_style(&self) -> String {
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

        let wrapper_class = classes.join(" ");

        let inner_class = if self.checked {
            if self.disabled {
                "ant-radio-button ant-radio-button-checked ant-radio-button-disabled"
            } else {
                "ant-radio-button ant-radio-button-checked"
            }
        } else if self.disabled {
            "ant-radio-button ant-radio-button-disabled"
        } else {
            "ant-radio-button"
        };

        // 使用css-in-rust的css宏来生成CSS样式字符串
        let style = css!(
            r#"
            /* 样式将由全局样式生成器提供 */
        "#
        );

        format!(
            "{} {} {} {}",
            wrapper_class, style, "data-radio-inner-class", inner_class
        )
    }
}

/// Radio Group 样式生成器
pub struct RadioGroupStyleGenerator {
    /// Radio组尺寸
    pub size: RadioSize,
    /// 是否使用按钮样式
    pub button_style: bool,
    /// 是否纵向排列
    pub vertical: bool,
}

impl Default for RadioGroupStyleGenerator {
    fn default() -> Self {
        Self {
            size: RadioSize::default(),
            button_style: false,
            vertical: false,
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

    /// 生成样式
    pub fn generate(&self) -> String {
        let mut classes = vec!["ant-radio-group"];

        let size_class = self.size.to_class();
        if !size_class.is_empty() {
            classes.push(size_class);
        }

        if self.button_style {
            classes.push("ant-radio-group-button-style");
        }

        if self.vertical {
            classes.push("ant-radio-group-vertical");
        }

        let group_class = classes.join(" ");

        // 使用css-in-rust的css宏来生成CSS样式字符串
        let style = css!(
            r#"
            /* 样式将由全局样式生成器提供 */
        "#
        );

        format!("{} {}", group_class, style)
    }
}

/// 生成 Radio 组件的基础 CSS
pub fn radio_base_css() -> String {
    css!(
        r#"
        .ant-radio-wrapper {
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

        .ant-radio {
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

        .ant-radio-input {
            position: absolute;
            inset: 0;
            z-index: 1;
            width: 100%;
            height: 100%;
            cursor: pointer;
            opacity: 0;
        }

        .ant-radio-inner {
            position: relative;
            top: 0;
            left: 0;
            display: block;
            width: 16px;
            height: 16px;
            direction: ltr;
            background-color: #ffffff;
            border: 1px solid #d9d9d9;
            border-radius: 50%;
            transition: all 0.3s;
        }

        .ant-radio-inner::after {
            position: absolute;
            top: 50%;
            left: 50%;
            display: block;
            width: 16px;
            height: 16px;
            margin-top: -8px;
            margin-left: -8px;
            background-color: #1677ff;
            border-top: 0;
            border-left: 0;
            border-radius: 16px;
            transform: scale(0);
            opacity: 0;
            transition: all 0.3s cubic-bezier(0.78, 0.14, 0.15, 0.86);
            content: " ";
        }

        .ant-radio-checked .ant-radio-inner {
            border-color: #1677ff;
        }

        .ant-radio-checked .ant-radio-inner::after {
            transform: scale(0.5);
            opacity: 1;
            transition: all 0.3s cubic-bezier(0.78, 0.14, 0.15, 0.86);
        }

        .ant-radio-wrapper-disabled {
            cursor: not-allowed;
        }

        .ant-radio-disabled {
            cursor: not-allowed;
        }

        .ant-radio-disabled .ant-radio-input {
            cursor: not-allowed;
        }

        .ant-radio-disabled .ant-radio-inner {
            background-color: rgba(0, 0, 0, 0.04);
            cursor: not-allowed;
            border-color: #d9d9d9 !important;
        }

        .ant-radio-disabled .ant-radio-inner::after {
            background-color: rgba(0, 0, 0, 0.2);
        }

        .ant-radio-disabled + span {
            color: rgba(0, 0, 0, 0.25);
            cursor: not-allowed;
        }

        .ant-radio-wrapper-checked .ant-radio-wrapper-disabled {
            cursor: not-allowed;
        }

        .ant-radio-wrapper-checked .ant-radio-wrapper-disabled .ant-radio-inner::after {
            transform: scale(0.5);
            opacity: 1;
            background-color: rgba(0, 0, 0, 0.2);
        }

        .ant-radio-label {
            padding-right: 8px;
            padding-left: 8px;
        }

        /* Radio Group */
        .ant-radio-group {
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

        .ant-radio-group-large .ant-radio-wrapper {
            font-size: 16px;
        }

        .ant-radio-group-small .ant-radio-wrapper {
            font-size: 12px;
        }

        .ant-radio-group-vertical {
            display: flex;
            flex-direction: column;
        }

        /* Radio Buttons */
        .ant-radio-button-wrapper {
            position: relative;
            display: inline-block;
            height: 32px;
            margin: 0;
            padding: 0 15px;
            color: rgba(0, 0, 0, 0.88);
            font-size: 14px;
            line-height: 30px;
            background: #ffffff;
            border: 1px solid #d9d9d9;
            border-left: 0;
            transition: all 0.3s;
            cursor: pointer;
        }

        .ant-radio-button-wrapper:first-child {
            border-left: 1px solid #d9d9d9;
            border-radius: 6px 0 0 6px;
        }

        .ant-radio-button-wrapper:last-child {
            border-radius: 0 6px 6px 0;
        }

        .ant-radio-button-wrapper-checked {
            z-index: 1;
            color: #1677ff;
            background: #ffffff;
            border-color: #1677ff;
        }

        .ant-radio-button-wrapper-checked::before {
            background-color: #1677ff;
        }

        .ant-radio-button-wrapper-disabled {
            color: rgba(0, 0, 0, 0.25);
            background-color: rgba(0, 0, 0, 0.04);
            cursor: not-allowed;
            border-color: #d9d9d9;
        }

        .ant-radio-button-wrapper-disabled:first-child,
        .ant-radio-button-wrapper-disabled:hover {
            color: rgba(0, 0, 0, 0.25);
            background-color: rgba(0, 0, 0, 0.04);
            border-color: #d9d9d9;
        }

        .ant-radio-button-wrapper-disabled.ant-radio-button-wrapper-checked {
            color: rgba(0, 0, 0, 0.25);
            background-color: rgba(0, 0, 0, 0.04);
            border-color: #d9d9d9;
        }

        .ant-radio-button-wrapper-lg {
            height: 40px;
            font-size: 16px;
            line-height: 38px;
        }

        .ant-radio-button-wrapper-sm {
            height: 24px;
            padding: 0 7px;
            font-size: 12px;
            line-height: 22px;
        }

        .ant-radio-button-wrapper-solid.ant-radio-button-wrapper-checked {
            color: #fff;
            background: #1677ff;
            border-color: #1677ff;
        }
    "#
    )
}

/// 在组件中使用此方法来确保样式已注入到DOM
pub fn use_radio_style() {
    use dioxus::prelude::*;

    // 在组件首次渲染时注入样式
    use_effect(move || {
        // 样式只会被注入一次
        static STYLE_INJECTED: std::sync::OnceLock<()> = std::sync::OnceLock::new();

        STYLE_INJECTED.get_or_init(|| {
            // 使用css-in-rust自动注入样式
            // 直接调用radio_base_css会触发css宏的样式注入逻辑
            let _ = radio_base_css();
        });
    });
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
        let style_gen = RadioStyleGenerator::new()
            .with_checked(true)
            .with_disabled(false)
            .with_size(RadioSize::Large);

        let style = style_gen.generate();
        assert!(style.contains("ant-radio-wrapper"));
        assert!(style.contains("ant-radio-wrapper-checked"));
        assert!(style.contains("ant-radio-checked"));
    }

    #[test]
    fn test_radio_button_style_generator() {
        let style_gen = RadioStyleGenerator::new()
            .with_checked(true)
            .with_disabled(false)
            .with_button(true)
            .with_size(RadioSize::Small)
            .with_button_style(RadioButtonStyle::Solid);

        let style = style_gen.generate();
        assert!(style.contains("ant-radio-button-wrapper"));
        assert!(style.contains("ant-radio-button-wrapper-checked"));
        assert!(style.contains("ant-radio-button-wrapper-sm"));
        assert!(style.contains("ant-radio-button-wrapper-solid"));
    }

    #[test]
    fn test_radio_group_style_generator() {
        let style_gen = RadioGroupStyleGenerator::new()
            .with_size(RadioSize::Large)
            .with_button_style(true)
            .with_vertical(true);

        let style = style_gen.generate();
        assert!(style.contains("ant-radio-group"));
        assert!(style.contains("ant-radio-group-large"));
        assert!(style.contains("ant-radio-group-button-style"));
        assert!(style.contains("ant-radio-group-vertical"));
    }
}
