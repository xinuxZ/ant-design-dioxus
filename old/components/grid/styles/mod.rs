/// Grid 栅格组件样式
///
/// 包含 Row 和 Col 组件的样式生成
use crate::theme::AliasToken;
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
    /// 主题令牌
    pub token: AliasToken,
}

impl Default for RowStyleGenerator {
    fn default() -> Self {
        Self {
            gutter: 0,
            justify: Justify::default(),
            align: Align::default(),
            wrap: true,
            token: AliasToken::default(),
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

    /// 设置主题令牌
    pub fn with_token(mut self, token: AliasToken) -> Self {
        self.token = token;
        self
    }

    /// 生成行样式类名
    pub fn generate(&self) -> String {
        let mut classes = vec!["ant-row"];

        // 添加对齐方式类名
        classes.push(self.justify.to_class());
        classes.push(self.align.to_class());

        if !self.wrap {
            classes.push("ant-row-no-wrap");
        }

        classes.join(" ")
    }

    /// 生成行内联样式
    pub fn generate_inline_style(&self) -> String {
        let margin_value = if self.gutter > 0 {
            format!("-{}px", self.gutter / 2)
        } else {
            "0".to_string()
        };

        css!(
            r#"
            margin-left: ${margin_left};
            margin-right: ${margin_right};
            "#,
            margin_left = &margin_value,
            margin_right = &margin_value
        )
        .to_string()
    }

    /// 生成 CSS 样式
    pub fn generate_css(&self) -> String {
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
            "#
        )
        .to_string()
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
    /// 主题令牌
    pub token: AliasToken,
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
            token: AliasToken::default(),
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

    /// 设置屏幕 < 576px 响应式栅格
    pub fn with_xs(mut self, xs: Option<ResponsiveConfig>) -> Self {
        self.xs = xs;
        self
    }

    /// 设置屏幕 ≥ 576px 响应式栅格
    pub fn with_sm(mut self, sm: Option<ResponsiveConfig>) -> Self {
        self.sm = sm;
        self
    }

    /// 设置屏幕 ≥ 768px 响应式栅格
    pub fn with_md(mut self, md: Option<ResponsiveConfig>) -> Self {
        self.md = md;
        self
    }

    /// 设置屏幕 ≥ 992px 响应式栅格
    pub fn with_lg(mut self, lg: Option<ResponsiveConfig>) -> Self {
        self.lg = lg;
        self
    }

    /// 设置屏幕 ≥ 1200px 响应式栅格
    pub fn with_xl(mut self, xl: Option<ResponsiveConfig>) -> Self {
        self.xl = xl;
        self
    }

    /// 设置屏幕 ≥ 1600px 响应式栅格
    pub fn with_xxl(mut self, xxl: Option<ResponsiveConfig>) -> Self {
        self.xxl = xxl;
        self
    }

    /// 设置主题令牌
    pub fn with_token(mut self, token: AliasToken) -> Self {
        self.token = token;
        self
    }

    /// 生成列样式类名
    pub fn generate(&self) -> String {
        let mut classes = vec!["ant-col".to_string()];

        // 添加基础类名
        if let Some(span) = self.span {
            classes.push(format!("ant-col-{}", span));
        }

        if self.offset > 0 {
            classes.push(format!("ant-col-offset-{}", self.offset));
        }

        if self.push > 0 {
            classes.push(format!("ant-col-push-{}", self.push));
        }

        if self.pull > 0 {
            classes.push(format!("ant-col-pull-{}", self.pull));
        }

        if let Some(order) = self.order {
            classes.push(format!("ant-col-order-{}", order));
        }

        // 添加响应式类名
        self.add_responsive_classes(&mut classes, "xs", &self.xs);
        self.add_responsive_classes(&mut classes, "sm", &self.sm);
        self.add_responsive_classes(&mut classes, "md", &self.md);
        self.add_responsive_classes(&mut classes, "lg", &self.lg);
        self.add_responsive_classes(&mut classes, "xl", &self.xl);
        self.add_responsive_classes(&mut classes, "xxl", &self.xxl);

        classes.join(" ")
    }

    /// 生成列内联样式
    pub fn generate_inline_style(&self) -> String {
        if self.gutter > 0 {
            let padding_value = format!("{}px", self.gutter / 2);
            css!(
                r#"
                padding-left: ${padding_left};
                padding-right: ${padding_right};
                "#,
                padding_left = &padding_value,
                padding_right = &padding_value
            )
            .to_string()
        } else {
            "".to_string()
        }
    }

    /// 生成 CSS 样式
    pub fn generate_css(&self) -> String {
        css!(
            r#"
            .ant-col {
                position: relative;
                max-width: 100%;
                min-height: 1px;
                box-sizing: border-box;
            }

            ${col_spans}
            ${col_offsets}
            ${col_pushes}
            ${col_pulls}
            ${col_orders}

            @media (max-width: 575px) {
                ${xs_spans}
                ${xs_offsets}
                ${xs_pushes}
                ${xs_pulls}
                ${xs_orders}
            }

            @media (min-width: 576px) {
                ${sm_spans}
                ${sm_offsets}
                ${sm_pushes}
                ${sm_pulls}
                ${sm_orders}
            }

            @media (min-width: 768px) {
                ${md_spans}
                ${md_offsets}
                ${md_pushes}
                ${md_pulls}
                ${md_orders}
            }

            @media (min-width: 992px) {
                ${lg_spans}
                ${lg_offsets}
                ${lg_pushes}
                ${lg_pulls}
                ${lg_orders}
            }

            @media (min-width: 1200px) {
                ${xl_spans}
                ${xl_offsets}
                ${xl_pushes}
                ${xl_pulls}
                ${xl_orders}
            }

            @media (min-width: 1600px) {
                ${xxl_spans}
                ${xxl_offsets}
                ${xxl_pushes}
                ${xxl_pulls}
                ${xxl_orders}
            }
            "#,
            col_spans = self.generate_col_spans(),
            col_offsets = self.generate_col_offsets(),
            col_pushes = self.generate_col_pushes(),
            col_pulls = self.generate_col_pulls(),
            col_orders = self.generate_col_orders(),
            xs_spans = self.generate_responsive_spans("xs"),
            xs_offsets = self.generate_responsive_offsets("xs"),
            xs_pushes = self.generate_responsive_pushes("xs"),
            xs_pulls = self.generate_responsive_pulls("xs"),
            xs_orders = self.generate_responsive_orders("xs"),
            sm_spans = self.generate_responsive_spans("sm"),
            sm_offsets = self.generate_responsive_offsets("sm"),
            sm_pushes = self.generate_responsive_pushes("sm"),
            sm_pulls = self.generate_responsive_pulls("sm"),
            sm_orders = self.generate_responsive_orders("sm"),
            md_spans = self.generate_responsive_spans("md"),
            md_offsets = self.generate_responsive_offsets("md"),
            md_pushes = self.generate_responsive_pushes("md"),
            md_pulls = self.generate_responsive_pulls("md"),
            md_orders = self.generate_responsive_orders("md"),
            lg_spans = self.generate_responsive_spans("lg"),
            lg_offsets = self.generate_responsive_offsets("lg"),
            lg_pushes = self.generate_responsive_pushes("lg"),
            lg_pulls = self.generate_responsive_pulls("lg"),
            lg_orders = self.generate_responsive_orders("lg"),
            xl_spans = self.generate_responsive_spans("xl"),
            xl_offsets = self.generate_responsive_offsets("xl"),
            xl_pushes = self.generate_responsive_pushes("xl"),
            xl_pulls = self.generate_responsive_pulls("xl"),
            xl_orders = self.generate_responsive_orders("xl"),
            xxl_spans = self.generate_responsive_spans("xxl"),
            xxl_offsets = self.generate_responsive_offsets("xxl"),
            xxl_pushes = self.generate_responsive_pushes("xxl"),
            xxl_pulls = self.generate_responsive_pulls("xxl"),
            xxl_orders = self.generate_responsive_orders("xxl")
        )
        .to_string()
    }

    /// 添加响应式类名
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

    /// 生成栅格占位样式
    fn generate_col_spans(&self) -> String {
        let mut styles = String::new();
        for i in 1..=24 {
            styles.push_str(&format!(
                ".ant-col-{} {{ flex: 0 0 {}%; max-width: {}%; }}\n",
                i,
                i as f32 * 100.0 / 24.0,
                i as f32 * 100.0 / 24.0
            ));
        }
        styles
    }

    /// 生成栅格左侧间隔样式
    fn generate_col_offsets(&self) -> String {
        let mut styles = String::new();
        for i in 1..=24 {
            styles.push_str(&format!(
                ".ant-col-offset-{} {{ margin-left: {}%; }}\n",
                i,
                i as f32 * 100.0 / 24.0
            ));
        }
        styles
    }

    /// 生成栅格右移样式
    fn generate_col_pushes(&self) -> String {
        let mut styles = String::new();
        for i in 1..=24 {
            styles.push_str(&format!(
                ".ant-col-push-{} {{ left: {}%; }}\n",
                i,
                i as f32 * 100.0 / 24.0
            ));
        }
        styles
    }

    /// 生成栅格左移样式
    fn generate_col_pulls(&self) -> String {
        let mut styles = String::new();
        for i in 1..=24 {
            styles.push_str(&format!(
                ".ant-col-pull-{} {{ right: {}%; }}\n",
                i,
                i as f32 * 100.0 / 24.0
            ));
        }
        styles
    }

    /// 生成栅格顺序样式
    fn generate_col_orders(&self) -> String {
        let mut styles = String::new();
        for i in 1..=24 {
            styles.push_str(&format!(".ant-col-order-{} {{ order: {}; }}\n", i, i));
        }
        styles
    }

    /// 生成响应式栅格占位样式
    fn generate_responsive_spans(&self, breakpoint: &str) -> String {
        let mut styles = String::new();
        for i in 1..=24 {
            styles.push_str(&format!(
                ".ant-col-{}-{} {{ flex: 0 0 {}%; max-width: {}%; }}\n",
                breakpoint,
                i,
                i as f32 * 100.0 / 24.0,
                i as f32 * 100.0 / 24.0
            ));
        }
        styles
    }

    /// 生成响应式栅格左侧间隔样式
    fn generate_responsive_offsets(&self, breakpoint: &str) -> String {
        let mut styles = String::new();
        for i in 1..=24 {
            styles.push_str(&format!(
                ".ant-col-{}-offset-{} {{ margin-left: {}%; }}\n",
                breakpoint,
                i,
                i as f32 * 100.0 / 24.0
            ));
        }
        styles
    }

    /// 生成响应式栅格右移样式
    fn generate_responsive_pushes(&self, breakpoint: &str) -> String {
        let mut styles = String::new();
        for i in 1..=24 {
            styles.push_str(&format!(
                ".ant-col-{}-push-{} {{ left: {}%; }}\n",
                breakpoint,
                i,
                i as f32 * 100.0 / 24.0
            ));
        }
        styles
    }

    /// 生成响应式栅格左移样式
    fn generate_responsive_pulls(&self, breakpoint: &str) -> String {
        let mut styles = String::new();
        for i in 1..=24 {
            styles.push_str(&format!(
                ".ant-col-{}-pull-{} {{ right: {}%; }}\n",
                breakpoint,
                i,
                i as f32 * 100.0 / 24.0
            ));
        }
        styles
    }

    /// 生成响应式栅格顺序样式
    fn generate_responsive_orders(&self, breakpoint: &str) -> String {
        let mut styles = String::new();
        for i in 1..=24 {
            styles.push_str(&format!(
                ".ant-col-{}-order-{} {{ order: {}; }}\n",
                breakpoint, i, i
            ));
        }
        styles
    }
}

/// 生成 Row 样式
pub fn generate_row_style(gutter: u32, justify: Justify, align: Align, wrap: bool) -> String {
    RowStyleGenerator::new()
        .with_gutter(gutter)
        .with_justify(justify)
        .with_align(align)
        .with_wrap(wrap)
        .generate()
}

/// 生成 Row 内联样式
pub fn generate_row_inline_style(gutter: u32) -> String {
    RowStyleGenerator::new()
        .with_gutter(gutter)
        .generate_inline_style()
}

/// 生成 Row CSS 样式
pub fn generate_row_css() -> String {
    RowStyleGenerator::new().generate_css()
}

/// 默认 Row 样式
pub fn default_row_style() -> String {
    RowStyleGenerator::new().generate()
}

/// 生成 Col 样式
pub fn generate_col_style(
    span: Option<u32>,
    offset: u32,
    push: u32,
    pull: u32,
    order: Option<i32>,
    xs: Option<ResponsiveConfig>,
    sm: Option<ResponsiveConfig>,
    md: Option<ResponsiveConfig>,
    lg: Option<ResponsiveConfig>,
    xl: Option<ResponsiveConfig>,
    xxl: Option<ResponsiveConfig>,
) -> String {
    ColStyleGenerator::new()
        .with_span(span)
        .with_offset(offset)
        .with_push(push)
        .with_pull(pull)
        .with_order(order)
        .with_xs(xs)
        .with_sm(sm)
        .with_md(md)
        .with_lg(lg)
        .with_xl(xl)
        .with_xxl(xxl)
        .generate()
}

/// 生成 Col 内联样式
pub fn generate_col_inline_style(gutter: u32) -> String {
    ColStyleGenerator::new()
        .with_gutter(gutter)
        .generate_inline_style()
}

/// 生成 Col CSS 样式
pub fn generate_col_css() -> String {
    ColStyleGenerator::new().generate_css()
}

/// 默认 Col 样式
pub fn default_col_style() -> String {
    ColStyleGenerator::new().generate()
}
