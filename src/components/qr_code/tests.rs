#[cfg(test)]
mod tests {
    use crate::components::qr_code::types::*;
    use dioxus::prelude::*;

    // 测试 QRCodeType 枚举
    #[test]
    fn test_qr_code_type_default() {
        assert_eq!(QRCodeType::default(), QRCodeType::Canvas);
    }

    #[test]
    fn test_qr_code_type_display() {
        assert_eq!(format!("{}", QRCodeType::Canvas), "canvas");
        assert_eq!(format!("{}", QRCodeType::Svg), "svg");
    }

    #[test]
    fn test_qr_code_type_from_str() {
        assert_eq!(QRCodeType::from_str("canvas"), QRCodeType::Canvas);
        assert_eq!(QRCodeType::from_str("svg"), QRCodeType::Svg);
        assert_eq!(QRCodeType::from_str("unknown"), QRCodeType::Canvas);
    }

    #[test]
    fn test_qr_code_type_to_string() {
        assert_eq!(QRCodeType::Canvas.to_string(), "canvas");
        assert_eq!(QRCodeType::Svg.to_string(), "svg");
    }

    #[test]
    fn test_qr_code_type_is_canvas() {
        assert!(QRCodeType::Canvas.is_canvas());
        assert!(!QRCodeType::Svg.is_canvas());
    }

    #[test]
    fn test_qr_code_type_is_svg() {
        assert!(!QRCodeType::Canvas.is_svg());
        assert!(QRCodeType::Svg.is_svg());
    }

    // 测试 QRCodeStatus 枚举
    #[test]
    fn test_qr_code_status_default() {
        assert_eq!(QRCodeStatus::default(), QRCodeStatus::Active);
    }

    #[test]
    fn test_qr_code_status_display() {
        assert_eq!(format!("{}", QRCodeStatus::Active), "active");
        assert_eq!(format!("{}", QRCodeStatus::Expired), "expired");
        assert_eq!(format!("{}", QRCodeStatus::Loading), "loading");
        assert_eq!(format!("{}", QRCodeStatus::Scanned), "scanned");
    }

    #[test]
    fn test_qr_code_status_from_str() {
        assert_eq!(QRCodeStatus::from_str("active"), QRCodeStatus::Active);
        assert_eq!(QRCodeStatus::from_str("expired"), QRCodeStatus::Expired);
        assert_eq!(QRCodeStatus::from_str("loading"), QRCodeStatus::Loading);
        assert_eq!(QRCodeStatus::from_str("scanned"), QRCodeStatus::Scanned);
        assert_eq!(QRCodeStatus::from_str("unknown"), QRCodeStatus::Active);
    }

    #[test]
    fn test_qr_code_status_to_string() {
        assert_eq!(QRCodeStatus::Active.to_string(), "active");
        assert_eq!(QRCodeStatus::Expired.to_string(), "expired");
        assert_eq!(QRCodeStatus::Loading.to_string(), "loading");
        assert_eq!(QRCodeStatus::Scanned.to_string(), "scanned");
    }

    #[test]
    fn test_qr_code_status_is_active() {
        assert!(QRCodeStatus::Active.is_active());
        assert!(!QRCodeStatus::Expired.is_active());
        assert!(!QRCodeStatus::Loading.is_active());
        assert!(!QRCodeStatus::Scanned.is_active());
    }

    #[test]
    fn test_qr_code_status_is_expired() {
        assert!(!QRCodeStatus::Active.is_expired());
        assert!(QRCodeStatus::Expired.is_expired());
        assert!(!QRCodeStatus::Loading.is_expired());
        assert!(!QRCodeStatus::Scanned.is_expired());
    }

    #[test]
    fn test_qr_code_status_is_loading() {
        assert!(!QRCodeStatus::Active.is_loading());
        assert!(!QRCodeStatus::Expired.is_loading());
        assert!(QRCodeStatus::Loading.is_loading());
        assert!(!QRCodeStatus::Scanned.is_loading());
    }

    #[test]
    fn test_qr_code_status_is_scanned() {
        assert!(!QRCodeStatus::Active.is_scanned());
        assert!(!QRCodeStatus::Expired.is_scanned());
        assert!(!QRCodeStatus::Loading.is_scanned());
        assert!(QRCodeStatus::Scanned.is_scanned());
    }

    #[test]
    fn test_qr_code_status_can_refresh() {
        assert!(!QRCodeStatus::Active.can_refresh());
        assert!(QRCodeStatus::Expired.can_refresh());
        assert!(!QRCodeStatus::Loading.can_refresh());
        assert!(!QRCodeStatus::Scanned.can_refresh());
    }

