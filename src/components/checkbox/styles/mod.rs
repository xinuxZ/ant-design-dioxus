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
    pub fn to_class(&self) -> &'static str {
        match self {
            CheckboxSize::Large => "ant-checkbox-lg",
            CheckboxSize::Middle => "",
            CheckboxSize::Small => "ant-checkbox-sm",
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
}

impl Default for CheckboxStyleGenerator {
    fn default() -> Self {
        Self {
            checked: false,
            disabled: false,
            indeterminate: false,
            size: CheckboxSize::default(),
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

    /// 生成样式
    pub fn generate(&self) -> String {
        let mut classes = vec!["ant-checkbox-wrapper"];
        let mut checkbox_classes = vec!["ant-checkbox"];

        // 添加尺寸类名
        let size_class = self.size.to_class();
        if !size_class.is_empty() {
            classes.push(size_class);
        }

        // 条件类名
        if self.disabled {
            classes.push("ant-checkbox-wrapper-disabled");
            checkbox_classes.push("ant-checkbox-disabled");
        }

        if self.checked {
            checkbox_classes.push("ant-checkbox-checked");
        }

        if self.indeterminate {
            checkbox_classes.push("ant-checkbox-indeterminate");
        }

        let wrapper_class = classes.join(" ");
        let checkbox_class = checkbox_classes.join(" ");

        // 使用css-in-rust的css宏来生成CSS样式字符串
        let style = css!(
            r#"
            /* 样式将由全局样式生成器提供 */
        "#
        );

        format!(
            "{} {} {} {}",
            wrapper_class, style, "data-checkbox-inner-class", checkbox_class
        )
    }
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
    )
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
