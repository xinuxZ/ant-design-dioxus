/// Grid 栅格组件样式
///
/// 包含 Row 和 Col 组件的样式生成
use css_in_rust::css;
use serde::{Deserialize, Serialize};

/// 栅格对齐方式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Justify {
    /// 左对齐
    Start,
    /// 右对齐
    End,
    /// 居中对齐
    Center,
    /// 两端对齐
    SpaceBetween,
    /// 分散对齐
    SpaceAround,
    /// 均匀分布
    SpaceEvenly,
}

impl Default for Justify {
    fn default() -> Self {
        Self::Start
    }
}

impl Justify {
    /// 转换为CSS类名
    pub fn to_class(&self) -> &'static str {
        match self {
            Justify::Start => "ant-row-start",
            Justify::End => "ant-row-end",
            Justify::Center => "ant-row-center",
            Justify::SpaceBetween => "ant-row-space-between",
            Justify::SpaceAround => "ant-row-space-around",
            Justify::SpaceEvenly => "ant-row-space-evenly",
        }
    }
}

/// 栅格垂直对齐方式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Align {
    /// 顶部对齐
    Top,
    /// 中间对齐
    Middle,
    /// 底部对齐
    Bottom,
    /// 拉伸对齐
    Stretch,
}

impl Default for Align {
    fn default() -> Self {
        Self::Top
    }
}

impl Align {
    /// 转换为CSS类名
    pub fn to_class(&self) -> &'static str {
        match self {
            Align::Top => "ant-row-top",
            Align::Middle => "ant-row-middle",
            Align::Bottom => "ant-row-bottom",
            Align::Stretch => "ant-row-stretch",
        }
    }
}

/// 响应式断点配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponsiveConfig {
    /// 栅格占位格数
    pub span: Option<u32>,
    /// 栅格左侧的间隔格数
    pub offset: Option<u32>,
    /// 栅格向右移动格数
    pub push: Option<u32>,
    /// 栅格向左移动格数
    pub pull: Option<u32>,
    /// 栅格顺序
    pub order: Option<i32>,
}

/// Row 行样式生成器
pub struct RowStyleGenerator {
    /// 栅格间隔
    pub gutter: u32,
    /// 水平排列方式
    pub justify: Justify,
    /// 垂直对齐方式
    pub align: Align,
    /// 是否自动换行
    pub wrap: bool,
}

impl Default for RowStyleGenerator {
    fn default() -> Self {
        Self {
            gutter: 0,
            justify: Justify::default(),
            align: Align::default(),
            wrap: true,
        }
    }
}

impl RowStyleGenerator {
    /// 创建新的样式生成器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置栅格间隔
    pub fn with_gutter(mut self, gutter: u32) -> Self {
        self.gutter = gutter;
        self
    }

    /// 设置水平排列方式
    pub fn with_justify(mut self, justify: Justify) -> Self {
        self.justify = justify;
        self
    }

    /// 设置垂直对齐方式
    pub fn with_align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    /// 设置是否自动换行
    pub fn with_wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    /// 生成行样式
    pub fn generate(&self) -> String {
        let mut classes = vec!["ant-row"];

        // 添加对齐方式类名
        classes.push(self.justify.to_class());
        classes.push(self.align.to_class());

        if !self.wrap {
            classes.push("ant-row-no-wrap");
        }

        let class_name = classes.join(" ");

        let margin_value = if self.gutter > 0 {
            format!("-{}px", self.gutter / 2)
        } else {
            "0".to_string()
        };

        // 注入样式
        let style = css!(
            r#"
            margin-left: ${margin_left};
            margin-right: ${margin_right};
        "#,
            margin_left = &margin_value,
            margin_right = &margin_value
        );

        format!("{} {}", class_name, style)
    }
}

/// Col 列样式生成器
pub struct ColStyleGenerator {
    /// 栅格占位格数
    pub span: Option<u32>,
    /// 栅格左侧的间隔格数
    pub offset: u32,
    /// 栅格向右移动格数
    pub push: u32,
    /// 栅格向左移动格数
    pub pull: u32,
    /// 栅格顺序
    pub order: Option<i32>,
    /// 栅格间隔 (由 Row 提供)
    pub gutter: u32,
    /// 屏幕 < 576px 响应式栅格
    pub xs: Option<ResponsiveConfig>,
    /// 屏幕 ≥ 576px 响应式栅格
    pub sm: Option<ResponsiveConfig>,
    /// 屏幕 ≥ 768px 响应式栅格
    pub md: Option<ResponsiveConfig>,
    /// 屏幕 ≥ 992px 响应式栅格
    pub lg: Option<ResponsiveConfig>,
    /// 屏幕 ≥ 1200px 响应式栅格
    pub xl: Option<ResponsiveConfig>,
    /// 屏幕 ≥ 1600px 响应式栅格
    pub xxl: Option<ResponsiveConfig>,
}

impl Default for ColStyleGenerator {
    fn default() -> Self {
        Self {
            span: None,
            offset: 0,
            push: 0,
            pull: 0,
            order: None,
            gutter: 0,
            xs: None,
            sm: None,
            md: None,
            lg: None,
            xl: None,
            xxl: None,
        }
    }
}

impl ColStyleGenerator {
    /// 创建新的样式生成器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置栅格占位格数
    pub fn with_span(mut self, span: Option<u32>) -> Self {
        self.span = span;
        self
    }

    /// 设置栅格左侧的间隔格数
    pub fn with_offset(mut self, offset: u32) -> Self {
        self.offset = offset;
        self
    }

    /// 设置栅格向右移动格数
    pub fn with_push(mut self, push: u32) -> Self {
        self.push = push;
        self
    }

    /// 设置栅格向左移动格数
    pub fn with_pull(mut self, pull: u32) -> Self {
        self.pull = pull;
        self
    }

