//! Icon 组件的工具函数

use super::types::{IconTheme, IconType};
use std::collections::HashMap;

/// SVG图标的基础结构
#[derive(Debug, Clone)]
pub struct SvgIcon {
    pub view_box: String,
    pub path: String,
    pub width: Option<String>,
    pub height: Option<String>,
}

/// 图标库管理器
pub struct IconLibrary {
    icons: HashMap<String, SvgIcon>,
}

impl IconLibrary {
    /// 创建新的图标库实例
    pub fn new() -> Self {
        Self {
            icons: HashMap::new(),
        }
    }

    /// 注册自定义图标
    pub fn register_icon(&mut self, name: String, icon: SvgIcon) {
        self.icons.insert(name, icon);
    }

    /// 获取图标
    pub fn get_icon(&self, name: &str) -> Option<&SvgIcon> {
        self.icons.get(name)
    }

    /// 批量注册图标
    pub fn register_icons(&mut self, icons: HashMap<String, SvgIcon>) {
        self.icons.extend(icons);
    }
}

/// 全局图标库实例
static mut GLOBAL_ICON_LIBRARY: Option<IconLibrary> = None;
static INIT: std::sync::Once = std::sync::Once::new();

/// 获取全局图标库
pub fn get_global_icon_library() -> &'static mut IconLibrary {
    unsafe {
        INIT.call_once(|| {
            GLOBAL_ICON_LIBRARY = Some(IconLibrary::new());
        });
        GLOBAL_ICON_LIBRARY.as_mut().unwrap()
    }
}

/// 注册全局图标
pub fn register_global_icon(name: String, icon: SvgIcon) {
    let library = get_global_icon_library();
    library.register_icon(name, icon);
}

/// 获取全局图标
pub fn get_global_icon(name: &str) -> Option<&SvgIcon> {
    let library = get_global_icon_library();
    library.get_icon(name)
}

