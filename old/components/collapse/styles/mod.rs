use crate::theme::AliasToken;
use css_in_rust::css;

/// 折叠面板样式生成器
pub struct CollapseStyleGenerator {
    /// 是否无边框
    pub borderless: bool,
    /// 是否使用幽灵模式
    pub ghost: bool,
    /// 面板大小
    pub size: CollapseSize,
    /// 图标位置
    pub icon_position: CollapseIconPosition,
    /// 主题令牌
    pub token: AliasToken,
}

/// 折叠面板大小
#[derive(Debug, Clone, PartialEq)]
pub enum CollapseSize {
    /// 小号
    Small,
    /// 默认
    Default,
    /// 大号
    Large,
}

impl Default for CollapseSize {
    fn default() -> Self {
        Self::Default
    }
}

/// 折叠面板图标位置
#[derive(Debug, Clone, PartialEq)]
pub enum CollapseIconPosition {
    /// 开始位置
    Start,
    /// 结束位置
    End,
}

impl Default for CollapseIconPosition {
    fn default() -> Self {
        Self::Start
    }
}

impl Default for CollapseStyleGenerator {
    fn default() -> Self {
        Self {
            borderless: false,
            ghost: false,
            size: CollapseSize::default(),
            icon_position: CollapseIconPosition::default(),
            token: AliasToken::default(),
        }
    }
}

impl CollapseStyleGenerator {
    /// 创建新的折叠面板样式生成器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置是否无边框
    pub fn with_borderless(mut self, borderless: bool) -> Self {
        self.borderless = borderless;
        self
    }

    /// 设置是否使用幽灵模式
    pub fn with_ghost(mut self, ghost: bool) -> Self {
        self.ghost = ghost;
        self
    }

    /// 设置面板大小
    pub fn with_size(mut self, size: CollapseSize) -> Self {
        self.size = size;
        self
    }

    /// 设置图标位置
    pub fn with_icon_position(mut self, icon_position: CollapseIconPosition) -> Self {
        self.icon_position = icon_position;
        self
    }

    /// 设置主题令牌
    pub fn with_token(mut self, token: AliasToken) -> Self {
        self.token = token;
        self
    }

    /// 生成样式类名
    pub fn generate(&self) -> String {
        let mut classes = vec!["ant-collapse"];

        // 添加无边框类名
        if self.borderless {
            classes.push("ant-collapse-borderless");
        }

        // 添加幽灵模式类名
        if self.ghost {
            classes.push("ant-collapse-ghost");
        }

        // 添加大小相关的类名
        match self.size {
            CollapseSize::Small => classes.push("ant-collapse-small"),
            CollapseSize::Default => {}
            CollapseSize::Large => classes.push("ant-collapse-large"),
        }

        // 添加图标位置类名
        match self.icon_position {
            CollapseIconPosition::Start => classes.push("ant-collapse-icon-position-start"),
            CollapseIconPosition::End => classes.push("ant-collapse-icon-position-end"),
        }

        classes.join(" ")
    }

