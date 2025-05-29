//! Upload 上传组件
//!
//! 文件选择上传和拖拽上传控件。
//!
//! ## 何时使用
//!
//! 上传是将信息（网页、文字、图片、视频等）发送到远程服务器上的过程。
//!
//! - 当需要上传一个或一些文件时。
//! - 当需要展示上传的进度时。
//! - 当需要使用拖拽交互时。
//!
//! ## 示例
//!
//! ```rust
//! use dioxus::prelude::*;
//! use ant_design_dioxus::Upload;
//!
//! #[component]
//! fn App() -> Element {
//!     let mut file_list = use_signal(|| Vec::new());
//!
//!     rsx! {
//!         Upload {
//!             action: "https://www.mocky.io/v2/5cc8019d300000980a055e76",
//!             file_list: file_list(),
//!             onchange: move |info| {
//!                 file_list.set(info.file_list);
//!             },
//!             "点击上传"
//!         }
//!     }
//! }
//! ```

use dioxus::events::FormEvent;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use web_sys::js_sys::eval;

// 引入样式
const STYLE: &str = include_str!("./style.css");

/// 上传文件状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UploadStatus {
    /// 上传中
    Uploading,
    /// 上传完成
    Done,
    /// 上传失败
    Error,
    /// 已移除
    Removed,
}

impl Default for UploadStatus {
    fn default() -> Self {
        Self::Done
    }
}

/// 上传列表类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UploadListType {
    /// 文本列表
    Text,
    /// 图片列表
    Picture,
    /// 图片卡片列表
    PictureCard,
}

impl Default for UploadListType {
    fn default() -> Self {
        Self::Text
    }
}

/// 上传文件信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UploadFile {
    /// 文件唯一标识
    pub uid: String,
    /// 文件名
    pub name: String,
    /// 上传状态
    pub status: UploadStatus,
    /// 文件URL
    pub url: Option<String>,
    /// 缩略图URL
    pub thumb_url: Option<String>,
    /// 上传进度
    pub percent: Option<f64>,
    /// 文件大小
    pub size: Option<u64>,
    /// 文件类型
    pub file_type: Option<String>,
    /// 响应数据
    pub response: Option<String>,
    /// 错误信息
    pub error: Option<String>,
}

impl UploadFile {
    pub fn new(uid: String, name: String) -> Self {
        Self {
            uid,
            name,
            status: UploadStatus::Done,
            url: None,
            thumb_url: None,
            percent: None,
            size: None,
            file_type: None,
            response: None,
            error: None,
        }
    }
}

/// 上传变更信息
#[derive(Debug, Clone, PartialEq)]
pub struct UploadChangeInfo {
    /// 当前操作的文件
    pub file: UploadFile,
    /// 当前文件列表
    pub file_list: Vec<UploadFile>,
    /// 事件类型
    pub event: Option<String>,
}

/// 上传前验证结果
#[derive(Debug, Clone, PartialEq)]
pub enum BeforeUploadResult {
    /// 允许上传
    Allow,
    /// 禁止上传
    Reject,
    /// 忽略该文件
    Ignore,
}