/// 常用图标的SVG数据
pub fn get_common_icon_svg(icon_type: &IconType) -> Option<SvgIcon> {
    match icon_type {
        IconType::Success => Some(SvgIcon {
            view_box: "0 0 1024 1024".to_string(),
            path: "M512 64C264.6 64 64 264.6 64 512s200.6 448 448 448 448-200.6 448-448S759.4 64 512 64zm193.5 301.7l-210.6 292a31.8 31.8 0 0 1-51.7 0L318.5 484.9c-3.8-5.3 0-12.7 6.5-12.7h46.9c10.2 0 19.9 4.9 25.9 13.3l71.2 98.8l157.2-218c6-8.3 15.6-13.3 25.9-13.3H699c6.5 0 10.3 7.4 6.5 12.7z".to_string(),
            width: None,
            height: None,
        }),
        IconType::Warning => Some(SvgIcon {
            view_box: "0 0 1024 1024".to_string(),
            path: "M512 64C264.6 64 64 264.6 64 512s200.6 448 448 448 448-200.6 448-448S759.4 64 512 64zm165.4 618.2l-66-.3L512 563.4l-99.3 118.4-66.1.3c-4.4 0-8-3.5-8-8 0-1.9.7-3.7 1.9-5.2l130.1-155L340.5 359a8.32 8.32 0 0 1-1.9-5.2c0-4.4 3.6-8 8-8l66.1.3L512 464.6l99.3-118.4 66-.3c4.4 0 8 3.5 8 8 0 1.9-.7 3.7-1.9 5.2L553.5 514l130 155c1.2 1.5 1.9 3.3 1.9 5.2 0 4.4-3.6 8-8 8z".to_string(),
            width: None,
            height: None,
        }),
        IconType::Error => Some(SvgIcon {
            view_box: "0 0 1024 1024".to_string(),
            path: "M512 64C264.6 64 64 264.6 64 512s200.6 448 448 448 448-200.6 448-448S759.4 64 512 64zm165.4 618.2l-66-.3L512 563.4l-99.3 118.4-66.1.3c-4.4 0-8-3.5-8-8 0-1.9.7-3.7 1.9-5.2l130.1-155L340.5 359a8.32 8.32 0 0 1-1.9-5.2c0-4.4 3.6-8 8-8l66.1.3L512 464.6l99.3-118.4 66-.3c4.4 0 8 3.5 8 8 0 1.9-.7 3.7-1.9 5.2L553.5 514l130 155c1.2 1.5 1.9 3.3 1.9 5.2 0 4.4-3.6 8-8 8z".to_string(),
            width: None,
            height: None,
        }),

        IconType::Home => Some(SvgIcon {
            view_box: "0 0 1024 1024".to_string(),
            path: "M946.5 505L560.1 118.8l-25.9-25.9a31.5 31.5 0 0 0-44.4 0L77.5 505a63.9 63.9 0 0 0-18.8 46c.4 35.2 29.7 63.3 64.9 63.3h42.5V940h691.8V614.3h43.4c17.1 0 33.2-6.7 45.3-18.8a63.6 63.6 0 0 0 18.7-45.3c0-17-6.7-33.1-18.8-45.2zM568 868H456V664h112v204zm217.9-325.7V868H632V640c0-22.1-17.9-40-40-40H432c-22.1 0-40 17.9-40 40v228H238.1V542.3h-96l370-369.7 23.1 23.1L882 542.3h-96.1z".to_string(),
            width: None,
            height: None,
        }),
        IconType::User => Some(SvgIcon {
            view_box: "0 0 1024 1024".to_string(),
            path: "M858.5 763.6a374 374 0 0 0-80.6-119.5 375.63 375.63 0 0 0-119.5-80.6c-.4-.2-.8-.3-1.2-.5C719.5 518 760 444.7 760 362c0-137-111-248-248-248S264 225 264 362c0 82.7 40.5 156 102.8 201.1-.4.2-.8.3-1.2.5-44.8 18.9-85 46-119.5 80.6a375.63 375.63 0 0 0-80.6 119.5A371.7 371.7 0 0 0 136 901.8a8 8 0 0 0 8 8.2h60c4.4 0 7.9-3.5 8-7.8 2-77.2 33-149.5 87.8-204.3 56.7-56.7 132-87.9 212.2-87.9s155.5 31.2 212.2 87.9C779 752.7 810 825 812 902.2c.1 4.4 3.6 7.8 8 7.8h60a8 8 0 0 0 8-8.2c-1-47.8-10.9-94.3-29.5-138.2zM512 534c-45.9 0-89.1-17.9-121.6-50.4S340 407.9 340 362c0-45.9 17.9-89.1 50.4-121.6S466.1 190 512 190s89.1 17.9 121.6 50.4S684 316.1 684 362c0 45.9-17.9 89.1-50.4 121.6S557.9 534 512 534z".to_string(),
            width: None,
            height: None,
        }),
        IconType::Setting => Some(SvgIcon {
            view_box: "0 0 1024 1024".to_string(),
            path: "M924.8 625.7l-65.5-56c3.1-19 4.7-38.4 4.7-57.8s-1.6-38.8-4.7-57.8l65.5-56a32.03 32.03 0 0 0 9.3-35.2l-.9-2.6a443.74 443.74 0 0 0-79.7-137.9l-1.8-2.1a32.12 32.12 0 0 0-35.1-9.5l-81.3 28.9c-30-24.6-63.5-44-99.7-57.6l-15.7-85a32.05 32.05 0 0 0-25.8-25.7l-2.7-.5c-52.1-9.4-106.9-9.4-159 0l-2.7.5a32.05 32.05 0 0 0-25.8 25.7l-15.8 85.4a351.86 351.86 0 0 0-99 57.4l-81.9-29.1a32 32 0 0 0-35.1 9.5l-1.8 2.1a446.02 446.02 0 0 0-79.7 137.9l-.9 2.6c-4.5 12.5-.8 26.5 9.3 35.2l66.3 56.6c-3.1 18.8-4.6 38-4.6 57.1 0 19.2 1.5 38.4 4.6 57.1L99 625.5a32.03 32.03 0 0 0-9.3 35.2l.9 2.6c18.1 50.4 44.9 96.9 79.7 137.9l1.8 2.1a32.12 32.12 0 0 0 35.1 9.5l81.9-29.1c29.8 24.5 63.1 43.9 99 57.4l15.8 85.4a32.05 32.05 0 0 0 25.8 25.7l2.7.5a449.4 449.4 0 0 0 159 0l2.7-.5a32.05 32.05 0 0 0 25.8-25.7l15.7-85a350 350 0 0 0 99.7-57.6l81.3 28.9a32 32 0 0 0 35.1-9.5l1.8-2.1c34.8-41.1 61.6-87.5 79.7-137.9l.9-2.6c4.5-12.3.8-26.3-9.3-35zM512 726c-118.6 0-214-95.4-214-214s95.4-214 214-214 214 95.4 214 214-95.4 214-214 214z".to_string(),
            width: None,
            height: None,
        }),
        IconType::Search => Some(SvgIcon {
            view_box: "0 0 1024 1024".to_string(),
            path: "M909.6 854.5L649.9 594.8C690.2 542.7 712 479 712 412c0-80.2-31.3-155.4-87.9-212.1-56.6-56.7-132-87.9-212.1-87.9s-155.5 31.3-212.1 87.9C143.2 256.5 112 331.8 112 412c0 80.1 31.3 155.5 87.9 212.1C256.5 680.8 331.8 712 412 712c67 0 130.6-21.8 182.7-62l259.7 259.6a8.2 8.2 0 0 0 11.6 0l43.6-43.5a8.2 8.2 0 0 0 0-11.6zM570.4 570.4C528 612.7 471.8 636 412 636s-116-23.3-158.4-65.6C211.3 528 188 471.8 188 412s23.3-116.1 65.6-158.4C296 211.3 352.2 188 412 188s116.1 23.2 158.4 65.6S636 352.2 636 412s-23.3 116.1-65.6 158.4z".to_string(),
            width: None,
            height: None,
        }),
        IconType::Close => Some(SvgIcon {
            view_box: "0 0 1024 1024".to_string(),
            path: "M563.8 512l262.5-312.9c4.4-5.2.7-13.1-6.1-13.1h-79.8c-4.7 0-9.2 2.1-12.3 5.7L511.6 449.8 295.1 191.7c-3.1-3.6-7.6-5.7-12.3-5.7H203c-6.8 0-10.5 7.9-6.1 13.1L459.4 512 196.9 824.9A7.95 7.95 0 0 0 203 838h79.8c4.7 0 9.2-2.1 12.3-5.7l216.5-258.1 216.5 258.1c3.1 3.6 7.6 5.7 12.3 5.7h79.8c6.8 0 10.5-7.9 6.1-13.1L563.8 512z".to_string(),
            width: None,
            height: None,
        }),
        IconType::Check => Some(SvgIcon {
            view_box: "0 0 1024 1024".to_string(),
            path: "M912 190h-69.9c-9.8 0-19.1 4.5-25.1 12.2L404.7 724.5 207 474a32 32 0 0 0-25.1-12.2H112c-6.7 0-10.4 7.7-6.3 12.9l273.9 347c12.8 16.2 37.4 16.2 50.3 0l488.4-618.9c4.1-5.1.4-12.8-6.3-12.8z".to_string(),
            width: None,
            height: None,
        }),
        IconType::Plus => Some(SvgIcon {
            view_box: "0 0 1024 1024".to_string(),
            path: "M482 152h60q8 0 8 8v704q0 8-8 8h-60q-8 0-8-8V160q0-8 8-8z M176 474h672q8 0 8 8v60q0 8-8 8H176q-8 0-8-8v-60q0-8 8-8z".to_string(),
            width: None,
            height: None,
        }),
        IconType::Minus => Some(SvgIcon {
            view_box: "0 0 1024 1024".to_string(),
            path: "M176 474h672q8 0 8 8v60q0 8-8 8H176q-8 0-8-8v-60q0-8 8-8z".to_string(),
            width: None,
            height: None,
        }),
        IconType::Edit => Some(SvgIcon {
            view_box: "0 0 1024 1024".to_string(),
            path: "M257.7 752c2 0 4-.2 6-.5L431.9 722c2-.4 3.9-1.3 5.3-2.8l423.9-423.9a9.96 9.96 0 0 0 0-14.1L694.9 114.9c-1.9-1.9-4.4-2.9-7.1-2.9s-5.2 1-7.1 2.9L256.8 538.8c-1.5 1.5-2.4 3.3-2.8 5.3l-29.5 168.2a33.5 33.5 0 0 0 9.4 29.8c6.6 6.4 14.9 9.9 23.8 9.9zm67.4-174.4L687.8 215l73.3 73.3-362.7 362.6-88.9 15.7 15.6-89zM880 836H144c-17.7 0-32 14.3-32 32v36c0 4.4 3.6 8 8 8h784c4.4 0 8-3.6 8-8v-36c0-17.7-14.3-32-32-32z".to_string(),
            width: None,
            height: None,
        }),
        IconType::Delete => Some(SvgIcon {
            view_box: "0 0 1024 1024".to_string(),
            path: "M360 184h-8c4.4 0 8-3.6 8-8v8h304v-8c0 4.4 3.6 8 8 8h-8v72h72v-80c0-35.3-28.7-64-64-64H352c-35.3 0-64 28.7-64 64v80h72v-72zm504 72H160c-17.7 0-32 14.3-32 32v32c0 4.4 3.6 8 8 8h60.4l24.7 523c1.6 34.1 29.8 61 63.9 61h454c34.1 0 62.3-26.9 63.9-61l24.7-523H888c4.4 0 8-3.6 8-8v-32c0-17.7-14.3-32-32-32zM731.3 840H292.7l-24.2-512h487l-24.2 512z".to_string(),
            width: None,
            height: None,
        }),
        IconType::Download => Some(SvgIcon {
            view_box: "0 0 1024 1024".to_string(),
            path: "M505.7 661a8 8 0 0 0 12.6 0l112-141.7c4.1-5.2.4-12.9-6.3-12.9h-74.1V168c0-4.4-3.6-8-8-8h-60c-4.4 0-8 3.6-8 8v338.3H400c-6.7 0-10.4 7.7-6.3 12.9l112 141.8zM878 626h-60c-4.4 0-8 3.6-8 8v154H214V634c0-4.4-3.6-8-8-8h-60c-4.4 0-8 3.6-8 8v198c0 17.7 14.3 32 32 32h684c17.7 0 32-14.3 32-32V634c0-4.4-3.6-8-8-8z".to_string(),
            width: None,
            height: None,
        }),
        IconType::Upload => Some(SvgIcon {
            view_box: "0 0 1024 1024".to_string(),
            path: "M400 317.7h73.9V656c0 4.4 3.6 8 8 8h60c4.4 0 8-3.6 8-8V317.7H624c6.7 0 10.4-7.7 6.3-12.9L518.3 163a8 8 0 0 0-12.6 0l-112 141.7c-4.1 5.3-.4 13 6.3 13zM878 626h-60c-4.4 0-8 3.6-8 8v154H214V634c0-4.4-3.6-8-8-8h-60c-4.4 0-8 3.6-8 8v198c0 17.7 14.3 32 32 32h684c17.7 0 32-14.3 32-32V634c0-4.4-3.6-8-8-8z".to_string(),
            width: None,
            height: None,
        }),
        IconType::Loading => Some(SvgIcon {
            view_box: "0 0 1024 1024".to_string(),
            path: "M988 548c-19.9 0-36-16.1-36-36 0-59.4-11.6-117-34.6-171.3a440.45 440.45 0 0 0-94.3-139.9 437.71 437.71 0 0 0-139.9-94.3C637 83.6 579.4 72 520 72s-117 11.6-171.3 34.6a440.45 440.45 0 0 0-139.9 94.3 437.71 437.71 0 0 0-94.3 139.9C91.6 395 80 452.6 80 512s11.6 117 34.6 171.3a440.45 440.45 0 0 0 94.3 139.9 437.71 437.71 0 0 0 139.9 94.3C395 940.4 452.6 952 512 952c59.4 0 117-11.6 171.3-34.6a440.45 440.45 0 0 0 139.9-94.3 437.71 437.71 0 0 0 94.3-139.9C940.4 629 952 571.4 952 512c0-19.9 16.1-36 36-36s36 16.1 36 36c0 256.1-207.9 464-464 464S48 768.1 48 512 255.9 48 512 48s464 207.9 464 464c0 19.9-16.1 36-36 36z".to_string(),
            width: None,
            height: None,
        }),
        IconType::ArrowDown => Some(SvgIcon {
            view_box: "0 0 1024 1024".to_string(),
            path: "M869 487.8L491.2 159.9c-2.9-2.5-6.6-3.9-10.5-3.9h-88.5c-7.4 0-10.8 9.2-5.2 14l350.2 304H152c-4.4 0-8 3.6-8 8v60c0 4.4 3.6 8 8 8h585.1L386.9 854c-5.6 4.9-2.2 14 5.2 14h91.5c1.9 0 3.8-.7 5.2-2L869 536.2a31.96 31.96 0 0 0 0-48.4z".to_string(),
            width: None,
            height: None,
        }),
        IconType::ArrowUp => Some(SvgIcon {
            view_box: "0 0 1024 1024".to_string(),
            path: "M868 545.5L536.1 163a31.96 31.96 0 0 0-48.3 0L155 545.5a7.97 7.97 0 0 0 6 13.2h81c4.6 0 9-2 12.1-5.5L474 300.9V864c0 4.4 3.6 8 8 8h60c4.4 0 8-3.6 8-8V300.9l218.9 252.3c3 3.5 7.4 5.5 12.1 5.5h81c6.8 0 10.3-8 6-13.2z".to_string(),
            width: None,
            height: None,
        }),
        IconType::ArrowLeft => Some(SvgIcon {
            view_box: "0 0 1024 1024".to_string(),
            path: "M872 474H286.9l350.2-304c5.6-4.9 2.2-14-5.2-14h-88.5c-3.9 0-7.6 1.4-10.5 3.9L155 487.8a31.96 31.96 0 0 0 0 48.4L535.1 866c1.5 1.3 3.3 2 5.2 2h91.5c7.4 0 10.8-9.2 5.2-14L286.9 550H872c4.4 0 8-3.6 8-8v-60c0-4.4-3.6-8-8-8z".to_string(),
            width: None,
            height: None,
        }),
        IconType::ArrowRight => Some(SvgIcon {
            view_box: "0 0 1024 1024".to_string(),
            path: "M869 487.8L491.2 159.9c-2.9-2.5-6.6-3.9-10.5-3.9h-88.5c-7.4 0-10.8 9.2-5.2 14l350.2 304H152c-4.4 0-8 3.6-8 8v60c0 4.4 3.6 8 8 8h585.1L386.9 854c-5.6 4.9-2.2 14 5.2 14h91.5c1.9 0 3.8-.7 5.2-2L869 536.2a31.96 31.96 0 0 0 0-48.4z".to_string(),
            width: None,
            height: None,
        }),
        IconType::CaretDown => Some(SvgIcon {
            view_box: "0 0 1024 1024".to_string(),
            path: "M840.4 300H183.6c-19.7 0-30.7 20.8-18.5 35l328.4 380.8c9.4 10.9 27.5 10.9 37 0L858.9 335c12.2-14.2 1.2-35-18.5-35z".to_string(),
            width: None,
            height: None,
        }),
        IconType::CaretUp => Some(SvgIcon {
            view_box: "0 0 1024 1024".to_string(),
            path: "M858.9 689L530.5 308.2c-9.4-10.9-27.5-10.9-37 0L165.1 689c-12.2 14.2-1.2 35 18.5 35h656.8c19.7 0 30.7-20.8 18.5-35z".to_string(),
            width: None,
            height: None,
        }),
        IconType::CaretLeft => Some(SvgIcon {
            view_box: "0 0 1024 1024".to_string(),
            path: "M724 218.3V141c0-6.7-7.7-10.4-12.9-6.3L260.3 486.8c-16.4 12.8-16.4 37.5 0 50.3l450.8 352.1c5.3 4.1 12.9 0.4 12.9-6.3v-77.3c0-4.9-2.3-9.6-6.1-12.6l-360-281 360-281.1c3.8-3 6.1-7.7 6.1-12.6z".to_string(),
            width: None,
            height: None,
        }),
        IconType::CaretRight => Some(SvgIcon {
            view_box: "0 0 1024 1024".to_string(),
            path: "M765.7 486.8L314.9 134.7c-5.3-4.1-12.9-0.4-12.9 6.3v77.3c0 4.9 2.3 9.6 6.1 12.6l360 281-360 281.1c-3.9 3-6.1 7.7-6.1 12.6V883c0 6.7 7.7 10.4 12.9 6.3l450.8-352.1c16.4-12.8 16.4-37.6 0-50.4z".to_string(),
            width: None,
            height: None,
        }),

        IconType::Question => Some(SvgIcon {
            view_box: "0 0 1024 1024".to_string(),
            path: "M512 64C264.6 64 64 264.6 64 512s200.6 448 448 448 448-200.6 448-448S759.4 64 512 64zm0 820c-205.4 0-372-166.6-372-372s166.6-372 372-372 372 166.6 372 372-166.6 372-372 372z M623.6 316.7C593.6 290.4 554 276 512 276s-81.6 14.5-111.6 40.7C369.2 344 352 380.7 352 420v7.6c0 6.4 2.5 12.5 7 17l99.7 99.7c5.3 5.3 13.8 5.3 19.1 0l99.7-99.7c4.5-4.5 7-10.6 7-17V420c0-39.3-17.2-76-48.4-103.3zM472 732h80c4.4 0 8-3.6 8-8V540c0-4.4-3.6-8-8-8h-80c-4.4 0-8 3.6-8 8v184c0 4.4 3.6 8 8 8z".to_string(),
            width: None,
            height: None,
        }),
        IconType::Info => Some(SvgIcon {
            view_box: "0 0 1024 1024".to_string(),
            path: "M512 64C264.6 64 64 264.6 64 512s200.6 448 448 448 448-200.6 448-448S759.4 64 512 64zm32 664c0 4.4-3.6 8-8 8h-48c-4.4 0-8-3.6-8-8V456c0-4.4 3.6-8 8-8h48c4.4 0 8 3.6 8 8v272zm-32-344a48.01 48.01 0 0 1 0-96 48.01 48.01 0 0 1 0 96z".to_string(),
            width: None,
            height: None,
        }),
        IconType::Exclamation => Some(SvgIcon {
            view_box: "0 0 1024 1024".to_string(),
            path: "M512 64C264.6 64 64 264.6 64 512s200.6 448 448 448 448-200.6 448-448S759.4 64 512 64zm-32 232c0-4.4 3.6-8 8-8h48c4.4 0 8 3.6 8 8v272c0 4.4-3.6 8-8 8h-48c-4.4 0-8-3.6-8-8V296zm32 440a48.01 48.01 0 0 1 0-96 48.01 48.01 0 0 1 0 96z".to_string(),
            width: None,
            height: None,
        }),
        IconType::Copy => Some(SvgIcon {
            view_box: "0 0 1024 1024".to_string(),
            path: "M832 64H296c-4.4 0-8 3.6-8 8v56c0 4.4 3.6 8 8 8h496v688c0 4.4 3.6 8 8 8h56c4.4 0 8-3.6 8-8V96c0-17.7-14.3-32-32-32zM704 192H192c-17.7 0-32 14.3-32 32v530.7c0 8.5 3.4 16.6 9.4 22.6l173.3 173.3c2.2 2.2 4.7 4 7.4 5.5v1.9h4.2c3.5 1.3 7.2 2 11 2H704c17.7 0 32-14.3 32-32V224c0-17.7-14.3-32-32-32zM350 856.2L263.9 770H350v86.2zM664 888H414V746c0-22.1-17.9-40-40-40H232V264h432v624z".to_string(),
            width: None,
            height: None,
        }),
        IconType::Save => Some(SvgIcon {
            view_box: "0 0 1024 1024".to_string(),
            path: "M893.3 293.3L730.7 130.7c-12.1-12.1-28.2-18.7-45.3-18.7H144c-17.7 0-32 14.3-32 32v736c0 17.7 14.3 32 32 32h736c17.7 0 32-14.3 32-32V338.5c0-17-6.7-33.2-18.7-45.2zM384 176h256v112H384V176zm128 554c-79.5 0-144-64.5-144-144s64.5-144 144-144 144 64.5 144 144-64.5 144-144 144zm0-216c-39.7 0-72 32.3-72 72s32.3 72 72 72 72-32.3 72-72-32.3-72-72-72z".to_string(),
            width: None,
            height: None,
        }),
        IconType::Menu => Some(SvgIcon {
            view_box: "0 0 1024 1024".to_string(),
            path: "M904 160H120c-4.4 0-8 3.6-8 8v64c0 4.4 3.6 8 8 8h784c4.4 0 8-3.6 8-8v-64c0-4.4-3.6-8-8-8zm0 624H120c-4.4 0-8 3.6-8 8v64c0 4.4 3.6 8 8 8h784c4.4 0 8-3.6 8-8v-64c0-4.4-3.6-8-8-8zm0-312H120c-4.4 0-8 3.6-8 8v64c0 4.4 3.6 8 8 8h784c4.4 0 8-3.6 8-8v-64c0-4.4-3.6-8-8-8z".to_string(),
            width: None,
            height: None,
        }),
        IconType::More => Some(SvgIcon {
            view_box: "0 0 1024 1024".to_string(),
            path: "M456 231a56 56 0 1 0 112 0 56 56 0 1 0-112 0zm0 280a56 56 0 1 0 112 0 56 56 0 1 0-112 0zm0 280a56 56 0 1 0 112 0 56 56 0 1 0-112 0z".to_string(),
            width: None,
            height: None,
        }),
        IconType::Custom(data) => Some(SvgIcon {
            view_box: "0 0 1024 1024".to_string(),
            path: data.to_string(),
            width: None,
            height: None,
        }),
        _ => Some(SvgIcon {
            view_box: "0 0 1024 1024".to_string(),
            path: "M456 231a56 56 0 1 0 112 0 56 56 0 1 0-112 0zm0 280a56 56 0 1 0 112 0 56 56 0 1 0-112 0zm0 280a56 56 0 1 0 112 0 56 56 0 1 0-112 0z".to_string(),
            width: None,
            height: None,
        })

    }
}

