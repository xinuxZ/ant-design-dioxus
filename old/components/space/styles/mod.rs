//! Space组件的样式生成器
//!
//! 提供Space组件的样式生成功能，包括水平/垂直间距、对齐方式、换行等样式的生成。

use css_in_rust::css;

/// Space组件的样式生成器
pub struct SpaceStyleGenerator {
    /// 方向
    direction: String,
    /// 尺寸
    size: String,
    /// 对齐方式
    align: String,
    /// 是否换行
    wrap: bool,
}

impl SpaceStyleGenerator {
    /// 创建一个新的Space样式生成器
    pub fn new() -> Self {
        Self {
            direction: "horizontal".to_string(),
            size: "middle".to_string(),
            align: "start".to_string(),
            wrap: false,
        }
    }

    /// 设置方向
    pub fn with_direction(mut self, direction: &str) -> Self {
        self.direction = direction.to_string();
        self
    }

    /// 设置尺寸
    pub fn with_size(mut self, size: &str) -> Self {
        self.size = size.to_string();
        self
    }

    /// 设置对齐方式
    pub fn with_align(mut self, align: &str) -> Self {
        self.align = align.to_string();
        self
    }

    /// 设置是否换行
    pub fn with_wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    /// 生成Space组件的CSS类名
    pub fn generate(&self) -> String {
        let mut classes = vec!["ant-space".to_string()];

        // 添加方向样式
        classes.push(format!("ant-space-{}", self.direction));

        // 添加尺寸样式
        classes.push(format!("ant-space-{}", self.size));

        // 添加对齐方式样式
        classes.push(format!("ant-space-align-{}", self.align));

        // 添加换行样式
        if self.wrap {
            classes.push("ant-space-wrap".to_string());
        }

        classes.join(" ")
    }

    /// 生成Space组件的基础样式
    pub fn base_style() -> String {
        css!(
            r#"
            .ant-space {
                display: inline-flex;
                flex-wrap: nowrap;
                gap: 8px;
            }

            .ant-space-vertical {
                flex-direction: column;
            }

            .ant-space-horizontal {
                flex-direction: row;
            }

            .ant-space-align-start {
                align-items: flex-start;
            }

            .ant-space-align-end {
                align-items: flex-end;
            }

            .ant-space-align-center {
                align-items: center;
            }

            .ant-space-align-baseline {
                align-items: baseline;
            }

            .ant-space-wrap {
                flex-wrap: wrap;
            }

            .ant-space-small {
                gap: 8px;
            }

            .ant-space-middle {
                gap: 16px;
            }

            .ant-space-large {
                gap: 24px;
            }

            .ant-space-custom {
                gap: var(--ant-space-gap, 8px);
            }

            .ant-space-split {
                position: relative;
            }

            .ant-space-split::before {
                position: absolute;
                top: 50%;
                width: 1px;
                height: 100%;
                transform: translateY(-50%);
                content: "";
                background-color: rgba(5, 5, 5, 0.06);
            }

            .ant-space-vertical .ant-space-split::before {
                top: 0;
                left: 50%;
                width: 100%;
                height: 1px;
                transform: translateX(-50%);
            }
            "#
        )
    }
}
