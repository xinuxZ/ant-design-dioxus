use chrono::{Datelike, Duration, NaiveDate, Weekday};
use dioxus::prelude::*;
use std::collections::HashMap;

mod styles;
use styles::use_calendar_style;

/// Calendar mode
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CalendarMode {
    Month,
    Year,
}

impl Default for CalendarMode {
    fn default() -> Self {
        Self::Month
    }
}

impl CalendarMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            CalendarMode::Month => "month",
            CalendarMode::Year => "year",
        }
    }
}

/// Calendar size
#[derive(Clone, Debug, PartialEq)]
pub enum CalendarSize {
    Default,
    Large,
}

impl Default for CalendarSize {
    fn default() -> Self {
        Self::Default
    }
}

impl CalendarSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            CalendarSize::Default => "default",
            CalendarSize::Large => "large",
        }
    }
}

/// Calendar panel mode
#[derive(Clone, Debug, PartialEq)]
pub enum CalendarPanelMode {
    Date,
    Week,
    Month,
    Quarter,
    Year,
    Decade,
}

impl CalendarPanelMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            CalendarPanelMode::Date => "date",
            CalendarPanelMode::Week => "week",
            CalendarPanelMode::Month => "month",
            CalendarPanelMode::Quarter => "quarter",
            CalendarPanelMode::Year => "year",
            CalendarPanelMode::Decade => "decade",
        }
    }
}

/// Calendar cell info
#[derive(Clone, Debug, PartialEq)]
pub struct CalendarCellInfo {
    /// Cell date
    pub date: NaiveDate,
    /// Cell type (today, selected, etc.)
    pub cell_type: CalendarCellType,
    /// Whether the cell is in current month/year
    pub in_view: bool,
    /// Whether the cell is disabled
    pub disabled: bool,
}

/// Calendar cell type
#[derive(Clone, Debug, PartialEq)]
pub enum CalendarCellType {
    Today,
    Selected,
    Normal,
}

impl CalendarCellType {
    pub fn as_str(&self) -> &'static str {
        match self {
            CalendarCellType::Today => "today",
            CalendarCellType::Selected => "selected",
            CalendarCellType::Normal => "normal",
        }
    }
}

/// Calendar header render info
#[derive(Clone, Debug, PartialEq)]
pub struct CalendarHeaderRenderInfo {
    /// Current value
    pub value: NaiveDate,
    /// Calendar type
    pub calendar_type: CalendarMode,
    /// Change handler
    pub on_change: fn(NaiveDate),
    /// Type change handler
    pub on_type_change: fn(CalendarMode),
}

/// Calendar properties
#[derive(Props, Clone, PartialEq)]
pub struct CalendarProps {
    /// Current value
    #[props(default)]
    pub value: Option<NaiveDate>,

    /// Default value
    #[props(default)]
    pub default_value: Option<NaiveDate>,

    /// Calendar mode
    #[props(default)]
    pub mode: CalendarMode,

    /// Full screen mode
    #[props(default = true)]
    pub fullscreen: bool,

    /// Calendar size
    #[props(default)]
    pub size: CalendarSize,

    /// Locale configuration
    #[props(default)]
    pub locale: Option<HashMap<String, String>>,

    /// Valid range
    #[props(default)]
    pub valid_range: Option<(NaiveDate, NaiveDate)>,

    /// Disabled date function
    #[props(default)]
    pub disabled_date: Option<fn(NaiveDate) -> bool>,

    /// Date cell render function
    #[props(default)]
    pub date_cell_render: Option<fn(NaiveDate) -> Element>,

    /// Date full cell render function
    #[props(default)]
    pub date_full_cell_render: Option<fn(NaiveDate) -> Element>,

    /// Month cell render function
    #[props(default)]
    pub month_cell_render: Option<fn(NaiveDate) -> Element>,

    /// Month full cell render function
    #[props(default)]
    pub month_full_cell_render: Option<fn(NaiveDate) -> Element>,

    /// Header render function
    #[props(default)]
    pub header_render: Option<fn(CalendarHeaderRenderInfo) -> Element>,

    /// Change callback
    #[props(default)]
    pub on_change: Option<EventHandler<NaiveDate>>,

