//! Select 选择器组件样式

/// Select 选择器样式生成器
pub struct SelectStyleBuilder {
    /// 尺寸
    size: SelectSize,
    /// 状态
    status: SelectStatus,
    /// 是否禁用
    disabled: bool,
    /// 是否多选
    multiple: bool,
    /// 是否支持搜索
    show_search: bool,
    /// 是否有边框
    bordered: bool,
}

/// Select 选择器尺寸
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectSize {
    /// 大尺寸
    Large,
    /// 中等尺寸（默认）
    Middle,
    /// 小尺寸
    Small,
}

impl Default for SelectSize {
    fn default() -> Self {
        Self::Middle
    }
}

/// Select 选择器状态
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectStatus {
    /// 正常状态
    Normal,
    /// 错误状态
    Error,
    /// 警告状态
    Warning,
}

impl Default for SelectStatus {
    fn default() -> Self {
        Self::Normal
    }
}

impl From<super::SelectSize> for SelectSize {
    fn from(size: super::SelectSize) -> Self {
        match size {
            super::SelectSize::Large => Self::Large,
            super::SelectSize::Middle => Self::Middle,
            super::SelectSize::Small => Self::Small,
        }
    }
}

impl From<super::SelectStatus> for SelectStatus {
    fn from(status: super::SelectStatus) -> Self {
        match status {
            super::SelectStatus::Normal => Self::Normal,
            super::SelectStatus::Error => Self::Error,
            super::SelectStatus::Warning => Self::Warning,
        }
    }
}

impl Default for SelectStyleBuilder {
    fn default() -> Self {
        Self {
            size: SelectSize::default(),
            status: SelectStatus::default(),
            disabled: false,
            multiple: false,
            show_search: false,
            bordered: true,
        }
    }
}

impl SelectStyleBuilder {
    /// 创建新实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置尺寸
    pub fn size(mut self, size: SelectSize) -> Self {
        self.size = size;
        self
    }

    /// 设置状态
    pub fn status(mut self, status: SelectStatus) -> Self {
        self.status = status;
        self
    }

    /// 设置是否禁用
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 设置是否多选
    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    /// 设置是否支持搜索
    pub fn show_search(mut self, show_search: bool) -> Self {
        self.show_search = show_search;
        self
    }

    /// 设置是否有边框
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    /// 基础样式
    fn base_style(&self) -> String {
        ".ant-select {
  position: relative;
  display: inline-block;
  min-width: 120px;
  font-size: 14px;
  line-height: 1.5715;
  color: rgba(0, 0, 0, 0.88);
  background-color: #ffffff;
  border: 1px solid #d9d9d9;
  border-radius: 6px;
  transition: all 0.2s cubic-bezier(0.645, 0.045, 0.355, 1);
  cursor: pointer;
}

.ant-select:hover {
  border-color: #4096ff;
}

.ant-select-focused {
  border-color: #4096ff;
  box-shadow: 0 0 0 2px rgba(5, 145, 255, 0.1);
  outline: 0;
}

.ant-select-open {
  border-color: #4096ff;
}"
        .to_string()
    }

    /// 选择器容器样式
    fn selector_style(&self) -> String {
        ".ant-select-selector {
  position: relative;
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  padding: 4px 11px;
  min-height: 32px;
  outline: none;
}

.ant-select-selection-item {
  flex: 1;
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
  font-weight: 400;
  color: rgba(0, 0, 0, 0.88);
}

.ant-select-selection-placeholder {
  flex: 1;
  color: rgba(0, 0, 0, 0.25);
  pointer-events: none;
}"
        .to_string()
    }

    /// 搜索框样式
    fn search_style(&self) -> String {
        ".ant-select-selection-search {
  position: relative;
  flex: 1;
  max-width: 100%;
}

