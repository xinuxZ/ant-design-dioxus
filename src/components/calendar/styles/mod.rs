use crate::theme::{use_token, AliasToken};
use css_in_rust::Style;

pub struct CalendarStyleGenerator {
    token: AliasToken,
}

impl CalendarStyleGenerator {
    pub fn new() -> Self {
        Self { token: use_token() }
    }

    pub fn generate_style(&self) -> String {
        let style = Style::new(format!(
            r#"
            /* Calendar Component Styles */
            .ant-picker-calendar {{
                color: {};
                font-size: {}px;
                line-height: 1.5714285714285714;
                list-style: none;
                font-family: {};
                background: {};
                border: 1px solid {};
                border-radius: {}px;
                box-sizing: border-box;
            }}

            .ant-picker-calendar-header {{
                display: flex;
                justify-content: space-between;
                align-items: center;
                padding: 16px 24px;
                border-bottom: 1px solid {};
            }}

            .ant-picker-calendar-header .ant-picker-calendar-year-select,
            .ant-picker-calendar-header .ant-picker-calendar-month-select {{
                margin-right: 8px;
                min-width: 80px;
            }}

            .ant-picker-calendar-header .ant-picker-calendar-mode-switch {{
                display: flex;
                align-items: center;
            }}

            .ant-picker-calendar-mode-switch .ant-radio-button-wrapper {{
                height: 28px;
                line-height: 26px;
                padding: 0 12px;
            }}

            .ant-picker-calendar-body {{
                padding: 8px 12px;
            }}

            /* Calendar table */
            .ant-picker-calendar table {{
                width: 100%;
                border-collapse: collapse;
                border-spacing: 0;
            }}

            .ant-picker-calendar table,
            .ant-picker-calendar th,
            .ant-picker-calendar td {{
                border: none;
            }}

            .ant-picker-calendar-thead>tr>th {{
                color: {};
                font-weight: 400;
                text-align: center;
                border-bottom: 1px solid {};
                padding: 12px 0;
            }}

            .ant-picker-calendar-tbody>tr>td {{
                position: relative;
                padding: 0;
                text-align: center;
                vertical-align: top;
                border-top: 1px solid {};
            }}

            .ant-picker-calendar-date {{
                position: relative;
                display: block;
                margin: 0 auto;
                padding: 4px;
                width: 24px;
                height: 24px;
                line-height: 24px;
                text-align: center;
                border-radius: 2px;
                transition: background 0.3s, border 0.3s;
                cursor: pointer;
            }}

            .ant-picker-calendar-date:hover {{
                background: {};
            }}

            .ant-picker-calendar-date-today {{
                border: 1px solid {};
            }}

            .ant-picker-calendar-date-selected {{
                background: {};
                color: {};
            }}

            .ant-picker-calendar-date-selected:hover {{
                background: {};
            }}

            .ant-picker-calendar-date-disabled {{
                color: {};
                cursor: not-allowed;
            }}

            .ant-picker-calendar-date-disabled:hover {{
                background: transparent;
            }}

            /* Month view */
            .ant-picker-calendar-month-panel {{
                display: flex;
                flex-direction: column;
                width: 100%;
            }}

            .ant-picker-calendar-month-panel .ant-picker-calendar-date {{
                height: 60px;
                line-height: 1.66;
                padding: 4px 8px;
                border-radius: 0;
                text-align: left;
                vertical-align: top;
            }}

            .ant-picker-calendar-month-panel .ant-picker-calendar-date-content {{
                position: absolute;
                left: 0;
                top: 0;
                right: 0;
                bottom: 0;
                padding: 4px 8px;
                overflow: hidden;
            }}

            .ant-picker-calendar-month-panel .ant-picker-calendar-date-value {{
                color: {};
                font-size: {}px;
                text-align: right;
                line-height: 1;
            }}

            .ant-picker-calendar-month-panel .ant-picker-calendar-date-today .ant-picker-calendar-date-value {{
                color: {};
            }}

            .ant-picker-calendar-month-panel .ant-picker-calendar-date-selected .ant-picker-calendar-date-value {{
                color: {};
            }}

            /* Year view */
            .ant-picker-calendar-year-panel .ant-picker-calendar-date {{
                height: 40px;
                line-height: 40px;
            }}

            /* Fullscreen mode */
            .ant-picker-calendar-fullscreen {{
                border: none;
                border-radius: 0;
            }}

            .ant-picker-calendar-fullscreen .ant-picker-calendar-header {{
                border-bottom: 1px solid {};
                padding: 16px 0;
            }}

            .ant-picker-calendar-fullscreen .ant-picker-calendar-body {{
                padding: 0;
            }}

            .ant-picker-calendar-fullscreen .ant-picker-calendar-date {{
                border-radius: 0;
                height: 116px;
                text-align: left;
                vertical-align: top;
                padding: 4px 8px;
            }}

            /* Compact size */
            .ant-picker-calendar-sm {{
                border-radius: {}px;
            }}

            .ant-picker-calendar-sm .ant-picker-calendar-header {{
                padding: 8px 16px;
            }}

            .ant-picker-calendar-sm .ant-picker-calendar-date {{
                height: 20px;
                line-height: 20px;
                width: 20px;
            }}

            /* Large size */
            .ant-picker-calendar-lg .ant-picker-calendar-header {{
                padding: 20px 32px;
            }}

            .ant-picker-calendar-lg .ant-picker-calendar-date {{
                height: 28px;
                line-height: 28px;
                width: 28px;
            }}

            /* RTL support */
            .ant-picker-calendar-rtl {{
                direction: rtl;
            }}

            .ant-picker-calendar-rtl .ant-picker-calendar-header .ant-picker-calendar-year-select,
            .ant-picker-calendar-rtl .ant-picker-calendar-header .ant-picker-calendar-month-select {{
                margin-right: 0;
                margin-left: 8px;
            }}
            "#,
            self.token.color_text,                  // Calendar text color
            self.token.font_size,                   // Font size
            self.token.font_family,                 // Font family
            self.token.color_bg_container,          // Background color
            self.token.color_border,                // Border color
            self.token.border_radius,               // Border radius
            self.token.color_border_secondary,      // Header border color
            self.token.color_text,                  // Thead text color
            self.token.color_border_secondary,      // Thead border color
            self.token.color_border_secondary,      // Tbody border color
            self.token.color_bg_text_hover,         // Date hover background
            self.token.color_primary,               // Today border color
            self.token.color_primary,               // Selected background
            self.token.color_white,                 // Selected text color
            self.token.color_primary_hover,         // Selected hover background
            self.token.color_text_disabled,         // Disabled text color
            self.token.color_text_secondary,        // Date value color
            self.token.font_size_sm,                // Date value font size
            self.token.color_primary,               // Today date value color
            self.token.color_white,                 // Selected date value color
            self.token.color_border_secondary,      // Fullscreen header border
            self.token.border_radius_sm,            // Small calendar border radius
        ))
        .unwrap();

        style.get_class_name()
    }
}

pub fn use_calendar_style() -> String {
    let style_generator = CalendarStyleGenerator::new();
    style_generator.generate_style()
}
