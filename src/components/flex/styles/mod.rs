//! Flex 组件的样式定义

use css_in_rust::css;
use serde::{Deserialize, Serialize};

/// Flex 主轴对齐方式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FlexJustify {
    /// 起始位置对齐
    FlexStart,
    /// 居中对齐
    Center,
    /// 末尾位置对齐
    FlexEnd,
    /// 两端对齐，项目之间的间隔都相等
    SpaceBetween,
    /// 每个项目两侧的间隔相等
    SpaceAround,
    /// 每个项目的间隔与项目和容器之间的间隔相等
    SpaceEvenly,
}

impl Default for FlexJustify {
    fn default() -> Self {
        Self::FlexStart
    }
}
impl FlexJustify {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::FlexStart => "flex-start",
            Self::Center => "center",
            Self::FlexEnd => "flex-end",
            Self::SpaceBetween => "space-between",
            Self::SpaceAround => "space-around",
            Self::SpaceEvenly => "space-evenly",
        }
    }
}

/// Flex 交叉轴对齐方式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FlexAlign {
    /// 起始位置对齐
    FlexStart,
    /// 居中对齐
    Center,
    /// 末尾位置对齐
    FlexEnd,
    /// 基线对齐
    Baseline,
    /// 拉伸对齐
    Stretch,
}
impl Default for FlexAlign {
    fn default() -> Self {
        Self::FlexStart
    }
}
impl FlexAlign {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::FlexStart => "flex-start",
            Self::Center => "center",
            Self::FlexEnd => "flex-end",
            Self::Baseline => "baseline",
            Self::Stretch => "stretch",
        }
    }
}

/// Flex 换行方式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FlexWrap {
    /// 不换行
    NoWrap,
    /// 换行，第一行在上方
    Wrap,
    /// 换行，第一行在下方
    WrapReverse,
}
impl Default for FlexWrap {
    fn default() -> Self {
        Self::NoWrap
    }
}

impl FlexWrap {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NoWrap => "nowrap",
            Self::Wrap => "wrap",
            Self::WrapReverse => "wrap-reverse",
        }
    }
}

/// Flex 方向
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FlexDirection {
    /// 水平方向，起点在左端
    Row,
    /// 水平方向，起点在右端
    RowReverse,
    /// 垂直方向，起点在上沿
    Column,
    /// 垂直方向，起点在下沿
    ColumnReverse,
}

impl Default for FlexDirection {
    fn default() -> Self {
        Self::Row
    }
}
impl FlexDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Row => "row",
            Self::RowReverse => "row-reverse",
            Self::Column => "column",
            Self::ColumnReverse => "column-reverse",
        }
    }
}

/// Flex 间距大小
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FlexGap {
    /// 小间距
    Small,
    /// 中等间距
    Middle,
    /// 大间距
    Large,
    /// 自定义间距（像素值）
    Custom(u32),
}
impl Default for FlexGap {
    fn default() -> Self {
        Self::Small
    }
}
impl FlexGap {
    pub fn to_px(&self) -> String {
        match self {
            Self::Small => "8px".to_string(),
            Self::Middle => "16px".to_string(),
            Self::Large => "24px".to_string(),
            Self::Custom(px) => format!("{px}px"),
        }
    }
}

/// Flex 样式生成器
pub struct FlexStyleGenerator {
    /// 主轴方向
    direction: FlexDirection,
    /// 主轴对齐方式
    justify: FlexJustify,
    /// 交叉轴对齐方式
    align: FlexAlign,
    /// 是否换行
    wrap: FlexWrap,
    /// 间距大小
    gap: Option<FlexGap>,
    /// 是否启用RTL支持
    rtl: bool,
}

impl Default for FlexStyleGenerator {
    fn default() -> Self {
        Self {
            direction: FlexDirection::Row,
            justify: FlexJustify::FlexStart,
            align: FlexAlign::FlexStart,
            wrap: FlexWrap::NoWrap,
            gap: None,
            rtl: false,
        }
    }
}

impl FlexStyleGenerator {
    /// 创建新的 Flex 样式生成器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置主轴方向
    pub fn with_direction(mut self, direction: FlexDirection) -> Self {
        self.direction = direction;
        self
    }

    /// 设置是否垂直布局（向后兼容）
    pub fn with_vertical(mut self, vertical: bool) -> Self {
        if vertical {
            self.direction = FlexDirection::Column;
        } else {
            self.direction = FlexDirection::Row;
        }
        self
    }

    /// 设置主轴对齐方式
    pub fn with_justify(mut self, justify: FlexJustify) -> Self {
        self.justify = justify;
        self
    }

    /// 设置交叉轴对齐方式
    pub fn with_align(mut self, align: FlexAlign) -> Self {
        self.align = align;
        self
    }

    /// 设置是否换行
    pub fn with_wrap(mut self, wrap: FlexWrap) -> Self {
        self.wrap = wrap;
        self
    }

    /// 设置间距大小
    pub fn with_gap(mut self, gap: Option<FlexGap>) -> Self {
        self.gap = gap;
        self
    }

    /// 设置是否启用RTL支持
    pub fn with_rtl(mut self, rtl: bool) -> Self {
        self.rtl = rtl;
        self
    }