.ant-select-selection-search-input {
  width: 100%;
  min-width: 4px;
  padding: 0;
  background: transparent;
  border: none;
  outline: none;
  appearance: none;
  font-size: inherit;
  line-height: inherit;
  color: inherit;
}"
        .to_string()
    }

    /// 多选样式
    fn multiple_style(&self) -> String {
        ".ant-select-multiple .ant-select-selector {
  padding: 1px 11px 1px 4px;
}

.ant-select-selection-overflow {
  position: relative;
  display: flex;
  flex: auto;
  flex-wrap: wrap;
  max-width: 100%;
}

.ant-select-selection-overflow-item {
  flex: none;
  max-width: 100%;
  margin: 2px 4px 2px 0;
}

.ant-select-multiple .ant-select-selection-item {
  position: relative;
  display: flex;
  flex: none;
  box-sizing: border-box;
  max-width: 100%;
  height: 24px;
  margin-top: 2px;
  margin-bottom: 2px;
  line-height: 22px;
  background: #f5f5f5;
  border: 1px solid #f0f0f0;
  border-radius: 4px;
  cursor: default;
  transition: font-size 0.3s, line-height 0.3s, height 0.3s;
  user-select: none;
  margin-inline-end: 4px;
  padding-inline-start: 8px;
  padding-inline-end: 4px;
}

.ant-select-selection-item-content {
  display: inline-block;
  margin-inline-end: 4px;
  overflow: hidden;
  white-space: pre;
  text-overflow: ellipsis;
}

.ant-select-selection-item-remove {
  display: inline-block;
  color: rgba(0, 0, 0, 0.45);
  font-weight: bold;
  font-size: 12px;
  line-height: inherit;
  cursor: pointer;
  transition: all 0.2s;
}

.ant-select-selection-item-remove:hover {
  color: rgba(0, 0, 0, 0.75);
}"
        .to_string()
    }

    /// 清除按钮样式
    fn clear_style(&self) -> String {
        ".ant-select-clear {
  position: absolute;
  top: 50%;
  inset-inline-end: 25px;
  z-index: 1;
  display: inline-block;
  width: 12px;
  height: 12px;
  margin-top: -6px;
  color: rgba(0, 0, 0, 0.25);
  font-size: 12px;
  font-style: normal;
  line-height: 12px;
  text-align: center;
  text-transform: none;
  background: #fff;
  cursor: pointer;
  opacity: 0;
  transition: color 0.3s ease, opacity 0.15s ease;
  text-rendering: auto;
}

.ant-select:hover .ant-select-clear {
  opacity: 1;
}

.ant-select-clear:hover {
  color: rgba(0, 0, 0, 0.45);
}"
        .to_string()
    }

    /// 箭头样式
    fn arrow_style(&self) -> String {
        ".ant-select-arrow {
  position: absolute;
  top: 50%;
  inset-inline-end: 11px;
  width: 12px;
  height: 12px;
  margin-top: -6px;
  color: rgba(0, 0, 0, 0.25);
  font-size: 12px;
  line-height: 1;
  text-align: center;
  pointer-events: none;
}

.ant-select-suffix {
  display: flex;
  flex: none;
  align-items: center;
  pointer-events: auto;
}"
        .to_string()
    }

    /// 下拉菜单样式
    fn dropdown_style(&self) -> String {
        ".ant-select-dropdown {
  position: absolute;
  top: 100%;
  left: 0;
  z-index: 1050;
  width: auto;
  min-width: 120px;
  max-height: 256px;
  margin: 4px 0;
  padding: 4px 0;
  overflow: auto;
  font-size: 14px;
  font-variant: initial;
  background-color: #ffffff;
  border-radius: 6px;
  outline: none;
  box-shadow: 0 6px 16px 0 rgba(0, 0, 0, 0.08), 0 3px 6px -4px rgba(0, 0, 0, 0.12), 0 9px 28px 8px rgba(0, 0, 0, 0.05);
}

.ant-select-dropdown-menu {
  max-height: 250px;
  margin: 0;
  padding: 0;
  overflow: auto;
  list-style: none;
  outline: none;
}

