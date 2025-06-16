use css_in_rust::theme::{core::css::CssGenerator, theme_types::Theme};

use crate::theme::adapter::provider::ThemeProviderAdapter;

/// Dioxus 框架适配器
///
/// 提供与 Dioxus 框架的集成，包括样式生成、主题管理和组件样式化功能。
///
/// # Examples
///
/// ```
/// use css_in_rust::theme::adapter::{DioxusAdapter, provider::ThemeProviderAdapter};
/// use css_in_rust::theme::core::manager::ThemeManager;
/// use std::sync::Arc;
///
/// // 创建主题管理器和提供者
/// let manager = Arc::new(ThemeManager::default());
/// let provider = ThemeProviderAdapter::default();
///
/// // 创建 Dioxus 适配器
/// let adapter = DioxusAdapter::new(provider);
///
/// // 生成组件样式
/// let style = adapter.style_component("button", "color: blue; padding: 8px 16px;");
/// println!("生成的类名: {}", style.class_name);
/// println!("生成的CSS: {}", style.css);
/// ```
#[derive(Debug, Clone)]
pub struct DioxusAdapter {
    /// 主题提供者
    provider: ThemeProviderAdapter,
    /// CSS 生成器
    css_generator: CssGenerator,

    /// 是否启用自动注入
    auto_inject: bool,
}

/// Dioxus 组件样式
///
/// 表示一个 Dioxus 组件的样式信息，包括类名、CSS 内容和组件名称。
///
/// # Examples
///
/// ```
/// use css_in_rust::theme::adapter::frameworks::dioxus::DioxusComponentStyle;
///
/// // 创建组件样式
/// let style = DioxusComponentStyle {
///     class_name: "css-abc123".to_string(),
///     css: ".css-abc123 { color: blue; }".to_string(),
///     component_name: "button".to_string(),
/// };
///
/// // 使用组件样式
/// println!("类名: {}", style.class_name);
/// println!("CSS: {}", style.css);
/// ```
#[derive(Debug, Clone)]
pub struct DioxusComponentStyle {
    /// 类名
    pub class_name: String,
    /// CSS 内容
    pub css: String,
    /// 组件名称
    pub component_name: String,
}

impl DioxusAdapter {
    /// 创建新的 Dioxus 适配器
    ///
    /// # Arguments
    ///
    /// * `provider` - 主题提供者适配器
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::adapter::{DioxusAdapter, provider::ThemeProviderAdapter};
    ///
    /// // 创建主题提供者
    /// let provider = ThemeProviderAdapter::default();
    ///
    /// // 创建 Dioxus 适配器
    /// let adapter = DioxusAdapter::new(provider);
    /// ```
    pub fn new(provider: ThemeProviderAdapter) -> Self {
        Self {
            provider,
            css_generator: CssGenerator::new(),
            auto_inject: true,
        }
    }

    /// 设置是否启用自动注入
    ///
    /// # Arguments
    ///
    /// * `auto_inject` - 是否启用自动注入
    ///
    /// # Returns
    ///
    /// 更新后的适配器实例
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::adapter::{DioxusAdapter, provider::ThemeProviderAdapter};
    ///
    /// let provider = ThemeProviderAdapter::default();
    /// let adapter = DioxusAdapter::new(provider).with_auto_inject(false);
    /// ```
    pub fn with_auto_inject(mut self, auto_inject: bool) -> Self {
        self.auto_inject = auto_inject;
        self
    }

    /// 获取当前主题
    ///
    /// # Returns
    ///
    /// 当前主题，如果没有设置则返回 `None`
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::adapter::{DioxusAdapter, provider::ThemeProviderAdapter};
    ///
    /// let provider = ThemeProviderAdapter::default();
    /// let adapter = DioxusAdapter::new(provider);
    ///
    /// if let Some(theme) = adapter.get_current_theme() {
    ///     println!("当前主题: {}", theme.name);
    /// } else {
    ///     println!("未设置主题");
    /// }
    /// ```
    pub fn get_current_theme(&self) -> Option<Theme> {
        self.provider.get_theme()
    }