    /// 生成 CSS 样式
    pub fn generate_css(&self) -> String {
        css!(
            r#"
            /* Collapse 折叠面板组件样式 */

            .ant-collapse {
              margin: 0;
              padding: 0;
              color: ${color_text};
              font-size: ${font_size}px;
              font-variant: tabular-nums;
              line-height: ${line_height};
              list-style: none;
              font-feature-settings: 'tnum';
              background-color: ${bg_color};
              border: 1px solid ${border_color};
              border-bottom: 0;
              border-radius: ${border_radius}px;
            }

            .ant-collapse>.ant-collapse-item {
              border-bottom: 1px solid ${border_color};
            }

            .ant-collapse>.ant-collapse-item:last-child {
              border-radius: 0 0 ${border_radius}px ${border_radius}px;
            }

            .ant-collapse>.ant-collapse-item:last-child>.ant-collapse-content {
              border-radius: 0 0 ${border_radius}px ${border_radius}px;
            }

            .ant-collapse-borderless {
              background-color: transparent;
              border: 0;
            }

            .ant-collapse-borderless>.ant-collapse-item {
              border-bottom: 1px solid ${border_color};
            }

            .ant-collapse-borderless>.ant-collapse-item:last-child {
              border-bottom: 0;
            }

            .ant-collapse-borderless>.ant-collapse-item>.ant-collapse-header {
              padding-left: 0;
              padding-right: 0;
            }

            .ant-collapse-borderless>.ant-collapse-item>.ant-collapse-content {
              background-color: transparent;
              border-top: 0;
            }

            .ant-collapse-borderless>.ant-collapse-item>.ant-collapse-content>.ant-collapse-content-box {
              padding-left: 0;
              padding-right: 0;
            }

            .ant-collapse-item {
              border-bottom: 0;
            }

            .ant-collapse-item-active>.ant-collapse-header {
              color: ${active_color};
            }

            .ant-collapse-item-active>.ant-collapse-header .ant-collapse-arrow {
              transform: rotate(90deg);
            }

            .ant-collapse-item-disabled>.ant-collapse-header {
              color: ${disabled_color};
              cursor: not-allowed;
            }

            .ant-collapse-item-disabled>.ant-collapse-header .ant-collapse-arrow {
              cursor: not-allowed;
            }

            .ant-collapse-header {
              position: relative;
              display: flex;
              flex-wrap: nowrap;
              align-items: flex-start;
              padding: ${padding_md}px ${padding_md}px;
              color: ${header_color};
              line-height: ${line_height};
              cursor: pointer;
              transition: all 0.3s, visibility 0s;
            }

            .ant-collapse-header:focus {
              outline: none;
            }

            .ant-collapse-header-text {
              flex: auto;
            }

            .ant-collapse-expand-icon {
              height: 22px;
              color: ${icon_color};
              line-height: 22px;
              cursor: pointer;
              transition: all 0.3s;
            }

            .ant-collapse-icon-position-start .ant-collapse-expand-icon {
              margin-right: ${margin_sm}px;
            }

            .ant-collapse-icon-position-end .ant-collapse-expand-icon {
              margin-left: ${margin_sm}px;
              order: 1;
            }

            .ant-collapse-arrow {
              display: inline-block;
              transition: transform 0.24s;
            }

            .ant-collapse-content {
              color: ${content_color};
              background-color: ${content_bg};
              border-top: 1px solid ${border_color};
              transition: all 0.3s ease-out;
              will-change: height;
            }

            .ant-collapse-content-active {
              height: auto;
            }

            .ant-collapse-content-inactive {
              height: 0;
              overflow: hidden;
            }

            .ant-collapse-content-box {
              padding: ${padding_md}px;
            }

            .ant-collapse-extra {
              margin-left: auto;
            }

            .ant-collapse-large {
              font-size: ${font_size_lg}px;
            }

            .ant-collapse-large>.ant-collapse-item>.ant-collapse-header {
              padding: ${padding_md}px ${padding_lg}px;
              font-size: ${font_size_lg}px;
              line-height: 1.5;
            }

            .ant-collapse-large>.ant-collapse-item>.ant-collapse-content>.ant-collapse-content-box {
              padding: ${padding_lg}px;
            }

            .ant-collapse-small {
              font-size: ${font_size_sm}px;
            }

            .ant-collapse-small>.ant-collapse-item>.ant-collapse-header {
              padding: ${padding_xs}px ${padding_sm}px;
              font-size: ${font_size_sm}px;
            }

            .ant-collapse-small>.ant-collapse-item>.ant-collapse-content>.ant-collapse-content-box {
              padding: ${padding_sm}px;
            }

            .ant-collapse-ghost {
              background-color: transparent;
              border: 0;
            }

            .ant-collapse-ghost>.ant-collapse-item {
              border-bottom: 0;
            }

            .ant-collapse-ghost>.ant-collapse-item>.ant-collapse-content {
              background-color: transparent;
              border: 0;
            }

            .ant-collapse-ghost>.ant-collapse-item>.ant-collapse-content>.ant-collapse-content-box {
              padding-top: ${padding_sm}px;
              padding-bottom: ${padding_sm}px;
            }

            /* 响应式设计 */
            @media (max-width: 768px) {
              .ant-collapse-header {
                padding: ${padding_xs}px ${padding_sm}px;
              }
              .ant-collapse-content-box {
                padding: ${padding_sm}px;
              }
            }

            /* 高对比度模式支持 */
            @media (prefers-contrast: high) {
              .ant-collapse {
                border: 1px solid #000;
              }
              .ant-collapse>.ant-collapse-item {
                border-bottom: 1px solid #000;
              }
              .ant-collapse-content {
                border-top: 1px solid #000;
              }
            }

            /* 减少动画模式支持 */
            @media (prefers-reduced-motion: reduce) {
              .ant-collapse-header,
              .ant-collapse-content,
              .ant-collapse-arrow,
              .ant-collapse-expand-icon {
                transition: none;
              }
            }
            "#,
            color_text = self.token.color_text,
            font_size = self.token.font_size,
            line_height = self.token.line_height,
            bg_color = self.token.color_bg_container,
            border_color = self.token.color_border,
            border_radius = self.token.border_radius,
            active_color = self.token.color_primary,
            disabled_color = self.token.color_text_disabled,
            header_color = self.token.color_text,
            icon_color = self.token.color_text_secondary,
            content_color = self.token.color_text,
            content_bg = self.token.color_bg_container,
            padding_xs = self.token.padding_xs,
            padding_sm = self.token.padding_sm,
            padding_md = self.token.padding_md,
            padding_lg = self.token.padding_lg,
            margin_sm = self.token.margin_sm,
            font_size_sm = self.token.font_size_sm,
            font_size_lg = self.token.font_size_lg
        ).to_string()
    }
}

/// 生成折叠面板样式
pub fn generate_collapse_style(
    borderless: bool,
    ghost: bool,
    size: CollapseSize,
    icon_position: CollapseIconPosition,
) -> String {
    CollapseStyleGenerator::new()
        .with_borderless(borderless)
        .with_ghost(ghost)
        .with_size(size)
        .with_icon_position(icon_position)
        .generate()
}

/// 生成折叠面板CSS样式
pub fn generate_collapse_css(
    borderless: bool,
    ghost: bool,
    size: CollapseSize,
    icon_position: CollapseIconPosition,
) -> String {
    CollapseStyleGenerator::new()
        .with_borderless(borderless)
        .with_ghost(ghost)
        .with_size(size)
        .with_icon_position(icon_position)
        .generate_css()
}

/// 默认折叠面板样式
pub fn default_collapse_style() -> String {
    CollapseStyleGenerator::new().generate()
}