/// 解析SVG字符串
pub fn parse_svg_string(svg_content: &str) -> Result<SvgIcon, String> {
    // 简单的SVG解析实现
    // 在实际项目中，可能需要使用更完善的XML解析库

    let view_box =
        extract_attribute(svg_content, "viewBox").unwrap_or_else(|| "0 0 24 24".to_string());

    let width = extract_attribute(svg_content, "width");
    let height = extract_attribute(svg_content, "height");

    // 提取path元素
    let path =
        extract_path_data(svg_content).ok_or_else(|| "No path data found in SVG".to_string())?;

    Ok(SvgIcon {
        view_box,
        path,
        width,
        height,
    })
}

/// 从SVG字符串中提取属性值
fn extract_attribute(svg_content: &str, attr_name: &str) -> Option<String> {
    let pattern = format!(
        r#"{}="([^"]*)""|{}='([^']*)'"|{}=([^\s>]*)"#,
        attr_name, attr_name, attr_name
    );
    // 这里应该使用正则表达式，但为了简化，使用字符串查找
    if let Some(start) = svg_content.find(&format!("{}=", attr_name)) {
        let start = start + attr_name.len() + 1;
        if let Some(quote_char) = svg_content.chars().nth(start) {
            if quote_char == '"' || quote_char == '\'' {
                let start = start + 1;
                if let Some(end) = svg_content[start..].find(quote_char) {
                    return Some(svg_content[start..start + end].to_string());
                }
            }
        }
    }
    None
}

