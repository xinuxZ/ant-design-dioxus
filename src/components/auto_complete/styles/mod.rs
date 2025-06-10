//! AutoComplete 自动完成组件样式

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
        r#".ant-auto-complete {
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
}"#.to_string()
    }

    /// 生成尺寸相关样式
    fn size_style(&self) -> String {
        match self.size {
            AutoCompleteSize::Small => r#".ant-auto-complete-small .ant-input,
.ant-auto-complete .ant-input-sm {
  padding: 0px 7px;
  font-size: 14px;
  border-radius: 4px;
}"#
            .to_string(),
            AutoCompleteSize::Middle => "".to_string(), // 默认尺寸，不需要额外样式
            AutoCompleteSize::Large => r#".ant-auto-complete-large .ant-input,
.ant-auto-complete .ant-input-lg {
  padding: 6.5px 11px;
  font-size: 16px;
  border-radius: 6px;
}"#
            .to_string(),
        }
    }

    /// 生成禁用状态样式
    fn disabled_style(&self) -> String {
        if self.disabled {
            r#".ant-auto-complete-disabled .ant-input,
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
  border-right-width: 1px;
}"#
            .to_string()
        } else {
            "".to_string()
        }
    }

    /// 生成状态相关样式
    fn status_style(&self) -> String {
        match self.status {
            AutoCompleteStatus::Default => "".to_string(), // 默认状态，不需要额外样式
            AutoCompleteStatus::Error => r#".ant-auto-complete-status-error .ant-input,
.ant-auto-complete .ant-input-status-error {
  border-color: #ff4d4f;
}

.ant-auto-complete-status-error .ant-input:hover,
.ant-auto-complete .ant-input-status-error:hover {
  border-color: #ff4d4f;
  border-right-width: 1px;
}

.ant-auto-complete-status-error .ant-input:focus,
.ant-auto-complete-status-error .ant-input-focused,
.ant-auto-complete .ant-input-status-error:focus,
.ant-auto-complete .ant-input-status-error.ant-input-focused {
  border-color: #ff7875;
  box-shadow: 0 0 0 2px rgba(255, 77, 79, 0.2);
  border-right-width: 1px;
  outline: 0;
}"#
            .to_string(),
            AutoCompleteStatus::Warning => r#".ant-auto-complete-status-warning .ant-input,
.ant-auto-complete .ant-input-status-warning {
  border-color: #faad14;
}

.ant-auto-complete-status-warning .ant-input:hover,
.ant-auto-complete .ant-input-status-warning:hover {
  border-color: #faad14;
  border-right-width: 1px;
}

.ant-auto-complete-status-warning .ant-input:focus,
.ant-auto-complete-status-warning .ant-input-focused,
.ant-auto-complete .ant-input-status-warning:focus,
.ant-auto-complete .ant-input-status-warning.ant-input-focused {
  border-color: #ffc53d;
  box-shadow: 0 0 0 2px rgba(250, 173, 20, 0.2);
  border-right-width: 1px;
  outline: 0;
}"#
            .to_string(),
        }
    }

    /// 生成清除按钮样式
    fn clear_style(&self) -> String {
        if self.allow_clear {
            r#".ant-auto-complete-clear {
  position: absolute;
  top: 50%;
  right: 8px;
  z-index: 1;
  display: inline-block;
  width: 12px;
  height: 12px;
  margin-top: -6px;
  color: rgba(0, 0, 0, 0.25);
  font-size: 12px;
  font-style: normal;
  line-height: 1;
  text-align: center;
  text-transform: none;
  background: #ffffff;
  cursor: pointer;
  opacity: 0;
  transition: color 0.3s ease, opacity 0.15s ease;
  text-rendering: auto;
}

.ant-auto-complete:hover .ant-auto-complete-clear {
  opacity: 1;
}

.ant-auto-complete-clear:hover {
  color: rgba(0, 0, 0, 0.45);
}

.ant-auto-complete-clear:active {
  color: rgba(0, 0, 0, 0.88);
}"#
            .to_string()
        } else {
            "".to_string()
        }
    }

    /// 生成下拉菜单样式
    fn dropdown_style(&self) -> String {
        format!(
            r#".ant-auto-complete-dropdown {{
  position: absolute;
  top: 100%;
  left: 0;
  z-index: 1050;
  box-sizing: border-box;
  width: 100%;
  margin: 4px 0 0;
  padding: 4px 0;
  overflow: auto;
  font-size: 14px;
  font-variant: initial;
  background-color: #ffffff;
  border-radius: 6px;
  outline: none;
  box-shadow: 0 6px 16px 0 rgba(0, 0, 0, 0.08), 0 3px 6px -4px rgba(0, 0, 0, 0.12), 0 9px 28px 8px rgba(0, 0, 0, 0.05);
}}