    /// 生成 CSS 类名
    pub fn generate(&self) -> String {
        let mut classes = vec!["ant-flex"];

        // 添加方向样式
        match self.direction {
            FlexDirection::Row => {} // 默认行方向，不需要额外类名
            FlexDirection::RowReverse => classes.push("ant-flex-row-reverse"),
            FlexDirection::Column => classes.push("ant-flex-vertical"),
            FlexDirection::ColumnReverse => classes.push("ant-flex-vertical-reverse"),
        }

        // 添加对齐样式
        classes.push(match self.justify {
            FlexJustify::FlexStart => "ant-flex-justify-start",
            FlexJustify::Center => "ant-flex-justify-center",
            FlexJustify::FlexEnd => "ant-flex-justify-end",
            FlexJustify::SpaceBetween => "ant-flex-justify-between",
            FlexJustify::SpaceAround => "ant-flex-justify-around",
            FlexJustify::SpaceEvenly => "ant-flex-justify-evenly",
        });

        classes.push(match self.align {
            FlexAlign::FlexStart => "ant-flex-align-start",
            FlexAlign::Center => "ant-flex-align-center",
            FlexAlign::FlexEnd => "ant-flex-align-end",
            FlexAlign::Baseline => "ant-flex-align-baseline",
            FlexAlign::Stretch => "ant-flex-align-stretch",
        });

        // 添加换行样式
        classes.push(match self.wrap {
            FlexWrap::NoWrap => "ant-flex-nowrap",
            FlexWrap::Wrap => "ant-flex-wrap",
            FlexWrap::WrapReverse => "ant-flex-wrap-reverse",
        });

        // 添加间距样式
        if let Some(gap) = &self.gap {
            classes.push(match gap {
                FlexGap::Small => "ant-flex-gap-small",
                FlexGap::Middle => "ant-flex-gap-middle",
                FlexGap::Large => "ant-flex-gap-large",
                FlexGap::Custom(_) => "ant-flex-gap-custom",
            });
        }

        // 添加RTL支持
        if self.rtl {
            classes.push("ant-flex-rtl");
        }

        classes.join(" ")
    }

    /// 生成基础样式
    pub fn base_style() -> String {
        css! {
            ".ant-flex" {
                display: "flex";
                flex-direction: "row";
                box-sizing: "border-box";
            }

            ".ant-flex-vertical" {
                flex-direction: "column";
            }

            ".ant-flex-vertical-reverse" {
                flex-direction: "column-reverse";
            }

            ".ant-flex-row-reverse" {
                flex-direction: "row-reverse";
            }

            // 主轴对齐方式
            ".ant-flex-justify-start" {
                justify-content: "flex-start";
            }

            ".ant-flex-justify-center" {
                justify-content: "center";
            }

            ".ant-flex-justify-end" {
                justify-content: "flex-end";
            }

            ".ant-flex-justify-between" {
                justify-content: "space-between";
            }

            ".ant-flex-justify-around" {
                justify-content: "space-around";
            }

            ".ant-flex-justify-evenly" {
                justify-content: "space-evenly";
            }

            // 交叉轴对齐方式
            ".ant-flex-align-start" {
                align-items: "flex-start";
            }

            ".ant-flex-align-center" {
                align-items: "center";
            }

            ".ant-flex-align-end" {
                align-items: "flex-end";
            }

            ".ant-flex-align-baseline" {
                align-items: "baseline";
            }

            ".ant-flex-align-stretch" {
                align-items: "stretch";
            }

            // 换行方式
            ".ant-flex-nowrap" {
                flex-wrap: "nowrap";
            }

            ".ant-flex-wrap" {
                flex-wrap: "wrap";
            }

            ".ant-flex-wrap-reverse" {
                flex-wrap: "wrap-reverse";
            }

            // 间距
            ".ant-flex-gap-small" {
                gap: "8px";
            }

            ".ant-flex-gap-middle" {
                gap: "16px";
            }

            ".ant-flex-gap-large" {
                gap: "24px";
            }

            // 常用布局模式
            ".ant-flex-center" {
                justify-content: "center";
                align-items: "center";
            }

            ".ant-flex-space-between" {
                justify-content: "space-between";
                align-items: "center";
            }

            // 子元素样式
            ".ant-flex > *" {
                flex-shrink: "0";
            }

            ".ant-flex-item" {
                flex: "1";
            }

            // 响应式设计
            "@media (max-width: 576px)" {
                ".ant-flex-sm-vertical" {
                    flex-direction: "column";
                }

                ".ant-flex-gap-small" {
                    gap: "6px";
                }

                ".ant-flex-gap-middle" {
                    gap: "12px";
                }

                ".ant-flex-gap-large" {
                    gap: "18px";
                }
            }

            "@media (max-width: 768px)" {
                ".ant-flex-md-wrap" {
                    flex-wrap: "wrap";
                }
            }

            // RTL 支持
            ".ant-flex-rtl" {
                direction: "rtl";
            }

            ".ant-flex-rtl.ant-flex-justify-start" {
                justify-content: "flex-end";
            }

            ".ant-flex-rtl.ant-flex-justify-end" {
                justify-content: "flex-start";
            }

            // 打印样式
            "@media print" {
                ".ant-flex" {
                    display: "block";
                }

                ".ant-flex > *" {
                    display: "block";
                    margin-bottom: "8px";
                }

                ".ant-flex > *:last-child" {
                    margin-bottom: "0";
                }
            }
        }
        .to_string()
    }
}

/// 注册 Flex 组件样式
pub fn register_styles() -> String {
    FlexStyleGenerator::base_style()
}
