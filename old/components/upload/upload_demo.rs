use crate::components::button::*;
use crate::components::upload::*;
use dioxus::prelude::*;

#[component]
pub fn UploadDemo() -> Element {
    // 基础上传状态
    let mut basic_file_list = use_signal(|| Vec::<UploadFile>::new());

    // 拖拽上传状态
    let mut drag_file_list = use_signal(|| Vec::<UploadFile>::new());

    // 图片上传状态
    let mut picture_file_list = use_signal(|| {
        vec![
            UploadFile {
                uid: "1".to_string(),
                name: "image1.png".to_string(),
                status: UploadStatus::Done,
                url: Some(
                    "https://zos.alipayobjects.com/rmsportal/jkjgkEfvpUPiUjhGlpQ.png".to_string(),
                ),
                thumb_url: Some(
                    "https://zos.alipayobjects.com/rmsportal/jkjgkEfvpUPiUjhGlpQ.png".to_string(),
                ),
                size: Some(1024),
                file_type: Some("image/png".to_string()),
                percent: Some(100),
                response: None,
                error: None,
            },
            UploadFile {
                uid: "2".to_string(),
                name: "image2.png".to_string(),
                status: UploadStatus::Done,
                url: Some("https://zos.alipayobjects.com/rmsportal/qqaSttnOsxjlpQ.png".to_string()),
                thumb_url: Some(
                    "https://zos.alipayobjects.com/rmsportal/qqaSttnOsxjlpQ.png".to_string(),
                ),
                size: Some(2048),
                file_type: Some("image/png".to_string()),
                percent: Some(100),
                response: None,
                error: None,
            },
        ]
    });

    // 图片卡片上传状态
    let mut picture_card_file_list = use_signal(|| {
        vec![UploadFile {
            uid: "1".to_string(),
            name: "image1.png".to_string(),
            status: UploadStatus::Done,
            url: Some(
                "https://zos.alipayobjects.com/rmsportal/jkjgkEfvpUPiUjhGlpQ.png".to_string(),
            ),
            thumb_url: Some(
                "https://zos.alipayobjects.com/rmsportal/jkjgkEfvpUPiUjhGlpQ.png".to_string(),
            ),
            size: Some(1024),
            file_type: Some("image/png".to_string()),
            percent: Some(100),
            response: None,
            error: None,
        }]
    });

    // 禁用状态
    let mut disabled = use_signal(|| false);

    // 多文件上传状态
    let mut multiple_file_list = use_signal(|| Vec::<UploadFile>::new());

    // 自定义上传状态
    let mut custom_file_list = use_signal(|| Vec::<UploadFile>::new());

    // 基础上传回调
    let basic_on_change = move |info: UploadChangeParam| {
        basic_file_list.set(info.file_list);

        match info.file.status {
            UploadStatus::Done => {
                web_sys::console::log_1(&format!("文件 {} 上传成功", info.file.name).into());
            }
            UploadStatus::Error => {
                web_sys::console::log_1(&format!("文件 {} 上传失败", info.file.name).into());
            }
            _ => {}
        }
    };

    // 拖拽上传回调
    let drag_on_change = move |info: UploadChangeParam| {
        drag_file_list.set(info.file_list);
    };

    // 图片上传回调
    let picture_on_change = move |info: UploadChangeParam| {
        picture_file_list.set(info.file_list);
    };

    // 图片卡片上传回调
    let picture_card_on_change = move |info: UploadChangeParam| {
        picture_card_file_list.set(info.file_list);
    };

    // 多文件上传回调
    let multiple_on_change = move |info: UploadChangeParam| {
        multiple_file_list.set(info.file_list);
    };

    // 自定义上传回调
    let custom_on_change = move |info: UploadChangeParam| {
        custom_file_list.set(info.file_list);
    };

    // 上传前验证
    let before_upload = move |file: &UploadFile| -> bool {
        let is_jpg_or_png = file
            .file_type
            .as_ref()
            .map(|t| t == "image/jpeg" || t == "image/png")
            .unwrap_or(false);

        if !is_jpg_or_png {
            web_sys::console::log_1(&"只能上传 JPG/PNG 格式的图片!".into());
            return false;
        }

        let is_lt_2m = file.size.unwrap_or(0) < 2 * 1024 * 1024;
        if !is_lt_2m {
            web_sys::console::log_1(&"图片大小必须小于 2MB!".into());
            return false;
        }

        true
    };

    // 自定义上传函数
    let custom_request = move |options: UploadRequestOption| {
        // 模拟上传过程
        spawn(async move {
            // 模拟上传延迟
            gloo_timers::future::TimeoutFuture::new(2000).await;

            // 模拟上传成功
            if let Some(on_success) = options.on_success {
                on_success(serde_json::json!({
                    "url": "https://www.mocky.io/v2/5cc8019d300000980a055e76",
                    "status": "done"
                }));
            }
        });
    };

    rsx! {
        div { class: "upload-demo",
            style { {include_str!("./style.css")} }

            h2 { "Upload 上传组件演示" }

            // 基础上传
            div { class: "demo-section",
                h3 { "基础上传" }
                p { "点击或拖拽文件到此区域进行上传" }

                Upload {
                    action: "https://www.mocky.io/v2/5cc8019d300000980a055e76",
                    file_list: basic_file_list(),
                    on_change: basic_on_change,

                    Button {
                        icon: "📁",
                        "点击上传"
                    }
                }
            }

            // 拖拽上传
            div { class: "demo-section",
                h3 { "拖拽上传" }
                p { "支持拖拽文件到上传区域" }

                Upload {
                    action: "https://www.mocky.io/v2/5cc8019d300000980a055e76",
                    file_list: drag_file_list(),
                    on_change: drag_on_change,
                    drag: true,
                    multiple: true,

                    div { class: "ant-upload-drag-container",
                        div { class: "ant-upload-drag-icon", "📁" }
                        div { class: "ant-upload-text", "点击或拖拽文件到此区域上传" }
                        div { class: "ant-upload-hint", "支持单个或批量上传。严禁上传公司数据或其他带有版权的文件" }
                    }
                }
            }

            // 图片上传
            div { class: "demo-section",
                h3 { "图片上传" }
                p { "支持图片预览和缩略图显示" }

                Upload {
                    action: "https://www.mocky.io/v2/5cc8019d300000980a055e76",
                    list_type: UploadListType::Picture,
                    file_list: picture_file_list(),
                    on_change: picture_on_change,
                    before_upload: before_upload,

                    Button {
                        icon: "🖼️",
                        "上传图片"
                    }
                }
            }

            // 图片卡片上传
            div { class: "demo-section",
                h3 { "图片卡片上传" }
                p { "用户可以上传图片并在列表中显示缩略图" }

                Upload {
                    action: "https://www.mocky.io/v2/5cc8019d300000980a055e76",
                    list_type: UploadListType::PictureCard,
                    file_list: picture_card_file_list(),
                    on_change: picture_card_on_change,
                    before_upload: before_upload,

                    if picture_card_file_list().len() < 3 {
                        div { class: "ant-upload-select",
                            div { class: "ant-upload-drag-icon", "+" }
                            div { class: "ant-upload-text", "上传" }
                        }
                    }
                }
            }

            // 禁用状态
            div { class: "demo-section",
                h3 { "禁用状态" }
                p { "禁用上传功能" }

                div { style: "margin-bottom: 16px;",
                    Button {
                        onclick: move |_| disabled.set(!disabled()),
                        if disabled() { "启用" } else { "禁用" }
                    }
                }

                Upload {
                    action: "https://www.mocky.io/v2/5cc8019d300000980a055e76",
                    disabled: disabled(),

                    Button {
                        icon: "📁",
                        disabled: disabled(),
                        "点击上传"
                    }
                }
            }

            // 多文件上传
            div { class: "demo-section",
                h3 { "多文件上传" }
                p { "支持同时选择多个文件进行上传" }

                Upload {
                    action: "https://www.mocky.io/v2/5cc8019d300000980a055e76",
                    multiple: true,
                    file_list: multiple_file_list(),
                    on_change: multiple_on_change,

                    Button {
                        icon: "📁",
                        "选择多个文件"
                    }
                }
            }

            // 自定义上传
            div { class: "demo-section",
                h3 { "自定义上传" }
                p { "使用自定义上传函数处理文件上传" }

                Upload {
                    custom_request: custom_request,
                    file_list: custom_file_list(),
                    on_change: custom_on_change,

                    Button {
                        icon: "⚙️",
                        "自定义上传"
                    }
                }
            }

            // 文件类型限制
            div { class: "demo-section",
                h3 { "文件类型限制" }
                p { "限制只能上传特定类型的文件" }

                Upload {
                    action: "https://www.mocky.io/v2/5cc8019d300000980a055e76",
                    accept: ".jpg,.jpeg,.png,.gif",
                    before_upload: before_upload,

                    Button {
                        icon: "🖼️",
                        "只能上传图片"
                    }
                }
            }

            // 手动上传
            div { class: "demo-section",
                h3 { "手动上传" }
                p { "选择文件后，需要手动触发上传" }

                Upload {
                    action: "https://www.mocky.io/v2/5cc8019d300000980a055e76",
                    before_upload: move |_| false, // 阻止自动上传

                    Button {
                        icon: "📁",
                        "选择文件"
                    }
                }

                div { style: "margin-top: 16px;",
                    Button {
                        button_type: ButtonType::Primary,
                        "开始上传"
                    }
                }
            }
        }
    }
}

// 演示样式
const DEMO_STYLE: &str = r#"
.upload-demo {
    padding: 24px;
    max-width: 1200px;
    margin: 0 auto;
}

.demo-section {
    margin-bottom: 48px;
    padding: 24px;
    border: 1px solid #f0f0f0;
    border-radius: 8px;
    background: #fafafa;
}

.demo-section h3 {
    margin-top: 0;
    margin-bottom: 8px;
    color: #1677ff;
    font-size: 16px;
    font-weight: 600;
}

.demo-section p {
    margin-bottom: 16px;
    color: #666;
    font-size: 14px;
}

.ant-upload-select {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    width: 104px;
    height: 104px;
    margin: 0 8px 8px 0;
    background: #fafafa;
    border: 1px dashed #d9d9d9;
    border-radius: 8px;
    cursor: pointer;
    transition: border-color 0.3s;
}

.ant-upload-select:hover {
    border-color: #1677ff;
}

.ant-upload-select .ant-upload-drag-icon {
    font-size: 24px;
    color: #999;
    margin-bottom: 4px;
}

.ant-upload-select .ant-upload-text {
    font-size: 12px;
    color: #666;
}
"#;
