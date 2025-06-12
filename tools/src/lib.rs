//! Ant Design Dioxus 开发工具集
//!
//! 提供组件开发、重构和质量保证的工具链

pub mod quality_checker;
pub mod structure_generator;
pub mod style_migrator;

// 导出主要的公共接口
pub use quality_checker::QualityChecker;
pub use structure_generator::StructureGenerator;
pub use style_migrator::StyleMigrator;

// 添加未使用的依赖以避免编译错误
use colorized as _;
use indoc as _;

/// 工具版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 工具集描述
pub const DESCRIPTION: &str = "Ant Design Dioxus 组件库开发工具集";

/// 支持的组件等级
#[derive(Debug, Clone, PartialEq)]
pub enum SupportedLevel {
    Level1,
    Level2,
    Level3,
}

/// 工具执行结果
#[derive(Debug, Clone)]
pub struct ToolResult {
    pub success: bool,
    pub message: String,
    pub details: Option<String>,
}

impl ToolResult {
    pub fn success(message: String) -> Self {
        Self {
            success: true,
            message,
            details: None,
        }
    }

    pub fn success_with_details(message: String, details: String) -> Self {
        Self {
            success: true,
            message,
            details: Some(details),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            message,
            details: None,
        }
    }

    pub fn error_with_details(message: String, details: String) -> Self {
        Self {
            success: false,
            message,
            details: Some(details),
        }
    }
}

/// 打印工具信息
pub fn print_tool_info() {
    println!("🛠️  {}", DESCRIPTION);
    println!("📦 版本: {}", VERSION);
    println!("🚀 可用工具:");
    println!("   • style_migrator  - CSS 样式迁移工具");
    println!("   • structure_generator - 组件结构生成器");
    println!("   • quality_checker - 质量检查工具");
}
