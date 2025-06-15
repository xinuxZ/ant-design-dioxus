use dioxus::prelude::*;
use std::borrow::Cow;

/// 按钮类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonType {
    Primary,
    Default,
    Dashed,
    Text,
    Link,
}

impl Default for ButtonType {
    fn default() -> Self {
        Self::Default
    }
}

/// 按钮颜色
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ButtonColor {
    Primary,
    Default,
    Danger,
    Custom(String),
}

impl Default for ButtonColor {
    fn default() -> Self {
        Self::Default
    }
}

/// 按钮变体
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Outlined,
    Solid,
    Dashed,
    Text,
    Link,
}

impl Default for ButtonVariant {
    fn default() -> Self {
        Self::Outlined
    }
}

/// 按钮大小
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    Large,
    Middle,
    Small,
}

impl Default for ButtonSize {
    fn default() -> Self {
        Self::Middle
    }
}

/// 按钮形状
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonShape {
    Default,
    Circle,
    Round,
}

impl Default for ButtonShape {
    fn default() -> Self {
        Self::Default
    }
}

/// 图标位置
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconPosition {
    Start,
    End,
}

impl Default for IconPosition {
    fn default() -> Self {
        Self::Start
    }
}

/// 加载配置
#[derive(Debug, Clone, PartialEq)]
pub enum LoadingConfig {
    NotLoading,
    Loading,
    DelayedLoading(u32), // 延迟时间（毫秒）
}

impl Default for LoadingConfig {
    fn default() -> Self {
        Self::NotLoading
    }
}

/// 按钮 HTML 类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HtmlType {
    Button,
    Submit,
    Reset,
}

impl Default for HtmlType {
    fn default() -> Self {
        Self::Button
    }
}

