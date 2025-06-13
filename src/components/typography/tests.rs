//! Typography 组件测试

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::{Theme, ThemeColors};
    use super::super::types::*;
    use super::super::styles::*;

    fn create_test_theme() -> Theme {
        Theme {
            colors: ThemeColors {
                primary: "#1677ff".to_string(),
                primary_hover: "#4096ff".to_string(),
                primary_active: "#0958d9".to_string(),
                success: "#52c41a".to_string(),
                success_hover: "#73d13d".to_string(),
                success_active: "#389e0d".to_string(),
                warning: "#faad14".to_string(),
                warning_hover: "#ffc53d".to_string(),
                warning_active: "#d48806".to_string(),
                error: "#ff4d4f".to_string(),
                error_hover: "#ff7875".to_string(),
                error_active: "#d9363e".to_string(),
                info: "#1677ff".to_string(),
                info_hover: "#4096ff".to_string(),
                info_active: "#0958d9".to_string(),
            },
        }
    }

    #[test]
    fn test_typography_style_generator_base_class() {
        let class = TypographyStyleGenerator::base_class();
        assert!(!class.is_empty());
        assert!(class.contains("typography"));
    }

    #[test]
    fn test_typography_style_generator_text_type_class() {
        let theme = create_test_theme();
        
        let default_class = TypographyStyleGenerator::text_type_class(&TextType::Default, &theme);
        assert!(default_class.contains("typography-default"));
        
        let success_class = TypographyStyleGenerator::text_type_class(&TextType::Success, &theme);
        assert!(success_class.contains("typography-success"));
        
        let disabled_class = TypographyStyleGenerator::text_type_class(&TextType::Disabled, &theme);
        assert!(disabled_class.contains("typography-disabled"));
    }

    #[test]
    fn test_typography_style_generator_decoration_class() {
        let class = TypographyStyleGenerator::decoration_class(
            true,  // strong
            false, // italic
            true,  // underline
            false, // delete
            false, // mark
            false, // code
            false, // keyboard
        );
        assert!(class.contains("typography-decoration"));
    }

    #[test]
    fn test_typography_style_generator_ellipsis_class() {
        // 测试单行省略
        let single_line_class = TypographyStyleGenerator::ellipsis_class(true, Some(1));
        assert!(single_line_class.contains("typography-ellipsis"));
        
        // 测试多行省略
        let multi_line_class = TypographyStyleGenerator::ellipsis_class(true, Some(3));
        assert!(multi_line_class.contains("typography-ellipsis"));
        
        // 测试无省略
        let no_ellipsis_class = TypographyStyleGenerator::ellipsis_class(false, None);
        assert!(no_ellipsis_class.is_empty());
    }

    #[test]
    fn test_typography_style_generator_action_button_class() {
        let class = TypographyStyleGenerator::action_button_class();
        assert!(class.contains("typography-action-btn"));
    }

    #[test]
    fn test_title_style_generator() {
        let theme = create_test_theme();
        
        let h1_class = TitleStyleGenerator::title_class(&HeadingLevel::H1, &theme);
        assert!(h1_class.contains("typography-title-1"));
        
        let h2_class = TitleStyleGenerator::title_class(&HeadingLevel::H2, &theme);
        assert!(h2_class.contains("typography-title-2"));
        
        let h5_class = TitleStyleGenerator::title_class(&HeadingLevel::H5, &theme);
        assert!(h5_class.contains("typography-title-5"));
    }

    #[test]
    fn test_paragraph_style_generator() {
        let class = ParagraphStyleGenerator::paragraph_class();
        assert!(class.contains("typography-paragraph"));
    }

    #[test]
    fn test_link_style_generator() {
        let theme = create_test_theme();
        
        // 测试正常链接
        let normal_class = LinkStyleGenerator::link_class(&LinkType::Default, false, false, &theme);
        assert!(normal_class.contains("typography-link"));
        
        // 测试禁用链接
        let disabled_class = LinkStyleGenerator::link_class(&LinkType::Default, true, false, &theme);
        assert!(disabled_class.contains("typography-link"));
        
        // 测试块级链接
        let block_class = LinkStyleGenerator::link_class(&LinkType::Default, false, true, &theme);
        assert!(block_class.contains("typography-link"));
        
        // 测试不同类型的链接
        let success_class = LinkStyleGenerator::link_class(&LinkType::Success, false, false, &theme);
        assert!(success_class.contains("typography-link"));
        
        let danger_class = LinkStyleGenerator::link_class(&LinkType::Danger, false, false, &theme);
        assert!(danger_class.contains("typography-link"));
    }

    #[test]
    fn test_edit_style_generator() {
        let input_class = EditStyleGenerator::edit_input_class();
        assert!(input_class.contains("typography-edit-input"));
        
        let actions_class = EditStyleGenerator::edit_actions_class();
        assert!(actions_class.contains("typography-edit-actions"));
        
        let action_btn_class = EditStyleGenerator::edit_action_button_class();
        assert!(action_btn_class.contains("typography-edit-action-btn"));
    }

    #[test]
    fn test_text_type_enum() {
        // 测试所有文本类型枚举值
        let types = vec![
            TextType::Default,
            TextType::Secondary,
            TextType::Success,
            TextType::Warning,
            TextType::Danger,
            TextType::Disabled,
        ];
        
        for text_type in types {
            let theme = create_test_theme();
            let class = TypographyStyleGenerator::text_type_class(&text_type, &theme);
            assert!(!class.is_empty());
        }
    }

    #[test]
    fn test_heading_level_enum() {
        // 测试标题级别
        assert_eq!(HeadingLevel::H1.level(), 1);
        assert_eq!(HeadingLevel::H2.level(), 2);
        assert_eq!(HeadingLevel::H3.level(), 3);
        assert_eq!(HeadingLevel::H4.level(), 4);
        assert_eq!(HeadingLevel::H5.level(), 5);
    }

    #[test]
    fn test_link_type_enum() {
        // 测试所有链接类型枚举值
        let types = vec![
            LinkType::Default,
            LinkType::Secondary,
            LinkType::Success,
            LinkType::Warning,
            LinkType::Danger,
        ];
        
        for link_type in types {
            let theme = create_test_theme();
            let class = LinkStyleGenerator::link_class(&link_type, false, false, &theme);
            assert!(!class.is_empty());
        }
    }

    #[test]
    fn test_link_target_enum() {
        // 测试链接目标
        assert_eq!(LinkTarget::Self_.as_str(), "_self");
        assert_eq!(LinkTarget::Blank.as_str(), "_blank");
        assert_eq!(LinkTarget::Parent.as_str(), "_parent");
        assert_eq!(LinkTarget::Top.as_str(), "_top");
    }

    #[test]
    fn test_copy_format_enum() {
        // 测试复制格式
        let formats = vec![
            CopyFormat::Text,
            CopyFormat::Html,
        ];
        
        for format in formats {
            // 这里只是确保枚举值可以正常创建
            match format {
                CopyFormat::Text => assert!(true),
                CopyFormat::Html => assert!(true),
            }
        }
    }

    #[test]
    fn test_edit_trigger_type_enum() {
        // 测试编辑触发类型
        let triggers = vec![
            EditTriggerType::Icon,
            EditTriggerType::Text,
            EditTriggerType::Both,
        ];
        
        for trigger in triggers {
            // 这里只是确保枚举值可以正常创建
            match trigger {
                EditTriggerType::Icon => assert!(true),
                EditTriggerType::Text => assert!(true),
                EditTriggerType::Both => assert!(true),
            }
        }
    }

    #[test]
    fn test_ellipsis_config() {
        let config = EllipsisConfig {
            rows: Some(3),
            expandable: true,
            suffix: Some("...".to_string()),
            suffix_width: Some("100px".to_string()),
            on_expand: None,
            on_ellipsis: None,
        };
        
        assert_eq!(config.rows, Some(3));
        assert_eq!(config.expandable, true);
        assert_eq!(config.suffix, Some("...".to_string()));
    }

    #[test]
    fn test_copyable_config() {
        let config = CopyableConfig {
            text: Some("复制文本".to_string()),
            format: Some(CopyFormat::Text),
            tooltip: Some("点击复制".to_string()),
            on_copy: None,
        };
        
        assert_eq!(config.text, Some("复制文本".to_string()));
        assert_eq!(config.format, Some(CopyFormat::Text));
        assert_eq!(config.tooltip, Some("点击复制".to_string()));
    }

    #[test]
    fn test_editable_config() {
        let config = EditableConfig {
            text: Some("编辑文本".to_string()),
            tooltip: Some("点击编辑".to_string()),
            trigger_type: Some(EditTriggerType::Icon),
            on_start: None,
            on_change: None,
            on_cancel: None,
            on_end: None,
            max_length: Some(100),
            auto_size: None,
            enter_icon: None,
            editing: false,
        };
        
        assert_eq!(config.text, Some("编辑文本".to_string()));
        assert_eq!(config.max_length, Some(100));
        assert_eq!(config.editing, false);
    }

    #[test]
    fn test_auto_size_config() {
        let config = AutoSizeConfig {
            min_rows: Some(2),
            max_rows: Some(6),
        };
        
        assert_eq!(config.min_rows, Some(2));
        assert_eq!(config.max_rows, Some(6));
    }
}