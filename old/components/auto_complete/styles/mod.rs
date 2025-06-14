//! AutoComplete 自动完成组件样式

use css_in_rust::css;

/// AutoComplete 尺寸
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AutoCompleteSize {
    /// 小尺寸
    Small,
    /// 中等尺寸（默认）
    Middle,
    /// 大尺寸
    Large,
}

impl Default for AutoCompleteSize {
    fn default() -> Self {
        Self::Middle
    }
}

/// AutoComplete 状态
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AutoCompleteStatus {
    /// 默认状态
    Default,
    /// 错误状态
    Error,
    /// 警告状态
    Warning,
}

impl Default for AutoCompleteStatus {
    fn default() -> Self {
        Self::Default
    }
}

/// AutoComplete 样式生成器
pub struct AutoCompleteStyleBuilder {
    /// 尺寸
    size: AutoCompleteSize,
    /// 状态
    status: AutoCompleteStatus,
    /// 是否禁用
    disabled: bool,
    /// 是否允许清除
    allow_clear: bool,
    /// 下拉菜单最大高度
    dropdown_max_height: u32,
    /// 是否使用暗色主题
    dark_theme: bool,
}

impl Default for AutoCompleteStyleBuilder {
    fn default() -> Self {
        Self {
            size: AutoCompleteSize::default(),
            status: AutoCompleteStatus::default(),
            disabled: false,
            allow_clear: false,
            dropdown_max_height: 256,
            dark_theme: false,
        }
    }
}

impl AutoCompleteStyleBuilder {
    /// 创建新的样式生成器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置尺寸
    pub fn size(mut self, size: AutoCompleteSize) -> Self {
        self.size = size;
        self
    }

    /// 设置状态
    pub fn status(mut self, status: AutoCompleteStatus) -> Self {
        self.status = status;
        self
    }

    /// 设置是否禁用
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 设置是否允许清除
    pub fn allow_clear(mut self, allow_clear: bool) -> Self {
        self.allow_clear = allow_clear;
        self
    }

    /// 设置下拉菜单最大高度
    pub fn dropdown_max_height(mut self, height: u32) -> Self {
        self.dropdown_max_height = height;
        self
    }

    /// 设置是否使用暗色主题
    pub fn dark_theme(mut self, dark_theme: bool) -> Self {
        self.dark_theme = dark_theme;
        self
    }

    /// 生成基础样式
    fn base_style(&self) -> String {
        css!(
            r#"
            .ant-auto-complete {
              position: relative;
              display: inline-block;
              width: 100%;
              box-sizing: border-box;
              margin: 0;
              padding: 0;
              color: rgba(0, 0, 0, 0.88);
              font-size: 14px;
              line-height: 1.5714285714285714;
              list-style: none;
              font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji';
            }

            .ant-auto-complete-input {
              position: relative;
              display: inline-block;
              width: 100%;
            }

            .ant-auto-complete .ant-input {
              position: relative;
              display: inline-block;
              width: 100%;
              min-width: 0;
              padding: 4px 11px;
              color: rgba(0, 0, 0, 0.88);
              font-size: 14px;
              line-height: 1.5714285714285714;
              background-color: #ffffff;
              background-image: none;
              border: 1px solid #d9d9d9;
              border-radius: 6px;
              transition: all 0.2s;
            }

            .ant-auto-complete .ant-input:hover {
              border-color: #4096ff;
              border-right-width: 1px;
            }

            .ant-auto-complete .ant-input:focus,
            .ant-auto-complete .ant-input-focused {
              border-color: #4096ff;
              box-shadow: 0 0 0 2px rgba(5, 145, 255, 0.1);
              border-right-width: 1px;
              outline: 0;
            }

            .ant-auto-complete .ant-input::placeholder {
              color: rgba(0, 0, 0, 0.25);
              user-select: none;
            }
            "#
        ).to_string()
    }

    /// 生成尺寸相关样式
    fn size_style(&self) -> String {
        match self.size {
            AutoCompleteSize::Small => css!(
                r#"
                .ant-auto-complete-small .ant-input,
                .ant-auto-complete .ant-input-sm {
                  padding: 0px 7px;
                  font-size: 14px;
                  border-radius: 4px;
                }
                "#
            )
            .to_string(),
            AutoCompleteSize::Middle => css!("").to_string(), // 默认尺寸，不需要额外样式
            AutoCompleteSize::Large => css!(
                r#"
                .ant-auto-complete-large .ant-input,
                .ant-auto-complete .ant-input-lg {
                  padding: 6.5px 11px;
                  font-size: 16px;
                  border-radius: 6px;
                }
                "#
            )
            .to_string(),
        }
    }

