/// Alert 组件样式生成器
///
/// 用于生成 Alert 组件的样式
use css_in_rust::css;

/// Alert 组件类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlertType {
    /// 成功提示
    Success,
    /// 消息通知
    Info,
    /// 警告提示
    Warning,
    /// 错误提示
    Error,
}

impl Default for AlertType {
    fn default() -> Self {
        Self::Info
    }
}

impl AlertType {
    /// 获取类型对应的CSS类名
    pub fn to_class(&self) -> &'static str {
        match self {
            AlertType::Success => "ant-alert-success",
            AlertType::Info => "ant-alert-info",
            AlertType::Warning => "ant-alert-warning",
            AlertType::Error => "ant-alert-error",
        }
    }

    /// 获取默认图标
    pub fn default_icon(&self) -> &'static str {
        match self {
            AlertType::Success => "✓",
            AlertType::Info => "ℹ",
            AlertType::Warning => "⚠",
            AlertType::Error => "✕",
        }
    }

    /// 获取类型对应的颜色
    pub fn get_color(&self) -> &'static str {
        match self {
            AlertType::Success => "#52c41a",
            AlertType::Info => "#1677ff",
            AlertType::Warning => "#faad14",
            AlertType::Error => "#ff4d4f",
        }
    }

    /// 获取类型对应的背景色
    pub fn get_background_color(&self) -> &'static str {
        match self {
            AlertType::Success => "#f6ffed",
            AlertType::Info => "#e6f4ff",
            AlertType::Warning => "#fffbe6",
            AlertType::Error => "#fff2f0",
        }
    }

    /// 获取类型对应的边框色
    pub fn get_border_color(&self) -> &'static str {
        match self {
            AlertType::Success => "#b7eb8f",
            AlertType::Info => "#91caff",
            AlertType::Warning => "#ffe58f",
            AlertType::Error => "#ffccc7",
        }
    }
}

/// Alert 样式生成器
pub struct AlertStyleGenerator {
    /// Alert类型
    pub alert_type: AlertType,
    /// 是否显示图标
    pub show_icon: bool,
    /// 是否有描述
    pub has_description: bool,
    /// 是否可关闭
    pub closable: bool,
    /// 是否用作顶部公告
    pub banner: bool,
    /// 是否无边框
    pub no_border: bool,
    /// 是否使用描边样式
    pub outlined: bool,
}

impl Default for AlertStyleGenerator {
    fn default() -> Self {
        Self {
            alert_type: AlertType::default(),
            show_icon: false,
            has_description: false,
            closable: false,
            banner: false,
            no_border: false,
            outlined: false,
        }
    }
}

impl AlertStyleGenerator {
    /// 创建新的样式生成器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置Alert类型
    pub fn with_type(mut self, alert_type: AlertType) -> Self {
        self.alert_type = alert_type;
        self
    }

    /// 设置是否显示图标
    pub fn with_icon(mut self, show_icon: bool) -> Self {
        self.show_icon = show_icon;
        self
    }

    /// 设置是否有描述
    pub fn with_description(mut self, has_description: bool) -> Self {
        self.has_description = has_description;
        self
    }

    /// 设置是否可关闭
    pub fn with_closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    /// 设置是否用作顶部公告
    pub fn with_banner(mut self, banner: bool) -> Self {
        self.banner = banner;
        self
    }

    /// 设置是否无边框
    pub fn with_no_border(mut self, no_border: bool) -> Self {
        self.no_border = no_border;
        self
    }

    /// 设置是否使用描边样式
    pub fn with_outlined(mut self, outlined: bool) -> Self {
        self.outlined = outlined;
        self
    }

