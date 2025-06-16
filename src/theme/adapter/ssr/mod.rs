use std::collections::HashMap;
use std::time::{Duration, Instant};

pub mod hydration;

pub use hydration::{HydrationConfig, HydrationEngine, HydrationMode, HydrationResult};

/// SSR支持
///
/// 提供服务端渲染的样式支持功能
pub struct SsrSupport {
    /// 样式缓存
    style_cache: HashMap<String, String>,
    /// 是否启用优化
    optimize_enabled: bool,
    /// 是否启用哈希
    hash_enabled: bool,
    /// 水合配置
    hydration_config: HydrationConfig,
}

/// SSR渲染结果
#[derive(Debug)]
pub struct SsrRenderResult {
    /// CSS内容
    pub css: String,
    /// CSS哈希
    pub hash: String,
    /// 样式标签ID
    pub style_id: String,
    /// 渲染时间
    pub render_time_ms: u64,
    /// 样式标签属性
    pub style_attributes: HashMap<String, String>,
}

impl SsrSupport {
    /// 创建新的SSR支持实例
    pub fn new() -> Self {
        Self {
            style_cache: HashMap::new(),
            optimize_enabled: true,
            hash_enabled: true,
            hydration_config: HydrationConfig::default(),
        }
    }

    /// 设置是否启用优化
    pub fn with_optimize(mut self, enable: bool) -> Self {
        self.optimize_enabled = enable;
        self
    }

    /// 设置是否启用哈希
    pub fn with_hash(mut self, enable: bool) -> Self {
        self.hash_enabled = enable;
        self
    }

    /// 设置水合配置
    pub fn with_hydration_config(mut self, config: HydrationConfig) -> Self {
        self.hydration_config = config;
        self
    }

    /// 渲染样式
    pub fn render_styles(&self, component_styles: HashMap<String, String>) -> SsrRenderResult {
        let start_time = Instant::now();
        let mut combined_css = String::new();

        // 合并所有组件样式
        for (_, css) in &component_styles {
            combined_css.push_str(css);
            combined_css.push('\n');
        }

        // 优化CSS（如果启用）
        let optimized_css = if self.optimize_enabled {
            self.optimize_css(&combined_css)
        } else {
            combined_css
        };

        // 计算哈希（如果启用）
        let hash = if self.hash_enabled {
            self.compute_hash(&optimized_css)
        } else {
            String::new()
        };

        // 生成样式ID
        let style_id = format!(
            "ssr-styles-{}",
            if hash.is_empty() { "default" } else { &hash }
        );

        // 生成样式标签属性
        let mut style_attributes = HashMap::new();
        style_attributes.insert("data-ssr".to_string(), "true".to_string());
        style_attributes.insert("id".to_string(), style_id.clone());

        if !hash.is_empty() {
            style_attributes.insert("data-hash".to_string(), hash.clone());
        }

        // 添加水合相关属性
        if self.hydration_config.enabled {
            style_attributes.insert(
                "data-hydration".to_string(),
                self.hydration_config.mode.to_string(),
            );

            // 添加自定义属性
            for (key, value) in &self.hydration_config.custom_attrs {
                style_attributes.insert(format!("data-{}", key), value.clone());
            }
        }

        // 计算渲染时间
        let render_time = start_time.elapsed();

        SsrRenderResult {
            css: optimized_css,
            hash,
            style_id,
            render_time_ms: render_time.as_millis() as u64,
            style_attributes,
        }
    }

    /// 优化CSS
    fn optimize_css(&self, css: &str) -> String {
        // 实现CSS优化逻辑
        // 这里只是一个简单的示例，实际实现可能更复杂
        css.replace("  ", " ")
            .replace("\n\n", "\n")
            .trim()
            .to_string()
    }

    /// 计算CSS哈希
    fn compute_hash(&self, css: &str) -> String {
        // 实现哈希计算逻辑
        // 这里只是一个简单的示例，实际实现应使用更好的哈希算法
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        css.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    /// 创建水合引擎
    pub fn create_hydration_engine(&self) -> HydrationEngine {
        HydrationEngine::new(self.hydration_config.clone())
    }

    /// 生成HTML样式标签
    pub fn generate_style_tag(&self, result: &SsrRenderResult) -> String {
        let mut attributes = String::new();

        for (key, value) in &result.style_attributes {
            attributes.push_str(&format!(" {}=\"{}\"", key, value));
        }

        format!("<style{}>\n{}\n</style>", attributes, result.css)
    }
}

impl Default for SsrSupport {
    fn default() -> Self {
        Self::new()
    }
}

impl ToString for HydrationMode {
    fn to_string(&self) -> String {
        match self {
            HydrationMode::Full => "full".to_string(),
            HydrationMode::Partial => "partial".to_string(),
            HydrationMode::Deferred => "deferred".to_string(),
            HydrationMode::Progressive => "progressive".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ssr_render_basic() {
        let ssr = SsrSupport::new();

        let mut styles = HashMap::new();
        styles.insert("button".to_string(), ".button { color: blue; }".to_string());
        styles.insert(
            "input".to_string(),
            ".input { border: 1px solid gray; }".to_string(),
        );

        let result = ssr.render_styles(styles);

        assert!(!result.css.is_empty());
        assert!(!result.hash.is_empty());
        assert!(result.style_id.starts_with("ssr-styles-"));
        assert!(result.render_time_ms >= 0);
        assert!(result.style_attributes.contains_key("data-ssr"));
    }

    #[test]
    fn test_generate_style_tag() {
        let ssr = SsrSupport::new();

        let mut styles = HashMap::new();
        styles.insert("button".to_string(), ".button { color: blue; }".to_string());

        let result = ssr.render_styles(styles);
        let tag = ssr.generate_style_tag(&result);

        assert!(tag.starts_with("<style"));
        assert!(tag.contains("data-ssr=\"true\""));
        assert!(tag.contains(".button { color: blue; }"));
        assert!(tag.ends_with("</style>"));
    }
}
