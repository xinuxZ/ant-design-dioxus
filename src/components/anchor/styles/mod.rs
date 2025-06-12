//! Anchor 锚点组件样式

use css_in_rust::css;

/// Anchor 锚点样式生成器
pub struct AnchorStyleBuilder {
    /// 是否使用暗色主题
    dark_theme: bool,
    /// 是否使用紧凑主题
    compact_theme: bool,
}

impl Default for AnchorStyleBuilder {
    fn default() -> Self {
        Self {
            dark_theme: false,
            compact_theme: false,
        }
    }
}

impl AnchorStyleBuilder {
    /// 创建新的样式生成器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置是否使用暗色主题
    pub fn dark_theme(mut self, dark_theme: bool) -> Self {
        self.dark_theme = dark_theme;
        self
    }

    /// 设置是否使用紧凑主题
    pub fn compact_theme(mut self, compact_theme: bool) -> Self {
        self.compact_theme = compact_theme;
        self
    }

    /// 基础样式
    fn base_style(&self) -> String {
        css!(
            r#"
            .ant-anchor {
              position: relative;
              padding-left: 4px;
              margin-left: 8px;
              color: rgba(0, 0, 0, 0.65);
              line-height: 1.5715;
              font-size: 14px;
              font-variant: tabular-nums;
              font-feature-settings: 'tnum';
            }

            .ant-anchor-wrapper {
              position: relative;
              margin-left: -4px;
              padding-left: 4px;
              overflow: auto;
              background-color: transparent;
            }

            .ant-anchor-ink {
              position: absolute;
              left: 0;
              top: 0;
              height: 100%;
              width: 2px;
              background-color: #f0f0f0;
            }

            .ant-anchor-ink-ball {
              position: absolute;
              left: 50%;
              width: 8px;
              height: 8px;
              border: 2px solid #1890ff;
              border-radius: 8px;
              background-color: #fff;
              transform: translateX(-50%);
              transition: top 0.3s ease-in-out;
              display: none;
            }

            .ant-anchor-ink-ball.visible {
              display: block;
            }

            .ant-anchor-content {
              position: relative;
            }

            .ant-anchor-link {
              position: relative;
              padding: 4px 0 4px 16px;
              line-height: 1.143;
            }

            .ant-anchor-link-title {
              position: relative;
              display: block;
              margin-bottom: 6px;
              color: rgba(0, 0, 0, 0.65);
              white-space: nowrap;
              overflow: hidden;
              text-overflow: ellipsis;
              text-decoration: none;
              transition: all 0.3s;
            }

            .ant-anchor-link-title:hover {
              color: #1890ff;
            }

            .ant-anchor-link-title:only-child {
              margin-bottom: 0;
            }

            .ant-anchor-link-active>.ant-anchor-link-title {
              color: #1890ff;
              font-weight: 600;
            }

            .ant-anchor-link-active::before {
              content: '';
              position: absolute;
              left: 0;
              top: 0;
              bottom: 0;
              width: 2px;
              background-color: #1890ff;
            }
            "#
        )
        .to_string()
    }

    /// 嵌套链接样式
    fn nested_links_style(&self) -> String {
        css!(
            r#"
            .ant-anchor-link .ant-anchor-link {
              padding-left: 32px;
            }

            .ant-anchor-link .ant-anchor-link .ant-anchor-link {
              padding-left: 48px;
            }

            .ant-anchor-link .ant-anchor-link .ant-anchor-link .ant-anchor-link {
              padding-left: 64px;
            }
            "#
        )
        .to_string()
    }

    /// 暗色主题样式
    fn dark_theme_style(&self) -> String {
        if self.dark_theme {
            css!(
                r#"
                .ant-theme-dark .ant-anchor {
                  color: rgba(255, 255, 255, 0.65);
                }

                .ant-theme-dark .ant-anchor-ink {
                  background-color: #434343;
                }

                .ant-theme-dark .ant-anchor-link-title {
                  color: rgba(255, 255, 255, 0.65);
                }

                .ant-theme-dark .ant-anchor-link-title:hover {
                  color: #1890ff;
                }

                .ant-theme-dark .ant-anchor-link-active>.ant-anchor-link-title {
                  color: #1890ff;
                }
                "#
            )
            .to_string()
        } else {
            "".to_string()
        }
    }

    /// 紧凑主题样式
    fn compact_theme_style(&self) -> String {
        if self.compact_theme {
            css!(
                r#"
                .ant-theme-compact .ant-anchor {
                  font-size: 12px;
                }

                .ant-theme-compact .ant-anchor-link {
                  padding: 2px 0 2px 12px;
                }

                .ant-theme-compact .ant-anchor-link-title {
                  margin-bottom: 4px;
                }
                "#
            )
            .to_string()
        } else {
            "".to_string()
        }
    }

    /// 响应式设计样式
    fn responsive_style(&self) -> String {
        css!(
            r#"
            @media (max-width: 768px) {
              .ant-anchor {
                display: none;
              }
            }
            "#
        )
        .to_string()
    }

    /// 高对比度模式样式
    fn high_contrast_style(&self) -> String {
        css!(
            r#"
            @media (prefers-contrast: more) {
              .ant-anchor-ink {
                background-color: #000;
                width: 3px;
              }

              .ant-anchor-link-title {
                color: #000;
              }

              .ant-anchor-link-active>.ant-anchor-link-title {
                color: #000;
                font-weight: 700;
              }

              .ant-anchor-link-active::before {
                background-color: #000;
                width: 3px;
              }
            }
            "#
        )
        .to_string()
    }

    /// 减少动画模式样式
    fn reduced_motion_style(&self) -> String {
        css!(
            r#"
            @media (prefers-reduced-motion: reduce) {
              .ant-anchor-ink-ball {
                transition: none;
              }

              .ant-anchor-link-title {
                transition: none;
              }
            }
            "#
        )
        .to_string()
    }

    /// 焦点样式
    fn focus_style(&self) -> String {
        css!(
            r#"
            .ant-anchor-link-title:focus {
              outline: 2px solid #1890ff;
              outline-offset: 1px;
            }
            "#
        )
        .to_string()
    }

    /// 打印样式
    fn print_style(&self) -> String {
        css!(
            r#"
            @media print {
              .ant-anchor {
                display: none;
              }
            }
            "#
        )
        .to_string()
    }

    /// 构建完整样式
    pub fn build(&self) -> String {
        let styles = vec![
            self.base_style(),
            self.nested_links_style(),
            self.dark_theme_style(),
            self.compact_theme_style(),
            self.responsive_style(),
            self.high_contrast_style(),
            self.reduced_motion_style(),
            self.focus_style(),
            self.print_style(),
        ];

        styles.join("\n")
    }
}

/// 生成 Anchor 样式
pub fn generate_anchor_style(dark_theme: bool, compact_theme: bool) -> String {
    AnchorStyleBuilder::new()
        .dark_theme(dark_theme)
        .compact_theme(compact_theme)
        .build()
}