.ant-auto-complete-dropdown-hidden {{
  display: none;
}}

.ant-auto-complete-dropdown-menu {{
  max-height: {}px;
  margin: 0;
  padding: 0;
  overflow: auto;
  list-style: none;
  outline: none;
}}

.ant-auto-complete-dropdown-menu-item {{
  position: relative;
  display: block;
  min-height: 32px;
  padding: 5px 12px;
  color: rgba(0, 0, 0, 0.88);
  font-weight: normal;
  line-height: 22px;
  cursor: pointer;
  transition: background 0.3s ease;
}}

.ant-auto-complete-dropdown-menu-item:hover {{
  background-color: rgba(0, 0, 0, 0.04);
}}

.ant-auto-complete-dropdown-menu-item-active {{
  background-color: rgba(0, 0, 0, 0.04);
}}

.ant-auto-complete-dropdown-menu-item-disabled {{
  color: rgba(0, 0, 0, 0.25);
  cursor: not-allowed;
}}

.ant-auto-complete-dropdown-menu-item-disabled:hover {{
  color: rgba(0, 0, 0, 0.25);
  background-color: transparent;
  cursor: not-allowed;
}}

.ant-auto-complete-dropdown-menu-item-selected {{
  color: rgba(0, 0, 0, 0.88);
  font-weight: 600;
  background-color: rgba(5, 145, 255, 0.1);
}}

.ant-auto-complete-dropdown-menu-item-selected:hover {{
  background-color: rgba(5, 145, 255, 0.15);
}}

.ant-auto-complete-dropdown-menu-empty {{
  padding: 5px 12px;
  color: rgba(0, 0, 0, 0.25);
}}