.ant-select-item {
  position: relative;
  display: flex;
  align-items: center;
  padding: 5px 12px;
  color: rgba(0, 0, 0, 0.88);
  font-weight: normal;
  font-size: 14px;
  line-height: 1.5;
  cursor: pointer;
  transition: background 0.3s ease;
}

.ant-select-item:hover {
  background-color: #f5f5f5;
}

.ant-select-item-option {
  min-height: 32px;
}

.ant-select-item-option-content {
  flex: auto;
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
}

.ant-select-item-option-selected {
  font-weight: 600;
  background-color: #e6f4ff;
}

.ant-select-item-option-disabled {
  color: rgba(0, 0, 0, 0.25);
  cursor: not-allowed;
}

.ant-select-item-option-disabled:hover {
  background-color: transparent;
}".to_string()
    }

    /// 选项组样式
    fn option_group_style(&self) -> String {
        ".ant-select-item-group {
  color: rgba(0, 0, 0, 0.45);
  font-size: 12px;
  cursor: default;
}

.ant-select-item-group-label {
  padding: 6px 12px;
  color: rgba(0, 0, 0, 0.45);
  font-size: 12px;
  line-height: 1.5;
}"
        .to_string()
    }

    /// 尺寸相关样式
    fn size_style(&self) -> String {
        match self.size {
            SelectSize::Large => ".ant-select-lg {
  font-size: 16px;
}

.ant-select-lg .ant-select-selector {
  padding: 6.5px 11px;
  min-height: 40px;
}

.ant-select-lg .ant-select-selection-item {
  line-height: 40px;
}"
            .to_string(),
            SelectSize::Middle => "".to_string(),
            SelectSize::Small => ".ant-select-sm {
  font-size: 14px;
}

.ant-select-sm .ant-select-selector {
  padding: 0px 7px;
  min-height: 24px;
}

.ant-select-sm .ant-select-selection-item {
  line-height: 24px;
}

.ant-select-sm .ant-select-arrow {
  width: 10px;
  margin-top: -5px;
}

.ant-select-sm .ant-select-clear {
  margin-top: -5px;
}"
            .to_string(),
        }
    }

    /// 状态样式
    fn status_style(&self) -> String {
        match self.status {
            SelectStatus::Normal => "".to_string(),
            SelectStatus::Error => ".ant-select-status-error {
  border-color: #ff4d4f;
}

.ant-select-status-error:hover {
  border-color: #ff4d4f;
}

.ant-select-status-error.ant-select-focused {
  border-color: #ff7875;
  box-shadow: 0 0 0 2px rgba(255, 77, 79, 0.2);
}"
            .to_string(),
            SelectStatus::Warning => ".ant-select-status-warning {
  border-color: #faad14;
}

.ant-select-status-warning:hover {
  border-color: #faad14;
}

.ant-select-status-warning.ant-select-focused {
  border-color: #ffc53d;
  box-shadow: 0 0 0 2px rgba(250, 173, 20, 0.2);
}"
            .to_string(),
        }
    }

    /// 禁用样式
    fn disabled_style(&self) -> String {
        if self.disabled {
            ".ant-select-disabled {
  color: rgba(0, 0, 0, 0.25);
  background: rgba(0, 0, 0, 0.04);
  cursor: not-allowed;
}

.ant-select-disabled:hover {
  border-color: #d9d9d9;
}

.ant-select-disabled .ant-select-selector {
  color: rgba(0, 0, 0, 0.25);
  background: rgba(0, 0, 0, 0.04);
  cursor: not-allowed;
}

.ant-select-disabled .ant-select-arrow {
  color: rgba(0, 0, 0, 0.25);
}"
            .to_string()
        } else {
            "".to_string()
        }
    }

    /// 无边框样式
    fn borderless_style(&self) -> String {
        if !self.bordered {
            ".ant-select-borderless {
  border-color: transparent !important;
  box-shadow: none !important;
}

.ant-select-borderless:hover {
  border-color: transparent !important;
}

.ant-select-borderless.ant-select-focused {
  border-color: transparent !important;
}"
            .to_string()
        } else {
            "".to_string()
        }
    }

    /// 搜索样式
    fn show_search_style(&self) -> String {
        if self.show_search {
            ".ant-select-show-search .ant-select-selection-item {
  pointer-events: none;
}

.ant-select-show-search .ant-select-selection-search {
  position: absolute;
  left: 11px;
  top: 0;
  bottom: 0;
  right: 11px;
}

.ant-select-show-search .ant-select-selection-search-input {
  cursor: auto;
}"
            .to_string()
        } else {
            "".to_string()
        }
    }

    /// RTL 样式
    fn rtl_style(&self) -> String {
        ".ant-select[dir=\"rtl\"] .ant-select-arrow {
  right: auto;
  left: 11px;
}