    /// Panel change callback
    #[props(default)]
    pub on_panel_change: Option<EventHandler<(NaiveDate, CalendarMode)>>,

    /// Select callback
    #[props(default)]
    pub on_select: Option<EventHandler<NaiveDate>>,

    /// CSS class name
    #[props(default)]
    pub class: Option<String>,

    /// Inline style
    #[props(default)]
    pub style: Option<String>,

    /// Element ID
    #[props(default)]
    pub id: Option<String>,
}

/// Calendar component
#[component]
pub fn Calendar(props: CalendarProps) -> Element {
    let mut current_value = use_signal(|| {
        props
            .value
            .or(props.default_value)
            .unwrap_or_else(|| chrono::Local::now().date_naive())
    });
    let mut current_mode = use_signal(|| props.mode.clone());
    let mut view_date = use_signal(|| *current_value.read());

    // 使用CSS-in-Rust样式
    let style_class = use_calendar_style();

    let class_name = format!(
        "{} {} {} {} {}",
        style_class,
        if props.fullscreen {
            "ant-calendar-fullscreen"
        } else {
            "ant-calendar-mini"
        },
        format!("ant-calendar-{}", props.size.as_str()),
        format!("ant-calendar-{}", current_mode.read().as_str()),
        props.class.as_deref().unwrap_or("")
    )
    .trim()
    .to_string();

    let handle_date_select = move |date: NaiveDate| {
        current_value.set(date);

        if let Some(on_change) = &props.on_change {
            on_change.call(date);
        }

        if let Some(on_select) = &props.on_select {
            on_select.call(date);
        }
    };

    let handle_mode_change = move |mode: CalendarMode| {
        current_mode.set(mode.clone());

        if let Some(on_panel_change) = &props.on_panel_change {
            on_panel_change.call((*current_value.read(), mode));
        }
    };

    let handle_view_change = move |date: NaiveDate| {
        view_date.set(date);
    };

    let _is_date_disabled = |date: NaiveDate| -> bool {
        if let Some(disabled_fn) = props.disabled_date {
            if disabled_fn(date) {
                return true;
            }
        }

        if let Some((start, end)) = props.valid_range {
            return date < start || date > end;
        }

        false
    };

    rsx! {
        div {
            class: "{class_name}",
            id: props.id,
            style: props.style,

            // Header
            if let Some(header_render) = props.header_render {
                {header_render(CalendarHeaderRenderInfo {
                    value: *current_value.read(),
                    calendar_type: current_mode.read().clone(),
                    on_change: |_date| {
                        // Simplified implementation
                    },
                    on_type_change: |_mode| {
                        // Simplified implementation
                    },
                })}
            } else {
                CalendarHeader {
                    value: *current_value.read(),
                    view_date: *view_date.read(),
                    mode: current_mode.read().clone(),
                    locale: props.locale.clone(),
                    on_view_change: handle_view_change,
                    on_mode_change: handle_mode_change,
                }
            }

            // Body
            div {
                class: "ant-calendar-body",

                match *current_mode.read() {
                    CalendarMode::Month => rsx! {
                        CalendarMonthPanel {
                            value: *current_value.read(),
                            view_date: *view_date.read(),
                            locale: props.locale.clone(),
                            disabled_date: props.disabled_date,
                            date_cell_render: props.date_cell_render,
                            date_full_cell_render: props.date_full_cell_render,
                            on_select: handle_date_select,
                        }
                    },
                    CalendarMode::Year => rsx! {
                        CalendarYearPanel {
                            value: *current_value.read(),
                            view_date: *view_date.read(),
                            locale: props.locale.clone(),
                            month_cell_render: props.month_cell_render,
                            month_full_cell_render: props.month_full_cell_render,
                            on_select: handle_date_select,
                        }
                    },
                }
            }
        }
    }
}