    /// 设置栅格顺序
    pub fn with_order(mut self, order: Option<i32>) -> Self {
        self.order = order;
        self
    }

    /// 设置栅格间隔
    pub fn with_gutter(mut self, gutter: u32) -> Self {
        self.gutter = gutter;
        self
    }

    /// 设置xs响应式配置
    pub fn with_xs(mut self, xs: Option<ResponsiveConfig>) -> Self {
        self.xs = xs;
        self
    }

    /// 设置sm响应式配置
    pub fn with_sm(mut self, sm: Option<ResponsiveConfig>) -> Self {
        self.sm = sm;
        self
    }

    /// 设置md响应式配置
    pub fn with_md(mut self, md: Option<ResponsiveConfig>) -> Self {
        self.md = md;
        self
    }

    /// 设置lg响应式配置
    pub fn with_lg(mut self, lg: Option<ResponsiveConfig>) -> Self {
        self.lg = lg;
        self
    }

    /// 设置xl响应式配置
    pub fn with_xl(mut self, xl: Option<ResponsiveConfig>) -> Self {
        self.xl = xl;
        self
    }

    /// 设置xxl响应式配置
    pub fn with_xxl(mut self, xxl: Option<ResponsiveConfig>) -> Self {
        self.xxl = xxl;
        self
    }

    /// 生成列样式
    pub fn generate(&self) -> String {
        let mut classes = Vec::new();

        // 添加基本类
        if let Some(span) = self.span {
            if span > 0 {
                classes.push(format!("ant-col-{}", span));
            }
        } else {
            classes.push("ant-col".to_string());
        }

        // 添加偏移类
        if self.offset > 0 {
            classes.push(format!("ant-col-offset-{}", self.offset));
        }

        // 添加推拉类
        if self.push > 0 {
            classes.push(format!("ant-col-push-{}", self.push));
        }

        if self.pull > 0 {
            classes.push(format!("ant-col-pull-{}", self.pull));
        }

        // 添加顺序类
        if let Some(order) = self.order {
            classes.push(format!("ant-col-order-{}", order));
        }

        // 添加响应式类
        self.add_responsive_classes(&mut classes, "xs", &self.xs);
        self.add_responsive_classes(&mut classes, "sm", &self.sm);
        self.add_responsive_classes(&mut classes, "md", &self.md);
        self.add_responsive_classes(&mut classes, "lg", &self.lg);
        self.add_responsive_classes(&mut classes, "xl", &self.xl);
        self.add_responsive_classes(&mut classes, "xxl", &self.xxl);

        let class_name = classes.join(" ");

        // 计算内边距
        let padding_value = if self.gutter > 0 {
            format!("{}px", self.gutter / 2)
        } else {
            "0".to_string()
        };

        // 注入样式
        let style = css!(
            r#"
            padding-left: ${padding_left};
            padding-right: ${padding_right};
        "#,
            padding_left = &padding_value,
            padding_right = &padding_value
        );

        format!("{} {}", class_name, style)
    }

    /// 添加响应式类
    fn add_responsive_classes(
        &self,
        classes: &mut Vec<String>,
        breakpoint: &str,
        config: &Option<ResponsiveConfig>,
    ) {
        if let Some(config) = config {
            if let Some(span) = config.span {
                classes.push(format!("ant-col-{}-{}", breakpoint, span));
            }

            if let Some(offset) = config.offset {
                if offset > 0 {
                    classes.push(format!("ant-col-{}-offset-{}", breakpoint, offset));
                }
            }

            if let Some(push) = config.push {
                if push > 0 {
                    classes.push(format!("ant-col-{}-push-{}", breakpoint, push));
                }
            }

            if let Some(pull) = config.pull {
                if pull > 0 {
                    classes.push(format!("ant-col-{}-pull-{}", breakpoint, pull));
                }
            }

            if let Some(order) = config.order {
                classes.push(format!("ant-col-{}-order-{}", breakpoint, order));
            }
        }
    }
}

/// 生成基础样式
pub fn grid_base_css() -> String {
    css!(
        r#"
        .ant-row {
            display: flex;
            flex-flow: row wrap;
            box-sizing: border-box;
        }

        .ant-row-no-wrap {
            flex-wrap: nowrap;
        }

        .ant-row-start {
            justify-content: flex-start;
        }

        .ant-row-center {
            justify-content: center;
        }

        .ant-row-end {
            justify-content: flex-end;
        }

        .ant-row-space-between {
            justify-content: space-between;
        }

        .ant-row-space-around {
            justify-content: space-around;
        }

        .ant-row-space-evenly {
            justify-content: space-evenly;
        }

        .ant-row-top {
            align-items: flex-start;
        }

        .ant-row-middle {
            align-items: center;
        }

        .ant-row-bottom {
            align-items: flex-end;
        }

        .ant-row-stretch {
            align-items: stretch;
        }

        .ant-col {
            position: relative;
            max-width: 100%;
            min-height: 1px;
            flex: 0 0 auto;
        }

        /* 构建网格系统的列宽 */
        .ant-col-1 {
            display: block;
            flex: 0 0 4.16666667%;
            max-width: 4.16666667%;
        }
        /* 其余列宽样式被简化 */
    "#
    )
}

/// 在组件中使用此方法来确保样式已注入到DOM
pub fn use_grid_style() {
    use dioxus::prelude::*;

    // 在组件首次渲染时注入样式
    use_effect(move || {
        // 样式只会被注入一次
        static STYLE_INJECTED: std::sync::OnceLock<()> = std::sync::OnceLock::new();

        STYLE_INJECTED.get_or_init(|| {
            // 使用css-in-rust自动注入样式
            let _ = grid_base_css();
        });
    });
}
