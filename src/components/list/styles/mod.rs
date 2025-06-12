use crate::theme::AliasToken;
use css_in_rust::css;

pub struct ListStyleGenerator {
    token: AliasToken,
}

impl ListStyleGenerator {
    pub fn new() -> Self {
        Self {
            token: AliasToken::default(),
        }
    }

    pub fn generate_style(&self) -> String {
        css!(
            r#"
            /* List 列表组件样式 */

            .ant-list {
              margin: 0;
              padding: 0;
              color: ${color_text};
              font-size: ${font_size}px;
              font-variant: tabular-nums;
              line-height: 1.5715;
              list-style: none;
              font-feature-settings: 'tnum';
              position: relative;
            }

            .ant-list * {
              outline: none;
            }

            .ant-list-pagination {
              margin-top: ${margin_lg}px;
              text-align: right;
            }

            .ant-list-pagination .ant-pagination-options {
              text-align: left;
            }

            .ant-list-more {
              margin-top: ${margin_lg}px;
              text-align: center;
            }

            .ant-list-more button {
              padding-right: ${padding_lg_2}px;
              padding-left: ${padding_lg_2}px;
            }

            .ant-list-spin {
              min-height: 40px;
              text-align: center;
            }

            .ant-list-empty-text {
              padding: ${padding_lg}px;
              color: ${color_text_secondary};
              font-size: ${font_size}px;
              text-align: center;
            }

            .ant-list-items {
              margin: 0;
              padding: 0;
              list-style: none;
            }

            .ant-list-item {
              display: flex;
              align-items: flex-start;
              justify-content: space-between;
              padding: ${padding_sm}px 0;
              color: ${color_text};
            }

            .ant-list-item-main {
              display: flex;
              flex: 1;
              flex-direction: column;
              justify-content: space-between;
              min-width: 0;
            }

            .ant-list-item-extra {
              margin-left: ${margin_extra}px;
            }

            .ant-list-item-meta {
              display: flex;
              flex: 1;
              align-items: flex-start;
              font-size: 0;
            }

            .ant-list-item-meta-avatar {
              margin-right: ${margin_sm}px;
            }

            .ant-list-item-meta-content {
              flex: 1 0;
              width: 0;
              color: ${color_text};
            }

            .ant-list-item-meta-title {
              margin-bottom: ${margin_xs}px;
              color: ${color_text};
              font-size: ${font_size}px;
              line-height: 1.5715;
            }

            .ant-list-item-meta-title>a {
              color: ${color_text};
              transition: all 0.3s;
            }

            .ant-list-item-meta-title>a:hover {
              color: ${color_primary};
            }

            .ant-list-item-meta-description {
              color: ${color_text_secondary};
              font-size: ${font_size}px;
              line-height: 1.5715;
            }

            .ant-list-item-action {
              flex: 0 0 auto;
              margin-left: ${margin_action}px;
              padding: 0;
              font-size: 0;
              list-style: none;
            }

            .ant-list-item-action>li {
              position: relative;
              display: inline-block;
              padding: 0 ${padding_xs}px;
              color: ${color_text_secondary};
              font-size: ${font_size}px;
              line-height: 1.5715;
              text-align: center;
              cursor: pointer;
            }

            .ant-list-item-action>li:first-child {
              padding-left: 0;
            }

            .ant-list-item-action-split {
              position: absolute;
              top: 50%;
              right: 0;
              width: 1px;
              height: 14px;
              margin-top: -7px;
              background-color: ${color_border};
            }

            .ant-list-header {
              background: transparent;
            }

            .ant-list-footer {
              background: transparent;
            }

            .ant-list-header,
            .ant-list-footer {
              padding-top: ${padding_sm}px;
              padding-bottom: ${padding_sm}px;
            }

            .ant-list-empty {
              padding: ${padding_lg}px 0;
              color: ${color_text_secondary};
              font-size: ${font_size}px;
              text-align: center;
            }

            .ant-list-split .ant-list-item {
              border-bottom: 1px solid ${color_border};
            }

            .ant-list-split .ant-list-item:last-child {
              border-bottom: none;
            }

            .ant-list-split .ant-list-header {
              border-bottom: 1px solid ${color_border};
            }

            .ant-list-split.ant-list-empty .ant-list-footer {
              border-top: 1px solid ${color_border};
            }

            .ant-list-loading .ant-list-spin-nested-loading {
              min-height: 32px;
            }

            .ant-list-something-after-last-item .ant-spin-container > div:last-child {
              border-bottom: 1px solid ${color_border};
              padding-bottom: 0;
            }

            .ant-list-lg .ant-list-item {
              padding: ${padding_lg}px 0;
            }

            .ant-list-sm .ant-list-item {
              padding: ${padding_xs}px 0;
            }

            .ant-list-vertical .ant-list-item {
              align-items: initial;
            }

            .ant-list-vertical .ant-list-item-main {
              display: block;
              flex: 1;
            }

            .ant-list-vertical .ant-list-item-extra {
              margin-left: ${margin_lg}px;
            }

            .ant-list-vertical .ant-list-item-meta {
              margin-bottom: ${margin_md}px;
            }

            .ant-list-vertical .ant-list-item-meta-title {
              margin-bottom: ${margin_xs}px;
              color: ${color_text};
              font-size: ${font_size}px;
              line-height: 1.5715;
            }

            .ant-list-vertical .ant-list-item-action {
              margin-top: ${margin_md}px;
              margin-left: auto;
            }

            .ant-list-vertical .ant-list-item-action>li {
              padding: 0 ${padding_xs}px;
            }

            .ant-list-vertical .ant-list-item-action>li:first-child {
              padding-left: 0;
            }

            .ant-list-grid .ant-col>.ant-list-item {
              display: block;
              max-width: 100%;
              margin-bottom: ${margin_lg}px;
              padding-top: 0;
              padding-bottom: 0;
              border-bottom: none;
            }

            .ant-list-grid .ant-col>.ant-list-item-no-flex {
              display: block;
            }

            .ant-list-item-no-flex {
              display: block;
            }

            .ant-list:not(.ant-list-vertical) .ant-list-item-no-flex .ant-list-item-action {
              float: right;
            }

            .ant-list-bordered {
              border: 1px solid ${color_border};
              border-radius: ${border_radius}px;
            }

            .ant-list-bordered .ant-list-header {
              padding-right: ${padding_lg}px;
              padding-left: ${padding_lg}px;
            }

            .ant-list-bordered .ant-list-footer {
              padding-right: ${padding_lg}px;
              padding-left: ${padding_lg}px;
            }

            .ant-list-bordered .ant-list-item {
              padding-right: ${padding_lg}px;
              padding-left: ${padding_lg}px;
            }

            .ant-list-bordered .ant-list-pagination {
              margin: 16px 24px;
            }

            .ant-list-bordered.ant-list-sm .ant-list-item {
              padding-right: ${padding_sm}px;
              padding-left: ${padding_sm}px;
            }

            .ant-list-bordered.ant-list-sm .ant-list-header,
            .ant-list-bordered.ant-list-sm .ant-list-footer {
              padding: ${padding_xs}px ${padding_sm}px;
            }

            .ant-list-bordered.ant-list-lg .ant-list-item {
              padding-right: ${padding_lg}px;
              padding-left: ${padding_lg}px;
            }

            .ant-list-bordered.ant-list-lg .ant-list-header,
            .ant-list-bordered.ant-list-lg .ant-list-footer {
              padding: ${padding_sm}px ${padding_lg}px;
            }

            @media screen and (max-width: 768px) {
              .ant-list-item-action {
                margin-left: ${margin_md}px;
              }
              .ant-list-vertical .ant-list-item-extra {
                margin-left: ${margin_md}px;
              }
            }

            @media screen and (max-width: 576px) {
              .ant-list-item {
                flex-wrap: wrap;
              }
              .ant-list-item-action {
                margin-left: ${margin_md}px;
              }
              .ant-list-vertical .ant-list-item {
                flex-wrap: wrap-reverse;
              }
              .ant-list-vertical .ant-list-item-main {
                min-width: 220px;
              }
              .ant-list-vertical .ant-list-item-extra {
                margin: auto auto ${margin_md}px;
              }
            }
            "#,
            color_text = self.token.color_text,
            font_size = self.token.font_size,
            margin_lg = self.token.margin_lg,
            padding_lg_2 = self.token.padding_lg * 2.0,
            padding_lg = self.token.padding_lg,
            color_text_secondary = self.token.color_text_secondary,
            padding_sm = self.token.padding_sm,
            margin_extra = self.token.margin_lg * 2.0 + self.token.margin_xs,
            margin_sm = self.token.margin_sm,
            margin_xs = self.token.margin_xs,
            color_primary = self.token.color_primary,
            margin_action = self.token.margin_lg * 3.0,
            padding_xs = self.token.padding_xs,
            color_border = self.token.color_border,
            margin_md = self.token.margin_md,
            border_radius = self.token.border_radius
        )
    }
}

pub fn use_list_style() -> String {
    ListStyleGenerator::new().generate_style()
}
