use crate::theme::{use_token, AliasToken};
use css_in_rust::Style;

pub struct ListStyleGenerator {
    token: AliasToken,
}

impl ListStyleGenerator {
    pub fn new() -> Self {
        Self { token: use_token() }
    }

    pub fn generate_style(&self) -> String {
        let style = Style::new(format!(
            r#"
            /* List 列表组件样式 */

            .ant-list {{
              margin: 0;
              padding: 0;
              color: {};
              font-size: {}px;
              font-variant: tabular-nums;
              line-height: 1.5715;
              list-style: none;
              font-feature-settings: 'tnum';
              position: relative;
            }}

            .ant-list * {{
              outline: none;
            }}

            .ant-list-pagination {{
              margin-top: {}px;
              text-align: right;
            }}

            .ant-list-pagination .ant-pagination-options {{
              text-align: left;
            }}

            .ant-list-more {{
              margin-top: {}px;
              text-align: center;
            }}

            .ant-list-more button {{
              padding-right: {}px;
              padding-left: {}px;
            }}

            .ant-list-spin {{
              min-height: 40px;
              text-align: center;
            }}

            .ant-list-empty-text {{
              padding: {}px;
              color: {};
              font-size: {}px;
              text-align: center;
            }}

            .ant-list-items {{
              margin: 0;
              padding: 0;
              list-style: none;
            }}

            .ant-list-item {{
              display: flex;
              align-items: flex-start;
              justify-content: space-between;
              padding: {}px 0;
              color: {};
            }}

            .ant-list-item-main {{
              display: flex;
              flex: 1;
              flex-direction: column;
              justify-content: space-between;
              min-width: 0;
            }}

            .ant-list-item-extra {{
              margin-left: {}px;
            }}

            .ant-list-item-meta {{
              display: flex;
              flex: 1;
              align-items: flex-start;
              font-size: 0;
            }}

            .ant-list-item-meta-avatar {{
              margin-right: {}px;
            }}

            .ant-list-item-meta-content {{
              flex: 1 0;
              width: 0;
              color: {};
            }}

            .ant-list-item-meta-title {{
              margin-bottom: {}px;
              color: {};
              font-size: {}px;
              line-height: 1.5715;
            }}

            .ant-list-item-meta-title>a {{
              color: {};
              transition: all 0.3s;
            }}

            .ant-list-item-meta-title>a:hover {{
              color: {};
            }}

            .ant-list-item-meta-description {{
              color: {};
              font-size: {}px;
              line-height: 1.5715;
            }}

            .ant-list-item-action {{
              flex: 0 0 auto;
              margin-left: {}px;
              padding: 0;
              font-size: 0;
              list-style: none;
            }}

            .ant-list-item-action>li {{
              position: relative;
              display: inline-block;
              padding: 0 {}px;
              color: {};
              font-size: {}px;
              line-height: 1.5715;
              text-align: center;
              cursor: pointer;
            }}

            .ant-list-item-action>li:first-child {{
              padding-left: 0;
            }}

            .ant-list-item-action-split {{
              position: absolute;
              top: 50%;
              right: 0;
              width: 1px;
              height: 14px;
              margin-top: -7px;
              background-color: {};
            }}

            .ant-list-header {{
              background: transparent;
            }}

            .ant-list-footer {{
              background: transparent;
            }}

            .ant-list-header,
            .ant-list-footer {{
              padding-top: {}px;
              padding-bottom: {}px;
            }}

            .ant-list-empty {{
              padding: {}px 0;
              color: {};
              font-size: {}px;
              text-align: center;
            }}

            .ant-list-split .ant-list-item {{
              border-bottom: 1px solid {};
            }}

            .ant-list-split .ant-list-item:last-child {{
              border-bottom: none;
            }}

            .ant-list-split .ant-list-header {{
              border-bottom: 1px solid {};
            }}

            .ant-list-split.ant-list-empty .ant-list-footer {{
              border-top: 1px solid {};
            }}

            .ant-list-loading .ant-list-spin-nested-loading {{
              min-height: 32px;
            }}

            .ant-list-split.ant-list-something-after-last-item .ant-list-item:last-child {{
              border-bottom: 1px solid {};
            }}

            .ant-list-lg .ant-list-item {{
              padding: {}px {}px;
            }}

            .ant-list-sm .ant-list-item {{
              padding: {}px {}px;
            }}

            .ant-list-vertical .ant-list-item {{
              align-items: initial;
            }}

            .ant-list-vertical .ant-list-item-main {{
              display: block;
              flex: 1;
            }}

            .ant-list-vertical .ant-list-item-extra {{
              margin-left: {}px;
            }}

            .ant-list-vertical .ant-list-item-meta {{
              margin-bottom: {}px;
            }}

            .ant-list-vertical .ant-list-item-meta-title {{
              margin-bottom: {}px;
              color: {};
              font-size: {}px;
              line-height: {}px;
            }}

            .ant-list-vertical .ant-list-item-action {{
              margin-top: {}px;
              margin-left: auto;
            }}

            .ant-list-vertical .ant-list-item-action>li {{
              padding: 0 {}px;
            }}

            .ant-list-vertical .ant-list-item-action>li:first-child {{
              padding-left: 0;
            }}

            .ant-list-bordered {{
              border: 1px solid {};
              border-radius: {}px;
            }}

            .ant-list-bordered .ant-list-header {{
              padding-right: {}px;
              padding-left: {}px;
            }}

            .ant-list-bordered .ant-list-footer {{
              padding-right: {}px;
              padding-left: {}px;
            }}

            .ant-list-bordered .ant-list-item {{
              padding-right: {}px;
              padding-left: {}px;
            }}

            .ant-list-bordered .ant-list-pagination {{
              margin: 16px 24px;
            }}

            .ant-list-bordered.ant-list-sm .ant-list-item {{
              padding-right: {}px;
              padding-left: {}px;
            }}

            .ant-list-bordered.ant-list-sm .ant-list-header,
            .ant-list-bordered.ant-list-sm .ant-list-footer {{
              padding: {}px {}px;
            }}

            .ant-list-bordered.ant-list-lg .ant-list-item {{
              padding-right: {}px;
              padding-left: {}px;
            }}

            .ant-list-bordered.ant-list-lg .ant-list-header,
            .ant-list-bordered.ant-list-lg .ant-list-footer {{
              padding: {}px {}px;
            }}

            /* 加载样式 */
            .ant-list-loading-content {{
              display: flex;
              flex-direction: column;
              padding: 0;
            }}

            .ant-list-item-loading {{
              animation: ant-list-loading 1.4s ease infinite;
            }}

            .ant-list-loading-block {{
              height: 14px;
              margin-bottom: 8px;
              background: linear-gradient(90deg, rgba(207, 216, 220, 0.2), rgba(207, 216, 220, 0.4), rgba(207, 216, 220, 0.2));
              background-size: 600% 600%;
              border-radius: {}px;
            }}

            .ant-list-loading-avatar {{
              width: 32px;
              height: 32px;
              border-radius: 50%;
            }}

            .ant-list-loading-title {{
              width: 60%;
              height: 16px;
            }}

            .ant-list-loading-description {{
              width: 50%;
              height: 12px;
            }}

            @keyframes ant-list-loading {{
              0% {{
                background-position: 0% 50%;
              }}
              50% {{
                background-position: 100% 50%;
              }}
              100% {{
                background-position: 0% 50%;
              }}
            }}

            /* RTL 支持 */
            .ant-list-rtl {{
              direction: rtl;
              text-align: right;
            }}

            .ant-list-rtl .ant-list-item-meta-avatar {{
              margin-right: 0;
              margin-left: {}px;
            }}

            .ant-list-rtl .ant-list-item-action {{
              margin-right: {}px;
              margin-left: 0;
            }}

            .ant-list-rtl .ant-list-item-action>li:first-child {{
              padding-right: 0;
              padding-left: {}px;
            }}

            .ant-list-rtl .ant-list-item-action-split {{
              right: auto;
              left: 0;
            }}

            .ant-list-rtl .ant-list-item-extra {{
              margin-right: {}px;
              margin-left: 0;
            }}

            .ant-list-rtl.ant-list-vertical .ant-list-item-action {{
              margin-right: auto;
              margin-left: 0;
            }}

            .ant-list-rtl.ant-list-vertical .ant-list-item-action>li:first-child {{
              padding-right: 0;
              padding-left: {}px;
            }}

            .ant-list-rtl.ant-list-bordered .ant-list-pagination {{
              text-align: left;
            }}
            "#,
            self.token.color_text,                 // 基础文本颜色
            self.token.font_size,                  // 基础字体大小
            self.token.margin_lg,                  // 分页上边距
            self.token.margin_md,                  // 加载更多上边距
            self.token.padding_lg * 2,             // 加载更多按钮内边距(右)
            self.token.padding_lg * 2,             // 加载更多按钮内边距(左)
            self.token.padding_md,                 // 空列表内边距
            self.token.color_text_quaternary,      // 空列表文本颜色
            self.token.font_size,                  // 空列表字体大小
            self.token.padding_md,                 // 列表项内边距
            self.token.color_text,                 // 列表项文本颜色
            self.token.margin_lg * 2 + self.token.margin_xs, // 列表项额外内容左边距
            self.token.margin_md,                  // 列表项元数据头像右边距
            self.token.color_text,                 // 列表项元数据内容文本颜色
            self.token.margin_xs,                  // 列表项元数据标题下边距
            self.token.color_text,                 // 列表项元数据标题文本颜色
            self.token.font_size,                  // 列表项元数据标题字体大小
            self.token.color_text,                 // 列表项元数据标题链接颜色
            self.token.color_primary,              // 列表项元数据标题链接悬浮颜色
            self.token.color_text_secondary,       // 列表项元数据描述文本颜色
            self.token.font_size,                  // 列表项元数据描述字体大小
            self.token.margin_lg * 3,              // 列表项操作左边距
            self.token.padding_xs,                 // 列表项操作项内边距
            self.token.color_text_secondary,       // 列表项操作项文本颜色
            self.token.font_size,                  // 列表项操作项字体大小
            self.token.color_split,                // 列表项操作分割线颜色
            self.token.padding_md,                 // 列表头部/底部上内边距
            self.token.padding_md,                 // 列表头部/底部下内边距
            self.token.padding_md,                 // 空列表内边距
            self.token.color_text_quaternary,      // 空列表文本颜色
            self.token.font_size_sm,               // 空列表字体大小
            self.token.color_split,                // 分割线颜色
            self.token.color_split,                // 头部分割线颜色
            self.token.color_split,                // 底部分割线颜色
            self.token.color_split,                // 最后一项分割线颜色
            self.token.padding_md,                 // 大尺寸列表项内边距(垂直)
            self.token.padding_lg,                 // 大尺寸列表项内边距(水平)
            self.token.padding_xs,                 // 小尺寸列表项内边距(垂直)
            self.token.padding_md,                 // 小尺寸列表项内边距(水平)
            self.token.margin_lg * 2 + self.token.margin_xs, // 垂直布局额外内容左边距
            self.token.margin_md,                  // 垂直布局元数据下边距
            self.token.margin_xs,                  // 垂直布局元数据标题下边距
            self.token.color_text,                 // 垂直布局元数据标题文本颜色
            self.token.font_size,                  // 垂直布局元数据标题字体大小
            self.token.line_height,                // 垂直布局元数据标题行高
            self.token.margin_md,                  // 垂直布局操作上边距
            self.token.padding_xs,                 // 垂直布局操作项内边距
            self.token.color_border,               // 边框列表边框颜色
            self.token.border_radius,              // 边框列表边框圆角
            self.token.padding_lg,                 // 边框列表头部右内边距
            self.token.padding_lg,                 // 边框列表头部左内边距
            self.token.padding_lg,                 // 边框列表底部右内边距
            self.token.padding_lg,                 // 边框列表底部左内边距
            self.token.padding_lg,                 // 边框列表项右内边距
            self.token.padding_lg,                 // 边框列表项左内边距
            self.token.padding_md,                 // 小尺寸边框列表项右内边距
            self.token.padding_md,                 // 小尺寸边框列表项左内边距
            self.token.padding_xs,                 // 小尺寸边框列表头部/底部内边距(垂直)
            self.token.padding_md,                 // 小尺寸边框列表头部/底部内边距(水平)
            self.token.padding_lg,                 // 大尺寸边框列表项右内边距
            self.token.padding_lg,                 // 大尺寸边框列表项左内边距
            self.token.padding_md,                 // 大尺寸边框列表头部/底部内边距(垂直)
            self.token.padding_lg,                 // 大尺寸边框列表头部/底部内边距(水平)
            self.token.border_radius_sm,           // 加载块边框圆角
            self.token.margin_md,                  // RTL模式元数据头像左边距
            self.token.margin_lg * 3,              // RTL模式操作右边距
            self.token.padding_xs,                 // RTL模式操作项左内边距
            self.token.margin_lg * 2 + self.token.margin_xs, // RTL模式额外内容右边距
            self.token.padding_xs,                 // RTL模式垂直布局操作项左内边距
        ))
        .unwrap();

        style.get_class_name()
    }
}

pub fn use_list_style() -> String {
    let style_generator = ListStyleGenerator::new();
    style_generator.generate_style()
}
