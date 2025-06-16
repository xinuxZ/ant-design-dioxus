//! 内容安全策略（CSP）和安全配置
//!
//! 提供动态样式的安全处理和CSP支持

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// CSP配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CspConfig {
    /// CSP nonce值，用于内联样式的安全验证
    pub nonce: Option<String>,
    /// 是否启用严格CSP模式
    pub strict_mode: bool,
    /// 允许的样式源
    pub style_src: Vec<String>,
    /// 允许的脚本源
    pub script_src: Vec<String>,
    /// 自定义CSP指令
    pub custom_directives: HashMap<String, Vec<String>>,
}

impl Default for CspConfig {
    fn default() -> Self {
        Self {
            nonce: None,
            strict_mode: false,
            style_src: vec!["'self'".to_string()],
            script_src: vec!["'self'".to_string()],
            custom_directives: HashMap::new(),
        }
    }
}

/// 安全配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// CSP配置
    pub csp: CspConfig,
    /// CSP配置（别名，向后兼容）
    pub csp_config: Option<CspConfig>,
    /// 是否启用样式隔离
    pub style_isolation: bool,
    /// 是否启用XSS防护
    pub xss_protection: bool,
    /// 是否验证动态样式内容
    pub validate_dynamic_styles: bool,
    /// 允许的CSS属性白名单
    pub allowed_css_properties: Option<Vec<String>>,
    /// 禁止的CSS属性黑名单
    pub blocked_css_properties: Vec<String>,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            csp: CspConfig::default(),
            csp_config: None,
            style_isolation: false,
            xss_protection: true,
            validate_dynamic_styles: true,
            allowed_css_properties: None,
            blocked_css_properties: vec![
                "javascript:".to_string(),
                "expression(".to_string(),
                "@import".to_string(),
            ],
        }
    }
}

/// 样式安全验证器
#[derive(Debug, Clone)]
pub struct StyleSecurityValidator {
    config: SecurityConfig,
}

/// 样式验证结果
#[derive(Debug, Clone)]
pub struct StyleValidationResult {
    /// 是否通过验证
    pub is_valid: bool,
    /// 清理后的样式内容
    pub sanitized_style: String,
    /// 验证错误信息
    pub errors: Vec<String>,
    /// 警告信息
    pub warnings: Vec<String>,
}

/// CSP nonce生成器
#[derive(Debug, Clone)]
pub struct NonceGenerator {
    /// nonce长度
    length: usize,
    /// 是否使用加密安全的随机数
    secure: bool,
}

impl CspConfig {
    /// 创建新的CSP配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置nonce值
    pub fn with_nonce(mut self, nonce: impl Into<String>) -> Self {
        self.nonce = Some(nonce.into());
        self
    }

    /// 启用严格模式
    pub fn with_strict_mode(mut self, strict: bool) -> Self {
        self.strict_mode = strict;
        self
    }

    /// 添加样式源
    pub fn add_style_src(mut self, src: impl Into<String>) -> Self {
        self.style_src.push(src.into());
        self
    }

    /// 添加脚本源
    pub fn add_script_src(mut self, src: impl Into<String>) -> Self {
        self.script_src.push(src.into());
        self
    }

    /// 添加自定义指令
    pub fn add_directive(mut self, directive: impl Into<String>, values: Vec<String>) -> Self {
        self.custom_directives.insert(directive.into(), values);
        self
    }

    /// 生成CSP头部字符串
    pub fn to_header_string(&self) -> String {
        let mut directives = Vec::new();

        // 样式源指令
        if !self.style_src.is_empty() {
            let mut style_directive = vec!["style-src".to_string()];
            style_directive.extend(self.style_src.clone());

            // 如果有nonce，添加到样式源
            if let Some(ref nonce) = self.nonce {
                style_directive.push(format!("'nonce-{}'", nonce));
            }

            directives.push(style_directive.join(" "));
        }

        // 脚本源指令
        if !self.script_src.is_empty() {
            let mut script_directive = vec!["script-src".to_string()];
            script_directive.extend(self.script_src.clone());

            // 如果有nonce，添加到脚本源
            if let Some(ref nonce) = self.nonce {
                script_directive.push(format!("'nonce-{}'", nonce));
            }

            directives.push(script_directive.join(" "));
        }

        // 自定义指令
        for (directive, values) in &self.custom_directives {
            let mut full_directive = vec![directive.clone()];
            full_directive.extend(values.clone());
            directives.push(full_directive.join(" "));
        }

        directives.join("; ")
    }
}

impl SecurityConfig {
    /// 创建新的安全配置
    pub fn new() -> Self {
        Self {
            csp: CspConfig::default(),
            csp_config: Some(CspConfig::default()),
            style_isolation: true,
            xss_protection: true,
            validate_dynamic_styles: true,
            allowed_css_properties: None,
            blocked_css_properties: vec![
                "javascript:".to_string(),
                "expression(".to_string(),
                "@import".to_string(),
            ],
        }
    }

    /// 设置CSP配置
    pub fn with_csp(mut self, csp: CspConfig) -> Self {
        self.csp = csp;
        self
    }

    /// 启用样式隔离
    pub fn with_style_isolation(mut self, isolation: bool) -> Self {
        self.style_isolation = isolation;
        self
    }

    /// 启用XSS防护
    pub fn with_xss_protection(mut self, protection: bool) -> Self {
        self.xss_protection = protection;
        self
    }