    #[test]
    fn test_qr_code_status_needs_loading() {
        assert!(!QRCodeStatus::Active.needs_loading());
        assert!(!QRCodeStatus::Expired.needs_loading());
        assert!(QRCodeStatus::Loading.needs_loading());
        assert!(!QRCodeStatus::Scanned.needs_loading());
    }

    // 测试 QRCodeSize 枚举
    #[test]
    fn test_qr_code_size_default() {
        assert_eq!(QRCodeSize::default(), QRCodeSize::Medium);
    }

    #[test]
    fn test_qr_code_size_display() {
        assert_eq!(format!("{}", QRCodeSize::Small), "small");
        assert_eq!(format!("{}", QRCodeSize::Medium), "medium");
        assert_eq!(format!("{}", QRCodeSize::Large), "large");
        assert_eq!(format!("{}", QRCodeSize::Custom(200)), "200");
    }

    #[test]
    fn test_qr_code_size_to_pixels() {
        assert_eq!(QRCodeSize::Small.to_pixels(), 120);
        assert_eq!(QRCodeSize::Medium.to_pixels(), 160);
        assert_eq!(QRCodeSize::Large.to_pixels(), 200);
        assert_eq!(QRCodeSize::Custom(300).to_pixels(), 300);
    }

    #[test]
    fn test_qr_code_size_from_pixels() {
        assert_eq!(QRCodeSize::from_pixels(120), QRCodeSize::Small);
        assert_eq!(QRCodeSize::from_pixels(160), QRCodeSize::Medium);
        assert_eq!(QRCodeSize::from_pixels(200), QRCodeSize::Large);
        assert_eq!(QRCodeSize::from_pixels(250), QRCodeSize::Custom(250));
    }

    #[test]
    fn test_qr_code_size_to_css_value() {
        assert_eq!(QRCodeSize::Small.to_css_value(), "120px");
        assert_eq!(QRCodeSize::Medium.to_css_value(), "160px");
        assert_eq!(QRCodeSize::Large.to_css_value(), "200px");
        assert_eq!(QRCodeSize::Custom(180).to_css_value(), "180px");
    }

    // 测试 QRCodeErrorLevel 枚举
    #[test]
    fn test_qr_code_error_level_default() {
        assert_eq!(QRCodeErrorLevel::default(), QRCodeErrorLevel::M);
    }

    #[test]
    fn test_qr_code_error_level_display() {
        assert_eq!(format!("{}", QRCodeErrorLevel::L), "L");
        assert_eq!(format!("{}", QRCodeErrorLevel::M), "M");
        assert_eq!(format!("{}", QRCodeErrorLevel::Q), "Q");
        assert_eq!(format!("{}", QRCodeErrorLevel::H), "H");
    }

    #[test]
    fn test_qr_code_error_level_from_str() {
        assert_eq!(QRCodeErrorLevel::from_str("L"), QRCodeErrorLevel::L);
        assert_eq!(QRCodeErrorLevel::from_str("M"), QRCodeErrorLevel::M);
        assert_eq!(QRCodeErrorLevel::from_str("Q"), QRCodeErrorLevel::Q);
        assert_eq!(QRCodeErrorLevel::from_str("H"), QRCodeErrorLevel::H);
        assert_eq!(QRCodeErrorLevel::from_str("unknown"), QRCodeErrorLevel::M);
    }

    #[test]
    fn test_qr_code_error_level_to_percentage() {
        assert_eq!(QRCodeErrorLevel::L.to_percentage(), "7%");
        assert_eq!(QRCodeErrorLevel::M.to_percentage(), "15%");
        assert_eq!(QRCodeErrorLevel::Q.to_percentage(), "25%");
        assert_eq!(QRCodeErrorLevel::H.to_percentage(), "30%");
    }

    #[test]
    fn test_qr_code_error_level_to_numeric_value() {
        assert_eq!(QRCodeErrorLevel::L.to_numeric_value(), 1);
        assert_eq!(QRCodeErrorLevel::M.to_numeric_value(), 0);
        assert_eq!(QRCodeErrorLevel::Q.to_numeric_value(), 3);
        assert_eq!(QRCodeErrorLevel::H.to_numeric_value(), 2);
    }