/// 按钮属性
#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    /// 按钮类型
    #[props(into, default)]
    pub button_type: Option<ButtonType>,

    /// 按钮颜色
    #[props(into, default)]
    pub color: Option<ButtonColor>,

    /// 按钮变体
    #[props(into, default)]
    pub variant: Option<ButtonVariant>,

    /// 将按钮宽度调整为其父宽度
    #[props(default)]
    pub block: bool,

    /// 设置危险按钮
    #[props(default)]
    pub danger: bool,

    /// 按钮失效状态
    #[props(default)]
    pub disabled: bool,

    /// 幽灵属性，使按钮背景透明
    #[props(default)]
    pub ghost: bool,

    /// 点击跳转的地址
    #[props(into, default)]
    pub href: Option<String>,

    /// 设置 button 原生的 type 值
    #[props(default)]
    pub html_type: HtmlType,

    /// 设置按钮的图标组件
    #[props(into, default)]
    pub icon: Option<Element>,

    /// 设置图标位置
    #[props(default)]
    pub icon_position: IconPosition,

    /// 设置按钮载入状态
    #[props(default)]
    pub loading: LoadingConfig,

    /// 设置按钮形状
    #[props(default)]
    pub shape: ButtonShape,

    /// 设置按钮大小
    #[props(default)]
    pub size: ButtonSize,

    /// 相当于 a 链接的 target 属性
    #[props(into, default)]
    pub target: Option<String>,

    /// CSS 类名
    #[props(into, default)]
    pub class: Option<String>,

    /// 内联样式
    #[props(into, default)]
    pub style: Option<String>,

    /// 点击按钮时的回调
    #[props(into, default)]
    pub on_click: Option<EventHandler<MouseEvent>>,

    /// 自动在两个中文字符之间插入空格
    #[props(default = true)]
    pub auto_insert_space: bool,

    /// 无障碍访问描述
    #[props(into, default)]
    pub aria_label: Option<String>,

    /// 子元素
    #[props(default)]
    pub children: Element,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试 ButtonType 的默认值
    #[test]
    fn test_button_type_default() {
        let default_type = ButtonType::default();
        assert_eq!(default_type, ButtonType::Default);
    }

    /// 测试 ButtonType 的克隆
    #[test]
    fn test_button_type_clone() {
        let original = ButtonType::Primary;
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    /// 测试 ButtonType 的所有变体
    #[test]
    fn test_button_type_variants() {
        let types = [
            ButtonType::Primary,
            ButtonType::Default,
            ButtonType::Dashed,
            ButtonType::Text,
            ButtonType::Link,
        ];

        for button_type in types.iter() {
            // 测试克隆
            let cloned = button_type.clone();
            assert_eq!(*button_type, cloned);
        }
    }

    /// 测试 ButtonSize 的默认值
    #[test]
    fn test_button_size_default() {
        let default_size = ButtonSize::default();
        assert_eq!(default_size, ButtonSize::Middle);
    }

    /// 测试 ButtonSize 的克隆
    #[test]
    fn test_button_size_clone() {
        let original = ButtonSize::Large;
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    /// 测试 ButtonSize 的所有变体
    #[test]
    fn test_button_size_variants() {
        let sizes = [ButtonSize::Large, ButtonSize::Middle, ButtonSize::Small];

        for size in sizes.iter() {
            // 测试克隆
            let cloned = size.clone();
            assert_eq!(*size, cloned);
        }
    }

    /// 测试 ButtonShape 的默认值
    #[test]
    fn test_button_shape_default() {
        let default_shape = ButtonShape::default();
        assert_eq!(default_shape, ButtonShape::Default);
    }

    /// 测试 ButtonShape 的克隆
    #[test]
    fn test_button_shape_clone() {
        let original = ButtonShape::Circle;
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    /// 测试 ButtonShape 的所有变体
    #[test]
    fn test_button_shape_variants() {
        let shapes = [
            ButtonShape::Default,
            ButtonShape::Circle,
            ButtonShape::Round,
        ];

        for shape in shapes.iter() {
            // 测试克隆
            let cloned = shape.clone();
            assert_eq!(*shape, cloned);
        }
    }

    /// 测试 ButtonVariant 的默认值
    #[test]
    fn test_button_variant_default() {
        let default_variant = ButtonVariant::default();
        assert_eq!(default_variant, ButtonVariant::Solid);
    }

    /// 测试 ButtonVariant 的克隆
    #[test]
    fn test_button_variant_clone() {
        let original = ButtonVariant::Outlined;
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    /// 测试 ButtonVariant 的所有变体
    #[test]
    fn test_button_variant_variants() {
        let variants = [
            ButtonVariant::Solid,
            ButtonVariant::Outlined,
            ButtonVariant::Dashed,
            // ButtonVariant::Filled,
            ButtonVariant::Text,
            ButtonVariant::Link,
        ];

        for variant in variants.iter() {
            // 测试克隆
            let cloned = variant.clone();
            assert_eq!(*variant, cloned);
        }
    }

    /// 测试 ButtonColor 的默认值
    #[test]
    fn test_button_color_default() {
        let default_color = ButtonColor::default();
        assert_eq!(default_color, ButtonColor::Default);
    }

    /// 测试 ButtonColor 的克隆
    #[test]
    fn test_button_color_clone() {
        let original = ButtonColor::Primary;
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    /// 测试 ButtonColor 的所有变体
    #[test]
    fn test_button_color_variants() {
        let colors = [
            ButtonColor::Default,
            ButtonColor::Primary,
            ButtonColor::Danger,
        ];

        for color in colors.iter() {
            // 测试克隆
            let cloned = color.clone();
            assert_eq!(*color, cloned);
        }
    }

    /// 测试 HtmlType 的默认值
    #[test]
    fn test_html_type_default() {
        let default_html_type = HtmlType::default();
        assert_eq!(default_html_type, HtmlType::Button);
    }

    /// 测试 HtmlType 的克隆
    #[test]
    fn test_html_type_clone() {
        let original = HtmlType::Submit;
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    /// 测试 HtmlType 的所有变体
    #[test]
    fn test_html_type_variants() {
        let html_types = [HtmlType::Button, HtmlType::Submit, HtmlType::Reset];

        for html_type in html_types.iter() {
            // 测试克隆
            let cloned = html_type.clone();
            assert_eq!(*html_type, cloned);
        }
    }

    /// 测试 LoadingConfig 的默认值
    #[test]
    fn test_loading_config_default() {
        let default_config = LoadingConfig::default();
        assert_eq!(default_config, LoadingConfig::DelayedLoading(0));
    }

    /// 测试 LoadingConfig 的克隆
    #[test]
    fn test_loading_config_clone() {
        let original = LoadingConfig::DelayedLoading(500);
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    /// 测试 LoadingConfig 的相等性
    #[test]
    fn test_loading_config_equality() {
        let config1 = LoadingConfig::DelayedLoading(300);
        let config2 = LoadingConfig::DelayedLoading(300);
        let config3 = LoadingConfig::DelayedLoading(500);

        assert_eq!(config1, config2);
        assert_ne!(config1, config3);
    }

    /// 测试 LoadingConfig 的不同延迟值
    #[test]
    fn test_loading_config_different_delays() {
        let delays = [0, 100, 300, 500, 1000];

        for delay in delays.iter() {
            let config = LoadingConfig::DelayedLoading(*delay);

            assert_eq!(config, LoadingConfig::DelayedLoading(*delay));

            // 测试克隆
            let cloned = config.clone();
            assert_eq!(config, cloned);
        }
    }

    /// 测试枚举的字符串表示（如果实现了 Display trait）
    #[test]
    fn test_enum_debug_representation() {
        // 测试 Debug trait 的实现
        let button_type = ButtonType::Primary;
        let debug_str = format!("{:?}", button_type);
        assert!(debug_str.contains("Primary"));

        let button_size = ButtonSize::Large;
        let debug_str = format!("{:?}", button_size);
        assert!(debug_str.contains("Large"));

        let button_shape = ButtonShape::Circle;
        let debug_str = format!("{:?}", button_shape);
        assert!(debug_str.contains("Circle"));
    }

    /// 测试枚举的匹配模式
    #[test]
    fn test_enum_pattern_matching() {
        // 测试 ButtonType 的模式匹配
        let button_type = ButtonType::Primary;
        match button_type {
            ButtonType::Primary => assert!(true),
            _ => assert!(false, "Pattern matching failed for ButtonType::Primary"),
        }

        // 测试 ButtonSize 的模式匹配
        let button_size = ButtonSize::Small;
        match button_size {
            ButtonSize::Small => assert!(true),
            _ => assert!(false, "Pattern matching failed for ButtonSize::Small"),
        }

        // 测试 ButtonShape 的模式匹配
        let button_shape = ButtonShape::Round;
        match button_shape {
            ButtonShape::Round => assert!(true),
            _ => assert!(false, "Pattern matching failed for ButtonShape::Round"),
        }
    }

    /// 测试类型的内存大小（确保枚举大小合理）
    #[test]
    fn test_enum_sizes() {
        use std::mem::size_of;

        // 确保枚举大小合理（通常应该是1个字节对于简单枚举）
        assert!(size_of::<ButtonType>() <= 8);
        assert!(size_of::<ButtonSize>() <= 8);
        assert!(size_of::<ButtonShape>() <= 8);
        assert!(size_of::<ButtonVariant>() <= 8);
        assert!(size_of::<ButtonColor>() <= 8);
        assert!(size_of::<HtmlType>() <= 8);

        // LoadingConfig 包含一个 u32，所以应该是 4 字节
        assert_eq!(size_of::<LoadingConfig>(), 4);
    }
}