    /// 设置允许的CSS属性
    pub fn with_allowed_properties(mut self, properties: Vec<String>) -> Self {
        self.allowed_css_properties = Some(properties);
        self
    }

    /// 添加禁止的CSS属性
    pub fn add_blocked_property(mut self, property: impl Into<String>) -> Self {
        self.blocked_css_properties.push(property.into());
        self
    }
}

impl StyleSecurityValidator {
    /// 创建新的样式安全验证器
    pub fn new(config: SecurityConfig) -> Self {
        Self { config }
    }

    /// 验证样式内容
    pub fn validate(&self, style_content: &str) -> StyleValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let mut sanitized_style = style_content.to_string();
        let mut is_valid = true;

        // XSS防护检查
        if self.config.xss_protection {
            for blocked_property in &self.config.blocked_css_properties {
                if style_content.contains(blocked_property) {
                    errors.push(format!("检测到危险的CSS内容: {}", blocked_property));
                    sanitized_style = sanitized_style.replace(blocked_property, "");
                    is_valid = false;
                }
            }
        }

        // 属性白名单检查
        if let Some(ref allowed_properties) = self.config.allowed_css_properties {
            // 简单的CSS属性提取（实际实现可能需要更复杂的解析）
            let properties: Vec<&str> = style_content
                .split(';')
                .filter_map(|prop| prop.split(':').next())
                .map(|prop| prop.trim())
                .collect();

            for property in properties {
                if !allowed_properties.contains(&property.to_string()) {
                    warnings.push(format!("CSS属性 '{}' 不在允许列表中", property));
                }
            }
        }

        StyleValidationResult {
            is_valid,
            sanitized_style,
            errors,
            warnings,
        }
    }

    /// 清理样式内容
    pub fn sanitize(&self, style_content: &str) -> String {
        self.validate(style_content).sanitized_style
    }
}

impl NonceGenerator {
    /// 创建新的nonce生成器
    pub fn new() -> Self {
        Self {
            length: 16,
            secure: true,
        }
    }

    /// 设置nonce长度
    pub fn with_length(mut self, length: usize) -> Self {
        self.length = length;
        self
    }

    /// 设置是否使用加密安全的随机数
    pub fn with_secure(mut self, secure: bool) -> Self {
        self.secure = secure;
        self
    }

    /// 生成nonce值
    /// 
    /// 使用基于时间戳和哈希的伪随机数生成器，避免依赖rand库
    pub fn generate(&self) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        use std::time::{SystemTime, UNIX_EPOCH};

        let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        
        // 使用时间戳和哈希作为种子
        let mut hasher = DefaultHasher::new();
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
            .hash(&mut hasher);
        
        // 添加额外的随机性
        std::ptr::addr_of!(self).hash(&mut hasher);
        
        let mut seed = hasher.finish();
        
        (0..self.length)
            .map(|_| {
                // 线性同余生成器
                seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
                let idx = (seed as usize) % charset.len();
                charset[idx] as char
            })
            .collect()
    }
}

impl Default for NonceGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// 样式注入安全包装器
#[derive(Debug, Clone)]
pub struct SecureStyleInjector {
    validator: StyleSecurityValidator,
    nonce_generator: NonceGenerator,
}

impl SecureStyleInjector {
    /// 创建新的安全样式注入器
    pub fn new(config: SecurityConfig) -> Self {
        Self {
            validator: StyleSecurityValidator::new(config),
            nonce_generator: NonceGenerator::new(),
        }
    }

    /// 安全地注入样式
    pub fn inject_style(&self, style_content: &str, target_id: &str) -> Result<String, String> {
        let validation_result = self.validator.validate(style_content);

        if !validation_result.is_valid {
            return Err(format!("样式验证失败: {:?}", validation_result.errors));
        }

        let nonce = self.nonce_generator.generate();
        let style_tag = format!(
            r#"<style id="{}" nonce="{}">{}</style>"#,
            target_id, nonce, validation_result.sanitized_style
        );

        Ok(style_tag)
    }

    /// 生成安全的内联样式属性
    pub fn create_inline_style(&self, style_content: &str) -> Result<String, String> {
        let validation_result = self.validator.validate(style_content);

        if !validation_result.is_valid {
            return Err(format!("样式验证失败: {:?}", validation_result.errors));
        }

        Ok(validation_result.sanitized_style)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csp_config() {
        let csp = CspConfig::new()
            .with_nonce("abc123")
            .with_strict_mode(true)
            .add_style_src("'self'")
            .add_script_src("'self'");

        let header = csp.to_header_string();
        assert!(header.contains("nonce-abc123"));
        assert!(header.contains("style-src"));
        assert!(header.contains("script-src"));
    }

    #[test]
    fn test_style_validation() {
        let config = SecurityConfig::new();
        let validator = StyleSecurityValidator::new(config);

        let malicious_style = "color: red; javascript:alert('xss');";
        let result = validator.validate(malicious_style);

        assert!(!result.is_valid);
        assert!(!result.errors.is_empty());
    }

    #[test]
    fn test_nonce_generation() {
        let generator = NonceGenerator::new().with_length(12);
        let nonce = generator.generate();

        assert_eq!(nonce.len(), 12);
        assert!(nonce.chars().all(|c| c.is_alphanumeric()));
    }
}