    // 测试 QRCodeConfig 结构体
    #[test]
    fn test_qr_code_config_default() {
        let config = QRCodeConfig::default();
        assert_eq!(config.value, "");
        assert_eq!(config.qr_type, QRCodeType::Canvas);
        assert_eq!(config.size, QRCodeSize::Medium);
        assert_eq!(config.status, QRCodeStatus::Active);
        assert_eq!(config.error_level, QRCodeErrorLevel::M);
        assert_eq!(config.color, "#000000");
        assert_eq!(config.bg_color, "#ffffff");
        assert!(config.icon.is_none());
        assert_eq!(config.icon_size, 40);
        assert_eq!(config.bordered, true);
        assert_eq!(config.border_color, "#d9d9d9");
    }

    #[test]
    fn test_qr_code_config_new() {
        let config = QRCodeConfig::new(
            "https://example.com".to_string(),
            QRCodeType::Svg,
            QRCodeSize::Large,
            QRCodeStatus::Active,
            QRCodeErrorLevel::H,
            "#1890ff".to_string(),
            "#f0f0f0".to_string(),
            Some("icon.png".to_string()),
            50,
            false,
            "#ff4d4f".to_string(),
        );

        assert_eq!(config.value, "https://example.com");
        assert_eq!(config.qr_type, QRCodeType::Svg);
        assert_eq!(config.size, QRCodeSize::Large);
        assert_eq!(config.status, QRCodeStatus::Active);
        assert_eq!(config.error_level, QRCodeErrorLevel::H);
        assert_eq!(config.color, "#1890ff");
        assert_eq!(config.bg_color, "#f0f0f0");
        assert_eq!(config.icon, Some("icon.png".to_string()));
        assert_eq!(config.icon_size, 50);
        assert_eq!(config.bordered, false);
        assert_eq!(config.border_color, "#ff4d4f");
    }

    #[test]
    fn test_qr_code_config_with_icon() {
        let mut config = QRCodeConfig::default();
        config = config.with_icon("logo.svg".to_string(), 60);

        assert_eq!(config.icon, Some("logo.svg".to_string()));
        assert_eq!(config.icon_size, 60);
    }

    #[test]
    fn test_qr_code_config_with_border() {
        let mut config = QRCodeConfig::default();
        config = config.with_border(true, "#52c41a".to_string());

        assert_eq!(config.bordered, true);
        assert_eq!(config.border_color, "#52c41a");
    }

    #[test]
    fn test_qr_code_config_with_colors() {
        let mut config = QRCodeConfig::default();
        config = config.with_colors("#722ed1".to_string(), "#f6ffed".to_string());

        assert_eq!(config.color, "#722ed1");
        assert_eq!(config.bg_color, "#f6ffed");
    }

    #[test]
    fn test_qr_code_config_is_valid() {
        let valid_config = QRCodeConfig {
            value: "test".to_string(),
            ..Default::default()
        };
        assert!(valid_config.is_valid());

        let invalid_config = QRCodeConfig {
            value: "".to_string(),
            ..Default::default()
        };
        assert!(!invalid_config.is_valid());
    }

    #[test]
    fn test_qr_code_config_needs_refresh() {
        let active_config = QRCodeConfig {
            status: QRCodeStatus::Active,
            ..Default::default()
        };
        assert!(!active_config.needs_refresh());

        let expired_config = QRCodeConfig {
            status: QRCodeStatus::Expired,
            ..Default::default()
        };
        assert!(expired_config.needs_refresh());
    }

    #[test]
    fn test_qr_code_config_can_scan() {
        let active_config = QRCodeConfig {
            status: QRCodeStatus::Active,
            ..Default::default()
        };
        assert!(active_config.can_scan());

        let loading_config = QRCodeConfig {
            status: QRCodeStatus::Loading,
            ..Default::default()
        };
        assert!(!loading_config.can_scan());
    }

    // 测试工具函数
    #[test]
    fn test_validate_qr_code_props() {
        let valid_props = QRCodeProps {
            value: "https://example.com".to_string(),
            qr_type: QRCodeType::Canvas,
            r#type: QRCodeType::Canvas,
            size: QRCodeSize::Medium,
            status: QRCodeStatus::Active,
            error_level: QRCodeErrorLevel::M,
            color: Some("#000000".to_string()),
            bg_color: Some("#ffffff".to_string()),
            icon: None,
            icon_size: Some(40),
            bordered: Some(true),
            border_color: Some("#d9d9d9".to_string()),
            status_render: None,
            class: None,
            style: None,
            on_refresh: None,
            children: None,
        };

        assert!(validate_qr_code_props(&valid_props).is_ok());
    }

