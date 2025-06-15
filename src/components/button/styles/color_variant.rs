use css_in_rust::css;

/// 生成颜色和变体样式
pub fn color_variant_styles() -> String {
    css!(
        r#"
        // 按钮变体样式
        .ant-btn-outlined {
            background-color: transparent;
        }

        .ant-btn-solid {
            background-color: #fff;
        }

        .ant-btn-outlined.ant-btn-primary {
            color: #1890ff;
            background: transparent;
            border-color: #1890ff;
        }

        .ant-btn-solid.ant-btn-primary {
            color: #fff;
            background: #1890ff;
            border-color: #1890ff;
        }

        // 自定义颜色支持
        .ant-btn-custom-red {
            color: #fff;
            background: #ff4d4f;
            border-color: #ff4d4f;
        }

        .ant-btn-custom-green {
            color: #fff;
            background: #52c41a;
            border-color: #52c41a;
        }

        .ant-btn-custom-blue {
            color: #fff;
            background: #1890ff;
            border-color: #1890ff;
        }

        .ant-btn-custom-yellow {
            color: #000;
            background: #fadb14;
            border-color: #fadb14;
        }

        .ant-btn-custom-purple {
            color: #fff;
            background: #722ed1;
            border-color: #722ed1;
        }

        .ant-btn-custom-orange {
            color: #fff;
            background: #fa8c16;
            border-color: #fa8c16;
        }

        .ant-btn-custom-cyan {
            color: #fff;
            background: #13c2c2;
            border-color: #13c2c2;
        }

        .ant-btn-custom-magenta {
            color: #fff;
            background: #eb2f96;
            border-color: #eb2f96;
        }

        // 变体与颜色组合
        .ant-btn-outlined.ant-btn-custom-red {
            color: #ff4d4f;
            background: transparent;
            border-color: #ff4d4f;
        }

        .ant-btn-outlined.ant-btn-custom-green {
            color: #52c41a;
            background: transparent;
            border-color: #52c41a;
        }

        .ant-btn-outlined.ant-btn-custom-blue {
            color: #1890ff;
            background: transparent;
            border-color: #1890ff;
        }

        .ant-btn-outlined.ant-btn-custom-yellow {
            color: #fadb14;
            background: transparent;
            border-color: #fadb14;
        }

        .ant-btn-outlined.ant-btn-custom-purple {
            color: #722ed1;
            background: transparent;
            border-color: #722ed1;
        }

        // 悬停效果
        .ant-btn-custom-red:hover,
        .ant-btn-custom-red:focus {
            color: #fff;
            background: #ff7875;
            border-color: #ff7875;
        }

        .ant-btn-custom-green:hover,
        .ant-btn-custom-green:focus {
            color: #fff;
            background: #73d13d;
            border-color: #73d13d;
        }

        .ant-btn-custom-blue:hover,
        .ant-btn-custom-blue:focus {
            color: #fff;
            background: #40a9ff;
            border-color: #40a9ff;
        }

        .ant-btn-outlined:hover,
        .ant-btn-outlined:focus {
            background-color: rgba(0, 0, 0, 0.018);
        }

        .ant-btn-outlined.ant-btn-primary:hover,
        .ant-btn-outlined.ant-btn-primary:focus {
            color: #40a9ff;
            background-color: transparent;
            border-color: #40a9ff;
        }

        .ant-btn-solid.ant-btn-primary:hover,
        .ant-btn-solid.ant-btn-primary:focus {
            color: #fff;
            background: #40a9ff;
            border-color: #40a9ff;
        }

        // 禁用状态
        .ant-btn-custom-red[disabled],
        .ant-btn-custom-red[disabled]:hover,
        .ant-btn-custom-red[disabled]:focus,
        .ant-btn-custom-red[disabled]:active {
            color: rgba(0, 0, 0, 0.25);
            background: #f5f5f5;
            border-color: #d9d9d9;
        }

        .ant-btn-custom-green[disabled],
        .ant-btn-custom-green[disabled]:hover,
        .ant-btn-custom-green[disabled]:focus,
        .ant-btn-custom-green[disabled]:active {
            color: rgba(0, 0, 0, 0.25);
            background: #f5f5f5;
            border-color: #d9d9d9;
        }

        .ant-btn-outlined[disabled],
        .ant-btn-outlined[disabled]:hover,
        .ant-btn-outlined[disabled]:focus,
        .ant-btn-outlined[disabled]:active {
            color: rgba(0, 0, 0, 0.25);
            background: transparent;
            border-color: #d9d9d9;
        }
    "#
    )
    .to_string()
}