.ant-auto-complete-open .ant-auto-complete-dropdown {{
  display: block;
}}"#,
            self.dropdown_max_height
        )
    }

    /// 生成暗色主题样式
    fn dark_theme_style(&self) -> String {
        if self.dark_theme {
            r#"@media (prefers-color-scheme: dark) {
  .ant-auto-complete {
    color: rgba(255, 255, 255, 0.85);
  }

  .ant-auto-complete .ant-input {
    color: rgba(255, 255, 255, 0.85);
    background-color: #141414;
    border-color: #424242;
  }

  .ant-auto-complete .ant-input:hover {
    border-color: #177ddc;
  }

  .ant-auto-complete .ant-input:focus,
  .ant-auto-complete .ant-input-focused {
    border-color: #177ddc;
    box-shadow: 0 0 0 2px rgba(23, 125, 220, 0.2);
  }

  .ant-auto-complete .ant-input::placeholder {
    color: rgba(255, 255, 255, 0.3);
  }

  .ant-auto-complete-disabled .ant-input,
  .ant-auto-complete .ant-input-disabled {
    color: rgba(255, 255, 255, 0.3);
    background-color: rgba(255, 255, 255, 0.08);
    border-color: #424242;
  }

  .ant-auto-complete-clear {
    background: #141414;
  }

  .ant-auto-complete-clear:hover {
    color: rgba(255, 255, 255, 0.45);
  }

  .ant-auto-complete-clear:active {
    color: rgba(255, 255, 255, 0.85);
  }

  .ant-auto-complete-dropdown {
    background-color: #1f1f1f;
    box-shadow: 0 6px 16px 0 rgba(0, 0, 0, 0.3), 0 3px 6px -4px rgba(0, 0, 0, 0.3), 0 9px 28px 8px rgba(0, 0, 0, 0.2);
  }

  .ant-auto-complete-dropdown-menu-item {
    color: rgba(255, 255, 255, 0.85);
  }

  .ant-auto-complete-dropdown-menu-item:hover {
    background-color: rgba(255, 255, 255, 0.08);
  }

  .ant-auto-complete-dropdown-menu-item-active {
    background-color: rgba(255, 255, 255, 0.08);
  }

  .ant-auto-complete-dropdown-menu-item-disabled {
    color: rgba(255, 255, 255, 0.3);
  }

  .ant-auto-complete-dropdown-menu-item-selected {
    color: rgba(255, 255, 255, 0.85);
    background-color: rgba(23, 125, 220, 0.2);
  }

  .ant-auto-complete-dropdown-menu-item-selected:hover {
    background-color: rgba(23, 125, 220, 0.3);
  }

  .ant-auto-complete-dropdown-menu-empty {
    color: rgba(255, 255, 255, 0.3);
  }
}"#.to_string()
        } else {
            "".to_string()
        }
    }

    /// 生成高对比度模式样式
    fn high_contrast_style(&self) -> String {
        r#"@media (prefers-contrast: high) {
  .ant-auto-complete .ant-input {
    border-color: #000;
    border-width: 2px;
  }

  .ant-auto-complete .ant-input:focus,
  .ant-auto-complete .ant-input-focused {
    border-color: #0000ff;
    box-shadow: none;
  }

  .ant-auto-complete-status-error .ant-input {
    border-color: #ff0000;
    border-width: 2px;
  }

  .ant-auto-complete-status-warning .ant-input {
    border-color: #ff8c00;
    border-width: 2px;
  }

  .ant-auto-complete-dropdown {
    border: 2px solid #000;
    box-shadow: none;
  }

  .ant-auto-complete-dropdown-menu-item-active {
    background-color: #0000ff;
    color: #fff;
  }
}"#
        .to_string()
    }

    /// 生成减少动画模式样式
    fn reduced_motion_style(&self) -> String {
        r#"@media (prefers-reduced-motion: reduce) {
  .ant-auto-complete .ant-input {
    transition: none;
  }

  .ant-auto-complete-clear {
    transition: none;
  }

  .ant-auto-complete-dropdown-menu-item {
    transition: none;
  }
}"#
        .to_string()
    }

    /// 生成焦点可见样式
    fn focus_visible_style(&self) -> String {
        r#".ant-auto-complete .ant-input:focus-visible {
  outline: 2px solid #1890ff;
  outline-offset: 1px;
}

.ant-auto-complete-dropdown-menu-item:focus-visible {
  outline: 2px solid #1890ff;
  outline-offset: -2px;
  background-color: rgba(0, 0, 0, 0.04);
}"#
        .to_string()
    }

    /// 生成打印样式
    fn print_style(&self) -> String {
        r#"@media print {
  .ant-auto-complete-dropdown {
    display: none;
  }

  .ant-auto-complete .ant-input {
    border-color: #000;
  }

  .ant-auto-complete-clear {
    display: none;
  }
}"#
        .to_string()
    }

    /// 生成响应式样式
    fn responsive_style(&self) -> String {
        r#"@media (max-width: 575px) {
  .ant-auto-complete-dropdown {
    max-width: 300px;
  }

  .ant-auto-complete-dropdown-menu-item {
    padding: 8px 12px;
  }
}"#
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

        styles
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

/// 生成AutoComplete样式
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
            super::AutoCompleteSize::Small => Self::Small,
            super::AutoCompleteSize::Middle => Self::Middle,
            super::AutoCompleteSize::Large => Self::Large,
        }
    }
}

impl From<super::AutoCompleteStatus> for AutoCompleteStatus {
    fn from(status: super::AutoCompleteStatus) -> Self {
        match status {
            super::AutoCompleteStatus::Default => Self::Default,
            super::AutoCompleteStatus::Error => Self::Error,
            super::AutoCompleteStatus::Warning => Self::Warning,
        }
    }
}