    #[test]
    fn test_validate_qr_code_props_empty_value() {
        let invalid_props = QRCodeProps {
            value: "".to_string(),
            qr_type: QRCodeType::Canvas,
            r#type: QRCodeType::Canvas,
            size: QRCodeSize::Medium,
            status: QRCodeStatus::Active,
            error_level: QRCodeErrorLevel::M,
            color: None,
            bg_color: None,
            icon: None,
            icon_size: None,
            bordered: None,
            border_color: None,
            status_render: None,
            class: None,
            style: None,
            on_refresh: None,
            children: None,
        };

        assert!(validate_qr_code_props(&invalid_props).is_err());
    }

    #[test]
    fn test_validate_qr_code_props_invalid_color() {
        let invalid_props = QRCodeProps {
            value: "test".to_string(),
            qr_type: QRCodeType::Canvas,
            r#type: QRCodeType::Canvas,
            size: QRCodeSize::Medium,
            status: QRCodeStatus::Active,
            error_level: QRCodeErrorLevel::M,
            color: Some("invalid-color".to_string()),
            bg_color: None,
            icon: None,
            icon_size: None,
            bordered: None,
            border_color: None,
            status_render: None,
            class: None,
            style: None,
            on_refresh: None,
            children: None,
        };

        assert!(validate_qr_code_props(&invalid_props).is_err());
    }

    #[test]
    fn test_create_qr_code_config() {
        let props = QRCodeProps {
            value: "https://ant.design".to_string(),
            qr_type: QRCodeType::Svg,
            r#type: QRCodeType::Svg,
            size: QRCodeSize::Large,
            status: QRCodeStatus::Active,
            error_level: QRCodeErrorLevel::H,
            color: Some("#1890ff".to_string()),
            bg_color: Some("#f0f2f5".to_string()),
            icon: Some("logo.png".to_string()),
            icon_size: Some(50),
            bordered: Some(false),
            border_color: Some("#52c41a".to_string()),
            status_render: None,
            class: None,
            style: None,
            on_refresh: None,
            children: None,
        };

        let config = create_qr_code_config(&props);
        assert_eq!(config.value, "https://ant.design");
        assert_eq!(config.qr_type, QRCodeType::Svg);
        assert_eq!(config.size, QRCodeSize::Large);
        assert_eq!(config.status, QRCodeStatus::Active);
        assert_eq!(config.error_level, QRCodeErrorLevel::H);
        assert_eq!(config.color, "#1890ff");
        assert_eq!(config.bg_color, "#f0f2f5");
        assert_eq!(config.icon, Some("logo.png".to_string()));
        assert_eq!(config.icon_size, 50);
        assert_eq!(config.bordered, false);
        assert_eq!(config.border_color, "#52c41a");
    }

    #[test]
    fn test_generate_qr_code_data() {
        let config = QRCodeConfig {
            value: "Hello World".to_string(),
            error_level: QRCodeErrorLevel::M,
            ..Default::default()
        };

        let qr_data = generate_qr_code_data(&config);
        assert!(qr_data.is_ok());

        let data = qr_data.unwrap();
        assert!(!data.is_empty());
    }

    #[test]
    fn test_generate_qr_code_data_invalid() {
        let config = QRCodeConfig {
            value: "".to_string(),
            ..Default::default()
        };

        let qr_data = generate_qr_code_data(&config);
        assert!(qr_data.is_err());
    }

    #[test]
    fn test_generate_qr_code_svg() {
        let config = QRCodeConfig {
            value: "SVG Test".to_string(),
            qr_type: QRCodeType::Svg,
            size: QRCodeSize::Medium,
            color: "#000000".to_string(),
            bg_color: "#ffffff".to_string(),
            ..Default::default()
        };

        let svg_result = generate_qr_code_svg(&config);
        assert!(svg_result.is_ok());

        let svg_content = svg_result.unwrap();
        assert!(svg_content.contains("<svg"));
        assert!(svg_content.contains("</svg>"));
        assert!(svg_content.contains("160"));
    }

    #[test]
    fn test_generate_qr_code_canvas_data() {
        let config = QRCodeConfig {
            value: "Canvas Test".to_string(),
            qr_type: QRCodeType::Canvas,
            size: QRCodeSize::Small,
            ..Default::default()
        };

        let canvas_data = generate_qr_code_canvas_data(&config);
        assert!(canvas_data.is_ok());

        let data = canvas_data.unwrap();
        assert!(data.starts_with("data:image/png;base64,"));
    }