    /// 生成禁用状态样式
    fn disabled_style(&self) -> String {
        if self.disabled {
            css!(
                r#"
                .ant-auto-complete-disabled .ant-input,
                .ant-auto-complete .ant-input-disabled {
                  color: rgba(0, 0, 0, 0.25);
                  background-color: rgba(0, 0, 0, 0.04);
                  border-color: #d9d9d9;
                  box-shadow: none;
                  cursor: not-allowed;
                  opacity: 1;
                }

                .ant-auto-complete-disabled .ant-input:hover,
                .ant-auto-complete .ant-input-disabled:hover {
                  border-color: #d9d9d9;
                }
                "#
            )
            .to_string()
        } else {
            css!("").to_string()
        }
    }

    /// 生成状态相关样式
    fn status_style(&self) -> String {
        match self.status {
            AutoCompleteStatus::Error => css!(
                r#"
                .ant-auto-complete-status-error .ant-input,
                .ant-auto-complete .ant-input-status-error {
                  border-color: #ff4d4f;
                }

                .ant-auto-complete-status-error .ant-input:hover,
                .ant-auto-complete .ant-input-status-error:hover {
                  border-color: #ff4d4f;
                }

                .ant-auto-complete-status-error .ant-input:focus,
                .ant-auto-complete-status-error .ant-input-focused,
                .ant-auto-complete .ant-input-status-error:focus,
                .ant-auto-complete .ant-input-status-error.ant-input-focused {
                  border-color: #ff4d4f;
                  box-shadow: 0 0 0 2px rgba(255, 77, 79, 0.1);
                  border-right-width: 1px;
                  outline: 0;
                }
                "#
            )
            .to_string(),
            AutoCompleteStatus::Warning => css!(
                r#"
                .ant-auto-complete-status-warning .ant-input,
                .ant-auto-complete .ant-input-status-warning {
                  border-color: #faad14;
                }

                .ant-auto-complete-status-warning .ant-input:hover,
                .ant-auto-complete .ant-input-status-warning:hover {
                  border-color: #faad14;
                }

                .ant-auto-complete-status-warning .ant-input:focus,
                .ant-auto-complete-status-warning .ant-input-focused,
                .ant-auto-complete .ant-input-status-warning:focus,
                .ant-auto-complete .ant-input-status-warning.ant-input-focused {
                  border-color: #faad14;
                  box-shadow: 0 0 0 2px rgba(250, 173, 20, 0.1);
                  border-right-width: 1px;
                  outline: 0;
                }
                "#
            )
            .to_string(),
            AutoCompleteStatus::Default => css!("").to_string(),
        }
    }

    /// 生成清除按钮样式
    fn clear_style(&self) -> String {
        if self.allow_clear {
            css!(
                r#"
                .ant-auto-complete-clear {
                  position: absolute;
                  top: 50%;
                  right: 11px;
                  z-index: 1;
                  display: inline-block;
                  width: 16px;
                  height: 16px;
                  margin-top: -8px;
                  color: rgba(0, 0, 0, 0.25);
                  font-size: 16px;
                  font-style: normal;
                  line-height: 1;
                  text-align: center;
                  text-transform: none;
                  background-color: #fff;
                  cursor: pointer;
                  opacity: 0;
                  transition: color 0.3s ease, opacity 0.15s ease;
                  text-rendering: auto;
                }

                .ant-auto-complete:hover .ant-auto-complete-clear {
                  opacity: 1;
                }
                "#
            )
            .to_string()
        } else {
            css!("").to_string()
        }
    }