/// Calendar header component
#[component]
fn CalendarHeader(
    value: NaiveDate,
    view_date: NaiveDate,
    mode: CalendarMode,
    locale: Option<HashMap<String, String>>,
    on_view_change: EventHandler<NaiveDate>,
    on_mode_change: EventHandler<CalendarMode>,
) -> Element {
    let _get_locale_text = |key: &str, default: &str| -> String {
        locale
            .as_ref()
            .and_then(|l| l.get(key))
            .map(|s| s.clone())
            .unwrap_or_else(|| default.to_string())
    };

    let handle_prev_click = move |_evt: MouseEvent| {
        let new_date = match mode {
            CalendarMode::Month => {
                view_date - Duration::days(view_date.day() as i64) + Duration::days(1)
                    - Duration::days(1)
            }
            CalendarMode::Year => {
                NaiveDate::from_ymd_opt(view_date.year() - 1, 1, 1).unwrap_or(view_date)
            }
        };
        on_view_change.call(new_date);
    };

    let handle_next_click = move |_evt: MouseEvent| {
        let new_date = match mode {
            CalendarMode::Month => {
                let next_month = if view_date.month() == 12 {
                    NaiveDate::from_ymd_opt(view_date.year() + 1, 1, 1)
                } else {
                    NaiveDate::from_ymd_opt(view_date.year(), view_date.month() + 1, 1)
                };
                next_month.unwrap_or(view_date)
            }
            CalendarMode::Year => {
                NaiveDate::from_ymd_opt(view_date.year() + 1, 1, 1).unwrap_or(view_date)
            }
        };
        on_view_change.call(new_date);
    };

    let handle_mode_click = move |_evt: MouseEvent| {
        let new_mode = match mode {
            CalendarMode::Month => CalendarMode::Year,
            CalendarMode::Year => CalendarMode::Month,
        };
        on_mode_change.call(new_mode);
    };

    let title_text = match mode {
        CalendarMode::Month => format!(
            "{} {}",
            get_month_name(view_date.month(), &locale),
            view_date.year()
        ),
        CalendarMode::Year => view_date.year().to_string(),
    };

    rsx! {
        div {
            class: "ant-calendar-header",

            div {
                class: "ant-calendar-header-left",

                button {
                    class: "ant-calendar-prev-button",
                    onclick: handle_prev_click,
                    "‹"
                }
            }

            div {
                class: "ant-calendar-header-center",

                button {
                    class: "ant-calendar-header-title",
                    onclick: handle_mode_click,
                    "{title_text}"
                }
            }

            div {
                class: "ant-calendar-header-right",

                button {
                    class: "ant-calendar-next-button",
                    onclick: handle_next_click,
                    "›"
                }
            }
        }
    }
}

/// Calendar month panel component
#[component]
fn CalendarMonthPanel(
    value: NaiveDate,
    view_date: NaiveDate,
    locale: Option<HashMap<String, String>>,
    disabled_date: Option<fn(NaiveDate) -> bool>,
    date_cell_render: Option<fn(NaiveDate) -> Element>,
    date_full_cell_render: Option<fn(NaiveDate) -> Element>,
    on_select: EventHandler<NaiveDate>,
) -> Element {
    let today = chrono::Local::now().date_naive();
    let first_day_of_month =
        NaiveDate::from_ymd_opt(view_date.year(), view_date.month(), 1).unwrap_or(view_date);
    let first_day_weekday = first_day_of_month.weekday();

    // Calculate the start date of the calendar grid (including previous month dates)
    let start_date =
        first_day_of_month - Duration::days(first_day_weekday.num_days_from_sunday() as i64);

    // Generate 42 days (6 weeks * 7 days)
    let calendar_dates: Vec<NaiveDate> = (0..42).map(|i| start_date + Duration::days(i)).collect();

    let get_weekday_name = |weekday: Weekday| -> String {
        let key = format!("weekday_{}", weekday.num_days_from_sunday());
        locale
            .as_ref()
            .and_then(|l| l.get(&key))
            .map(|s| s.clone())
            .unwrap_or_else(|| match weekday {
                Weekday::Sun => "Sun".to_string(),
                Weekday::Mon => "Mon".to_string(),
                Weekday::Tue => "Tue".to_string(),
                Weekday::Wed => "Wed".to_string(),
                Weekday::Thu => "Thu".to_string(),
                Weekday::Fri => "Fri".to_string(),
                Weekday::Sat => "Sat".to_string(),
            })
    };

    let handle_date_click = move |date: NaiveDate| {
        if let Some(disabled_fn) = disabled_date {
            if disabled_fn(date) {
                return;
            }
        }
        on_select.call(date);
    };

    rsx! {
        div {
            class: "ant-calendar-month-panel",

            // Week header
            div {
                class: "ant-calendar-week-header",
                for weekday in [Weekday::Sun, Weekday::Mon, Weekday::Tue, Weekday::Wed, Weekday::Thu, Weekday::Fri, Weekday::Sat] {
                    div {
                        key: "{weekday:?}",
                        class: "ant-calendar-week-header-cell",
                        {get_weekday_name(weekday)}
                    }
                }
            }

            // Calendar grid
            div {
                class: "ant-calendar-date-panel",
                for (_index, date) in calendar_dates.iter().enumerate() {
                    CalendarDateCell {
                        key: "{date}",
                        date: *date,
                        value,
                        view_date,
                        today,
                        disabled_date,
                        date_cell_render,
                        date_full_cell_render,
                        on_click: move |d| handle_date_click(d),
                    }
                }
            }
        }
    }
}