.ant-select[dir=\"rtl\"] .ant-select-clear {
  right: auto;
  left: 25px;
}"
        .to_string()
    }

    /// 响应式样式
    fn responsive_style(&self) -> String {
        "@media (max-width: 575px) {
  .ant-select {
    width: 100%;
  }

  .ant-select-dropdown {
    max-width: calc(100vw - 16px);
  }
}"
        .to_string()
    }

    /// 动画样式
    fn animation_style(&self) -> String {
        "@keyframes ant-select-dropdown-slide-up {
  0% {
    opacity: 0;
    transform-origin: 0 0;
    transform: scaleY(0);
  }

  100% {
    opacity: 1;
    transform-origin: 0 0;
    transform: scaleY(1);
  }
}

.ant-select-dropdown {
  animation-name: ant-select-dropdown-slide-up;
  animation-duration: 0.3s;
}"
        .to_string()
    }

    /// 高对比度模式样式
    fn contrast_style(&self) -> String {
        "@media (prefers-contrast: high) {
  .ant-select {
    border: 1px solid transparent;
  }

  .ant-select-focused {
    outline: 2px solid #000;
    outline-offset: 1px;
    transition: none;
  }
}"
        .to_string()
    }

    /// 暗黑模式样式
    fn dark_mode_style(&self) -> String {
        "@media (prefers-color-scheme: dark) {
  .ant-select {
    background-color: #141414;
    border-color: #424242;
    color: rgba(255, 255, 255, 0.85);
  }

  .ant-select:hover {
    border-color: #177ddc;
  }

  .ant-select-focused {
    border-color: #177ddc;
    box-shadow: 0 0 0 2px rgba(23, 125, 220, 0.2);
  }

  .ant-select-dropdown {
    background-color: #1f1f1f;
    box-shadow: 0 3px 6px -4px rgba(0, 0, 0, 0.48), 0 6px 16px 0 rgba(0, 0, 0, 0.32), 0 9px 28px 8px rgba(0, 0, 0, 0.2);
  }

  .ant-select-item {
    color: rgba(255, 255, 255, 0.85);
  }

  .ant-select-item:hover {
    background-color: rgba(255, 255, 255, 0.08);
  }

  .ant-select-selection-placeholder {
    color: rgba(255, 255, 255, 0.25);
  }

  .ant-select-disabled {
    color: rgba(255, 255, 255, 0.3);
    background: rgba(255, 255, 255, 0.08);
  }
}".to_string()
    }

    /// 构建完整样式
    pub fn build(&self) -> String {
        let styles = vec![
            self.base_style(),
            self.selector_style(),
            self.search_style(),
            self.multiple_style(),
            self.clear_style(),
            self.arrow_style(),
            self.dropdown_style(),
            self.option_group_style(),
            self.size_style(),
            self.status_style(),
            self.disabled_style(),
            self.borderless_style(),
            self.show_search_style(),
            self.rtl_style(),
            self.responsive_style(),
            self.animation_style(),
            self.contrast_style(),
            self.dark_mode_style(),
        ];

        styles.join("\n\n")
    }
}