/// Upload 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct UploadProps {
    /// 上传的地址
    #[props(default = None)]
    pub action: Option<String>,

    /// 接受上传的文件类型
    #[props(default = None)]
    pub accept: Option<String>,

    /// 上传文件之前的钩子
    #[props(default = None)]
    pub before_upload: Option<EventHandler<(UploadFile, BeforeUploadResult)>>,

    /// 自定义上传实现
    #[props(default = None)]
    pub custom_request: Option<EventHandler<UploadFile>>,

    /// 上传请求的数据
    #[props(default = None)]
    pub data: Option<HashMap<String, String>>,

    /// 默认已经上传的文件列表
    #[props(default = Vec::new())]
    pub default_file_list: Vec<UploadFile>,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 已经上传的文件列表（受控）
    #[props(default = None)]
    pub file_list: Option<Vec<UploadFile>>,

    /// 上传请求的 header
    #[props(default = None)]
    pub headers: Option<HashMap<String, String>>,

    /// 上传列表的内建样式
    #[props(default = UploadListType::Text)]
    pub list_type: UploadListType,

    /// 限制上传数量
    #[props(default = None)]
    pub max_count: Option<usize>,

    /// 上传请求时文件的参数名
    #[props(default = "file".to_string())]
    pub name: String,

    /// 是否支持多选文件
    #[props(default = false)]
    pub multiple: bool,

    /// 点击预览文件时的回调
    #[props(default = None)]
    pub on_preview: Option<EventHandler<UploadFile>>,

    /// 点击移除文件时的回调
    #[props(default = None)]
    pub on_remove: Option<EventHandler<UploadFile>>,

    /// 上传文件改变时的回调
    #[props(default = None)]
    pub onchange: Option<EventHandler<UploadChangeInfo>>,

    /// 上传进度改变时的回调
    #[props(default = None)]
    pub on_progress: Option<EventHandler<(UploadFile, f64)>>,

    /// 是否展示上传列表
    #[props(default = true)]
    pub show_upload_list: bool,

    /// 发送请求时是否携带 cookie
    #[props(default = false)]
    pub with_credentials: bool,

    /// 是否支持拖拽上传
    #[props(default = false)]
    pub drag: bool,

    /// 自定义类名
    #[props(default = None)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default = None)]
    pub style: Option<String>,

    /// 自定义 id
    #[props(default = None)]
    pub id: Option<String>,

    /// 子元素
    children: Element,
}