/// Calendar year panel component
#[component]
fn CalendarYearPanel(
    value: NaiveDate,
    view_date: NaiveDate,
    locale: Option<HashMap<String, String>>,
    month_cell_render: Option<fn(NaiveDate) -> Element>,
    month_full_cell_render: Option<fn(NaiveDate) -> Element>,
    on_select: EventHandler<NaiveDate>,
) -> Element {
    let months: Vec<u32> = (1..=12).collect();

    let handle_month_click = move |month: u32| {
        let new_date = NaiveDate::from_ymd_opt(view_date.year(), month, 1).unwrap_or(view_date);
        on_select.call(new_date);
    };

    rsx! {
        div {
            class: "ant-calendar-year-panel",

            div {
                class: "ant-calendar-month-panel-body",
                for month in months {
                    CalendarMonthCell {
                        key: "{month}",
                        month,
                        year: view_date.year(),
                        value,
                        locale: locale.clone(),
                        month_cell_render,
                        month_full_cell_render,
                        on_click: move |m| handle_month_click(m),
                    }
                }
            }
        }
    }
}

/// Calendar date cell component
#[component]
fn CalendarDateCell(
    date: NaiveDate,
    value: NaiveDate,
    view_date: NaiveDate,
    today: NaiveDate,
    disabled_date: Option<fn(NaiveDate) -> bool>,
    date_cell_render: Option<fn(NaiveDate) -> Element>,
    date_full_cell_render: Option<fn(NaiveDate) -> Element>,
    on_click: EventHandler<NaiveDate>,
) -> Element {
    let is_today = date == today;
    let is_selected = date == value;
    let is_in_view = date.month() == view_date.month() && date.year() == view_date.year();
    let is_disabled = disabled_date.map_or(false, |f| f(date));

    let cell_class = format!(
        "ant-calendar-date {} {} {} {} {}",
        if is_today {
            "ant-calendar-date-today"
        } else {
            ""
        },
        if is_selected {
            "ant-calendar-date-selected"
        } else {
            ""
        },
        if !is_in_view {
            "ant-calendar-date-other-month"
        } else {
            ""
        },
        if is_disabled {
            "ant-calendar-date-disabled"
        } else {
            ""
        },
        "ant-calendar-date-cell"
    )
    .trim()
    .to_string();

    let handle_click = move |_evt: MouseEvent| {
        if !is_disabled {
            on_click.call(date);
        }
    };

    rsx! {
        div {
            class: "{cell_class}",
            onclick: handle_click,

            if let Some(full_cell_render) = date_full_cell_render {
                {full_cell_render(date)}
            } else {
                div {
                    class: "ant-calendar-date-value",
                    "{date.day()}"
                }

                if let Some(cell_render) = date_cell_render {
                    div {
                        class: "ant-calendar-date-content",
                        {cell_render(date)}
                    }
                }
            }
        }
    }
}

