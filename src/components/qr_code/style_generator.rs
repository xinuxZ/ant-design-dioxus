//!
//! QRCode组件样式管理
//!
//! 使用样式生成器模式管理QRCode组件的样式
//! 使用 css! 宏的样式生成器模式，提供 QRCode 组件的完整样式生成功能

use crate::components::qr_code::types::*;
use crate::shared::styles::mixins::*;
use css_in_rust::css;

/// QRCode样式生成器
pub struct QRCodeStyleGenerator {
    pub size: u32,
    pub error_level: QRCodeErrorLevel,
    pub bordered: bool,
    pub status: QRCodeStatus,
}

impl QRCodeStyleGenerator {
    /// 创建新的QRCode样式生成器
    pub fn new() -> Self {
        Self {
            size: 160,
            error_level: QRCodeErrorLevel::M,
            bordered: true,
            status: QRCodeStatus::Active,
        }
    }

    /// 设置二维码尺寸
    pub fn with_size(mut self, size: u32) -> Self {
        self.size = size;
        self
    }

    /// 设置错误纠正级别
    pub fn with_error_level(mut self, error_level: QRCodeErrorLevel) -> Self {
        self.error_level = error_level;
        self
    }

    /// 设置是否有边框
    pub fn with_bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    /// 设置状态
    pub fn with_status(mut self, status: QRCodeStatus) -> Self {
        self.status = status;
        self
    }

    /// 生成完整的QRCode样式类名
    pub fn generate(&self) -> String {
        let mut classes = vec![self.base_style()];

        classes.push(self.size_style());

        if self.bordered {
            classes.push(self.bordered_style());
        }

        classes.push(self.status_style());

        classes.join(" ")
    }

    /// 基础QRCode样式
    fn base_style(&self) -> String {
        css!(
            r#"
            display: inline-block;
            background-color: #fff;
            border-radius: 8px;
            position: relative;
            overflow: hidden;
            "#
        )
        .to_string()
    }

    /// 二维码尺寸样式
    fn size_style(&self) -> String {
        css!(
            r#"
            width: {}px;
            height: {}px;
            "#,
            self.size,
            self.size
        )
        .to_string()
    }

    /// 边框样式
    fn bordered_style(&self) -> String {
        css!(
            r#"
            border: 1px solid #d9d9d9;
            padding: 8px;
            "#
        )
        .to_string()
    }

    /// 状态样式
    fn status_style(&self) -> String {
        match self.status {
            QRCodeStatus::Active => String::new(),
            QRCodeStatus::Expired => css!(
                r#"
                position: relative;
                
                &::after {
                    content: '';
                    position: absolute;
                    top: 0;
                    left: 0;
                    right: 0;
                    bottom: 0;
                    background-color: rgba(255, 255, 255, 0.96);
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    font-size: 14px;
                    color: rgba(0, 0, 0, 0.88);
                }
                "#
            ),
            QRCodeStatus::Loading => css!(
                r#"
                position: relative;
                
                &::after {
                    content: '';
                    position: absolute;
                    top: 0;
                    left: 0;
                    right: 0;
                    bottom: 0;
                    background-color: rgba(255, 255, 255, 0.96);
                    display: flex;
                    align-items: center;
                    justify-content: center;
                }
                "#
            ),
            QRCodeStatus::Scanned => css!(
                r#"
                opacity: 0.6;
                "#
            ),
        }
        .to_string()
    }
}

impl Default for QRCodeStyleGenerator {
    fn default() -> Self {
        Self::new()
    }
}