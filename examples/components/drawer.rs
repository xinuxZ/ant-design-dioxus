//! Drawer 抽屉组件演示
//!
//! 屏幕边缘滑出的浮层面板。

use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// Drawer 演示页面
#[component]
pub fn DrawerDemo() -> Element {
    rsx! {
        div {
            class: "drawer-demo",
            h1 { "Drawer 抽屉" }
            p { "屏幕边缘滑出的浮层面板。" }

            div {
                style: "display: flex; flex-direction: column; gap: 24px;",

                // 基本用法
                BasicDrawer {}

                // 自定义位置
                PlacementDrawer {}

                // 多层抽屉
                MultiLevelDrawer {}

                // 预设宽度
                SizeDrawer {}

                // 额外操作
                ExtraDrawer {}

                // 信息预览抽屉
                InfoDrawer {}
            }
        }
    }
}

/// 基本抽屉
#[component]
fn BasicDrawer() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        div {
            h3 { "基本用法" }
            p { "基础抽屉，点击触发按钮抽屉从右滑出，点击遮罩区关闭。" }

            Button {
                button_type: ButtonType::Primary,
                onclick: move |_| open.set(true),
                "打开抽屉"
            }

            Drawer {
                title: "基本抽屉",
                placement: DrawerPlacement::Right,
                open: open(),
                on_close: move |_| open.set(false),

                p { "这是抽屉的内容。" }
                p { "这是抽屉的内容。" }
                p { "这是抽屉的内容。" }
            }
        }
    }
}

/// 自定义位置
#[component]
fn PlacementDrawer() -> Element {
    let mut open = use_signal(|| false);
    let mut placement = use_signal(|| DrawerPlacement::Right);

    rsx! {
        div {
            h3 { "自定义位置" }
            p { "自定义位置，点击触发按钮抽屉从相应的位置滑出。" }

            div {
                style: "display: flex; gap: 8px; margin-bottom: 16px;",
                Button {
                    onclick: move |_| {
                        placement.set(DrawerPlacement::Top);
                        open.set(true);
                    },
                    "顶部抽屉"
                }
                Button {
                    onclick: move |_| {
                        placement.set(DrawerPlacement::Right);
                        open.set(true);
                    },
                    "右侧抽屉"
                }
                Button {
                    onclick: move |_| {
                        placement.set(DrawerPlacement::Bottom);
                        open.set(true);
                    },
                    "底部抽屉"
                }
                Button {
                    onclick: move |_| {
                        placement.set(DrawerPlacement::Left);
                        open.set(true);
                    },
                    "左侧抽屉"
                }
            }

            Drawer {
                title: "自定义位置抽屉",
                placement: placement(),
                open: open(),
                on_close: move |_| open.set(false),

                p { "这是一个 {placement():?} 位置的抽屉。" }
            }
        }
    }
}

/// 多层抽屉
#[component]
fn MultiLevelDrawer() -> Element {
    let mut open = use_signal(|| false);
    let mut child_open = use_signal(|| false);

    rsx! {
        div {
            h3 { "多层抽屉" }
            p { "在抽屉内打开新的抽屉，用以解决多分支任务的复杂状况。" }

            Button {
                button_type: ButtonType::Primary,
                onclick: move |_| open.set(true),
                "打开抽屉"
            }

            Drawer {
                title: "多层抽屉",
                placement: DrawerPlacement::Right,
                open: open(),
                on_close: move |_| open.set(false),

                Button {
                    button_type: ButtonType::Primary,
                    onclick: move |_| child_open.set(true),
                    "打开子抽屉"
                }

                Drawer {
                    title: "子抽屉",
                    placement: DrawerPlacement::Right,
                    open: child_open(),
                    on_close: move |_| child_open.set(false),

                    p { "这是子抽屉的内容。" }
                }
            }
        }
    }
}

/// 预设宽度
#[component]
fn SizeDrawer() -> Element {
    let mut open = use_signal(|| false);
    let mut size = use_signal(|| DrawerSize::Default);

    rsx! {
        div {
            h3 { "预设宽度" }
            p { "抽屉的默认宽度为 378px，另外还提供一个大号抽屉 736px。" }

            div {
                style: "display: flex; gap: 8px; margin-bottom: 16px;",
                Button {
                    onclick: move |_| {
                        size.set(DrawerSize::Default);
                        open.set(true);
                    },
                    "默认尺寸抽屉"
                }
                Button {
                    onclick: move |_| {
                        size.set(DrawerSize::Large);
                        open.set(true);
                    },
                    "大尺寸抽屉"
                }
            }

            Drawer {
                title: "预设宽度抽屉",
                placement: DrawerPlacement::Right,
                drawer_size: size(),
                open: open(),
                on_close: move |_| open.set(false),

                p { "这是一个 {size():?} 尺寸的抽屉。" }
            }
        }
    }
}

/// 额外操作
#[component]
fn ExtraDrawer() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        div {
            h3 { "额外操作" }
            p { "在抽屉右上角添加额外的操作按钮。" }

            Button {
                button_type: ButtonType::Primary,
                onclick: move |_| open.set(true),
                "打开抽屉"
            }

            Drawer {
                title: "额外操作抽屉",
                placement: DrawerPlacement::Right,
                open: open(),
                on_close: move |_| open.set(false),
                extra: rsx! {
                    div {
                        style: "display: flex; gap: 8px; margin-bottom: 16px;",
                        Button {
                            onclick: move |_| open.set(false),
                            "取消"
                        }
                        Button {
                            button_type: ButtonType::Primary,
                            onclick: move |_| open.set(false),
                            "确定"
                        }
                    }
                },

                p { "这是抽屉的内容。" }
                p { "注意右上角的额外操作按钮。" }
            }
        }
    }
}

/// 信息预览抽屉
#[component]
fn InfoDrawer() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        div {
            h3 { "信息预览抽屉" }
            p { "需要快速预览对象概要时使用，点击遮罩区关闭。" }

            Button {
                button_type: ButtonType::Primary,
                onclick: move |_| open.set(true),
                "查看详情"
            }

            Drawer {
                title: "用户信息",
                placement: DrawerPlacement::Right,
                open: open(),
                on_close: move |_| open.set(false),

                div {
                    style: "padding: 16px 0;",

                    div {
                        style: "margin-bottom: 16px;",
                        strong { "姓名：" }
                        span { "张三" }
                    }

                    div {
                        style: "margin-bottom: 16px;",
                        strong { "年龄：" }
                        span { "28" }
                    }

                    div {
                        style: "margin-bottom: 16px;",
                        strong { "职位：" }
                        span { "前端工程师" }
                    }

                    div {
                        style: "margin-bottom: 16px;",
                        strong { "邮箱：" }
                        span { "zhangsan@example.com" }
                    }

                    div {
                        style: "margin-bottom: 16px;",
                        strong { "电话：" }
                        span { "138-0013-8000" }
                    }

                    div {
                        strong { "地址：" }
                        span { "北京市朝阳区某某街道" }
                    }
                }
            }
        }
    }
}
