//! Space 组件嵌套使用示例
//! 
//! 本示例展示了如何使用 Space.Compact 组件的嵌套功能，
//! 类似于 Ant Design 官方的 compact-nested 示例。

use dioxus::prelude::*;
use ant_design_dioxus::{
    Space, SpaceCompact, SpaceDirection, CompactSize,
    Button, Input, InputNumber, Select, TimePicker
};

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div {
            style: "padding: 20px;",
            
            h2 { "Space.Compact 嵌套使用示例" }
            
            // 第一个嵌套示例：多层嵌套的紧凑组件
            div {
                style: "margin-bottom: 20px;",
                h3 { "多层嵌套示例" }
                
                SpaceCompact {
                    block: true,
                    
                    // 第一层嵌套
                    SpaceCompact {
                        // 第二层嵌套 - 搜索输入组合
                        SpaceCompact {
                            Input {
                                style: "width: 90px;",
                                placeholder: "输入...",
                            }
                            Button {
                                "🔍"
                            }
                        }
                        
                        // 第二层嵌套 - 数字输入和选择器组合
                        SpaceCompact {
                            InputNumber {
                                default_value: 12,
                            }
                            Select {
                                default_value: "选项1",
                                style: "width: 80px;",
                                "选项1"
                            }
                        }
                    }
                    
                    // 分隔按钮
                    Button {
                        r#type: "primary",
                        "分隔符"
                    }
                    
                    // 第一层嵌套 - 右侧组合
                    SpaceCompact {
                        // 第二层嵌套 - 搜索和提交组合
                        SpaceCompact {
                            Input {
                                style: "width: 110px;",
                                placeholder: "搜索",
                            }
                            Button {
                                r#type: "primary",
                                "提交"
                            }
                        }
                        
                        // 第二层嵌套 - 输入和复制组合
                        SpaceCompact {
                            Input {
                                default_value: "mysite",
                            }
                            Button {
                                "📋"
                            }
                        }
                    }
                }
            }
            
            // 第二个嵌套示例：时间选择器和级联选择器
            div {
                style: "margin-bottom: 20px;",
                h3 { "时间和级联选择器嵌套" }
                
                SpaceCompact {
                    block: true,
                    
                    SpaceCompact {
                        TimePicker {}
                        Button {
                            r#type: "primary",
                            "提交"
                        }
                    }
                    
                    SpaceCompact {
                        Select {
                            placeholder: "选择地址",
                            style: "width: 200px;",
                            "浙江省 - 杭州市 - 西湖区"
                        }
                        Button {
                            r#type: "primary",
                            "确认"
                        }
                    }
                }
            }
            
            // 第三个示例：垂直方向的嵌套
            div {
                h3 { "垂直方向嵌套" }
                
                Space {
                    direction: SpaceDirection::Vertical,
                    
                    SpaceCompact {
                        Button { "按钮1" }
                        Button { "按钮2" }
                        Button { "按钮3" }
                    }
                    
                    SpaceCompact {
                        Input { placeholder: "输入框1" }
                        Input { placeholder: "输入框2" }
                    }
                    
                    SpaceCompact {
                        Select {
                            placeholder: "选择器1",
                            style: "width: 120px;",
                            "选项A"
                        }
                        Select {
                            placeholder: "选择器2",
                            style: "width: 120px;",
                            "选项B"
                        }
                    }
                }
            }
        }
    }
}