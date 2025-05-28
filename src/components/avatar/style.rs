//! Avatar 组件样式

use dioxus::prelude::*;

/// 注入 Avatar 组件样式
pub fn inject_avatar_style() {
    const AVATAR_STYLE: &str = include_str!("./style.css");

    // 在文档头部注入样式
    document::eval(&format!(
        r#"
        if (!document.querySelector('style[data-ant-avatar]')) {{
            const style = document.createElement('style');
            style.setAttribute('data-ant-avatar', 'true');
            style.textContent = `{}`;
            document.head.appendChild(style);
        }}
        "#,
        AVATAR_STYLE
    ));
}

/// Avatar 组件样式 Hook
pub fn use_avatar_style() {
    use_effect(|| {
        inject_avatar_style();
    });
}
