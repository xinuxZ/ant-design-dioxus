#![allow(non_snake_case)]
//!
//! 展示 Upload 组件的各种用法和样式

use crate::common::*;
use ant_design_dioxus::prelude::*;
use dioxus::prelude::*;

/// Upload 组件演示
#[component]
pub fn UploadDemo() -> Element {
    let mut file_list = use_signal(|| {
        vec![
            UploadFile {
                uid: "-1".to_string(),
                name: "image.png".to_string(),
                status: UploadStatus::Done,
                url: Some(
                    "https://zos.alipayobjects.com/rmsportal/jkjgkEfvpUPiUjhimbqI.jpg".to_string(),
                ),
                thumb_url: None,
                percent: None,
                size: None,
                file_type: None,
                response: None,
                error: None,
            },
            UploadFile {
                uid: "-2".to_string(),
                name: "image.png".to_string(),
                status: UploadStatus::Done,
                url: Some(
                    "https://zos.alipayobjects.com/rmsportal/taLLVXItOcnSrlc.jpg".to_string(),
                ),
                thumb_url: None,
                percent: None,
                size: None,
                file_type: None,
                response: None,
                error: None,
            },
            UploadFile {
                uid: "-3".to_string(),
                name: "image.png".to_string(),
                status: UploadStatus::Error,
                url: None,
                thumb_url: None,
                percent: None,
                size: None,
                file_type: None,
                response: None,
                error: None,
            },
            UploadFile {
                uid: "-4".to_string(),
                name: "image.png".to_string(),
                status: UploadStatus::Done,
                url: Some(
                    "https://zos.alipayobjects.com/rmsportal/BeeJkLWHEEnbUDL.jpg".to_string(),
                ),
                thumb_url: None,
                percent: None,
                size: None,
                file_type: None,
                response: None,
                error: None,
            },
            UploadFile {
                uid: "-xxx".to_string(),
                name: "image.png".to_string(),
                status: UploadStatus::Done,
                url: Some(
                    "https://zos.alipayobjects.com/rmsportal/BeeJkLWHEEnbUDL.jpg".to_string(),
                ),
                percent: Some(90.0),
                thumb_url: None,
                size: None,
                file_type: None,
                response: None,
                error: None,
            },
        ]
    });

    let mut image_file_list = use_signal(|| {
        vec![
            UploadFile {
                uid: "-1".to_string(),
                name: "image.png".to_string(),
                status: UploadStatus::Done,
                url: Some(
                    "https://zos.alipayobjects.com/rmsportal/jkjgkEfvpUPiUjhimbqI.jpg".to_string(),
                ),
                thumb_url: None,
                percent: None,
                size: None,
                file_type: None,
                response: None,
                error: None,
            },
            UploadFile {
                uid: "-2".to_string(),
                name: "image.png".to_string(),
                status: UploadStatus::Done,
                url: Some(
                    "https://zos.alipayobjects.com/rmsportal/taLLVXItOcnSrlc.jpg".to_string(),
                ),
                thumb_url: None,
                percent: None,
                size: None,
                file_type: None,
                response: None,
                error: None,
            },
        ]
    });

    let mut disabled = use_signal(|| false);
    let mut loading = use_signal(|| false);

    rsx! {
        div {
            class: "component-showcase",

            h1 {
                style: "margin: 0 0 16px 0; color: #262626; font-size: 28px;",
                "Upload 上传"
            }

            p {
                style: "margin: 0 0 24px 0; color: #666; font-size: 14px;",
                "文件选择上传和拖拽上传控件。"
            }

            // 点击上传
            DemoSection {
                title: "点击上传",
                description: "经典款式，用户点击按钮弹出文件选择框。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Upload {
                        action: "https://www.mocky.io/v2/5cc8019d300000980a055e76",
                        on_change: move |info| {
                            let mut new_file_list = info.file_list;

                            // 1. Limit the number of uploaded files
                            // Only to show two recent uploaded files, and old ones will be replaced by the new
                            new_file_list = new_file_list.into_iter().rev().take(2).rev().collect();

                            // 2. Read from response and show file link
                            new_file_list = new_file_list.into_iter().map(|mut file| {
                                if file.response.is_some() {
                                    // Component will show file.url as link
                                    file.url = Some("https://www.google.com".to_string());
                                }
                                file
                            }).collect();

                            file_list.set(new_file_list);
                        },
                        file_list: file_list(),

                        Button {
                            icon: Icon { icon_type: "upload-outlined".to_string() },
                            "Click to Upload"
                        }
                    }
                }
            }

            // 默认已上传的文件
            DemoSection {
                title: "默认已上传的文件",
                description: "使用 defaultFileList 设置已上传的内容。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Upload {
                        action: "https://www.mocky.io/v2/5cc8019d300000980a055e76",
                        default_file_list: vec![
                            UploadFile {
                                uid: "1".to_string(),
                                name: "xxx.png".to_string(),
                                status: UploadStatus::Done,
                                url: Some("http://www.baidu.com/xxx.png".to_string()),
                            },
                            UploadFile {
                                uid: "2".to_string(),
                                name: "yyy.png".to_string(),
                                status: UploadStatus::Done,
                                url: Some("http://www.baidu.com/yyy.png".to_string()),
                            },
                            UploadFile {
                                uid: "3".to_string(),
                                name: "zzz.png".to_string(),
                                status: UploadStatus::Error,
                                url: None,
                            },
                        ],

                        Button {
                            icon: Icon { icon_type: "upload-outlined".to_string() },
                            "Upload"
                        }
                    }
                }
            }

            // 图片列表样式
            DemoSection {
                title: "图片列表样式",
                description: "上传文件为图片，可展示本地缩略图。IE8/9 不支持浏览器本地缩略图展示（Ref），可以写 thumbUrl 属性来代替。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Upload {
                        action: "https://www.mocky.io/v2/5cc8019d300000980a055e76",
                        list_type: UploadListType::Picture,
                        default_file_list: vec![
                            UploadFile {
                                uid: "-1".to_string(),
                                name: "image.png".to_string(),
                                status: UploadStatus::Done,
                                url: Some("https://zos.alipayobjects.com/rmsportal/jkjgkEfvpUPiUjhimbqI.jpg".to_string()),
                            },
                            UploadFile {
                                uid: "-2".to_string(),
                                name: "image.png".to_string(),
                                status: UploadStatus::Done,
                                url: Some("https://zos.alipayobjects.com/rmsportal/taLLVXItOcnSrlc.jpg".to_string()),
                            },
                        ],

                        Button {
                            icon: Icon { icon_type: "upload-outlined".to_string() },
                            "Upload"
                        }
                    }
                }
            }

            // 完全控制的上传列表
            DemoSection {
                title: "完全控制的上传列表",
                description: "使用 fileList 对列表进行完全控制，可以实现各种自定义功能，以下演示三种情况：1) 上传列表数量的限制。2) 读取远程路径并显示链接。3) 按照服务器返回信息筛选成功上传的文件。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Upload {
                        action: "https://www.mocky.io/v2/5cc8019d300000980a055e76",
                        on_change: move |info| {
                            let mut new_file_list = info.file_list;

                            // 1. Limit the number of uploaded files
                            // Only to show two recent uploaded files, and old ones will be replaced by the new
                            new_file_list = new_file_list.into_iter().rev().take(2).rev().collect();

                            // 2. Read from response and show file link
                            new_file_list = new_file_list.into_iter().map(|mut file| {
                                if file.response.is_some() {
                                    // Component will show file.url as link
                                    file.url = Some("https://www.google.com".to_string());
                                }
                                file
                            }).collect();

                            file_list.set(new_file_list);
                        },
                        file_list: file_list(),

                        Button {
                            icon: Icon { icon_type: "upload-outlined".to_string() },
                            "Upload"
                        }
                    }
                }
            }

            // 拖拽上传
            DemoSection {
                title: "拖拽上传",
                description: "把文件拖入指定区域，完成上传，同样支持点击上传。设置 multiple 后，在 IE10+ 可以一次上传多个文件。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Upload::Dragger {
                        name: "file",
                        multiple: true,
                        action: "https://www.mocky.io/v2/5cc8019d300000980a055e76",
                        on_change: move |info| {
                            let status = &info.file.status;
                            if *status != UploadStatus::Uploading {
                                println!("{:?} {:?}", info.file, info.file_list);
                            }
                            if *status == UploadStatus::Done {
                                message::success(format!("{} file uploaded successfully.", info.file.name));
                            } else if *status == UploadStatus::Error {
                                message::error(format!("{} file upload failed.", info.file.name));
                            }
                        },
                        on_drop: move |e| {
                            println!("Dropped files: {:?}", e.data_transfer.files);
                        },

                        p {
                            class: "ant-upload-drag-icon",
                            Icon {
                                icon_type: "inbox-outlined".to_string()
                            }
                        }
                        p {
                            class: "ant-upload-text",
                            "Click or drag file to this area to upload"
                        }
                        p {
                            class: "ant-upload-hint",
                            "Support for a single or bulk upload. Strictly prohibit from uploading company data or other band files"
                        }
                    }
                }
            }

            // 照片墙
            DemoSection {
                title: "照片墙",
                description: "用户可以上传图片并在列表中显示缩略图。当上传照片数到达限制后，上传按钮消失。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Upload {
                        action: "https://www.mocky.io/v2/5cc8019d300000980a055e76",
                        list_type: UploadListType::PictureCard,
                        file_list: image_file_list(),
                        on_change: move |info| {
                            image_file_list.set(info.file_list);
                        },
                        on_preview: move |file| {
                            // Handle preview
                        },

                        if image_file_list().len() >= 8 {
                            rsx! { div {} }
                        } else {
                            rsx! {
                                div {
                                    Icon {
                                        icon_type: "plus-outlined".to_string()
                                    }
                                    div {
                                        style: "margin-top: 8px;",
                                        "Upload"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // 自定义上传按钮
            DemoSection {
                title: "自定义上传按钮",
                description: "使用 showUploadList 设置列表交互功能。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Upload {
                        action: "https://www.mocky.io/v2/5cc8019d300000980a055e76",
                        show_upload_list: UploadListProps {
                            show_preview_icon: true,
                            show_remove_icon: true,
                            show_download_icon: true,
                            remove_icon: Some(rsx! {
                                Icon {
                                    icon_type: "delete-outlined".to_string(),
                                    onclick: move |_| {
                                        println!("custom removeIcon");
                                    }
                                }
                            }),
                            download_icon: Some(rsx! {
                                Icon {
                                    icon_type: "download-outlined".to_string(),
                                    onclick: move |_| {
                                        println!("custom downloadIcon");
                                    }
                                }
                            }),
                        },

                        Button {
                            icon: Icon { icon_type: "upload-outlined".to_string() },
                            "Upload"
                        }
                    }
                }
            }

            // 手动上传
            DemoSection {
                title: "手动上传",
                description: "beforeUpload 返回 false 后，手动上传文件。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Upload {
                        file_list: file_list(),
                        on_change: move |info| {
                            file_list.set(info.file_list);
                        },
                        before_upload: move |file| {
                            false
                        },

                        Button {
                            icon: Icon { icon_type: "upload-outlined".to_string() },
                            "Select File"
                        }
                    }

                    Button {
                        button_type: ButtonType::Primary,
                        onclick: move |_| {
                            // Handle manual upload
                            loading.set(true);
                            // Simulate upload
                            // After upload complete, set loading to false
                        },
                        disabled: file_list().is_empty(),
                        loading: loading(),
                        style: "margin-top: 16px;",
                        if loading() {
                            "Uploading"
                        } else {
                            "Start Upload"
                        }
                    }
                }
            }

            // 上传前裁切图片
            DemoSection {
                title: "上传前裁切图片",
                description: "配合 antd 的 Upload 和 Modal 组件，实现一个在上传前裁切图片的功能。",

                div {
                    style: "padding: 24px; background: #fafafa; border-radius: 6px;",

                    Upload {
                        action: "https://www.mocky.io/v2/5cc8019d300000980a055e76",
                        list_type: UploadListType::PictureCard,
                        file_list: image_file_list(),
                        on_change: move |info| {
                            image_file_list.set(info.file_list);
                        },
                        before_upload: move |file| {
                            let is_jpg_or_png = file.file_type == "image/jpeg" || file.file_type == "image/png";
                            if !is_jpg_or_png {
                                message::error("You can only upload JPG/PNG file!");
                            }
                            let is_lt_2m = file.size / 1024 / 1024 < 2;
                            if !is_lt_2m {
                                message::error("Image must smaller than 2MB!");
                            }
                            is_jpg_or_png && is_lt_2m
                        },

                        if image_file_list().len() >= 1 {
                            rsx! { div {} }
                        } else {
                            rsx! {
                                div {
                                    Icon {
                                        icon_type: "plus-outlined".to_string()
                                    }
                                    div {
                                        style: "margin-top: 8px;",
                                        "Upload"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // API 文档
            ApiDocumentation {
                component_name: "Upload",
                props: vec![
                    PropDoc {
                        name: "accept".to_string(),
                        prop_type: "String".to_string(),
                        default: "-".to_string(),
                        description: "接受上传的文件类型, 详见 input accept Attribute".to_string(),
                    },
                    PropDoc {
                        name: "action".to_string(),
                        prop_type: "String | Function".to_string(),
                        default: "-".to_string(),
                        description: "上传的地址".to_string(),
                    },
                    PropDoc {
                        name: "before_upload".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "上传文件之前的钩子，参数为上传的文件，若返回 false 则停止上传。支持返回一个 Promise 对象，Promise 对象 reject 时则停止上传，resolve 时开始上传（ resolve 传入 File 或 Blob 对象则上传 resolve 传入对象）。注意：IE9 不支持该方法".to_string(),
                    },
                    PropDoc {
                        name: "custom_request".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "通过覆盖默认的上传行为，可以自定义自己的上传实现".to_string(),
                    },
                    PropDoc {
                        name: "data".to_string(),
                        prop_type: "Object | Function".to_string(),
                        default: "-".to_string(),
                        description: "上传所需额外参数或返回上传额外参数的方法".to_string(),
                    },
                    PropDoc {
                        name: "default_file_list".to_string(),
                        prop_type: "Vec<UploadFile>".to_string(),
                        default: "-".to_string(),
                        description: "默认已经上传的文件列表".to_string(),
                    },
                    PropDoc {
                        name: "directory".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "支持上传文件夹（caniuse）".to_string(),
                    },
                    PropDoc {
                        name: "disabled".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "是否禁用".to_string(),
                    },
                    PropDoc {
                        name: "file_list".to_string(),
                        prop_type: "Vec<UploadFile>".to_string(),
                        default: "-".to_string(),
                        description: "已经上传的文件列表（受控），使用此参数时，如果遇到 onChange 只调用一次的问题，请参考 #2423".to_string(),
                    },
                    PropDoc {
                        name: "headers".to_string(),
                        prop_type: "Object".to_string(),
                        default: "-".to_string(),
                        description: "设置上传的请求头部，IE10 以上有效".to_string(),
                    },
                    PropDoc {
                        name: "icon_render".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "自定义显示 icon".to_string(),
                    },
                    PropDoc {
                        name: "is_image_url".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "自定义缩略图是否使用 <img /> 标签进行显示".to_string(),
                    },
                    PropDoc {
                        name: "item_render".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "自定义上传列表项".to_string(),
                    },
                    PropDoc {
                        name: "list_type".to_string(),
                        prop_type: "String".to_string(),
                        default: "'text'".to_string(),
                        description: "上传列表的内建样式，支持三种基本样式 text, picture 和 picture-card".to_string(),
                    },
                    PropDoc {
                        name: "max_count".to_string(),
                        prop_type: "Number".to_string(),
                        default: "-".to_string(),
                        description: "限制上传数量。当为 1 时，始终用最新上传的文件代替当前文件".to_string(),
                    },
                    PropDoc {
                        name: "method".to_string(),
                        prop_type: "String".to_string(),
                        default: "'post'".to_string(),
                        description: "上传请求的 http method".to_string(),
                    },
                    PropDoc {
                        name: "multiple".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "是否支持多选文件，ie10+ 支持。开启后按住 ctrl 可选择多个文件".to_string(),
                    },
                    PropDoc {
                        name: "name".to_string(),
                        prop_type: "String".to_string(),
                        default: "'file'".to_string(),
                        description: "发到后台的文件参数名".to_string(),
                    },
                    PropDoc {
                        name: "open_file_dialog_on_click".to_string(),
                        prop_type: "bool".to_string(),
                        default: "true".to_string(),
                        description: "点击打开文件对话框".to_string(),
                    },
                    PropDoc {
                        name: "preview_file".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "自定义文件预览逻辑".to_string(),
                    },
                    PropDoc {
                        name: "progress".to_string(),
                        prop_type: "Object".to_string(),
                        default: "{ strokeWidth: 2, showInfo: false }".to_string(),
                        description: "自定义进度条样式".to_string(),
                    },
                    PropDoc {
                        name: "show_upload_list".to_string(),
                        prop_type: "bool | Object".to_string(),
                        default: "true".to_string(),
                        description: "是否展示文件列表, 可设为一个对象，用于单独设定 showPreviewIcon, showRemoveIcon, showDownloadIcon, removeIcon 和 downloadIcon".to_string(),
                    },
                    PropDoc {
                        name: "with_credentials".to_string(),
                        prop_type: "bool".to_string(),
                        default: "false".to_string(),
                        description: "上传请求时是否携带 cookie".to_string(),
                    },
                    PropDoc {
                        name: "on_change".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "上传文件改变时的状态，详见 onChange".to_string(),
                    },
                    PropDoc {
                        name: "on_download".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "点击下载文件时的回调，如果没有指定，则默认跳转到文件 url 对应的标签页".to_string(),
                    },
                    PropDoc {
                        name: "on_drop".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "当文件被拖入上传区域时执行的回调功能".to_string(),
                    },
                    PropDoc {
                        name: "on_preview".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "点击文件链接或预览图标时的回调".to_string(),
                    },
                    PropDoc {
                        name: "on_remove".to_string(),
                        prop_type: "Function".to_string(),
                        default: "-".to_string(),
                        description: "点击移除文件时的回调，返回值为 false 时不移除。支持返回一个 Promise 对象，Promise 对象 resolve(false) 或 reject 时不移除".to_string(),
                    },
                ]
            }

            // Upload.Dragger
            ApiDocumentation {
                component_name: "Upload.Dragger",
                props: vec![
                    PropDoc {
                        name: "height".to_string(),
                        prop_type: "Number".to_string(),
                        default: "-".to_string(),
                        description: "拖拽区域高度，单位为 px".to_string(),
                    },
                ]
            }
        }
    }
}
