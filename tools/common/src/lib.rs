//! 公共类型和工具函数
//!
//! 为所有工具提供共享的类型定义和实用函数

use std::path::{Path, PathBuf};
use std::fs;

// 避免未使用依赖警告
use serde as _;
use walkdir as _;
use log as _;
use env_logger as _;
use colorized as _;

/// 组件复杂度等级
#[derive(Debug, Clone, PartialEq)]
pub enum ComponentLevel {
    /// Level 1: 基础组件（简单结构）
    Level1,
    /// Level 2: 复杂组件（中等结构）
    Level2,
    /// Level 3: 模块化组件（复杂结构）
    Level3,
}

impl ComponentLevel {
    /// 从字符串解析组件等级
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "1" | "level1" | "basic" => Some(Self::Level1),
            "2" | "level2" | "complex" => Some(Self::Level2),
            "3" | "level3" | "modular" => Some(Self::Level3),
            _ => None,
        }
    }

    /// 获取等级描述
    pub fn description(&self) -> &'static str {
        match self {
            Self::Level1 => "基础组件（简单结构）",
            Self::Level2 => "复杂组件（中等结构）",
            Self::Level3 => "模块化组件（复杂结构）",
        }
    }
}

/// 工具执行结果
#[derive(Debug, Clone)]
pub struct ToolResult {
    pub success: bool,
    pub message: String,
    pub details: Option<String>,
    pub files_created: Vec<PathBuf>,
    pub files_modified: Vec<PathBuf>,
}

impl ToolResult {
    /// 创建成功结果
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            success: true,
            message: message.into(),
            details: None,
            files_created: Vec::new(),
            files_modified: Vec::new(),
        }
    }

    /// 创建错误结果
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: message.into(),
            details: None,
            files_created: Vec::new(),
            files_modified: Vec::new(),
        }
    }

    /// 添加详细信息
    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }

    /// 添加创建的文件
    pub fn with_created_file(mut self, file: impl Into<PathBuf>) -> Self {
        self.files_created.push(file.into());
        self
    }

    /// 添加修改的文件
    pub fn with_modified_file(mut self, file: impl Into<PathBuf>) -> Self {
        self.files_modified.push(file.into());
        self
    }
}

/// 文件操作工具
pub struct FileUtils;

impl FileUtils {
    /// 确保目录存在
    pub fn ensure_dir_exists(path: &Path) -> Result<(), std::io::Error> {
        if !path.exists() {
            fs::create_dir_all(path)?;
        }
        Ok(())
    }

    /// 安全地写入文件
    pub fn write_file_safe(path: &Path, content: &str) -> Result<(), std::io::Error> {
        if let Some(parent) = path.parent() {
            Self::ensure_dir_exists(parent)?;
        }
        fs::write(path, content)
    }

    /// 检查文件是否存在
    pub fn file_exists(path: &Path) -> bool {
        path.exists() && path.is_file()
    }

    /// 检查目录是否存在
    pub fn dir_exists(path: &Path) -> bool {
        path.exists() && path.is_dir()
    }
}

/// 日志工具
pub struct Logger;

impl Logger {
    /// 初始化日志
    pub fn init() {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .init();
    }

    /// 打印成功信息
    pub fn success(msg: &str) {
        colorized::colorize_print(format!("✅ {}", msg), colorized::Colors::GreenFg);
    }

    /// 打印错误信息
    pub fn error(msg: &str) {
        colorized::colorize_print(format!("❌ {}", msg), colorized::Colors::RedFg);
    }

    /// 打印警告信息
    pub fn warning(msg: &str) {
        colorized::colorize_print(format!("⚠️  {}", msg), colorized::Colors::YellowFg);
    }

    /// 打印信息
    pub fn info(msg: &str) {
        println!("ℹ️  {}", msg);
    }
}