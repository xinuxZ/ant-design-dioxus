//! 设计令牌(Design Token)模块
//!
//! 本模块定义了Ant Design的设计令牌系统，包括颜色、字体、间距、动画等设计元素。
//! 设计令牌是设计系统的基础，确保整个组件库的视觉一致性。

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 颜色令牌
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColorToken {
    /// 主色调
    pub primary: String,
    /// 主色调悬停态
    pub primary_hover: String,
    /// 主色调激活态
    pub primary_active: String,
    /// 成功色
    pub success: String,
    /// 警告色
    pub warning: String,
    /// 错误色
    pub error: String,
    /// 信息色
    pub info: String,
    /// 文本主色
    pub text: String,
    /// 文本次要色
    pub text_secondary: String,
    /// 文本禁用色
    pub text_disabled: String,
    /// 背景色
    pub background: String,
    /// 容器背景色
    pub background_container: String,
    /// 边框色
    pub border: String,
    /// 分割线色
    pub border_split: String,
}

impl Default for ColorToken {
    fn default() -> Self {
        Self {
            primary: "#1890ff".to_string(),
            primary_hover: "#40a9ff".to_string(),
            primary_active: "#096dd9".to_string(),
            success: "#52c41a".to_string(),
            warning: "#faad14".to_string(),
            error: "#ff4d4f".to_string(),
            info: "#1890ff".to_string(),
            text: "rgba(0, 0, 0, 0.85)".to_string(),
            text_secondary: "rgba(0, 0, 0, 0.65)".to_string(),
            text_disabled: "rgba(0, 0, 0, 0.25)".to_string(),
            background: "#fff".to_string(),
            background_container: "#fafafa".to_string(),
            border: "#d9d9d9".to_string(),
            border_split: "#f0f0f0".to_string(),
        }
    }
}

/// 暗色主题颜色令牌
impl ColorToken {
    pub fn dark() -> Self {
        Self {
            primary: "#1890ff".to_string(),
            primary_hover: "#40a9ff".to_string(),
            primary_active: "#096dd9".to_string(),
            success: "#52c41a".to_string(),
            warning: "#faad14".to_string(),
            error: "#ff4d4f".to_string(),
            info: "#1890ff".to_string(),
            text: "rgba(255, 255, 255, 0.85)".to_string(),
            text_secondary: "rgba(255, 255, 255, 0.65)".to_string(),
            text_disabled: "rgba(255, 255, 255, 0.25)".to_string(),
            background: "#141414".to_string(),
            background_container: "#1f1f1f".to_string(),
            border: "#434343".to_string(),
            border_split: "#303030".to_string(),
        }
    }
}

/// 字体令牌
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FontToken {
    /// 基础字体大小
    pub size_base: String,
    /// 大字体
    pub size_lg: String,
    /// 小字体
    pub size_sm: String,
    /// 超小字体
    pub size_xs: String,
    /// 基础行高
    pub line_height_base: String,
    /// 字体族
    pub family: String,
    /// 字重
    pub weight_normal: u16,
    pub weight_medium: u16,
    pub weight_bold: u16,
}

impl Default for FontToken {
    fn default() -> Self {
        Self {
            size_base: "14px".to_string(),
            size_lg: "16px".to_string(),
            size_sm: "12px".to_string(),
            size_xs: "10px".to_string(),
            line_height_base: "1.5715".to_string(),
            family: "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif".to_string(),
            weight_normal: 400,
            weight_medium: 500,
            weight_bold: 600,
        }
    }
}

/// 间距令牌
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpacingToken {
    /// 超小间距
    pub xs: String,
    /// 小间距
    pub sm: String,
    /// 中等间距
    pub md: String,
    /// 大间距
    pub lg: String,
    /// 超大间距
    pub xl: String,
    /// 超超大间距
    pub xxl: String,
}

