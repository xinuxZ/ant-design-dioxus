use crate::theme::AliasToken;
use css_in_rust::css;

pub struct BreadcrumbStyleGenerator {
    token: AliasToken,
}

impl BreadcrumbStyleGenerator {
    pub fn new() -> Self {
        Self {
            token: AliasToken::default(),
        }
    }

    pub fn generate_style(&self) -> String {
        css!(
            r#"
            /* Breadcrumb styles */
            .ant-breadcrumb {
              margin: 0;
              padding: 0;
              color: ${color_text};
              font-size: ${font_size}px;
              line-height: 1.5715;
              list-style: none;
              font-feature-settings: 'tnum';
              box-sizing: border-box;
            }

            .ant-breadcrumb ol {
              display: flex;
              flex-wrap: wrap;
              margin: 0;
              padding: 0;
              list-style: none;
            }

            .ant-breadcrumb a {
              color: ${link_color};
              text-decoration: none;
              transition: color 0.3s;
            }

            .ant-breadcrumb a:hover {
              color: ${link_hover_color};
            }

            .ant-breadcrumb-list {
              display: flex;
              flex-wrap: wrap;
              align-items: center;
              margin: 0;
              padding: 0;
              list-style: none;
            }

            .ant-breadcrumb-item {
              display: flex;
              align-items: center;
              margin: 0;
              padding: 0;
              color: ${item_color};
              font-size: ${font_size}px;
              line-height: 1.5715;
              cursor: default;
              transition: color 0.3s;
            }

            .ant-breadcrumb-item:last-child {
              color: ${last_item_color};
              font-weight: 600;
            }

            .ant-breadcrumb-item:last-child .ant-breadcrumb-link {
              color: ${last_item_link_color};
            }

            .ant-breadcrumb-item:last-child .ant-breadcrumb-link:hover {
              color: ${last_item_link_hover_color};
            }

            .ant-breadcrumb-link {
              display: flex;
              align-items: center;
              color: ${link_color};
              text-decoration: none;
              transition: color 0.3s;
              cursor: pointer;
            }

            .ant-breadcrumb-link:hover {
              color: ${link_hover_color};
            }

            .ant-breadcrumb-link-disabled {
              color: ${disabled_color};
              cursor: not-allowed;
            }

            .ant-breadcrumb-link-disabled:hover {
              color: ${disabled_color};
            }

            .ant-breadcrumb-icon {
              margin-right: 4px;
              font-size: ${icon_font_size}px;
              line-height: 1;
            }

            .ant-breadcrumb-text {
              display: inline-block;
            }

            .ant-breadcrumb-separator {
              display: flex;
              align-items: center;
              margin: 0 8px;
              color: ${separator_color};
              font-size: ${separator_font_size}px;
              line-height: 1;
              cursor: default;
            }

            .ant-breadcrumb-separator:last-child {
              display: none;
            }

            .ant-breadcrumb-item:hover {
              color: ${item_hover_color};
            }

            .ant-breadcrumb-item:hover .ant-breadcrumb-link {
              color: ${link_hover_color};
            }

            .ant-breadcrumb a:active {
              color: ${link_active_color};
            }

            .ant-breadcrumb-item.ant-breadcrumb-item-disabled {
              color: ${disabled_color};
              cursor: not-allowed;
            }

            .ant-breadcrumb-item.ant-breadcrumb-item-disabled .ant-breadcrumb-link {
              color: ${disabled_color};
              cursor: not-allowed;
            }

            .ant-breadcrumb-item.ant-breadcrumb-item-disabled .ant-breadcrumb-link:hover {
              color: ${disabled_color};
            }

            .ant-breadcrumb .anticon {
              font-size: ${icon_font_size}px;
            }

            .ant-breadcrumb .anticon+span {
              margin-left: 4px;
            }

            .ant-breadcrumb-separator .anticon {
              font-size: ${separator_icon_font_size}px;
              color: ${separator_color};
            }

            .ant-breadcrumb-item {
              max-width: 200px;
            }

            .ant-breadcrumb-text {
              overflow: hidden;
              text-overflow: ellipsis;
              white-space: nowrap;
            }

            @media (max-width: 576px) {
              .ant-breadcrumb {
                font-size: ${mobile_font_size}px;
              }
              .ant-breadcrumb-item {
                font-size: ${mobile_item_font_size}px;
              }
              .ant-breadcrumb-separator {
                margin: 0 4px;
                font-size: ${mobile_separator_font_size}px;
              }
              .ant-breadcrumb-icon {
                margin-right: 2px;
                font-size: ${mobile_icon_font_size}px;
              }
            }

            @media (max-width: 768px) {
              .ant-breadcrumb-item {
                max-width: 120px;
              }
            }

            @media (max-width: 480px) {
              .ant-breadcrumb-item {
                max-width: 80px;
              }
            }

            /* RTL support */
            .ant-breadcrumb-rtl {
              direction: rtl;
            }

            .ant-breadcrumb-rtl .ant-breadcrumb-icon {
              margin-right: 0;
              margin-left: 4px;
            }

            .ant-breadcrumb-rtl .ant-breadcrumb-separator {
              transform: rotate(180deg);
            }

            .ant-breadcrumb-rtl .ant-breadcrumb .anticon+span {
              margin-right: 4px;
              margin-left: 0;
            }

            @media (max-width: 576px) {
              .ant-breadcrumb-rtl .ant-breadcrumb-icon {
                margin-right: 0;
                margin-left: 2px;
              }
            }
            "#,
            color_text = self.token.color_text,
            font_size = self.token.font_size,
            link_color = self.token.color_text_secondary,
            link_hover_color = self.token.color_primary,
            item_color = self.token.color_text,
            last_item_color = self.token.color_text,
            last_item_link_color = self.token.color_text,
            last_item_link_hover_color = self.token.color_primary,
            disabled_color = self.token.color_text_disabled,
            icon_font_size = self.token.font_size - 2.0,
            separator_color = self.token.color_text_secondary,
            separator_font_size = self.token.font_size - 2.0,
            item_hover_color = self.token.color_primary,
            link_active_color = self.token.color_primary_active,
            separator_icon_font_size = self.token.font_size - 2.0,
            mobile_font_size = self.token.font_size - 2.0,
            mobile_item_font_size = self.token.font_size - 2.0,
            mobile_separator_font_size = self.token.font_size - 2.0,
            mobile_icon_font_size = self.token.font_size - 2.0
        )
    }
}

pub fn use_breadcrumb_style() -> String {
    BreadcrumbStyleGenerator::new().generate_style()
}