    /// 生成样式
    pub fn generate(&self) -> String {
        let mut classes = vec!["ant-alert"];

        // 添加类型类名
        classes.push(self.alert_type.to_class());

        // 条件类名
        if self.show_icon {
            classes.push("ant-alert-with-icon");
        }

        if self.has_description {
            classes.push("ant-alert-with-description");
        }

        if self.banner {
            classes.push("ant-alert-banner");
        }

        if self.closable {
            classes.push("ant-alert-closable");
        }

        if self.no_border {
            classes.push("ant-alert-no-border");
        }

        if self.outlined {
            classes.push("ant-alert-outlined");
        }

        // 使用css-in-rust的css宏来生成CSS样式字符串
        let base_style = css!(
            r#"
            box-sizing: border-box;
            margin: 0;
            padding: 0;
            color: rgba(0, 0, 0, 0.88);
            font-size: 14px;
            line-height: 1.5714285714285714;
            list-style: none;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif;
            position: relative;
            display: flex;
            align-items: flex-start;
            padding: 8px 15px;
            word-wrap: break-word;
            border-radius: 6px;
            margin-bottom: 16px;
            transition: all 0.3s;
            background-color: ${background_color};
            border: 1px solid ${border_color};
        "#,
            background_color = self.alert_type.get_background_color(),
            border_color = self.alert_type.get_border_color()
        );

        // 如果是描边样式，添加额外的样式
        let outlined_style = if self.outlined {
            css!(
                r#"
                background-color: transparent;
                color: ${color};
                border-color: ${color};
            "#,
                color = self.alert_type.get_color()
            )
        } else {
            "".to_string()
        };

        // 如果是banner模式，添加额外的样式
        let banner_style = if self.banner {
            css!(
                r#"
                margin-bottom: 0;
                border: 0;
                border-radius: 0;
            "#
            )
        } else {
            "".to_string()
        };

        // 如果无边框模式，添加额外的样式
        let no_border_style = if self.no_border {
            css!(r#"border: none;"#)
        } else {
            "".to_string()
        };

        // 组合所有样式
        format!(
            "{} {} {} {} {}",
            classes.join(" "),
            base_style,
            outlined_style,
            banner_style,
            no_border_style
        )
    }
}

/// 生成 Alert 组件的基础 CSS
pub fn alert_base_css() -> String {
    css!(
        r#"
        .ant-alert {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
            color: rgba(0, 0, 0, 0.88);
            font-size: 14px;
            line-height: 1.5714285714285714;
            list-style: none;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif;
            position: relative;
            display: flex;
            align-items: flex-start;
            padding: 8px 15px;
            word-wrap: break-word;
            border-radius: 6px;
            margin-bottom: 16px;
            transition: all 0.3s;
        }

        .ant-alert-success {
            background-color: #f6ffed;
            border: 1px solid #b7eb8f;
        }

        .ant-alert-info {
            background-color: #e6f4ff;
            border: 1px solid #91caff;
        }

        .ant-alert-warning {
            background-color: #fffbe6;
            border: 1px solid #ffe58f;
        }

        .ant-alert-error {
            background-color: #fff2f0;
            border: 1px solid #ffccc7;
        }

        .ant-alert-success .ant-alert-icon {
            color: #52c41a;
        }

        .ant-alert-info .ant-alert-icon {
            color: #1677ff;
        }

        .ant-alert-warning .ant-alert-icon {
            color: #faad14;
        }

        .ant-alert-error .ant-alert-icon {
            color: #ff4d4f;
        }

        .ant-alert-icon {
            margin-right: 8px;
            margin-top: 1px;
            line-height: 0;
        }

        .ant-alert-content {
            flex: 1;
            min-width: 0;
        }

        .ant-alert-message {
            color: rgba(0, 0, 0, 0.88);
            font-size: 14px;
            line-height: 1.5714285714285714;
        }

        .ant-alert-description {
            color: rgba(0, 0, 0, 0.65);
            font-size: 14px;
            line-height: 1.5714285714285714;
            margin-top: 4px;
        }

        .ant-alert-with-icon {
            padding: 8px 15px 8px 38px;
        }

        .ant-alert-with-icon .ant-alert-icon {
            position: absolute;
            left: 16px;
            top: 10px;
        }

        .ant-alert-with-description {
            align-items: flex-start;
            padding: 15px 15px 15px 64px;
        }

        .ant-alert-with-description .ant-alert-icon {
            position: absolute;
            left: 24px;
            top: 16px;
            font-size: 24px;
        }

        .ant-alert-with-description .ant-alert-message {
            font-size: 16px;
            line-height: 1.5;
            margin-bottom: 4px;
            display: block;
        }

        .ant-alert-close-icon {
            position: absolute;
            top: 8px;
            right: 16px;
            padding: 0;
            overflow: hidden;
            font-size: 12px;
            line-height: 12px;
            background-color: transparent;
            border: none;
            outline: none;
            cursor: pointer;
            color: rgba(0, 0, 0, 0.45);
            transition: color 0.3s;
        }

        .ant-alert-close-icon:hover {
            color: rgba(0, 0, 0, 0.75);
        }

        .ant-alert-closable {
            padding-right: 37px;
        }

        .ant-alert-banner {
            margin-bottom: 0;
            border: 0;
            border-radius: 0;
        }

        .ant-alert-action {
            margin-left: 8px;
            margin-right: -4px;
        }

        .ant-alert-no-border {
            border: none;
        }

        .ant-alert-outlined {
            background-color: transparent;
        }

        .ant-alert-outlined.ant-alert-success {
            border-color: #52c41a;
            color: #52c41a;
        }

        .ant-alert-outlined.ant-alert-info {
            border-color: #1677ff;
            color: #1677ff;
        }

        .ant-alert-outlined.ant-alert-warning {
            border-color: #faad14;
            color: #faad14;
        }

        .ant-alert-outlined.ant-alert-error {
            border-color: #ff4d4f;
            color: #ff4d4f;
        }
    "#
    )
}

/// 在组件中使用此方法来确保样式已注入到DOM
pub fn use_alert_style() {
    use dioxus::prelude::*;

    // 在组件首次渲染时注入样式
    use_effect(move || {
        // 样式只会被注入一次
        static STYLE_INJECTED: std::sync::OnceLock<()> = std::sync::OnceLock::new();

        STYLE_INJECTED.get_or_init(|| {
            // 使用css-in-rust自动注入样式
            // 直接调用alert_base_css会触发css宏的样式注入逻辑
            let _ = alert_base_css();
        });
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_type_default() {
        assert_eq!(AlertType::default(), AlertType::Info);
    }

    #[test]
    fn test_alert_type_to_class() {
        assert_eq!(AlertType::Success.to_class(), "ant-alert-success");
        assert_eq!(AlertType::Info.to_class(), "ant-alert-info");
        assert_eq!(AlertType::Warning.to_class(), "ant-alert-warning");
        assert_eq!(AlertType::Error.to_class(), "ant-alert-error");
    }

    #[test]
    fn test_alert_style_generator() {
        let style_gen = AlertStyleGenerator::new()
            .with_type(AlertType::Success)
            .with_icon(true)
            .with_description(true)
            .with_closable(true);

        let style = style_gen.generate();
        assert!(style.contains("ant-alert"));
        assert!(style.contains("ant-alert-success"));
        assert!(style.contains("ant-alert-with-icon"));
        assert!(style.contains("ant-alert-with-description"));
        assert!(style.contains("ant-alert-closable"));
    }

    #[test]
    fn test_outlined_style() {
        let style_gen = AlertStyleGenerator::new()
            .with_type(AlertType::Error)
            .with_outlined(true);

        let style = style_gen.generate();
        assert!(style.contains("background-color: transparent"));
        assert!(style.contains("ant-alert-outlined"));
    }
}
