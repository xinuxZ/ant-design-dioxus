use crate::theme::AliasToken;
use css_in_rust::css;

pub struct CalendarStyleGenerator {
    token: AliasToken,
}

impl CalendarStyleGenerator {
    pub fn new() -> Self {
        Self {
            token: AliasToken::default(),
        }
    }

    pub fn generate_style(&self) -> String {
        css!(
            r#"
            /* Calendar Component Styles */
            .ant-picker-calendar {
              color: ${text_color};
              font-size: ${font_size}px;
              line-height: 1.5714285714285714;
              list-style: none;
              font-family: ${font_family};
              background: ${bg_color};
              border: 1px solid ${border_color};
              border-radius: ${border_radius}px;
              box-sizing: border-box;
            }

            .ant-picker-calendar-header {
              display: flex;
              justify-content: space-between;
              align-items: center;
              padding: 16px 24px;
              border-bottom: 1px solid ${header_border_color};
            }

            .ant-picker-calendar-header .ant-picker-calendar-year-select,
            .ant-picker-calendar-header .ant-picker-calendar-month-select {
              margin-right: 8px;
              min-width: 80px;
            }

            .ant-picker-calendar-header .ant-picker-calendar-mode-switch {
              display: flex;
              align-items: center;
            }

            .ant-picker-calendar-mode-switch .ant-radio-button-wrapper {
              height: 28px;
              line-height: 26px;
              padding: 0 12px;
            }

            .ant-picker-calendar-body {
              padding: 8px 12px;
            }

            /* Calendar table */
            .ant-picker-calendar table {
              width: 100%;
              border-collapse: collapse;
              border-spacing: 0;
            }

            .ant-picker-calendar table,
            .ant-picker-calendar th,
            .ant-picker-calendar td {
              border: none;
            }

            .ant-picker-calendar-thead>tr>th {
              color: ${thead_color};
              font-weight: 400;
              text-align: center;
              border-bottom: 1px solid ${thead_border_color};
              padding: 12px 0;
            }

            .ant-picker-calendar-tbody>tr>td {
              position: relative;
              padding: 0;
              text-align: center;
              vertical-align: top;
              border-top: 1px solid ${tbody_border_color};
            }

            .ant-picker-calendar-date {
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
            }

            .ant-picker-calendar-date:hover {
              background: ${date_hover_bg};
            }

            .ant-picker-calendar-date-today {
              border: 1px solid ${today_border_color};
            }

            .ant-picker-calendar-date-selected {
              background: ${selected_bg};
              color: ${selected_color};
            }

            .ant-picker-calendar-date-selected:hover {
              background: ${selected_hover_bg};
            }

            .ant-picker-calendar-date-disabled {
              color: ${disabled_color};
              cursor: not-allowed;
            }

            .ant-picker-calendar-date-disabled:hover {
              background: transparent;
            }

            /* Month view */
            .ant-picker-calendar-month-panel {
              display: flex;
              flex-direction: column;
              width: 100%;
            }

            .ant-picker-calendar-month-panel .ant-picker-calendar-date {
              height: 60px;
              line-height: 1.66;
              padding: 4px 8px;
              border-radius: 0;
              text-align: left;
              vertical-align: top;
            }

            .ant-picker-calendar-month-panel .ant-picker-calendar-date-content {
              position: absolute;
              left: 0;
              top: 0;
              right: 0;
              bottom: 0;
              padding: 4px 8px;
              overflow: hidden;
            }

            .ant-picker-calendar-month-panel .ant-picker-calendar-date-value {
              color: ${date_value_color};
              font-size: ${date_value_font_size}px;
              text-align: right;
              line-height: 1;
            }

            .ant-picker-calendar-month-panel .ant-picker-calendar-date-today .ant-picker-calendar-date-value {
              color: ${today_value_color};
            }

            .ant-picker-calendar-month-panel .ant-picker-calendar-date-selected .ant-picker-calendar-date-value {
              color: ${selected_value_color};
            }

            /* Year view */
            .ant-picker-calendar-year-panel .ant-picker-calendar-date {
              height: 40px;
              line-height: 40px;
            }

            /* Fullscreen mode */
            .ant-picker-calendar-fullscreen {
              border: none;
              border-radius: 0;
            }

            .ant-picker-calendar-fullscreen .ant-picker-calendar-header {
              border-bottom: 1px solid ${fullscreen_header_border};
              padding: 16px 0;
            }

            .ant-picker-calendar-fullscreen .ant-picker-calendar-body {
              padding: 0;
            }

            .ant-picker-calendar-fullscreen .ant-picker-calendar-date {
              border-radius: 0;
              height: 116px;
              text-align: left;
              vertical-align: top;
              padding: 4px 8px;
            }

            /* Compact size */
            .ant-picker-calendar-sm {
              border-radius: ${sm_border_radius}px;
            }

            .ant-picker-calendar-sm .ant-picker-calendar-header {
              padding: 8px 16px;
            }

            .ant-picker-calendar-sm .ant-picker-calendar-body {
              padding: 4px 8px;
            }

            /* RTL support */
            .ant-picker-calendar-rtl {
              direction: rtl;
            }

            .ant-picker-calendar-rtl .ant-picker-calendar-header .ant-picker-calendar-year-select,
            .ant-picker-calendar-rtl .ant-picker-calendar-header .ant-picker-calendar-month-select {
              margin-right: 0;
              margin-left: 8px;
            }

            /* Responsive design */
            @media (max-width: 768px) {
              .ant-picker-calendar-header {
                padding: 8px 16px;
              }

              .ant-picker-calendar-body {
                padding: 4px 8px;
              }

              .ant-picker-calendar-month-panel .ant-picker-calendar-date {
                height: 40px;
              }
            }

            /* High contrast mode */
            @media (prefers-contrast: high) {
              .ant-picker-calendar {
                border: 1px solid #000;
              }

              .ant-picker-calendar-header {
                border-bottom: 1px solid #000;
              }

              .ant-picker-calendar-date-today {
                border: 2px solid #000;
              }

              .ant-picker-calendar-date-selected {
                background: #000;
                color: #fff;
              }
            }

            /* Reduced motion mode */
            @media (prefers-reduced-motion: reduce) {
              .ant-picker-calendar-date {
                transition: none;
              }
            }
            "#,
            text_color = self.token.color_text,
            font_size = self.token.font_size,
            font_family = self.token.font_family,
            bg_color = self.token.color_bg_container,
            border_color = self.token.color_border,
            border_radius = self.token.border_radius,
            header_border_color = self.token.color_border,
            thead_color = self.token.color_text_secondary,
            thead_border_color = self.token.color_border,
            tbody_border_color = self.token.color_border,
            date_hover_bg = self.token.color_bg,
            today_border_color = self.token.color_primary,
            selected_bg = self.token.color_primary,
            selected_color = "#fff",
            selected_hover_bg = self.token.color_primary_hover,
            disabled_color = self.token.color_text_disabled,
            date_value_color = self.token.color_text,
            date_value_font_size = self.token.font_size,
            today_value_color = self.token.color_primary,
            selected_value_color = "#fff",
            fullscreen_header_border = self.token.color_border,
            sm_border_radius = self.token.border_radius_sm
        )
    }
}

pub fn use_calendar_style() -> String {
    CalendarStyleGenerator::new().generate_style()
}
