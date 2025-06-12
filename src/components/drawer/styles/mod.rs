use crate::theme::AliasToken;
use css_in_rust::css;

/// 抽屉位置枚举
#[derive(Debug, Clone, PartialEq)]
pub enum DrawerPlacement {
    /// 左侧
    Left,
    /// 右侧
    Right,
    /// 顶部
    Top,
    /// 底部
    Bottom,
}

impl Default for DrawerPlacement {
    fn default() -> Self {
        Self::Right
    }
}

/// 抽屉尺寸枚举
#[derive(Debug, Clone, PartialEq)]
pub enum DrawerSize {
    /// 默认尺寸
    Default,
    /// 大尺寸
    Large,
}

impl Default for DrawerSize {
    fn default() -> Self {
        Self::Default
    }
}

/// 抽屉样式生成器
pub struct DrawerStyleGenerator {
    /// 抽屉位置
    pub placement: DrawerPlacement,
    /// 抽屉尺寸
    pub size: DrawerSize,
    /// 是否显示遮罩
    pub mask: bool,
    /// 是否显示关闭按钮
    pub closable: bool,
    /// 主题令牌
    pub token: AliasToken,
}

impl Default for DrawerStyleGenerator {
    fn default() -> Self {
        Self {
            placement: DrawerPlacement::default(),
            size: DrawerSize::default(),
            mask: true,
            closable: true,
            token: AliasToken::default(),
        }
    }
}

impl DrawerStyleGenerator {
    /// 创建新的抽屉样式生成器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置抽屉位置
    pub fn with_placement(mut self, placement: DrawerPlacement) -> Self {
        self.placement = placement;
        self
    }

    /// 设置抽屉尺寸
    pub fn with_size(mut self, size: DrawerSize) -> Self {
        self.size = size;
        self
    }

    /// 设置是否显示遮罩
    pub fn with_mask(mut self, mask: bool) -> Self {
        self.mask = mask;
        self
    }

    /// 设置是否显示关闭按钮
    pub fn with_closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    /// 设置主题令牌
    pub fn with_token(mut self, token: AliasToken) -> Self {
        self.token = token;
        self
    }

    /// 生成样式类名
    pub fn generate(&self) -> String {
        let mut classes = vec!["ant-drawer-root"];

        // 添加位置相关的类名
        match self.placement {
            DrawerPlacement::Left => classes.push("ant-drawer-left"),
            DrawerPlacement::Right => classes.push("ant-drawer-right"),
            DrawerPlacement::Top => classes.push("ant-drawer-top"),
            DrawerPlacement::Bottom => classes.push("ant-drawer-bottom"),
        }

        // 添加尺寸相关的类名
        match self.size {
            DrawerSize::Default => {}
            DrawerSize::Large => classes.push("ant-drawer-large"),
        }

        // 添加遮罩相关的类名
        if self.mask {
            classes.push("ant-drawer-has-mask");
        }

        // 添加关闭按钮相关的类名
        if self.closable {
            classes.push("ant-drawer-closable");
        }

        classes.join(" ")
    }

