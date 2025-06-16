use crate::theme::adapter::injection::StyleInjector;
use css_in_rust::theme::{core::manager::ThemeManager, theme_types::Theme, ThemeVariant};
use std::collections::HashMap;
use std::sync::Arc;

/// 主题提供者适配器
///
/// 扩展核心主题提供者，添加高级功能
#[derive(Debug, Clone)]
pub struct ThemeProviderAdapter {
    /// 主题管理器
    manager: Arc<ThemeManager>,
    /// 样式注入器
    style_injector: StyleInjector,
    /// 配置
    config: ThemeProviderConfig,
    /// 缓存
    cache: HashMap<String, String>,
    /// 性能监控
    performance_metrics: PerformanceMetrics,
}

/// 主题切换结果
pub struct ThemeSwitchResult {
    /// 是否成功
    pub success: bool,
    /// 切换耗时（毫秒）
    pub duration_ms: u64,
    /// 更新的变量数量
    pub updated_variables: usize,
    /// 错误信息
    pub error: Option<String>,
}

/// 性能指标
#[derive(Debug, Clone, Default)]
pub struct PerformanceMetrics {
    /// 主题切换次数
    pub theme_switches: u32,
    /// 平均切换时间（毫秒）
    pub avg_switch_time_ms: f64,
    /// 最大切换时间（毫秒）
    pub max_switch_time_ms: u64,
    /// 变量更新次数
    pub variable_updates: u32,
    /// CSS生成次数
    pub css_generations: u32,
    /// 平均CSS生成时间（毫秒）
    pub avg_css_generation_time_ms: f64,
}

/// 主题预设
pub struct ThemePreset {
    /// 预设名称
    pub name: String,
    /// 预设描述
    pub description: String,
    /// 主题实例
    pub theme: Theme,
    /// 预设标签
    pub tags: Vec<String>,
    /// 是否为内置预设
    pub builtin: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThemeProviderConfig {
    /// 是否自动检测系统主题
    pub auto_detect_system_theme: bool,
    /// 是否启用主题持久化
    pub enable_persistence: bool,
    /// 持久化存储键
    pub storage_key: String,
}

impl Default for ThemeProviderConfig {
    fn default() -> Self {
        Self {
            auto_detect_system_theme: true,
            enable_persistence: true,
            storage_key: "theme-preference".to_string(),
        }
    }
}

impl ThemeProviderAdapter {
    /// 创建新的主题提供者适配器
    pub fn new(manager: Arc<ThemeManager>, config: ThemeProviderConfig) -> Self {
        Self {
            manager,
            style_injector: StyleInjector::new(":root"),
            config,
            cache: HashMap::new(),
            performance_metrics: PerformanceMetrics::default(),
        }
    }

    /// 设置样式注入器
    pub fn with_injector(mut self, injector: StyleInjector) -> Self {
        self.style_injector = injector;
        self
    }

    /// 获取当前主题
    pub fn get_theme(&self) -> Option<Theme> {
        self.manager.get_current_theme()
    }

    /// 切换主题
    pub fn switch_theme(&mut self, theme_name: &str) -> Result<ThemeSwitchResult, String> {
        let start_time = std::time::Instant::now();

        // 获取主题
        let theme = match self.get_theme() {
            Some(theme) => theme,
            None => return Err("无法获取当前主题".to_string()),
        };

        // 切换主题
        if let Err(e) = self.manager.set_theme(theme) {
            return Err(format!("切换主题失败: {:?}", e));
        }

        // 生成CSS变量
        let css_variables = self.generate_css_variables()?;

        // 注入CSS变量
        if let Err(e) = self.style_injector.inject_css_variables(&css_variables) {
            return Err(format!("注入CSS变量失败: {}", e));
        }

        // 计算切换时间
        let duration = start_time.elapsed().as_millis() as u64;

        // 更新性能指标
        self.performance_metrics.theme_switches += 1;
        self.performance_metrics.avg_switch_time_ms =
            ((self.performance_metrics.avg_switch_time_ms
                * (self.performance_metrics.theme_switches - 1) as f64)
                + duration as f64)
                / self.performance_metrics.theme_switches as f64;

        if duration > self.performance_metrics.max_switch_time_ms {
            self.performance_metrics.max_switch_time_ms = duration;
        }

        Ok(ThemeSwitchResult {
            success: true,
            duration_ms: duration,
            updated_variables: css_variables.len(),
            error: None,
        })
    }