    /// 生成组件样式
    ///
    /// # Arguments
    ///
    /// * `component_name` - 组件名称
    /// * `css_rules` - CSS 规则字符串
    ///
    /// # Returns
    ///
    /// 生成的组件样式
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::adapter::{DioxusAdapter, provider::ThemeProviderAdapter};
    ///
    /// let provider = ThemeProviderAdapter::default();
    /// let adapter = DioxusAdapter::new(provider);
    ///
    /// // 生成按钮样式
    /// let button_style = adapter.style_component(
    ///     "button",
    ///     "color: white; background-color: blue; padding: 8px 16px; border-radius: 4px;"
    /// );
    ///
    /// // 在 Dioxus 组件中使用
    /// // <button class={button_style.class_name}>Click me</button>
    /// ```
    pub fn style_component(&self, component_name: &str, css_rules: &str) -> DioxusComponentStyle {
        // 生成唯一类名
        let class_name = format!("css-{}", self.generate_hash(component_name, css_rules));

        // 处理CSS规则
        let processed_css = self.process_css_rules(css_rules, &class_name);

        // 如果启用了自动注入，则注入样式
        if self.auto_inject {
            // 注入样式的逻辑（在实际实现中需要根据Dioxus的API进行适配）
            // 这里只是一个占位符
        }

        DioxusComponentStyle {
            class_name,
            css: processed_css,
            component_name: component_name.to_string(),
        }
    }

    /// 处理CSS规则
    ///
    /// 将CSS规则包装在类选择器中
    ///
    /// # Arguments
    ///
    /// * `css_rules` - CSS 规则字符串
    /// * `class_name` - 类名
    ///
    /// # Returns
    ///
    /// 处理后的CSS字符串
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::adapter::{DioxusAdapter, provider::ThemeProviderAdapter};
    ///
    /// let provider = ThemeProviderAdapter::default();
    /// let adapter = DioxusAdapter::new(provider);
    ///
    /// let processed_css = adapter.process_css_rules(
    ///     "color: blue; font-size: 16px;",
    ///     "my-class"
    /// );
    /// assert_eq!(processed_css, ".my-class { color: blue; font-size: 16px; }");
    /// ```
    fn process_css_rules(&self, css_rules: &str, class_name: &str) -> String {
        // 将CSS规则包装在类选择器中
        format!(".{} {{ {} }}", class_name, css_rules)
    }

    /// 生成哈希
    ///
    /// 基于组件名称、CSS规则和当前主题生成唯一哈希
    ///
    /// # Arguments
    ///
    /// * `component_name` - 组件名称
    /// * `css_rules` - CSS 规则字符串
    ///
    /// # Returns
    ///
    /// 生成的哈希字符串
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::adapter::{DioxusAdapter, provider::ThemeProviderAdapter};
    ///
    /// let provider = ThemeProviderAdapter::default();
    /// let adapter = DioxusAdapter::new(provider);
    ///
    /// let hash = adapter.generate_hash("button", "color: blue;");
    /// ```
    fn generate_hash(&self, component_name: &str, css_rules: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        component_name.hash(&mut hasher);
        css_rules.hash(&mut hasher);

        if let Some(theme) = self.get_current_theme() {
            theme.name.hash(&mut hasher);
        }

        format!("{:x}", hasher.finish())
    }

    /// 切换主题
    ///
    /// 在亮色和暗色主题之间切换
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::adapter::{DioxusAdapter, provider::ThemeProviderAdapter};
    ///
    /// let provider = ThemeProviderAdapter::default();
    /// let mut adapter = DioxusAdapter::new(provider);
    ///
    /// // 切换主题
    /// adapter.toggle_theme();
    /// ```
    pub fn toggle_theme(&mut self) {
        if let Err(e) = self.provider.toggle_theme_mode() {
            eprintln!("切换主题失败: {}", e);
        }
    }

    /// 应用主题配置
    ///
    /// # Arguments
    ///
    /// * `config` - 主题配置
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::adapter::{DioxusAdapter, provider::ThemeProviderAdapter};
    /// use css_in_rust::theme::core::types::ThemeConfig;
    ///
    /// let provider = ThemeProviderAdapter::default();
    /// let mut adapter = DioxusAdapter::new(provider);
    /// let config = ThemeConfig::default();
    ///
    /// adapter.apply_theme(&config);
    /// ```
    pub fn apply_theme(&self, _config: &crate::ThemeConfig) {
        // TODO: 实现主题应用逻辑
        // 这里可以根据配置更新样式、CSS变量等
        // 使用内部可变性来修改状态
    }