/// 从SVG字符串中提取path数据
fn extract_path_data(svg_content: &str) -> Option<String> {
    if let Some(path_start) = svg_content.find("<path") {
        if let Some(path_end) = svg_content[path_start..].find(">") {
            let path_tag = &svg_content[path_start..path_start + path_end + 1];
            return extract_attribute(path_tag, "d");
        }
    }
    None
}

/// 生成SVG元素的HTML字符串
pub fn generate_svg_html(
    icon: &SvgIcon,
    theme: &IconTheme,
    two_tone_color: Option<&str>,
) -> String {
    let width = icon.width.as_deref().unwrap_or("1em");
    let height = icon.height.as_deref().unwrap_or("1em");

    match theme {
        IconTheme::TwoTone => {
            let primary_color = two_tone_color.unwrap_or("#1890ff");
            format!(
                r#"<svg width="{}" height="{}" viewBox="{}" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="{}" class="ant-icon-two-tone-primary" fill="{}"/>
                    <path d="{}" class="ant-icon-two-tone-secondary" fill="{}" opacity="0.3"/>
                </svg>"#,
                width, height, icon.view_box, icon.path, primary_color, icon.path, primary_color
            )
        }
        IconTheme::Filled => {
            format!(
                r#"<svg width="{}" height="{}" viewBox="{}" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
                    <path d="{}"/>
                </svg>"#,
                width, height, icon.view_box, icon.path
            )
        }
        IconTheme::Outlined => {
            format!(
                r#"<svg width="{}" height="{}" viewBox="{}" fill="none" stroke="currentColor" xmlns="http://www.w3.org/2000/svg">
                    <path d="{}" stroke-width="1"/>
                </svg>"#,
                width, height, icon.view_box, icon.path
            )
        }
    }
}

