//! Modal 对话框组件样式

/// Modal 样式生成器
pub struct ModalStyleBuilder {
    /// 是否垂直居中
    centered: bool,
    /// z-index 值
    z_index: i32,
    /// 宽度
    width: Option<String>,
}

impl Default for ModalStyleBuilder {
    fn default() -> Self {
        Self {
            centered: true,
            z_index: 1000,
            width: None,
        }
    }
}

impl ModalStyleBuilder {
    /// 创建新的样式生成器
    pub fn new() -> Self {
        Default::default()
    }

    /// 设置是否垂直居中
    pub fn centered(mut self, centered: bool) -> Self {
        self.centered = centered;
        self
    }

    /// 设置z-index值
    pub fn z_index(mut self, z_index: i32) -> Self {
        self.z_index = z_index;
        self
    }

    /// 设置宽度
    pub fn width(mut self, width: Option<String>) -> Self {
        self.width = width;
        self
    }

    /// 生成根元素样式
    pub fn root_style(&self) -> String {
        format!(
            ".ant-modal-root {{
                position: fixed;
                top: 0;
                left: 0;
                right: 0;
                bottom: 0;
                z-index: {};
            }}",
            self.z_index
        )
    }

    /// 生成遮罩层样式
    pub fn mask_style(&self) -> String {
        format!(
            ".ant-modal-mask {{
                position: fixed;
                top: 0;
                left: 0;
                right: 0;
                bottom: 0;
                background-color: rgba(0, 0, 0, 0.45);
                z-index: {};
                animation: ant-modal-mask-fade-in 0.2s ease-out;
            }}",
            self.z_index
        )
    }

    /// 生成包装器样式
    pub fn wrap_style(&self) -> String {
        let centered_class = if self.centered {
            ".ant-modal-wrap.ant-modal-centered {
                align-items: center;
                padding: 0;
            }"
        } else {
            ""
        };

        format!(
            ".ant-modal-wrap {{
                position: fixed;
                top: 0;
                left: 0;
                right: 0;
                bottom: 0;
                overflow: auto;
                outline: 0;
                z-index: {};
                display: flex;
                align-items: flex-start;
                justify-content: center;
                padding: 100px 0 40px;
            }}
            {}",
            self.z_index, centered_class
        )
    }

    /// 生成对话框样式
    pub fn modal_style(&self) -> String {
        let width_style = self
            .width
            .as_ref()
            .map_or("width: auto;", |w| format!("width: {};", w).as_str());

        format!(
            ".ant-modal {{
                position: relative;
                {}
                max-width: calc(100vw - 32px);
                margin: 0 auto;
                animation: ant-modal-zoom-in 0.2s ease-out;
            }}",
            width_style
        )
    }

    /// 生成内容样式
    pub fn content_style(&self) -> String {
        format!(
            ".ant-modal-content {{
                position: relative;
                background-color: #fff;
                background-clip: padding-box;
                border: 0;
                border-radius: 6px;
                box-shadow: 0 6px 16px 0 rgba(0, 0, 0, 0.08), 0 3px 6px -4px rgba(0, 0, 0, 0.12), 0 9px 28px 8px rgba(0, 0, 0, 0.05);
                pointer-events: auto;
            }}"
        )
    }

    /// 生成关闭按钮样式
    pub fn close_style(&self) -> String {
        ".ant-modal-close {
            position: absolute;
            top: 0;
            right: 0;
            z-index: 10;
            padding: 0;
            color: rgba(0, 0, 0, 0.45);
            font-weight: 700;
            line-height: 1;
            text-decoration: none;
            background: transparent;
            border: 0;
            outline: 0;
            cursor: pointer;
            transition: color 0.2s;
            width: 56px;
            height: 56px;
            display: flex;
            align-items: center;
            justify-content: center;
        }

        .ant-modal-close:hover {
            color: rgba(0, 0, 0, 0.75);
        }

        .ant-modal-close-x {
            display: flex;
            align-items: center;
            justify-content: center;
            width: 22px;
            height: 22px;
            font-size: 16px;
            font-style: normal;
            line-height: 22px;
            text-align: center;
            text-transform: none;
            text-rendering: auto;
        }"
        .to_string()
    }

    /// 生成标题样式
    pub fn header_style(&self) -> String {
        ".ant-modal-header {
            padding: 16px 24px;
            color: rgba(0, 0, 0, 0.88);
            background: #fff;
            border-bottom: 1px solid rgba(5, 5, 5, 0.06);
            border-radius: 6px 6px 0 0;
        }

        .ant-modal-title {
            margin: 0;
            color: rgba(0, 0, 0, 0.88);
            font-weight: 600;
            font-size: 16px;
            line-height: 1.5;
            word-wrap: break-word;
        }"
        .to_string()
    }

    /// 生成内容区样式
    pub fn body_style(&self) -> String {
        ".ant-modal-body {
            padding: 20px 24px;
            font-size: 14px;
            line-height: 1.5715;
            word-wrap: break-word;
            color: rgba(0, 0, 0, 0.88);
        }"
        .to_string()
    }

    /// 生成页脚样式
    pub fn footer_style(&self) -> String {
        ".ant-modal-footer {
            padding: 10px 16px;
            text-align: right;
            background: transparent;
            border-top: 1px solid rgba(5, 5, 5, 0.06);
            border-radius: 0 0 6px 6px;
            display: flex;
            justify-content: flex-end;
            gap: 8px;
        }"
        .to_string()
    }

    /// 生成按钮样式
    pub fn button_style(&self) -> String {
        ".ant-btn {
            position: relative;
            display: inline-block;
            font-weight: 400;
            white-space: nowrap;
            text-align: center;
            background-image: none;
            border: 1px solid transparent;
            box-shadow: 0 2px 0 rgba(0, 0, 0, 0.02);
            cursor: pointer;
            transition: all 0.2s cubic-bezier(0.645, 0.045, 0.355, 1);
            user-select: none;
            touch-action: manipulation;
            height: 32px;
            padding: 4px 15px;
            font-size: 14px;
            border-radius: 6px;
            outline: 0;
            text-decoration: none;
        }

        .ant-btn-default {
            color: rgba(0, 0, 0, 0.88);
            border-color: #d9d9d9;
            background: #fff;
        }

        .ant-btn-default:hover {
            color: #4096ff;
            border-color: #4096ff;
        }

        .ant-btn-primary {
            color: #fff;
            border-color: #1677ff;
            background: #1677ff;
        }

        .ant-btn-primary:hover {
            color: #fff;
            border-color: #4096ff;
            background: #4096ff;
        }

        .ant-btn-loading {
            position: relative;
            pointer-events: none;
        }

        .ant-btn-loading-icon {
            display: inline-block;
            margin-right: 8px;
            animation: ant-modal-loading-rotate 1s linear infinite;
        }

        .ant-btn:disabled {
            color: rgba(0, 0, 0, 0.25);
            background: rgba(0, 0, 0, 0.04);
            border-color: #d9d9d9;
            cursor: not-allowed;
        }"
        .to_string()
    }

    /// 生成确认对话框样式
    pub fn confirm_style(&self) -> String {
        ".ant-modal-confirm-body {
            display: flex;
            align-items: flex-start;
            gap: 12px;
        }

        .ant-modal-confirm-icon {
            flex-shrink: 0;
            margin-top: 4px;
            font-size: 22px;
            line-height: 1;
            color: #faad14;
        }

        .ant-modal-confirm-content {
            flex: 1;
            font-size: 14px;
            line-height: 1.5715;
            color: rgba(0, 0, 0, 0.88);
        }"
        .to_string()
    }

    /// 生成动画样式
    pub fn animation_style(&self) -> String {
        "@keyframes ant-modal-mask-fade-in {
            0% {
                opacity: 0;
            }

            100% {
                opacity: 1;
            }
        }

        @keyframes ant-modal-zoom-in {
            0% {
                opacity: 0;
                transform: scale(0.2);
            }

            100% {
                opacity: 1;
                transform: scale(1);
            }
        }

        @keyframes ant-modal-loading-rotate {
            100% {
                transform: rotate(360deg);
            }
        }"
        .to_string()
    }

    /// 生成响应式样式
    pub fn responsive_style(&self) -> String {
        "@media (max-width: 768px) {
            .ant-modal {
                max-width: calc(100vw - 16px);
                margin: 8px auto;
            }

            .ant-modal-wrap {
                padding: 8px 0;
            }

            .ant-modal-wrap.ant-modal-centered {
                align-items: center;
                padding: 0;
            }
        }"
        .to_string()
    }

    /// 生成RTL(从右到左)布局样式
    pub fn rtl_style(&self) -> String {
        ".ant-modal[dir=\"rtl\"] .ant-modal-close {
            right: auto;
            left: 0;
        }

        .ant-modal[dir=\"rtl\"] .ant-modal-footer {
            text-align: left;
        }

        .ant-modal[dir=\"rtl\"] .ant-modal-confirm-body {
            direction: rtl;
        }"
        .to_string()
    }

    /// 生成深色模式样式
    pub fn dark_mode_style(&self) -> String {
        "@media (prefers-color-scheme: dark) {
            .ant-modal-content {
                background-color: #1f1f1f;
                box-shadow: 0 6px 16px 0 rgba(0, 0, 0, 0.48), 0 3px 6px -4px rgba(0, 0, 0, 0.52), 0 9px 28px 8px rgba(0, 0, 0, 0.35);
            }

            .ant-modal-header {
                background: #1f1f1f;
                border-bottom: 1px solid #303030;
            }

            .ant-modal-title {
                color: rgba(255, 255, 255, 0.85);
            }

            .ant-modal-body {
                color: rgba(255, 255, 255, 0.85);
            }

            .ant-modal-footer {
                border-top: 1px solid #303030;
            }

            .ant-modal-close {
                color: rgba(255, 255, 255, 0.45);
            }

            .ant-modal-close:hover {
                color: rgba(255, 255, 255, 0.75);
            }

            .ant-btn-default {
                color: rgba(255, 255, 255, 0.85);
                border-color: #424242;
                background: transparent;
            }

            .ant-btn-default:hover {
                color: #177ddc;
                border-color: #177ddc;
            }
        }".to_string()
    }

    /// 生成高对比度模式样式
    pub fn high_contrast_style(&self) -> String {
        "@media (prefers-contrast: high) {
            .ant-modal-content {
                border: 1px solid #000;
            }

            .ant-btn {
                border: 1px solid #000;
            }
        }"
        .to_string()
    }

    /// 生成减弱动画模式样式
    pub fn reduced_motion_style(&self) -> String {
        "@media (prefers-reduced-motion: reduce) {
            .ant-modal {
                animation: none;
            }

            .ant-modal-mask {
                animation: none;
            }

            .ant-btn-loading-icon {
                animation: none;
            }
        }"
        .to_string()
    }

    /// 生成打印模式样式
    pub fn print_mode_style(&self) -> String {
        "@media print {
            .ant-modal-dropdown {
                display: none;
            }

            .ant-modal {
                box-shadow: none;
                border: 1px solid #eee;
            }

            .ant-modal-clear,
            .ant-modal-close {
                display: none;
            }
        }"
        .to_string()
    }

    /// 生成完整样式
    pub fn build(&self) -> String {
        format!(
            "{}
            {}
            {}
            {}
            {}
            {}
            {}
            {}
            {}
            {}
            {}
            {}
            {}
            {}
            {}
            {}
            {}",
            self.root_style(),
            self.mask_style(),
            self.wrap_style(),
            self.modal_style(),
            self.content_style(),
            self.close_style(),
            self.header_style(),
            self.body_style(),
            self.footer_style(),
            self.button_style(),
            self.confirm_style(),
            self.animation_style(),
            self.responsive_style(),
            self.rtl_style(),
            self.dark_mode_style(),
            self.high_contrast_style(),
            self.reduced_motion_style(),
            self.print_mode_style(),
        )
    }
}