    /// 生成CSS变量
    fn generate_css_variables(&mut self) -> Result<HashMap<String, String>, String> {
        let start_time = std::time::Instant::now();

        let mut theme = match self.get_theme() {
            Some(theme) => theme,
            None => return Err("无法获取当前主题".to_string()),
        };

        // 生成CSS变量
        let css = theme.to_css_variables();

        // 解析CSS变量
        let variables = self.parse_css_variables(&css);

        // 更新性能指标
        self.performance_metrics.css_generations += 1;
        let duration = start_time.elapsed().as_millis() as u64;
        self.performance_metrics.avg_css_generation_time_ms =
            ((self.performance_metrics.avg_css_generation_time_ms
                * (self.performance_metrics.css_generations - 1) as f64)
                + duration as f64)
                / self.performance_metrics.css_generations as f64;

        Ok(variables)
    }

    /// 解析CSS变量
    fn parse_css_variables(&self, css: &str) -> HashMap<String, String> {
        let mut variables = HashMap::new();

        for line in css.lines() {
            let line = line.trim();
            if line.is_empty() || !line.starts_with("--") {
                continue;
            }

            if let Some((name, value)) = line.split_once(':') {
                let name = name.trim();
                let value = value.trim().trim_end_matches(';').trim();
                variables.insert(name.to_string(), value.to_string());
            }
        }

        variables
    }

    /// 切换主题模式
    pub fn toggle_theme_mode(&mut self) -> Result<ThemeSwitchResult, String> {
        let start_time = std::time::Instant::now();

        // 切换模式
        self.manager.toggle_theme_mode();

        // 生成CSS变量
        let css_variables = self.generate_css_variables()?;

        // 注入CSS变量
        if let Err(e) = self.style_injector.inject_css_variables(&css_variables) {
            return Err(format!("注入CSS变量失败: {}", e));
        }

        // 计算切换时间
        let duration = start_time.elapsed().as_millis() as u64;

        Ok(ThemeSwitchResult {
            success: true,
            duration_ms: duration,
            updated_variables: css_variables.len(),
            error: None,
        })
    }

    /// 获取性能指标
    pub fn get_performance_metrics(&self) -> &PerformanceMetrics {
        &self.performance_metrics
    }

    /// 重置性能指标
    pub fn reset_performance_metrics(&mut self) {
        self.performance_metrics = PerformanceMetrics::default();
    }
}

impl ThemePreset {
    /// 创建内置预设
    pub fn builtin(name: impl Into<String>, description: impl Into<String>, theme: Theme) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            theme,
            tags: Vec::new(),
            builtin: true,
        }
    }

    /// 创建用户预设
    pub fn user(name: impl Into<String>, description: impl Into<String>, theme: Theme) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            theme,
            tags: Vec::new(),
            builtin: false,
        }
    }

    /// 添加标签
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// 添加多个标签
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags.extend(tags);
        self
    }

    /// 获取内置预设列表
    pub fn builtin_presets() -> Vec<ThemePreset> {
        vec![
            ThemePreset::builtin(
                "default",
                "默认亮色主题",
                Theme::default().with_mode(ThemeVariant::Light),
            ),
            ThemePreset::builtin(
                "dark",
                "默认暗色主题",
                Theme::default().with_mode(ThemeVariant::Dark),
            ),
        ]
    }
}