/// Upload 上传组件
///
/// ## 属性
///
/// - `action`: 上传的地址
/// - `accept`: 接受上传的文件类型
/// - `before_upload`: 上传文件之前的钩子
/// - `custom_request`: 自定义上传实现
/// - `data`: 上传请求的数据
/// - `default_file_list`: 默认已经上传的文件列表
/// - `disabled`: 是否禁用
/// - `file_list`: 已经上传的文件列表（受控）
/// - `headers`: 上传请求的 header
/// - `list_type`: 上传列表的内建样式
/// - `max_count`: 限制上传数量
/// - `name`: 上传请求时文件的参数名
/// - `multiple`: 是否支持多选文件
/// - `on_preview`: 点击预览文件时的回调
/// - `on_remove`: 点击移除文件时的回调
/// - `onchange`: 上传文件改变时的回调
/// - `on_progress`: 上传进度改变时的回调
/// - `show_upload_list`: 是否展示上传列表
/// - `with_credentials`: 发送请求时是否携带 cookie
/// - `drag`: 是否支持拖拽上传
/// - `class`: 自定义类名
/// - `style`: 自定义样式
/// - `id`: 自定义 id
#[component]
pub fn Upload(props: UploadProps) -> Element {
    // 内部文件列表状态
    let mut internal_file_list = use_signal(|| props.default_file_list.clone());
    let mut drag_over = use_signal(|| false);

    // 使用受控模式还是非受控模式
    let is_controlled = props.file_list.is_some();
    let current_file_list = if is_controlled {
        props.file_list.clone().unwrap_or_default()
    } else {
        internal_file_list()
    };

    // 处理文件选择
    let handle_file_select = {
        let current_file_list = current_file_list.clone();
        move |evt: FormEvent| {
            if let Some(files) = evt.files() {
                let mut new_files = Vec::new();

                // 使用 FileEngine 的 files() 方法获取文件名列表
                let file_names = files.files();
                for file_name in &file_names {
                    // 获取文件大小（异步操作，这里先设为 None）
                    let file_size = None; // TODO: 可以通过 files.file_size(file_name).await 获取

                    let upload_file = UploadFile {
                        uid: format!("{}-{}", js_sys::Date::now() as u64, file_name),
                        name: file_name.clone(),
                        status: UploadStatus::Uploading,
                        url: None,
                        thumb_url: None,
                        percent: Some(0.0),
                        size: file_size,
                        file_type: None, // TODO: 可以从文件扩展名推断
                        response: None,
                        error: None,
                    };

                    // 检查上传前验证
                    if let Some(before_upload) = &props.before_upload {
                        before_upload.call((upload_file.clone(), BeforeUploadResult::Allow));
                    }

                    new_files.push(upload_file);
                }

                // 更新文件列表
                let mut updated_list = current_file_list.clone();
                updated_list.extend(new_files.clone());

                // 检查数量限制
                if let Some(max_count) = props.max_count {
                    if updated_list.len() > max_count {
                        updated_list = updated_list
                            .into_iter()
                            .rev()
                            .take(max_count)
                            .rev()
                            .collect();
                    }
                }

                if !is_controlled {
                    internal_file_list.set(updated_list.clone());
                }

                // 触发变更回调
                if let Some(onchange) = &props.onchange {
                    for file in new_files {
                        onchange.call(UploadChangeInfo {
                            file: file.clone(),
                            file_list: updated_list.clone(),
                            event: Some("add".to_string()),
                        });
                    }
                }
            }
        }
    };

    // 处理文件移除
    let handle_remove = {
        let current_file_list = current_file_list.clone();
        move |file: UploadFile| {
            let mut updated_list = current_file_list.clone();
            updated_list.retain(|f| f.uid != file.uid);

            if !is_controlled {
                internal_file_list.set(updated_list.clone());
            }

            if let Some(on_remove) = &props.on_remove {
                on_remove.call(file.clone());
            }

            if let Some(onchange) = &props.onchange {
                onchange.call(UploadChangeInfo {
                    file,
                    file_list: updated_list,
                    event: Some("remove".to_string()),
                });
            }
        }
    };

    // 处理文件预览
    let handle_preview = {
        let _current_file_list = current_file_list.clone();
        move |file: UploadFile| {
            if let Some(on_preview) = &props.on_preview {
                on_preview.call(file);
            }
        }
    };

    // 处理拖拽事件
    let handle_drag_over = move |evt: DragEvent| {
        evt.prevent_default();
        if props.drag && !props.disabled {
            drag_over.set(true);
        }
    };

    let handle_drag_leave = move |_: DragEvent| {
        if props.drag {
            drag_over.set(false);
        }
    };

    let handle_drop = move |evt: DragEvent| {
        evt.prevent_default();
        if props.drag && !props.disabled {
            drag_over.set(false);
            // 处理拖拽文件
            // 注意：在实际实现中需要处理 DataTransfer 中的文件
        }
    };

    // 构建类名
    let mut class_names = vec!["ant-upload-wrapper"];

    if props.disabled {
        class_names.push("ant-upload-disabled");
    }

    if props.drag {
        class_names.push("ant-upload-drag");
        if drag_over() {
            class_names.push("ant-upload-drag-hover");
        }
    }

    if let Some(custom_class) = &props.class {
        class_names.push(custom_class);
    }

    let class_str = class_names.join(" ");

    rsx! {
        style { {STYLE} }
        div {
            class: "{class_str}",
            style: props.style.as_deref().unwrap_or(""),
            id: props.id.as_deref().unwrap_or(""),

            div {
                class: if props.drag { "ant-upload ant-upload-drag" } else { "ant-upload ant-upload-select" },
                ondragover: handle_drag_over,
                ondragleave: handle_drag_leave,
                ondrop: handle_drop,

                input {
                    r#type: "file",
                    class: "ant-upload-input",
                    accept: props.accept.as_deref().unwrap_or(""),
                    multiple: props.multiple,
                    disabled: props.disabled,
                    onchange: handle_file_select,
                    style: "display: none;",
                }

                div {
                    class: "ant-upload-btn",
                    onclick: move |_| {
                        if !props.disabled {
                            // 触发文件选择
                            eval("document.querySelector('.ant-upload-input').click()");
                        }
                    },

                    if props.drag {
                        div {
                            class: "ant-upload-drag-container",
                            div {
                                class: "ant-upload-drag-icon",
                                "📁"
                            }
                            div {
                                class: "ant-upload-text",
                                "点击或拖拽文件到此区域上传"
                            }
                            div {
                                class: "ant-upload-hint",
                                "支持单个或批量上传"
                            }
                        }
                    } else {
                        {props.children}
                    }
                }
            }

            if props.show_upload_list && !current_file_list.is_empty() {
                UploadList {
                    files: current_file_list,
                    list_type: props.list_type.clone(),
                    on_remove: handle_remove,
                    on_preview: handle_preview,
                }
            }
        }
    }
}