    #[test]
    fn test_get_qr_code_container_class() {
        let config = QRCodeConfig {
            status: QRCodeStatus::Active,
            bordered: true,
            ..Default::default()
        };

        let class_name = get_qr_code_container_class(&config);
        assert!(class_name.contains("ant-qrcode"));
        assert!(class_name.contains("ant-qrcode-bordered"));
    }

    #[test]
    fn test_get_qr_code_container_class_expired() {
        let config = QRCodeConfig {
            status: QRCodeStatus::Expired,
            bordered: false,
            ..Default::default()
        };

        let class_name = get_qr_code_container_class(&config);
        assert!(class_name.contains("ant-qrcode"));
        assert!(class_name.contains("ant-qrcode-expired"));
        assert!(!class_name.contains("ant-qrcode-bordered"));
    }

    #[test]
    fn test_get_qr_code_container_styles() {
        let config = QRCodeConfig {
            size: QRCodeSize::Large,
            border_color: "#1890ff".to_string(),
            bordered: true,
            ..Default::default()
        };

        let styles = get_qr_code_container_styles(&config);
        assert!(styles.contains("width: 200px"));
        assert!(styles.contains("height: 200px"));
        assert!(styles.contains("border-color: #1890ff"));
    }

    // 注意：QRCode组件的刷新事件处理是在组件内部实现的，
    // 不需要单独的handle_qr_code_refresh函数测试

    #[test]
    fn test_is_valid_color_format() {
        assert!(is_valid_color_format("#000000"));
        assert!(is_valid_color_format("#fff"));
        assert!(is_valid_color_format("rgb(255, 255, 255)"));
        assert!(is_valid_color_format("rgba(0, 0, 0, 0.5)"));
        assert!(is_valid_color_format("red"));
        assert!(is_valid_color_format("transparent"));

        assert!(!is_valid_color_format("invalid"));
        assert!(!is_valid_color_format("#gggggg"));
        assert!(!is_valid_color_format(""));
    }

    // 测试样式生成
    #[test]
    fn test_qr_code_styles_base() {
        let base_styles = QRCodeStyles::base();
        assert!(base_styles.contains(".ant-qrcode"));
        assert!(base_styles.contains("display: inline-block"));
        assert!(base_styles.contains("position: relative"));
    }

    #[test]
    fn test_qr_code_styles_size_styles() {
        let small_styles = QRCodeStyles::size_styles(&QRCodeSize::Small);
        assert!(small_styles.contains("width: 120px"));
        assert!(small_styles.contains("height: 120px"));

        let medium_styles = QRCodeStyles::size_styles(&QRCodeSize::Medium);
        assert!(medium_styles.contains("width: 160px"));
        assert!(medium_styles.contains("height: 160px"));

        let large_styles = QRCodeStyles::size_styles(&QRCodeSize::Large);
        assert!(large_styles.contains("width: 200px"));
        assert!(large_styles.contains("height: 200px"));

        let custom_styles = QRCodeStyles::size_styles(&QRCodeSize::Custom(250));
        assert!(custom_styles.contains("width: 250px"));
        assert!(custom_styles.contains("height: 250px"));
    }

    #[test]
    fn test_qr_code_styles_status_styles() {
        let active_styles = QRCodeStyles::status_styles(&QRCodeStatus::Active);
        assert!(active_styles.contains("ant-qrcode-active"));

        let expired_styles = QRCodeStyles::status_styles(&QRCodeStatus::Expired);
        assert!(expired_styles.contains("ant-qrcode-expired"));
        assert!(expired_styles.contains("opacity: 0.3"));

        let loading_styles = QRCodeStyles::status_styles(&QRCodeStatus::Loading);
        assert!(loading_styles.contains("ant-qrcode-loading"));

        let scanned_styles = QRCodeStyles::status_styles(&QRCodeStatus::Scanned);
        assert!(scanned_styles.contains("ant-qrcode-scanned"));
    }

    #[test]
    fn test_qr_code_styles_border_styles() {
        let bordered_styles = QRCodeStyles::border_styles(true, "#d9d9d9");
        assert!(bordered_styles.contains("border: 1px solid #d9d9d9"));

        let no_border_styles = QRCodeStyles::border_styles(false, "#000000");
        assert!(no_border_styles.is_empty());
    }

    #[test]
    fn test_qr_code_styles_icon_styles() {
        let icon_styles = QRCodeStyles::icon_styles(40);
        assert!(icon_styles.contains("width: 40px"));
        assert!(icon_styles.contains("height: 40px"));
        assert!(icon_styles.contains("position: absolute"));
        assert!(icon_styles.contains("top: 50%"));
        assert!(icon_styles.contains("left: 50%"));
        assert!(icon_styles.contains("transform: translate(-50%, -50%)"));
    }