/// 验证图标名称是否有效
pub fn validate_icon_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
}

/// 获取图标的缓存键
pub fn get_icon_cache_key(
    name: &str,
    theme: &IconTheme,
    size: Option<&str>,
    color: Option<&str>,
) -> String {
    format!(
        "{}:{}:{}:{}",
        name,
        match theme {
            IconTheme::Outlined => "outlined",
            IconTheme::Filled => "filled",
            IconTheme::TwoTone => "two-tone",
        },
        size.unwrap_or("default"),
        color.unwrap_or("default")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_library() {
        let mut library = IconLibrary::new();
        let icon = SvgIcon {
            view_box: "0 0 24 24".to_string(),
            path: "M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z".to_string(),
            width: None,
            height: None,
        };

        library.register_icon("star".to_string(), icon);
        assert!(library.get_icon("star").is_some());
        assert!(library.get_icon("nonexistent").is_none());
    }

    #[test]
    fn test_validate_icon_name() {
        assert!(validate_icon_name("home"));
        assert!(validate_icon_name("user-circle"));
        assert!(validate_icon_name("icon_name"));
        assert!(!validate_icon_name(""));
        assert!(!validate_icon_name("icon with spaces"));
        assert!(!validate_icon_name("icon@symbol"));
    }

    #[test]
    fn test_get_icon_cache_key() {
        let key = get_icon_cache_key("home", &IconTheme::Outlined, Some("16px"), Some("#ff0000"));
        assert_eq!(key, "home:outlined:16px:#ff0000");

        let key = get_icon_cache_key("user", &IconTheme::Filled, None, None);
        assert_eq!(key, "user:filled:default:default");
    }
}