    /// 生成下拉菜单样式
    fn dropdown_style(&self) -> String {
        css!(
            r#"
            .ant-auto-complete-dropdown {
              position: absolute;
              top: 100%;
              left: 0;
              z-index: 1050;
              width: 100%;
              margin-top: 4px;
              padding: 4px 0;
              overflow: hidden;
              color: rgba(0, 0, 0, 0.88);
              font-size: 14px;
              font-variant: initial;
              background-color: #fff;
              border-radius: 6px;
              outline: none;
              box-shadow: 0 6px 16px 0 rgba(0, 0, 0, 0.08), 0 3px 6px -4px rgba(0, 0, 0, 0.12), 0 9px 28px 8px rgba(0, 0, 0, 0.05);
            }

            .ant-auto-complete-dropdown-hidden {
              display: none;
            }

            .ant-auto-complete-dropdown-empty {
              padding: 4px 0;
              color: rgba(0, 0, 0, 0.25);
              text-align: center;
            }

            .ant-auto-complete-dropdown-menu {
              max-height: ${dropdown_max_height}px;
              margin: 0;
              padding: 0;
              overflow: auto;
              list-style: none;
              outline: none;
            }

            .ant-auto-complete-dropdown-menu-item {
              position: relative;
              display: flex;
              align-items: center;
              padding: 5px 12px;
              color: rgba(0, 0, 0, 0.88);
              font-weight: normal;
              font-size: 14px;
              line-height: 22px;
              cursor: pointer;
              transition: background 0.3s ease;
            }

            .ant-auto-complete-dropdown-menu-item:hover {
              background-color: rgba(0, 0, 0, 0.04);
            }

            .ant-auto-complete-dropdown-menu-item-selected {
              background-color: rgba(0, 0, 0, 0.04);
              font-weight: 600;
            }

            .ant-auto-complete-dropdown-menu-item-active {
              background-color: rgba(0, 0, 0, 0.04);
            }

            .ant-auto-complete-dropdown-menu-item-disabled {
              color: rgba(0, 0, 0, 0.25);
              cursor: not-allowed;
            }

            .ant-auto-complete-dropdown-menu-item-disabled:hover {
              color: rgba(0, 0, 0, 0.25);
              background-color: transparent;
              cursor: not-allowed;
            }

            .ant-auto-complete-dropdown-menu-item-divider {
              height: 1px;
              margin: 4px 0;
              overflow: hidden;
              line-height: 0;
              background-color: rgba(5, 5, 5, 0.06);
            }
            "#,
            dropdown_max_height = self.dropdown_max_height
        ).to_string()
    }

    /// 生成暗色主题样式
    fn dark_theme_style(&self) -> String {
        if self.dark_theme {
            css!(
                r#"
                .ant-auto-complete.ant-auto-complete-dark,
                .ant-auto-complete.ant-auto-complete-dark .ant-auto-complete-input {
                  color: rgba(255, 255, 255, 0.85);
                }

                .ant-auto-complete.ant-auto-complete-dark .ant-input {
                  color: rgba(255, 255, 255, 0.85);
                  background-color: transparent;
                  border-color: #434343;
                }

                .ant-auto-complete.ant-auto-complete-dark .ant-input:hover {
                  border-color: #165996;
                }

                .ant-auto-complete.ant-auto-complete-dark .ant-input:focus,
                .ant-auto-complete.ant-auto-complete-dark .ant-input-focused {
                  border-color: #177ddc;
                  box-shadow: 0 0 0 2px rgba(23, 125, 220, 0.2);
                }

                .ant-auto-complete.ant-auto-complete-dark .ant-input::placeholder {
                  color: rgba(255, 255, 255, 0.3);
                }

                .ant-auto-complete.ant-auto-complete-dark .ant-auto-complete-clear {
                  color: rgba(255, 255, 255, 0.3);
                  background-color: transparent;
                }

                .ant-auto-complete.ant-auto-complete-dark .ant-auto-complete-dropdown {
                  background-color: #1f1f1f;
                  box-shadow: 0 6px 16px 0 rgba(0, 0, 0, 0.85), 0 3px 6px -4px rgba(0, 0, 0, 0.85), 0 9px 28px 8px rgba(0, 0, 0, 0.65);
                }

                .ant-auto-complete.ant-auto-complete-dark .ant-auto-complete-dropdown-empty {
                  color: rgba(255, 255, 255, 0.3);
                }

                .ant-auto-complete.ant-auto-complete-dark .ant-auto-complete-dropdown-menu-item {
                  color: rgba(255, 255, 255, 0.85);
                }

                .ant-auto-complete.ant-auto-complete-dark .ant-auto-complete-dropdown-menu-item:hover {
                  background-color: rgba(255, 255, 255, 0.08);
                }

                .ant-auto-complete.ant-auto-complete-dark .ant-auto-complete-dropdown-menu-item-selected {
                  background-color: rgba(255, 255, 255, 0.08);
                }

                .ant-auto-complete.ant-auto-complete-dark .ant-auto-complete-dropdown-menu-item-active {
                  background-color: rgba(255, 255, 255, 0.08);
                }

                .ant-auto-complete.ant-auto-complete-dark .ant-auto-complete-dropdown-menu-item-disabled {
                  color: rgba(255, 255, 255, 0.3);
                }

                .ant-auto-complete.ant-auto-complete-dark .ant-auto-complete-dropdown-menu-item-divider {
                  background-color: #303030;
                }
                "#
            ).to_string()
        } else {
            css!("").to_string()
        }
    }