    /// 设置主题
    ///
    /// # Arguments
    ///
    /// * `theme` - 要设置的主题
    ///
    /// # Returns
    ///
    /// 成功返回 `Ok(())`, 失败返回错误信息
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::adapter::{DioxusAdapter, provider::ThemeProviderAdapter};
    /// use css_in_rust::theme::theme_types::{Theme, ThemeMode};
    ///
    /// let provider = ThemeProviderAdapter::default();
    /// let mut adapter = DioxusAdapter::new(provider);
    ///
    /// // 创建并设置自定义主题
    /// let custom_theme = Theme::new("custom")
    ///     .with_mode(ThemeMode::Dark)
    ///     .with_custom_variable("--primary-color", "#3366ff");
    ///
    /// adapter.set_theme(custom_theme).unwrap();
    /// ```
    pub fn set_theme(&mut self, theme: Theme) -> Result<(), String> {
        self.provider
            .switch_theme(&theme.name)
            .map(|_| ())
            .map_err(|e| e.to_string())
    }
}

/// Dioxus 主题钩子
///
/// 提供在 Dioxus 组件中使用主题的功能
pub mod hooks {
    use super::*;

    /// 使用主题
    ///
    /// 在 Dioxus 组件中获取当前主题
    ///
    /// # Arguments
    ///
    /// * `adapter` - Dioxus 适配器
    ///
    /// # Returns
    ///
    /// 当前主题，如果没有设置则返回 `None`
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::adapter::frameworks::dioxus::hooks;
    /// use css_in_rust::theme::adapter::{DioxusAdapter, provider::ThemeProviderAdapter};
    ///
    /// let provider = ThemeProviderAdapter::default();
    /// let adapter = DioxusAdapter::new(provider);
    ///
    /// // 在 Dioxus 组件中使用
    /// if let Some(theme) = hooks::use_theme(&adapter) {
    ///     // 使用主题属性
    ///     println!("主题模式: {:?}", theme.mode);
    /// }
    /// ```
    pub fn use_theme(adapter: &DioxusAdapter) -> Option<Theme> {
        adapter.get_current_theme()
    }

    /// 使用主题切换
    ///
    /// 在 Dioxus 组件中切换主题
    ///
    /// # Arguments
    ///
    /// * `adapter` - Dioxus 适配器
    ///
    /// # Returns
    ///
    /// 切换主题的闭包函数
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::adapter::frameworks::dioxus::hooks;
    /// use css_in_rust::theme::adapter::{DioxusAdapter, provider::ThemeProviderAdapter};
    ///
    /// let provider = ThemeProviderAdapter::default();
    /// let mut adapter = DioxusAdapter::new(provider);
    ///
    /// // 在 Dioxus 组件中使用
    /// let toggle = hooks::use_theme_toggle(&mut adapter);
    /// // 在按钮点击事件中调用
    /// // toggle();
    /// ```
    pub fn use_theme_toggle(adapter: &mut DioxusAdapter) -> impl FnMut() + '_ {
        move || {
            adapter.toggle_theme();
        }
    }

    /// 使用组件样式
    ///
    /// 在 Dioxus 组件中使用样式
    ///
    /// # Arguments
    ///
    /// * `adapter` - Dioxus 适配器
    /// * `component_name` - 组件名称
    /// * `css_rules` - CSS 规则字符串
    ///
    /// # Returns
    ///
    /// 生成的类名
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::adapter::frameworks::dioxus::hooks;
    /// use css_in_rust::theme::adapter::{DioxusAdapter, provider::ThemeProviderAdapter};
    ///
    /// let provider = ThemeProviderAdapter::default();
    /// let adapter = DioxusAdapter::new(provider);
    ///
    /// // 在 Dioxus 组件中使用
    /// let button_class = hooks::use_styled(&adapter, "button", "color: blue; padding: 8px 16px;");
    /// // <button class={button_class}>Click me</button>
    /// ```
    pub fn use_styled(adapter: &DioxusAdapter, component_name: &str, css_rules: &str) -> String {
        adapter
            .style_component(component_name, css_rules)
            .class_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::adapter::provider::{ThemeProviderAdapter, ThemeProviderConfig};
    use css_in_rust::theme::core::manager::{ThemeManager, ThemeManagerConfig};
    use std::sync::Arc;

    #[test]
    fn test_dioxus_adapter_basic() {
        // 创建适配器
        let manager = Arc::new(ThemeManager::new(ThemeManagerConfig::default()));
        let provider = ThemeProviderAdapter::new(manager, ThemeProviderConfig::default());
        let adapter = DioxusAdapter::new(provider);

        // 生成组件样式
        let style = adapter.style_component("button", "color: blue;");

        // 验证样式
        assert!(!style.class_name.is_empty());
        assert!(style.css.contains("color: blue;"));
        assert_eq!(style.component_name, "button");
    }
}