impl Default for SpacingToken {
    fn default() -> Self {
        Self {
            xs: "8px".to_string(),
            sm: "12px".to_string(),
            md: "16px".to_string(),
            lg: "24px".to_string(),
            xl: "32px".to_string(),
            xxl: "48px".to_string(),
        }
    }
}

/// 边框令牌
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BorderToken {
    /// 基础圆角
    pub radius_base: String,
    /// 小圆角
    pub radius_sm: String,
    /// 大圆角
    pub radius_lg: String,
    /// 边框宽度
    pub width: String,
    /// 边框样式
    pub style: String,
}

impl Default for BorderToken {
    fn default() -> Self {
        Self {
            radius_base: "6px".to_string(),
            radius_sm: "4px".to_string(),
            radius_lg: "8px".to_string(),
            width: "1px".to_string(),
            style: "solid".to_string(),
        }
    }
}

/// 阴影令牌
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShadowToken {
    /// 基础阴影
    pub base: String,
    /// 卡片阴影
    pub card: String,
    /// 弹出层阴影
    pub popup: String,
    /// 抽屉阴影
    pub drawer: String,
}

impl Default for ShadowToken {
    fn default() -> Self {
        Self {
            base: "0 3px 6px -4px rgba(0, 0, 0, 0.12), 0 6px 16px 0 rgba(0, 0, 0, 0.08), 0 9px 28px 8px rgba(0, 0, 0, 0.05)".to_string(),
            card: "0 1px 2px -2px rgba(0, 0, 0, 0.16), 0 3px 6px 0 rgba(0, 0, 0, 0.12), 0 5px 12px 4px rgba(0, 0, 0, 0.09)".to_string(),
            popup: "0 6px 16px 0 rgba(0, 0, 0, 0.08), 0 3px 6px -4px rgba(0, 0, 0, 0.12), 0 9px 28px 8px rgba(0, 0, 0, 0.05)".to_string(),
            drawer: "0 8px 10px -5px rgba(0, 0, 0, 0.2), 0 16px 24px 2px rgba(0, 0, 0, 0.14), 0 6px 30px 5px rgba(0, 0, 0, 0.12)".to_string(),
        }
    }
}

/// 动画令牌
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MotionToken {
    /// 慢速动画时长
    pub duration_slow: String,
    /// 中速动画时长
    pub duration_mid: String,
    /// 快速动画时长
    pub duration_fast: String,
    /// 缓出动画
    pub ease_out: String,
    /// 缓入缓出动画
    pub ease_in_out: String,
    /// 缓入动画
    pub ease_in: String,
}

impl Default for MotionToken {
    fn default() -> Self {
        Self {
            duration_slow: "0.3s".to_string(),
            duration_mid: "0.2s".to_string(),
            duration_fast: "0.1s".to_string(),
            ease_out: "cubic-bezier(0.215, 0.61, 0.355, 1)".to_string(),
            ease_in_out: "cubic-bezier(0.645, 0.045, 0.355, 1)".to_string(),
            ease_in: "cubic-bezier(0.55, 0.055, 0.675, 0.19)".to_string(),
        }
    }
}

/// 完整的设计令牌集合
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DesignToken {
    pub color: ColorToken,
    pub font: FontToken,
    pub spacing: SpacingToken,
    pub border: BorderToken,
    pub shadow: ShadowToken,
    pub motion: MotionToken,
}

impl Default for DesignToken {
    fn default() -> Self {
        Self {
            color: ColorToken::default(),
            font: FontToken::default(),
            spacing: SpacingToken::default(),
            border: BorderToken::default(),
            shadow: ShadowToken::default(),
            motion: MotionToken::default(),
        }
    }
}

impl DesignToken {
    /// 创建暗色主题令牌
    pub fn dark() -> Self {
        Self {
            color: ColorToken::dark(),
            font: FontToken::default(),
            spacing: SpacingToken::default(),
            border: BorderToken::default(),
            shadow: ShadowToken::default(),
            motion: MotionToken::default(),
        }
    }

