//! Ant Design 颜色预设
//!
//! 本模块定义了 Ant Design 设计系统的颜色调色板和预设。
//! 包括主色、功能色、中性色等完整的颜色体系。

use serde::{Deserialize, Serialize};

/// 颜色色阶（1-10级）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ColorScale {
    pub c1: String,
    pub c2: String,
    pub c3: String,
    pub c4: String,
    pub c5: String,
    pub c6: String,
    pub c7: String,
    pub c8: String,
    pub c9: String,
    pub c10: String,
}

impl ColorScale {
    /// 获取指定级别的颜色
    pub fn get_level(&self, level: u8) -> Option<&String> {
        match level {
            1 => Some(&self.c1),
            2 => Some(&self.c2),
            3 => Some(&self.c3),
            4 => Some(&self.c4),
            5 => Some(&self.c5),
            6 => Some(&self.c6),
            7 => Some(&self.c7),
            8 => Some(&self.c8),
            9 => Some(&self.c9),
            10 => Some(&self.c10),
            _ => None,
        }
    }

    /// 转换为 CSS 变量
    pub fn to_css_variables(&self, prefix: &str) -> String {
        format!(
            "  --{}-1: {};\n\
             --{}-2: {};\n\
             --{}-3: {};\n\
             --{}-4: {};\n\
             --{}-5: {};\n\
             --{}-6: {};\n\
             --{}-7: {};\n\
             --{}-8: {};\n\
             --{}-9: {};\n\
             --{}-10: {};\n",
            prefix,
            self.c1,
            prefix,
            self.c2,
            prefix,
            self.c3,
            prefix,
            self.c4,
            prefix,
            self.c5,
            prefix,
            self.c6,
            prefix,
            self.c7,
            prefix,
            self.c8,
            prefix,
            self.c9,
            prefix,
            self.c10,
        )
    }
}

/// Ant Design 颜色预设
pub struct AntDesignColors;

impl AntDesignColors {
    /// 蓝色色阶（主色）
    pub fn blue() -> ColorScale {
        ColorScale {
            c1: "#e6f4ff".to_string(),
            c2: "#bae0ff".to_string(),
            c3: "#91caff".to_string(),
            c4: "#69b1ff".to_string(),
            c5: "#4096ff".to_string(),
            c6: "#1677ff".to_string(), // 主色
            c7: "#0958d9".to_string(),
            c8: "#003eb3".to_string(),
            c9: "#002c8c".to_string(),
            c10: "#001d66".to_string(),
        }
    }

    /// 绿色色阶（成功色）
    pub fn green() -> ColorScale {
        ColorScale {
            c1: "#f6ffed".to_string(),
            c2: "#d9f7be".to_string(),
            c3: "#b7eb8f".to_string(),
            c4: "#95de64".to_string(),
            c5: "#73d13d".to_string(),
            c6: "#52c41a".to_string(), // 成功色
            c7: "#389e0d".to_string(),
            c8: "#237804".to_string(),
            c9: "#135200".to_string(),
            c10: "#092b00".to_string(),
        }
    }

    /// 红色色阶（错误色）
    pub fn red() -> ColorScale {
        ColorScale {
            c1: "#fff2f0".to_string(),
            c2: "#ffece6".to_string(),
            c3: "#ffd4cc".to_string(),
            c4: "#ffb3a6".to_string(),
            c5: "#ff8a80".to_string(),
            c6: "#ff4d4f".to_string(), // 错误色
            c7: "#d9363e".to_string(),
            c8: "#b32d33".to_string(),
            c9: "#8c2629".to_string(),
            c10: "#661f1f".to_string(),
        }
    }

    /// 橙色色阶（警告色）
    pub fn orange() -> ColorScale {
        ColorScale {
            c1: "#fff7e6".to_string(),
            c2: "#ffe7ba".to_string(),
            c3: "#ffd591".to_string(),
            c4: "#ffc069".to_string(),
            c5: "#ffa940".to_string(),
            c6: "#fa8c16".to_string(),
            c7: "#d46b08".to_string(),
            c8: "#ad4e00".to_string(),
            c9: "#873800".to_string(),
            c10: "#612500".to_string(),
        }
    }

    /// 金色色阶（警告色变体）
    pub fn gold() -> ColorScale {
        ColorScale {
            c1: "#fffbe6".to_string(),
            c2: "#fff1b8".to_string(),
            c3: "#ffe58f".to_string(),
            c4: "#ffd666".to_string(),
            c5: "#ffc53d".to_string(),
            c6: "#faad14".to_string(), // 警告色
            c7: "#d48806".to_string(),
            c8: "#ad6800".to_string(),
            c9: "#874d00".to_string(),
            c10: "#613400".to_string(),
        }
    }