    #[test]
    fn test_qr_code_styles_refresh_button_styles() {
        let refresh_styles = QRCodeStyles::refresh_button_styles();
        assert!(refresh_styles.contains(".ant-qrcode-refresh"));
        assert!(refresh_styles.contains("position: absolute"));
        assert!(refresh_styles.contains("cursor: pointer"));
    }

    // 测试组件渲染
    #[tokio::test]
    async fn test_qr_code_component_basic_render() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                QRCode {
                    value: "https://ant.design",
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 基本渲染测试通过
    }

    #[tokio::test]
    async fn test_qr_code_component_svg_type() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                QRCode {
                    value: "SVG QR Code",
                    qr_type: QRCodeType::Svg,
                    size: QRCodeSize::Large,
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // SVG类型渲染测试通过
    }

    #[tokio::test]
    async fn test_qr_code_component_canvas_type() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                QRCode {
                    value: "Canvas QR Code",
                    qr_type: QRCodeType::Canvas,
                    size: QRCodeSize::Medium,
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // Canvas类型渲染测试通过
    }

    #[tokio::test]
    async fn test_qr_code_component_with_icon() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                QRCode {
                    value: "QR Code with Icon",
                    icon: "logo.png",
                    icon_size: 50,
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 带图标渲染测试通过
    }

    #[tokio::test]
    async fn test_qr_code_component_different_sizes() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                div {
                    QRCode {
                        value: "Small QR",
                        size: QRCodeSize::Small,
                    }
                    QRCode {
                        value: "Medium QR",
                        size: QRCodeSize::Medium,
                    }
                    QRCode {
                        value: "Large QR",
                        size: QRCodeSize::Large,
                    }
                    QRCode {
                        value: "Custom QR",
                        size: QRCodeSize::Custom(300),
                    }
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 不同尺寸渲染测试通过
    }

    #[tokio::test]
    async fn test_qr_code_component_different_statuses() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                div {
                    QRCode {
                        value: "Active QR",
                        status: QRCodeStatus::Active,
                    }
                    QRCode {
                        value: "Expired QR",
                        status: QRCodeStatus::Expired,
                    }
                    QRCode {
                        value: "Loading QR",
                        status: QRCodeStatus::Loading,
                    }
                    QRCode {
                        value: "Scanned QR",
                        status: QRCodeStatus::Scanned,
                    }
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 不同状态渲染测试通过
    }

    #[tokio::test]
    async fn test_qr_code_component_with_custom_colors() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                QRCode {
                    value: "Colored QR Code",
                    color: "#1890ff",
                    bg_color: "#f0f2f5",
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 自定义颜色渲染测试通过
    }

    #[tokio::test]
    async fn test_qr_code_component_with_border() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                div {
                    QRCode {
                        value: "Bordered QR",
                        bordered: true,
                        border_color: "#52c41a",
                    }
                    QRCode {
                        value: "No Border QR",
                        bordered: false,
                    }
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 边框渲染测试通过
    }

    #[tokio::test]
    async fn test_qr_code_component_different_error_levels() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                div {
                    QRCode {
                        value: "Error Level L",
                        error_level: QRCodeErrorLevel::L,
                    }
                    QRCode {
                        value: "Error Level M",
                        error_level: QRCodeErrorLevel::M,
                    }
                    QRCode {
                        value: "Error Level Q",
                        error_level: QRCodeErrorLevel::Q,
                    }
                    QRCode {
                        value: "Error Level H",
                        error_level: QRCodeErrorLevel::H,
                    }
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 不同纠错级别渲染测试通过
    }

    #[tokio::test]
    async fn test_qr_code_component_with_refresh_handler() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                QRCode {
                    value: "Refreshable QR",
                    status: QRCodeStatus::Expired,
                    on_refresh: move |_| {
                        // 刷新处理逻辑
                    },
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 刷新事件渲染测试通过
    }

    #[tokio::test]
    async fn test_qr_code_component_with_custom_styles() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                QRCode {
                    value: "Styled QR Code",
                    class: "custom-qrcode",
                    style: "margin: 20px; box-shadow: 0 2px 8px rgba(0,0,0,0.1);",
                }
            }
        });

        let _ = dom.rebuild_to_vec();
        // 自定义样式渲染测试通过
    }
}