/// 上传列表属性
#[derive(Props, Clone, PartialEq)]
struct UploadListProps {
    files: Vec<UploadFile>,
    list_type: UploadListType,
    on_remove: EventHandler<UploadFile>,
    on_preview: EventHandler<UploadFile>,
}

/// 上传列表组件
#[component]
fn UploadList(props: UploadListProps) -> Element {
    let list_class = match props.list_type {
        UploadListType::Text => "ant-upload-list ant-upload-list-text",
        UploadListType::Picture => "ant-upload-list ant-upload-list-picture",
        UploadListType::PictureCard => "ant-upload-list ant-upload-list-picture-card",
    };

    rsx! {
        div {
            class: "{list_class}",

            for file in props.files {
                UploadListItem {
                    key: "{file.uid}",
                    file: file.clone(),
                    list_type: props.list_type.clone(),
                    on_remove: props.on_remove,
                    on_preview: props.on_preview,
                }
            }
        }
    }
}

/// 上传列表项属性
#[derive(Props, Clone, PartialEq)]
struct UploadListItemProps {
    file: UploadFile,
    list_type: UploadListType,
    on_remove: EventHandler<UploadFile>,
    on_preview: EventHandler<UploadFile>,
}

/// 上传列表项组件
#[component]
fn UploadListItem(props: UploadListItemProps) -> Element {
    let file = props.file.clone();

    let status_class = match file.status {
        UploadStatus::Uploading => "ant-upload-list-item-uploading",
        UploadStatus::Done => "ant-upload-list-item-done",
        UploadStatus::Error => "ant-upload-list-item-error",
        UploadStatus::Removed => "ant-upload-list-item-removed",
    };

    let item_class = format!("ant-upload-list-item {}", status_class);

    rsx! {
        div {
            class: "{item_class}",

            div {
                class: "ant-upload-list-item-info",

                if matches!(props.list_type, UploadListType::Picture | UploadListType::PictureCard) {
                    div {
                        class: "ant-upload-list-item-thumbnail",
                        if let Some(thumb_url) = &file.thumb_url {
                            img {
                                src: "{thumb_url}",
                                alt: "{file.name}",
                            }
                        } else {
                            div {
                                class: "ant-upload-list-item-thumbnail-icon",
                                "📄"
                            }
                        }
                    }
                }

                span {
                    class: "ant-upload-list-item-name",
                    onclick: {
                        let file_clone = file.clone();
                        move |_| props.on_preview.call(file_clone.clone())
                    },
                    "{file.name}"
                }

                if matches!(file.status, UploadStatus::Uploading) {
                    div {
                        class: "ant-upload-list-item-progress",
                        div {
                            class: "ant-progress ant-progress-line ant-progress-status-active",
                            div {
                                class: "ant-progress-outer",
                                div {
                                    class: "ant-progress-inner",
                                    div {
                                        class: "ant-progress-bg",
                                        style: "width: {file.percent.unwrap_or(0.0)}%; height: 2px;",
                                    }
                                }
                            }
                        }
                    }
                }
            }

            div {
                class: "ant-upload-list-item-actions",

                if matches!(file.status, UploadStatus::Done) {
                    button {
                        class: "ant-upload-list-item-action",
                        onclick: {
                            let file_clone = file.clone();
                            move |_| props.on_preview.call(file_clone.clone())
                        },
                        title: "预览",
                        "👁"
                    }
                }

                button {
                    class: "ant-upload-list-item-action",
                    onclick: {
                        let file_clone = file.clone();
                        move |_| props.on_remove.call(file_clone.clone())
                    },
                    title: "删除",
                    "🗑"
                }
            }
        }
    }
}