    /// 紫色色阶
    pub fn purple() -> ColorScale {
        ColorScale {
            c1: "#f9f0ff".to_string(),
            c2: "#efdbff".to_string(),
            c3: "#d3adf7".to_string(),
            c4: "#b37feb".to_string(),
            c5: "#9254de".to_string(),
            c6: "#722ed1".to_string(),
            c7: "#531dab".to_string(),
            c8: "#391085".to_string(),
            c9: "#22075e".to_string(),
            c10: "#120338".to_string(),
        }
    }

    /// 青色色阶
    pub fn cyan() -> ColorScale {
        ColorScale {
            c1: "#e6fffb".to_string(),
            c2: "#b5f5ec".to_string(),
            c3: "#87e8de".to_string(),
            c4: "#5cdbd3".to_string(),
            c5: "#36cfc9".to_string(),
            c6: "#13c2c2".to_string(),
            c7: "#08979c".to_string(),
            c8: "#006d75".to_string(),
            c9: "#00474f".to_string(),
            c10: "#002329".to_string(),
        }
    }

    /// 灰色色阶（中性色）
    pub fn gray() -> ColorScale {
        ColorScale {
            c1: "#ffffff".to_string(),
            c2: "#fafafa".to_string(),
            c3: "#f5f5f5".to_string(),
            c4: "#f0f0f0".to_string(),
            c5: "#d9d9d9".to_string(),
            c6: "#bfbfbf".to_string(),
            c7: "#8c8c8c".to_string(),
            c8: "#595959".to_string(),
            c9: "#434343".to_string(),
            c10: "#262626".to_string(),
        }
    }

    /// 获取所有颜色预设
    pub fn all_colors() -> std::collections::HashMap<String, ColorScale> {
        let mut colors = std::collections::HashMap::new();
        colors.insert("blue".to_string(), Self::blue());
        colors.insert("green".to_string(), Self::green());
        colors.insert("red".to_string(), Self::red());
        colors.insert("orange".to_string(), Self::orange());
        colors.insert("gold".to_string(), Self::gold());
        colors.insert("purple".to_string(), Self::purple());
        colors.insert("cyan".to_string(), Self::cyan());
        colors.insert("gray".to_string(), Self::gray());
        colors
    }
}

/// 边框颜色配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BorderColors {
    pub primary: String,
    pub secondary: String,
    pub inverse: String,
}

impl Default for BorderColors {
    fn default() -> Self {
        Self {
            primary: "#d9d9d9".to_string(),
            secondary: "#f0f0f0".to_string(),
            inverse: "#434343".to_string(),
        }
    }
}

impl BorderColors {
    /// 深色主题的边框颜色
    pub fn dark() -> Self {
        Self {
            primary: "#424242".to_string(),
            secondary: "#303030".to_string(),
            inverse: "#d9d9d9".to_string(),
        }
    }

    /// 获取指定路径的颜色值
    pub fn get_value(&self, path: &str) -> Option<String> {
        match path {
            "primary" => Some(self.primary.clone()),
            "secondary" => Some(self.secondary.clone()),
            "inverse" => Some(self.inverse.clone()),
            _ => None,
        }
    }

    /// 转换为 CSS 变量
    pub fn to_css_variables(&self) -> String {
        format!(
            "  --color-border-primary: {};\n\
             --color-border-secondary: {};\n\
             --color-border-inverse: {};\n",
            self.primary, self.secondary, self.inverse
        )
    }
}

/// 文本颜色配置
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct TextColors {
    pub primary: String,
    pub secondary: String,
    pub tertiary: String,
    pub quaternary: String,
    pub disabled: String,
}

impl Default for TextColors {
    fn default() -> Self {
        Self {
            primary: "rgba(0, 0, 0, 0.88)".to_string(),
            secondary: "rgba(0, 0, 0, 0.65)".to_string(),
            tertiary: "rgba(0, 0, 0, 0.45)".to_string(),
            quaternary: "rgba(0, 0, 0, 0.25)".to_string(),
            disabled: "rgba(0, 0, 0, 0.25)".to_string(),
        }
    }
}

impl TextColors {
    /// 深色主题的文本颜色
    pub fn dark() -> Self {
        Self {
            primary: "rgba(255, 255, 255, 0.85)".to_string(),
            secondary: "rgba(255, 255, 255, 0.65)".to_string(),
            tertiary: "rgba(255, 255, 255, 0.45)".to_string(),
            quaternary: "rgba(255, 255, 255, 0.25)".to_string(),
            disabled: "rgba(255, 255, 255, 0.25)".to_string(),
        }
    }

    /// 转换为 CSS 变量
    pub fn to_css_variables(&self) -> String {
        format!(
            "  --color-text-primary: {};\n\
             --color-text-secondary: {};\n\
             --color-text-tertiary: {};\n\
             --color-text-quaternary: {};\n\
             --color-text-disabled: {};\n",
            self.primary, self.secondary, self.tertiary, self.quaternary, self.disabled
        )
    }
}
