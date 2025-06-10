use crate::theme::{use_token, AliasToken};
use css_in_rust::Style;

pub struct BreadcrumbStyleGenerator {
    token: AliasToken,
}

impl BreadcrumbStyleGenerator {
    pub fn new() -> Self {
        Self { token: use_token() }
    }

    pub fn generate_style(&self) -> String {
        let style = Style::new(format!(
            r#"
            /* Breadcrumb styles */
            .ant-breadcrumb {{
                margin: 0;
                padding: 0;
                color: {};
                font-size: {}px;
                line-height: 1.5715;
                list-style: none;
                font-feature-settings: 'tnum';
                box-sizing: border-box;
            }}

            .ant-breadcrumb ol {{
                display: flex;
                flex-wrap: wrap;
                margin: 0;
                padding: 0;
                list-style: none;
            }}

            .ant-breadcrumb a {{
                color: {};
                text-decoration: none;
                transition: color 0.3s;
            }}

            .ant-breadcrumb a:hover {{
                color: {};
            }}

            .ant-breadcrumb-list {{
                display: flex;
                flex-wrap: wrap;
                align-items: center;
                margin: 0;
                padding: 0;
                list-style: none;
            }}

            .ant-breadcrumb-item {{
                display: flex;
                align-items: center;
                margin: 0;
                padding: 0;
                color: {};
                font-size: {}px;
                line-height: 1.5715;
                cursor: default;
                transition: color 0.3s;
            }}

            .ant-breadcrumb-item:last-child {{
                color: {};
                font-weight: 600;
            }}

            .ant-breadcrumb-item:last-child .ant-breadcrumb-link {{
                color: {};
            }}

            .ant-breadcrumb-item:last-child .ant-breadcrumb-link:hover {{
                color: {};
            }}

            .ant-breadcrumb-link {{
                display: flex;
                align-items: center;
                color: {};
                text-decoration: none;
                transition: color 0.3s;
                cursor: pointer;
            }}

            .ant-breadcrumb-link:hover {{
                color: {};
            }}

            .ant-breadcrumb-link-disabled {{
                color: {};
                cursor: not-allowed;
            }}

            .ant-breadcrumb-link-disabled:hover {{
                color: {};
            }}

            .ant-breadcrumb-icon {{
                margin-right: 4px;
                font-size: {}px;
                line-height: 1;
            }}

            .ant-breadcrumb-text {{
                display: inline-block;
            }}

            .ant-breadcrumb-separator {{
                display: flex;
                align-items: center;
                margin: 0 8px;
                color: {};
                font-size: {}px;
                line-height: 1;
                cursor: default;
            }}

            .ant-breadcrumb-separator:last-child {{
                display: none;
            }}

            .ant-breadcrumb-item:hover {{
                color: {};
            }}

            .ant-breadcrumb-item:hover .ant-breadcrumb-link {{
                color: {};
            }}

            .ant-breadcrumb a:active {{
                color: {};
            }}

            .ant-breadcrumb-item.ant-breadcrumb-item-disabled {{
                color: {};
                cursor: not-allowed;
            }}

            .ant-breadcrumb-item.ant-breadcrumb-item-disabled .ant-breadcrumb-link {{
                color: {};
                cursor: not-allowed;
            }}

            .ant-breadcrumb-item.ant-breadcrumb-item-disabled .ant-breadcrumb-link:hover {{
                color: {};
            }}

            .ant-breadcrumb .anticon {{
                font-size: {}px;
            }}

            .ant-breadcrumb .anticon+span {{
                margin-left: 4px;
            }}

            .ant-breadcrumb-separator .anticon {{
                font-size: {}px;
                color: {};
            }}

            .ant-breadcrumb-item {{
                max-width: 200px;
            }}

            .ant-breadcrumb-text {{
                overflow: hidden;
                text-overflow: ellipsis;
                white-space: nowrap;
            }}

            @media (max-width: 576px) {{
                .ant-breadcrumb {{
                    font-size: {}px;
                }}
                .ant-breadcrumb-item {{
                    font-size: {}px;
                }}
                .ant-breadcrumb-separator {{
                    margin: 0 4px;
                    font-size: {}px;
                }}
                .ant-breadcrumb-icon {{
                    margin-right: 2px;
                    font-size: {}px;
                }}
            }}

            @media (max-width: 768px) {{
                .ant-breadcrumb-item {{
                    max-width: 120px;
                }}
            }}

            @media (max-width: 480px) {{
                .ant-breadcrumb-item {{
                    max-width: 80px;
                }}
            }}

            @media print {{
                .ant-breadcrumb {{
                    color: #000;
                }}
                .ant-breadcrumb-item {{
                    color: #000;
                }}
                .ant-breadcrumb-link {{
                    color: #000;
                }}
                .ant-breadcrumb-separator {{
                    color: #000;
                }}
            }}
            "#,
            self.token.color_text,           // Breadcrumb color
            self.token.font_size,            // Breadcrumb font size
            self.token.color_text_secondary, // Link color
            self.token.color_text,           // Link hover color
            self.token.color_text_secondary, // Item color
            self.token.font_size,            // Item font size
            self.token.color_text,           // Last item color
            self.token.color_text,           // Last item link color
            self.token.color_text,           // Last item link hover color
            self.token.color_text_secondary, // Link color
            self.token.color_text,           // Link hover color
            self.token.color_text_disabled,  // Disabled link color
            self.token.color_text_disabled,  // Disabled link hover color
            self.token.font_size,            // Icon font size
            self.token.color_text_secondary, // Separator color
            self.token.font_size,            // Separator font size
            self.token.color_text,           // Item hover color
            self.token.color_text,           // Item link hover color
            self.token.color_primary,        // Active link color
            self.token.color_text_disabled,  // Disabled item color
            self.token.color_text_disabled,  // Disabled item link color
            self.token.color_text_disabled,  // Disabled item link hover color
            self.token.font_size - 2,        // Icon font size
            self.token.font_size - 2,        // Separator icon font size
            self.token.color_text_secondary, // Separator icon color
            self.token.font_size - 2,        // Mobile font size
            self.token.font_size - 2,        // Mobile item font size
            self.token.font_size - 2,        // Mobile separator font size
            self.token.font_size - 2,        // Mobile icon font size
        ))
        .unwrap();

        style.get_class_name()
    }
}

pub fn use_breadcrumb_style() -> String {
    let style_generator = BreadcrumbStyleGenerator::new();
    style_generator.generate_style()
}