    /// 生成 CSS 样式
    pub fn generate_css(&self) -> String {
        css!(
            r#"
            /* Drawer 抽屉样式 */

            /* 抽屉根容器 */
            .ant-drawer-root {
              position: fixed;
              inset: 0;
              z-index: 1000;
              pointer-events: none;
            }

            /* 遮罩层 */
            .ant-drawer-mask {
              position: absolute;
              inset: 0;
              background-color: ${mask_bg_color};
              pointer-events: auto;
              animation: ant-drawer-mask-fade-in 0.3s cubic-bezier(0.78, 0.14, 0.15, 0.86);
            }

            /* 抽屉内容包装器 */
            .ant-drawer-content-wrapper {
              position: absolute;
              pointer-events: auto;
            }

            /* 抽屉基础样式 */
            .ant-drawer {
              position: relative;
              background: ${bg_color};
              box-shadow: ${box_shadow};
              transition: transform 0.3s cubic-bezier(0.78, 0.14, 0.15, 0.86);
              display: flex;
              flex-direction: column;
              height: 100%;
            }

            /* 右侧抽屉 */
            .ant-drawer-right {
              top: 0;
              right: 0;
              height: 100%;
              transform: translateX(100%);
            }

            .ant-drawer-right.ant-drawer-open {
              transform: translateX(0);
            }

            .ant-drawer-content-wrapper .ant-drawer-right {
              right: 0;
              top: 0;
              bottom: 0;
            }

            /* 左侧抽屉 */
            .ant-drawer-left {
              top: 0;
              left: 0;
              height: 100%;
              transform: translateX(-100%);
            }

            .ant-drawer-left.ant-drawer-open {
              transform: translateX(0);
            }

            .ant-drawer-content-wrapper .ant-drawer-left {
              left: 0;
              top: 0;
              bottom: 0;
            }

            /* 顶部抽屉 */
            .ant-drawer-top {
              top: 0;
              left: 0;
              right: 0;
              width: 100%;
              transform: translateY(-100%);
            }

            .ant-drawer-top.ant-drawer-open {
              transform: translateY(0);
            }

            .ant-drawer-content-wrapper .ant-drawer-top {
              top: 0;
              left: 0;
              right: 0;
            }

            /* 底部抽屉 */
            .ant-drawer-bottom {
              bottom: 0;
              left: 0;
              right: 0;
              width: 100%;
              transform: translateY(100%);
            }

            .ant-drawer-bottom.ant-drawer-open {
              transform: translateY(0);
            }

            .ant-drawer-content-wrapper .ant-drawer-bottom {
              bottom: 0;
              left: 0;
              right: 0;
            }

            /* 抽屉内容 */
            .ant-drawer-content {
              display: flex;
              flex-direction: column;
              width: 100%;
              height: 100%;
              overflow: hidden;
            }

            /* 抽屉头部 */
            .ant-drawer-header {
              flex-shrink: 0;
              padding: ${padding_md}px ${padding_lg}px;
              border-bottom: 1px solid ${border_color};
              background: ${header_bg};
            }

            .ant-drawer-header-title {
              display: flex;
              align-items: center;
              justify-content: space-between;
              min-height: 22px;
            }

            .ant-drawer-title {
              flex: 1;
              margin: 0;
              color: ${title_color};
              font-weight: 600;
              font-size: ${font_size_lg}px;
              line-height: ${line_height};
            }

            .ant-drawer-extra {
              display: flex;
              align-items: center;
              gap: ${gap}px;
            }

            /* 关闭按钮 */
            .ant-drawer-close {
              position: relative;
              display: inline-block;
              width: 22px;
              height: 22px;
              padding: 0;
              color: ${close_color};
              font-weight: 600;
              font-size: ${close_font_size}px;
              font-style: normal;
              line-height: 22px;
              text-align: center;
              text-transform: none;
              text-decoration: none;
              background: transparent;
              border: 0;
              border-radius: ${border_radius_sm}px;
              outline: 0;
              cursor: pointer;
              transition: color 0.2s, background-color 0.2s;
              user-select: none;
            }

            .ant-drawer-close:hover {
              color: ${close_hover_color};
              background-color: ${close_bg_hover};
            }

            .ant-drawer-close:active {
              background-color: ${close_bg_active};
            }

            .ant-drawer-close-x {
              display: block;
              width: 100%;
              height: 100%;
              font-size: ${close_font_size}px;
              line-height: 22px;
            }

            /* 抽屉主体 */
            .ant-drawer-body {
              flex: 1;
              min-height: 0;
              padding: ${padding_lg}px;
              overflow: auto;
            }

            /* 抽屉页脚 */
            .ant-drawer-footer {
              flex-shrink: 0;
              padding: ${padding_md}px ${padding_lg}px;
              border-top: 1px solid ${border_color};
            }

            /* 抽屉动画 */
            @keyframes ant-drawer-mask-fade-in {
              0% {
                opacity: 0;
              }
              100% {
                opacity: 1;
              }
            }

            @keyframes ant-drawer-mask-fade-out {
              0% {
                opacity: 1;
              }
              100% {
                opacity: 0;
              }
            }

            /* 关闭时的遮罩层动画 */
            .ant-drawer-mask-closing {
              animation: ant-drawer-mask-fade-out 0.3s cubic-bezier(0.78, 0.14, 0.15, 0.86) forwards;
            }

            /* 响应式设计 */
            @media (max-width: 768px) {
              .ant-drawer-content-wrapper {
                width: 100% !important;
              }

              .ant-drawer-header {
                padding: ${padding_sm}px ${padding_md}px;
              }

              .ant-drawer-body {
                padding: ${padding_md}px;
              }

              .ant-drawer-footer {
                padding: ${padding_sm}px ${padding_md}px;
              }
            }

            /* 大尺寸抽屉 */
            .ant-drawer-large .ant-drawer-header {
              padding: ${padding_lg}px ${padding_lg}px;
            }

            .ant-drawer-large .ant-drawer-body {
              padding: ${padding_lg}px ${padding_xl}px;
            }

            .ant-drawer-large .ant-drawer-footer {
              padding: ${padding_lg}px ${padding_lg}px;
            }

            /* 无遮罩抽屉 */
            .ant-drawer-no-mask .ant-drawer-mask {
              display: none;
            }

            /* RTL支持 */
            .ant-drawer-rtl {
              direction: rtl;
            }

            .ant-drawer-rtl .ant-drawer-close {
              margin-right: 0;
              margin-left: ${margin_sm}px;
            }

            /* 高对比度模式支持 */
            @media (prefers-contrast: high) {
              .ant-drawer {
                outline: 2px solid #000;
              }
              .ant-drawer-header {
                border-bottom: 2px solid #000;
              }
              .ant-drawer-footer {
                border-top: 2px solid #000;
              }
            }
            "#,
            mask_bg_color = self.token.color_bg_mask,
            bg_color = self.token.color_bg_container,
            box_shadow = self.token.box_shadow,
            border_color = self.token.color_split,
            header_bg = self.token.color_bg_container,
            title_color = self.token.color_text,
            font_size_lg = self.token.font_size_lg,
            line_height = self.token.line_height,
            gap = self.token.margin_xs,
            close_color = self.token.color_text_secondary,
            close_font_size = self.token.font_size_lg,
            border_radius_sm = self.token.border_radius_sm,
            close_hover_color = self.token.color_icon_hover,
            close_bg_hover = self.token.color_bg_text_hover,
            close_bg_active = self.token.color_bg_text_active,
            padding_sm = self.token.padding_sm,
            padding_md = self.token.padding_md,
            padding_lg = self.token.padding_lg,
            padding_xl = self.token.padding_lg * 2,
            margin_sm = self.token.margin_sm
        ).to_string()
    }
}

/// 生成抽屉样式
pub fn generate_drawer_style(
    placement: DrawerPlacement,
    size: DrawerSize,
    mask: bool,
    closable: bool,
) -> String {
    DrawerStyleGenerator::new()
        .with_placement(placement)
        .with_size(size)
        .with_mask(mask)
        .with_closable(closable)
        .generate()
}

/// 生成抽屉CSS样式
pub fn generate_drawer_css(
    placement: DrawerPlacement,
    size: DrawerSize,
    mask: bool,
    closable: bool,
) -> String {
    DrawerStyleGenerator::new()
        .with_placement(placement)
        .with_size(size)
        .with_mask(mask)
        .with_closable(closable)
        .generate_css()
}

/// 默认抽屉样式
pub fn default_drawer_style() -> String {
    DrawerStyleGenerator::new().generate()
}
