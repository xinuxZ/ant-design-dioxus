//! Switch 开关组件样式

/// Switch 组件尺寸
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SwitchSize {
    /// 默认尺寸
    Default,
    /// 小尺寸
    Small,
}

impl Default for SwitchSize {
    fn default() -> Self {
        Self::Default
    }
}

/// Switch 样式生成器
pub struct SwitchStyleGenerator {
    size: SwitchSize,
    checked: bool,
    disabled: bool,
    loading: bool,
    rtl: bool,
}

impl SwitchStyleGenerator {
    /// 创建一个新的 Switch 样式生成器
    pub fn new() -> Self {
        Self {
            size: SwitchSize::Default,
            checked: false,
            disabled: false,
            loading: false,
            rtl: false,
        }
    }

    /// 设置尺寸
    pub fn with_size(mut self, size: SwitchSize) -> Self {
        self.size = size;
        self
    }

    /// 设置选中状态
    pub fn with_checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// 设置禁用状态
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 设置加载状态
    pub fn with_loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// 设置 RTL 方向
    pub fn with_rtl(mut self, rtl: bool) -> Self {
        self.rtl = rtl;
        self
    }

    /// 生成基础样式
    fn generate_base_style(&self) -> String {
        let mut css = String::new();

        css.push_str(
            r#"
/* Switch 开关组件样式 */
/* 基于 Ant Design 5.25.3 版本的开关样式 */

.ant-switch {
  position: relative;
  display: inline-block;
  box-sizing: border-box;
  min-width: 44px;
  height: 22px;
  line-height: 22px;
  vertical-align: middle;
  background-color: rgba(0, 0, 0, 0.25);
  border: 0;
  border-radius: 100px;
  cursor: pointer;
  transition: all 0.2s;
  user-select: none;
  outline: none;
}

.ant-switch:focus {
  outline: 0;
  box-shadow: 0 0 0 2px rgba(5, 145, 255, 0.1);
}

.ant-switch:focus:hover {
  box-shadow: 0 0 0 2px rgba(5, 145, 255, 0.1);
}

.ant-switch-checked {
  background-color: #1677ff;
}

.ant-switch:hover:not(.ant-switch-disabled) {
  background-color: rgba(0, 0, 0, 0.45);
}

.ant-switch-checked:hover:not(.ant-switch-disabled) {
  background-color: #4096ff;
}

.ant-switch-loading,
.ant-switch-disabled {
  cursor: not-allowed;
  opacity: 0.4;
}

.ant-switch-loading *,
.ant-switch-disabled * {
  box-shadow: none;
  cursor: not-allowed;
}

.ant-switch-loading:hover:not(.ant-switch-disabled),
.ant-switch-disabled:hover {
  background-color: rgba(0, 0, 0, 0.25);
}

.ant-switch-checked.ant-switch-loading:hover:not(.ant-switch-disabled),
.ant-switch-checked.ant-switch-disabled:hover {
  background-color: #1677ff;
}

/* Switch 手柄 */
.ant-switch-handle {
  position: absolute;
  top: 2px;
  left: 2px;
  width: 18px;
  height: 18px;
  transition: all 0.2s ease-in-out;
}

.ant-switch-handle::before {
  position: absolute;
  top: 0;
  right: 0;
  bottom: 0;
  left: 0;
  background-color: #fff;
  border-radius: 9px;
  box-shadow: 0 2px 4px 0 rgba(0, 35, 11, 0.2);
  transition: all 0.2s ease-in-out;
  content: '';
}

.ant-switch-checked .ant-switch-handle {
  left: calc(100% - 20px);
}

.ant-switch-loading-icon {
  position: relative;
  display: inline-block;
  color: #1677ff;
  font-size: 12px;
  transform: translateY(-1px);
  animation: antSwitchLoadingIcon 1s infinite linear;
}

.ant-switch-checked .ant-switch-loading-icon {
  color: #fff;
}

/* Switch 内容区域 */
.ant-switch-inner {
  display: block;
  overflow: hidden;
  border-radius: 100px;
  height: 100%;
  padding-left: 24px;
  padding-right: 9px;
  transition: padding-left 0.2s ease-in-out, padding-right 0.2s ease-in-out;
}

.ant-switch-checked .ant-switch-inner {
  padding-left: 9px;
  padding-right: 24px;
}

.ant-switch-inner-checked,
.ant-switch-inner-unchecked {
  display: block;
  color: #fff;
  font-size: 12px;
  transition: margin-left 0.2s ease-in-out, margin-right 0.2s ease-in-out;
  pointer-events: none;
}

.ant-switch-inner-checked {
  margin-left: -18px;
  margin-right: 0;
}

.ant-switch-inner-unchecked {
  margin-top: -22px;
  margin-left: 0;
  margin-right: -18px;
}

.ant-switch-checked .ant-switch-inner-checked {
  margin-left: 0;
  margin-right: -18px;
}

.ant-switch-checked .ant-switch-inner-unchecked {
  margin-left: -18px;
  margin-right: 0;
}
"#,
        );

        css
    }