    /// 高对比度模式样式
    fn high_contrast_style(&self) -> String {
        css!(
            r#"
            @media (prefers-contrast: more) {
              .ant-auto-complete .ant-input {
                border-color: black;
              }

              .ant-auto-complete .ant-input:focus,
              .ant-auto-complete .ant-input-focused {
                border-color: black;
                box-shadow: 0 0 0 2px rgba(0, 0, 0, 0.1);
              }

              .ant-auto-complete-dropdown {
                border: 1px solid black;
                box-shadow: none;
              }

              .ant-auto-complete-dropdown-menu-item-selected {
                background-color: rgba(0, 0, 0, 0.1);
              }
            }
            "#
        )
        .to_string()
    }

    /// 减少动画样式
    fn reduced_motion_style(&self) -> String {
        css!(
            r#"
            @media (prefers-reduced-motion: reduce) {
              .ant-auto-complete .ant-input,
              .ant-auto-complete-clear,
              .ant-auto-complete-dropdown-menu-item {
                transition: none;
              }
            }
            "#
        )
        .to_string()
    }

    /// 焦点可见样式
    fn focus_visible_style(&self) -> String {
        css!(
            r#"
            @media (prefers-reduced-motion: reduce) {
              .ant-auto-complete .ant-input:focus-visible {
                outline: 2px solid #000;
                outline-offset: 1px;
                transition: none;
              }
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
              .ant-auto-complete {
                color: #000;
              }

              .ant-auto-complete .ant-input {
                border-color: #000;
              }

              .ant-auto-complete-dropdown {
                display: none;
              }
            }
            "#
        )
        .to_string()
    }

    /// 响应式样式
    fn responsive_style(&self) -> String {
        css!(
            r#"
            @media (max-width: 575px) {
              .ant-auto-complete .ant-input {
                font-size: 14px;
              }

              .ant-auto-complete-dropdown {
                max-width: 300px;
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
            self.size_style(),
            self.disabled_style(),
            self.status_style(),
            self.clear_style(),
            self.dropdown_style(),
            self.dark_theme_style(),
            self.high_contrast_style(),
            self.reduced_motion_style(),
            self.focus_visible_style(),
            self.print_style(),
            self.responsive_style(),
        ];

        styles.join("\n")
    }
}

/// 生成 AutoComplete 样式
pub fn generate_auto_complete_style(
    size: AutoCompleteSize,
    status: AutoCompleteStatus,
    disabled: bool,
    allow_clear: bool,
    dropdown_max_height: u32,
    dark_theme: bool,
) -> String {
    AutoCompleteStyleBuilder::new()
        .size(size)
        .status(status)
        .disabled(disabled)
        .allow_clear(allow_clear)
        .dropdown_max_height(dropdown_max_height)
        .dark_theme(dark_theme)
        .build()
}

impl From<super::AutoCompleteSize> for AutoCompleteSize {
    fn from(size: super::AutoCompleteSize) -> Self {
        match size {
            super::AutoCompleteSize::Small => AutoCompleteSize::Small,
            super::AutoCompleteSize::Middle => AutoCompleteSize::Middle,
            super::AutoCompleteSize::Large => AutoCompleteSize::Large,
        }
    }
}

impl From<super::AutoCompleteStatus> for AutoCompleteStatus {
    fn from(status: super::AutoCompleteStatus) -> Self {
        match status {
            super::AutoCompleteStatus::Default => AutoCompleteStatus::Default,
            super::AutoCompleteStatus::Error => AutoCompleteStatus::Error,
            super::AutoCompleteStatus::Warning => AutoCompleteStatus::Warning,
        }
    }
}
