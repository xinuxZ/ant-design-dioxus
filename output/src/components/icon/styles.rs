//! Icon 组件样式

use css_in_rust::css;
use std::collections::HashMap;

/// 生成图标基础样式
pub fn generate_icon_styles() -> String {
    css! (
        ".ant-icon {
            display: inline-block;
            color: inherit;
            font-style: normal;
            line-height: 0;
            text-align: center;
            text-transform: none;
            vertical-align: -0.125em;
            text-rendering: optimizeLegibility;
            -webkit-font-smoothing: antialiased;
            -moz-osx-font-smoothing: grayscale;
        }

        .ant-icon > * {
            line-height: 1;
        }

        .ant-icon svg {
            display: inline-block;
        }

        .ant-icon-spin {
            animation: ant-icon-spin 1s infinite linear;
        }

        @keyframes ant-icon-spin {
            "0%" {
                transform: rotate(0deg);
            }
            "100%" {
                transform: rotate(360deg);
            }
        }"
    )
}

/// 生成加载图标SVG
pub fn get_loading_icon_svg() -> String {
    r#"<svg viewBox="0 0 1024 1024" focusable="false" data-icon="loading" width="1em" height="1em" fill="currentColor" aria-hidden="true">
        <path d="M988 548c-19.9 0-36-16.1-36-36 0-59.4-11.6-117-34.6-171.3a440.45 440.45 0 00-94.3-139.9 437.71 437.71 0 00-139.9-94.3C637 83.6 579.4 72 520 72s-117 11.6-171.3 34.6a440.45 440.45 0 00-139.9 94.3 437.71 437.71 0 00-94.3 139.9C91.6 395 80 452.6 80 512s11.6 117 34.6 171.3a440.45 440.45 0 0094.3 139.9 437.71 437.71 0 00139.9 94.3C395 941.4 452.6 953 512 953s117-11.6 171.3-34.6a440.45 440.45 0 00139.9-94.3 437.71 437.71 0 0094.3-139.9C941.4 629 953 571.4 953 512c0-19.9 16.1-36 36-36s36 16.1 36 36c0 256.1-207.9 464-464 464S48 768.1 48 512 255.9 48 512 48s464 207.9 464 464c0 19.9-16.1 36-36 36z"></path>
    </svg>"#.to_string()
}

/// 获取图标SVG内容
pub fn get_icon_svg(icon_type: &str) -> String {
    match icon_type {
        "loading" => get_loading_icon_svg(),
        "check" => r#"<svg viewBox="64 64 896 896" focusable="false" data-icon="check" width="1em" height="1em" fill="currentColor" aria-hidden="true">
            <path d="M912 190h-69.9c-9.8 0-19.1 4.5-25.1 12.2L404.7 724.5 207 474a32 32 0 00-25.1-12.2H112c-6.7 0-10.4 7.7-6.3 12.9l273.9 347c12.8 16.2 37.4 16.2 50.3 0l488.4-618.9c4.1-5.1.4-12.8-6.3-12.8z"></path>
        </svg>"#.to_string(),
        "close" => r#"<svg viewBox="64 64 896 896" focusable="false" data-icon="close" width="1em" height="1em" fill="currentColor" aria-hidden="true">
            <path d="M563.8 512l262.5-312.9c4.4-5.2.7-13.1-6.1-13.1h-79.8c-4.7 0-9.2 2.1-12.3 5.7L511.6 449.8 295.1 191.7c-3.1-3.6-7.6-5.7-12.3-5.7H203c-6.8 0-10.5 7.9-6.1 13.1L459.4 512 196.9 824.9A7.95 7.95 0 00203 838h79.8c4.7 0 9.2-2.1 12.3-5.7l216.5-258.1 216.5 258.1c3.1 3.6 7.6 5.7 12.3 5.7h79.8c6.8 0 10.5-7.9 6.1-13.1L563.8 512z"></path>
        </svg>"#.to_string(),
        "plus" => r#"<svg viewBox="64 64 896 896" focusable="false" data-icon="plus" width="1em" height="1em" fill="currentColor" aria-hidden="true">
            <path d="M482 152h60q8 0 8 8v704q0 8-8 8h-60q-8 0-8-8V160q0-8 8-8z"></path>
            <path d="M176 474h672q8 0 8 8v60q0 8-8 8H176q-8 0-8-8v-60q0-8 8-8z"></path>
        </svg>"#.to_string(),
        "minus" => r#"<svg viewBox="64 64 896 896" focusable="false" data-icon="minus" width="1em" height="1em" fill="currentColor" aria-hidden="true">
            <path d="M176 474h672q8 0 8 8v60q0 8-8 8H176q-8 0-8-8v-60q0-8 8-8z"></path>
        </svg>"#.to_string(),
        _ => get_loading_icon_svg(), // 默认使用加载图标
    }
}

/// 生成图标样式映射
pub fn generate_icon_style_map() -> HashMap<String, String> {
    let mut styles = HashMap::new();

    styles.insert(
        "ant-icon".to_string(),
        "display: inline-block; color: inherit; font-style: normal; line-height: 0; text-align: center; text-transform: none; vertical-align: -0.125em; text-rendering: optimizeLegibility; -webkit-font-smoothing: antialiased; -moz-osx-font-smoothing: grayscale;".to_string(),
    );

    styles.insert(
        "ant-icon-spin".to_string(),
        "animation: ant-icon-spin 1s infinite linear;".to_string(),
    );

    styles
}