    /// 生成小尺寸样式
    fn generate_small_style(&self) -> String {
        if self.size != SwitchSize::Small {
            return String::new();
        }

        let mut css = String::new();

        css.push_str(
            r#"
/* 小尺寸开关 */
.ant-switch-small {
  min-width: 28px;
  height: 16px;
  line-height: 16px;
}

.ant-switch-small .ant-switch-handle {
  width: 12px;
  height: 12px;
  top: 2px;
  left: 2px;
}

.ant-switch-small .ant-switch-handle::before {
  border-radius: 6px;
}

.ant-switch-small.ant-switch-checked .ant-switch-handle {
  left: calc(100% - 14px);
}

.ant-switch-small .ant-switch-inner {
  padding-left: 18px;
  padding-right: 6px;
  font-size: 12px;
}

.ant-switch-small.ant-switch-checked .ant-switch-inner {
  padding-left: 6px;
  padding-right: 18px;
}

.ant-switch-small .ant-switch-inner-checked {
  margin-left: -12px;
  margin-right: 0;
}

.ant-switch-small .ant-switch-inner-unchecked {
  margin-top: -16px;
  margin-left: 0;
  margin-right: -12px;
}

.ant-switch-small.ant-switch-checked .ant-switch-inner-checked {
  margin-left: 0;
  margin-right: -12px;
}

.ant-switch-small.ant-switch-checked .ant-switch-inner-unchecked {
  margin-left: -12px;
  margin-right: 0;
}

.ant-switch-small .ant-switch-loading-icon {
  font-size: 9px;
  transform: translateY(-0.5px);
}
"#,
        );

        css
    }

    /// 生成动画样式
    fn generate_animation_style(&self) -> String {
        let mut css = String::new();

        css.push_str(
            r#"
/* 加载动画 */
@keyframes antSwitchLoadingIcon {
  0% {
    transform: rotate(0deg) translateY(-1px);
  }

  100% {
    transform: rotate(360deg) translateY(-1px);
  }
}

.ant-switch-small .ant-switch-loading-icon {
  animation: antSwitchLoadingIconSmall 1s infinite linear;
}

@keyframes antSwitchLoadingIconSmall {
  0% {
    transform: rotate(0deg) translateY(-0.5px);
  }

  100% {
    transform: rotate(360deg) translateY(-0.5px);
  }
}
"#,
        );

        css
    }

    /// 生成 RTL 样式
    fn generate_rtl_style(&self) -> String {
        if !self.rtl {
            return String::new();
        }

        let mut css = String::new();

        css.push_str(
            r#"
/* RTL 支持 */
.ant-switch-rtl {
  direction: rtl;
}

.ant-switch-rtl .ant-switch-handle {
  right: 2px;
  left: auto;
}

.ant-switch-rtl.ant-switch-checked .ant-switch-handle {
  right: calc(100% - 20px);
  left: auto;
}

.ant-switch-rtl.ant-switch-small .ant-switch-handle {
  right: 2px;
  left: auto;
}

.ant-switch-rtl.ant-switch-small.ant-switch-checked .ant-switch-handle {
  right: calc(100% - 14px);
  left: auto;
}
"#,
        );

        css
    }

    /// 生成完整样式
    pub fn generate(&self) -> String {
        let mut css = String::new();

        css.push_str(&self.generate_base_style());
        css.push_str(&self.generate_small_style());
        css.push_str(&self.generate_animation_style());
        css.push_str(&self.generate_rtl_style());

        css
    }
}

/// 生成 Switch 样式
pub fn generate_switch_style(
    size: SwitchSize,
    checked: bool,
    disabled: bool,
    loading: bool,
    rtl: bool,
) -> String {
    SwitchStyleGenerator::new()
        .with_size(size)
        .with_checked(checked)
        .with_disabled(disabled)
        .with_loading(loading)
        .with_rtl(rtl)
        .generate()
}

/// 默认 Switch 样式
pub fn default_switch_style() -> String {
    SwitchStyleGenerator::new().generate()
}