/// Calendar month cell component
#[component]
fn CalendarMonthCell(
    month: u32,
    year: i32,
    value: NaiveDate,
    locale: Option<HashMap<String, String>>,
    month_cell_render: Option<fn(NaiveDate) -> Element>,
    month_full_cell_render: Option<fn(NaiveDate) -> Element>,
    on_click: EventHandler<u32>,
) -> Element {
    let month_date = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let is_selected = value.year() == year && value.month() == month;
    let is_current = {
        let now = chrono::Local::now().date_naive();
        now.year() == year && now.month() == month
    };

    let cell_class = format!(
        "ant-calendar-month-panel-cell {} {}",
        if is_selected {
            "ant-calendar-month-panel-selected-cell"
        } else {
            ""
        },
        if is_current {
            "ant-calendar-month-panel-current-cell"
        } else {
            ""
        }
    )
    .trim()
    .to_string();

    let handle_click = move |_evt: MouseEvent| {
        on_click.call(month);
    };

    rsx! {
        div {
            class: "{cell_class}",
            onclick: handle_click,

            if let Some(full_cell_render) = month_full_cell_render {
                {full_cell_render(month_date)}
            } else {
                div {
                    class: "ant-calendar-month-panel-month",
                    {get_month_name(month, &locale)}
                }

                if let Some(cell_render) = month_cell_render {
                    div {
                        class: "ant-calendar-month-panel-content",
                        {cell_render(month_date)}
                    }
                }
            }
        }
    }
}

/// Helper function to get month name
fn get_month_name(month: u32, locale: &Option<HashMap<String, String>>) -> String {
    let key = format!("month_{}", month);
    locale
        .as_ref()
        .and_then(|l| l.get(&key))
        .map(|s| s.clone())
        .unwrap_or_else(|| match month {
            1 => "Jan".to_string(),
            2 => "Feb".to_string(),
            3 => "Mar".to_string(),
            4 => "Apr".to_string(),
            5 => "May".to_string(),
            6 => "Jun".to_string(),
            7 => "Jul".to_string(),
            8 => "Aug".to_string(),
            9 => "Sep".to_string(),
            10 => "Oct".to_string(),
            11 => "Nov".to_string(),
            12 => "Dec".to_string(),
            _ => month.to_string(),
        })
}

/// Helper trait for weekday calculations
trait WeekdayExt {
    fn num_days_from_sunday(&self) -> u32;
}

impl WeekdayExt for Weekday {
    fn num_days_from_sunday(&self) -> u32 {
        match self {
            Weekday::Sun => 0,
            Weekday::Mon => 1,
            Weekday::Tue => 2,
            Weekday::Wed => 3,
            Weekday::Thu => 4,
            Weekday::Fri => 5,
            Weekday::Sat => 6,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_calendar_mode_as_str() {
        assert_eq!(CalendarMode::Month.as_str(), "month");
        assert_eq!(CalendarMode::Year.as_str(), "year");
    }

    #[test]
    fn test_calendar_size_as_str() {
        assert_eq!(CalendarSize::Default.as_str(), "default");
        assert_eq!(CalendarSize::Large.as_str(), "large");
    }

    #[test]
    fn test_calendar_cell_type_as_str() {
        assert_eq!(CalendarCellType::Today.as_str(), "today");
        assert_eq!(CalendarCellType::Selected.as_str(), "selected");
        assert_eq!(CalendarCellType::Normal.as_str(), "normal");
    }

    #[test]
    fn test_get_month_name() {
        let locale = None;
        assert_eq!(get_month_name(1, &locale), "Jan");
        assert_eq!(get_month_name(12, &locale), "Dec");
        assert_eq!(get_month_name(13, &locale), "13");

        let mut custom_locale = HashMap::new();
        custom_locale.insert("month_1".to_string(), "January".to_string());
        let locale = Some(custom_locale);
        assert_eq!(get_month_name(1, &locale), "January");
        assert_eq!(get_month_name(2, &locale), "Feb");
    }

    #[test]
    fn test_weekday_num_days_from_sunday() {
        assert_eq!(Weekday::Sun.num_days_from_sunday(), 0);
        assert_eq!(Weekday::Mon.num_days_from_sunday(), 1);
        assert_eq!(Weekday::Sat.num_days_from_sunday(), 6);
    }

    #[test]
    fn test_calendar_cell_info() {
        let date = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
        let cell_info = CalendarCellInfo {
            date,
            cell_type: CalendarCellType::Today,
            in_view: true,
            disabled: false,
        };

        assert_eq!(cell_info.date, date);
        assert_eq!(cell_info.cell_type, CalendarCellType::Today);
        assert!(cell_info.in_view);
        assert!(!cell_info.disabled);
    }
}