    /// 转换为CSS变量映射
    pub fn to_css_vars(&self) -> HashMap<String, String> {
        let mut vars = HashMap::new();

        // 颜色变量
        vars.insert(
            "--ant-primary-color".to_string(),
            self.color.primary.clone(),
        );
        vars.insert(
            "--ant-primary-color-hover".to_string(),
            self.color.primary_hover.clone(),
        );
        vars.insert(
            "--ant-primary-color-active".to_string(),
            self.color.primary_active.clone(),
        );
        vars.insert(
            "--ant-success-color".to_string(),
            self.color.success.clone(),
        );
        vars.insert(
            "--ant-warning-color".to_string(),
            self.color.warning.clone(),
        );
        vars.insert("--ant-error-color".to_string(), self.color.error.clone());
        vars.insert("--ant-info-color".to_string(), self.color.info.clone());
        vars.insert("--ant-text-color".to_string(), self.color.text.clone());
        vars.insert(
            "--ant-text-color-secondary".to_string(),
            self.color.text_secondary.clone(),
        );
        vars.insert(
            "--ant-text-color-disabled".to_string(),
            self.color.text_disabled.clone(),
        );
        vars.insert("--ant-bg-color".to_string(), self.color.background.clone());
        vars.insert(
            "--ant-bg-color-container".to_string(),
            self.color.background_container.clone(),
        );
        vars.insert("--ant-border-color".to_string(), self.color.border.clone());
        vars.insert(
            "--ant-border-color-split".to_string(),
            self.color.border_split.clone(),
        );

        // 字体变量
        vars.insert(
            "--ant-font-size-base".to_string(),
            self.font.size_base.clone(),
        );
        vars.insert("--ant-font-size-lg".to_string(), self.font.size_lg.clone());
        vars.insert("--ant-font-size-sm".to_string(), self.font.size_sm.clone());
        vars.insert(
            "--ant-line-height-base".to_string(),
            self.font.line_height_base.clone(),
        );
        vars.insert("--ant-font-family".to_string(), self.font.family.clone());

        // 间距变量
        vars.insert("--ant-padding-xs".to_string(), self.spacing.xs.clone());
        vars.insert("--ant-padding-sm".to_string(), self.spacing.sm.clone());
        vars.insert("--ant-padding-md".to_string(), self.spacing.md.clone());
        vars.insert("--ant-padding-lg".to_string(), self.spacing.lg.clone());
        vars.insert("--ant-padding-xl".to_string(), self.spacing.xl.clone());

        // 边框变量
        vars.insert(
            "--ant-border-radius".to_string(),
            self.border.radius_base.clone(),
        );
        vars.insert(
            "--ant-border-radius-sm".to_string(),
            self.border.radius_sm.clone(),
        );
        vars.insert(
            "--ant-border-radius-lg".to_string(),
            self.border.radius_lg.clone(),
        );

        // 阴影变量
        vars.insert(
            "--ant-box-shadow-base".to_string(),
            self.shadow.base.clone(),
        );
        vars.insert(
            "--ant-box-shadow-card".to_string(),
            self.shadow.card.clone(),
        );

        // 动画变量
        vars.insert(
            "--ant-motion-duration-slow".to_string(),
            self.motion.duration_slow.clone(),
        );
        vars.insert(
            "--ant-motion-duration-mid".to_string(),
            self.motion.duration_mid.clone(),
        );
        vars.insert(
            "--ant-motion-duration-fast".to_string(),
            self.motion.duration_fast.clone(),
        );
        vars.insert(
            "--ant-motion-ease-out".to_string(),
            self.motion.ease_out.clone(),
        );
        vars.insert(
            "--ant-motion-ease-in-out".to_string(),
            self.motion.ease_in_out.clone(),
        );

        vars
    }

    /// 生成CSS变量声明字符串
    pub fn to_css_declaration(&self) -> String {
        let vars = self.to_css_vars();
        let mut declarations = Vec::new();

        for (key, value) in vars {
            declarations.push(format!("{}: {};", key, value));
        }

        declarations.join("\n")
    }
}
