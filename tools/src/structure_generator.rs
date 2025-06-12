//! 组件结构生成器
//!
//! 根据组件复杂度自动生成新的目录结构

use clap::{Arg, Command};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use log;
use toml;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComponentLevel {
    Level1, // 基础组件
    Level2, // 复杂组件
    Level3, // 模块化组件
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentInfo {
    pub name: String,
    pub level: ComponentLevel,
    pub has_subcomponents: bool,
    pub has_variants: bool,
    pub has_complex_state: bool,
    pub estimated_files: usize,
}

#[derive(Debug, Clone)]
pub struct DirectoryStructure {
    pub component_name: String,
    pub level: ComponentLevel,
    pub files: Vec<FileTemplate>,
    pub directories: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FileTemplate {
    pub path: String,
    pub content: String,
    pub is_required: bool,
}

pub struct StructureGenerator {
    components_dir: PathBuf,
    output_dir: PathBuf,
}

impl StructureGenerator {
    pub fn new(components_dir: PathBuf, output_dir: PathBuf) -> Self {
        Self {
            components_dir,
            output_dir,
        }
    }

    /// 分析组件复杂度
    fn analyze_component(
        &self,
        component_path: &Path,
    ) -> Result<ComponentInfo, Box<dyn std::error::Error>> {
        let component_name = component_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let mut has_subcomponents = false;
        let mut has_variants = false;
        let mut has_complex_state = false;
        let mut file_count = 0;

        // 检查 mod.rs 文件内容
        let mod_file = component_path.join("mod.rs");
        if mod_file.exists() {
            let content = fs::read_to_string(&mod_file)?;

            // 检查是否有子组件
            if content.contains("pub mod ") || content.contains("mod ") {
                has_subcomponents = true;
            }

            // 检查是否有变体
            if content.contains("enum")
                && (content.contains("Type")
                    || content.contains("Size")
                    || content.contains("Variant"))
            {
                has_variants = true;
            }

            // 检查是否有复杂状态
            if content.contains("use_state")
                || content.contains("use_effect")
                || content.contains("use_reducer")
            {
                has_complex_state = true;
            }
        }

        // 统计文件数量
        for entry in WalkDir::new(component_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                file_count += 1;
            }
        }

        // 根据特征确定组件等级
        let level = self.determine_component_level(
            &component_name,
            has_subcomponents,
            has_variants,
            has_complex_state,
            file_count,
        );

        Ok(ComponentInfo {
            name: component_name,
            level,
            has_subcomponents,
            has_variants,
            has_complex_state,
            estimated_files: file_count,
        })
    }

    /// 确定组件等级
    fn determine_component_level(
        &self,
        component_name: &str,
        has_subcomponents: bool,
        has_variants: bool,
        has_complex_state: bool,
        file_count: usize,
    ) -> ComponentLevel {
        // Level 3: 模块化组件（复杂组件，通常有多个子组件）
        let level3_components = [
            "form",
            "table",
            "tree",
            "transfer",
            "upload",
            "calendar",
            "date_picker",
            "time_picker",
            "cascader",
            "auto_complete",
            "mentions",
            "layout",
            "menu",
            "steps",
            "tour",
            "config_provider",
        ];

        if level3_components.contains(&component_name.to_lowercase().as_str())
            || (has_subcomponents && has_complex_state && file_count > 5)
        {
            return ComponentLevel::Level3;
        }

        // Level 2: 复杂组件（有变体或复杂交互）
        let level2_components = [
            "button",
            "input",
            "select",
            "checkbox",
            "radio",
            "switch",
            "slider",
            "rate",
            "progress",
            "spin",
            "pagination",
            "tabs",
            "collapse",
            "carousel",
            "anchor",
            "affix",
            "back_top",
            "breadcrumb",
            "dropdown",
            "menu",
            "steps",
            "alert",
            "drawer",
            "modal",
            "notification",
            "popconfirm",
            "popover",
            "tooltip",
            "card",
            "list",
            "descriptions",
            "empty",
            "statistic",
            "timeline",
            "tag",
            "badge",
            "avatar",
            "comment",
            "image",
            "segmented",
            "float_button",
        ];

        if level2_components.contains(&component_name.to_lowercase().as_str())
            || (has_variants && (has_complex_state || file_count > 3))
        {
            return ComponentLevel::Level2;
        }

        // Level 1: 基础组件（简单组件）
        ComponentLevel::Level1
    }

    /// 生成目录结构
    fn generate_structure(&self, component_info: &ComponentInfo) -> DirectoryStructure {
        match component_info.level {
            ComponentLevel::Level1 => self.generate_level1_structure(component_info),
            ComponentLevel::Level2 => self.generate_level2_structure(component_info),
            ComponentLevel::Level3 => self.generate_level3_structure(component_info),
        }
    }

    /// 生成 Level 1 结构（基础组件）
    fn generate_level1_structure(&self, component_info: &ComponentInfo) -> DirectoryStructure {
        let component_name = &component_info.name;

        let files = vec![
            FileTemplate {
                path: "mod.rs".to_string(),
                content: self.generate_mod_rs_content(component_info),
                is_required: true,
            },
            FileTemplate {
                path: "styles.rs".to_string(),
                content: self.generate_styles_rs_content(component_info),
                is_required: true,
            },
        ];

        DirectoryStructure {
            component_name: component_name.clone(),
            level: ComponentLevel::Level1,
            files,
            directories: vec![],
        }
    }

    /// 生成 Level 2 结构（复杂组件）
    fn generate_level2_structure(&self, component_info: &ComponentInfo) -> DirectoryStructure {
        let component_name = &component_info.name;

        let mut files = vec![
            FileTemplate {
                path: "mod.rs".to_string(),
                content: self.generate_mod_rs_content(component_info),
                is_required: true,
            },
            FileTemplate {
                path: "component.rs".to_string(),
                content: self.generate_component_rs_content(component_info),
                is_required: true,
            },
            FileTemplate {
                path: "types.rs".to_string(),
                content: self.generate_types_rs_content(component_info),
                is_required: true,
            },
            FileTemplate {
                path: "styles/mod.rs".to_string(),
                content: self.generate_styles_mod_rs_content(component_info),
                is_required: true,
            },
            FileTemplate {
                path: "styles/base.rs".to_string(),
                content: self.generate_base_styles_content(component_info),
                is_required: true,
            },
        ];

        if component_info.has_variants {
            files.push(FileTemplate {
                path: "styles/variants.rs".to_string(),
                content: self.generate_variants_styles_content(component_info),
                is_required: true,
            });
        }

        DirectoryStructure {
            component_name: component_name.clone(),
            level: ComponentLevel::Level2,
            files,
            directories: vec!["styles".to_string()],
        }
    }

    /// 生成 Level 3 结构（模块化组件）
    fn generate_level3_structure(&self, component_info: &ComponentInfo) -> DirectoryStructure {
        let component_name = &component_info.name;

        let mut files = vec![
            FileTemplate {
                path: "mod.rs".to_string(),
                content: self.generate_mod_rs_content(component_info),
                is_required: true,
            },
            FileTemplate {
                path: "component.rs".to_string(),
                content: self.generate_component_rs_content(component_info),
                is_required: true,
            },
            FileTemplate {
                path: "types.rs".to_string(),
                content: self.generate_types_rs_content(component_info),
                is_required: true,
            },
            FileTemplate {
                path: "hooks/mod.rs".to_string(),
                content: self.generate_hooks_mod_rs_content(component_info),
                is_required: true,
            },
            FileTemplate {
                path: "utils/mod.rs".to_string(),
                content: self.generate_utils_mod_rs_content(component_info),
                is_required: true,
            },
            FileTemplate {
                path: "styles/mod.rs".to_string(),
                content: self.generate_styles_mod_rs_content(component_info),
                is_required: true,
            },
            FileTemplate {
                path: "styles/base.rs".to_string(),
                content: self.generate_base_styles_content(component_info),
                is_required: true,
            },
        ];

        if component_info.has_variants {
            files.push(FileTemplate {
                path: "styles/variants.rs".to_string(),
                content: self.generate_variants_styles_content(component_info),
                is_required: true,
            });
        }

        DirectoryStructure {
            component_name: component_name.clone(),
            level: ComponentLevel::Level3,
            files,
            directories: vec![
                "hooks".to_string(),
                "utils".to_string(),
                "styles".to_string(),
            ],
        }
    }

    /// 生成 mod.rs 内容
    fn generate_mod_rs_content(&self, component_info: &ComponentInfo) -> String {
        let component_name = &component_info.name;
        let pascal_name = self.to_pascal_case(component_name);

        let mut content = format!(
            "//! {} 组件\n//!\n//! 此文件由结构生成器自动生成\n\n",
            pascal_name
        );

        match component_info.level {
            ComponentLevel::Level1 => {
                content.push_str(&format!(
                    "mod styles;\n\npub use styles::*;\n\n/// {} 组件\npub fn {}() -> Element {{\n    todo!(\"实现 {} 组件\")\n}}\n",
                    pascal_name, pascal_name, pascal_name
                ));
            }
            ComponentLevel::Level2 | ComponentLevel::Level3 => {
                content.push_str("mod component;\n");
                content.push_str("mod types;\n");
                content.push_str("mod styles;\n");

                if matches!(component_info.level, ComponentLevel::Level3) {
                    content.push_str("mod hooks;\n");
                    content.push_str("mod utils;\n");
                }

                content.push_str("\npub use component::*;\n");
                content.push_str("pub use types::*;\n");
                content.push_str("pub use styles::*;\n");

                if matches!(component_info.level, ComponentLevel::Level3) {
                    content.push_str("pub use hooks::*;\n");
                    content.push_str("pub use utils::*;\n");
                }
            }
        }

        content
    }

    /// 生成 component.rs 内容
    fn generate_component_rs_content(&self, component_info: &ComponentInfo) -> String {
        let component_name = &component_info.name;
        let pascal_name = self.to_pascal_case(component_name);

        format!(
            "//! {} 组件实现\n\nuse dioxus::prelude::*;\nuse super::types::*;\nuse super::styles::*;\n\n/// {} 组件属性\n#[derive(Props, Clone, PartialEq)]\npub struct {}Props {{\n    /// 子元素\n    #[props(optional)]\n    pub children: Option<Element>,\n    \n    /// 自定义类名\n    #[props(optional)]\n    pub class: Option<String>,\n    \n    /// 自定义样式\n    #[props(optional)]\n    pub style: Option<String>,\n}}\n\n/// {} 组件\n#[component]\npub fn {}(props: {}Props) -> Element {{\n    let styles = use_memo(move || generate_{}_styles());\n    \n    rsx! {{\n        div {{\n            class: format!(\"{{}} {{}}\", styles().base, props.class.unwrap_or_default()),\n            style: props.style,\n            {{props.children}}\n        }}\n    }}\n}}\n",
            pascal_name, pascal_name, pascal_name, pascal_name, pascal_name, pascal_name, component_name.to_lowercase()
        )
    }

    /// 生成 types.rs 内容
    fn generate_types_rs_content(&self, component_info: &ComponentInfo) -> String {
        let component_name = &component_info.name;
        let pascal_name = self.to_pascal_case(component_name);

        let mut content = format!(
            "//! {} 组件类型定义\n\nuse serde::{{Deserialize, Serialize}};\n\n",
            pascal_name
        );

        if component_info.has_variants {
            content.push_str(&format!(
                "/// {} 尺寸\n#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]\npub enum {}Size {{\n    Small,\n    Middle,\n    Large,\n}}\n\n/// {} 类型\n#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]\npub enum {}Type {{\n    Default,\n    Primary,\n    Secondary,\n}}\n\n",
                pascal_name, pascal_name, pascal_name, pascal_name
            ));
        }

        content
    }

    /// 生成 styles.rs 内容（Level 1）
    fn generate_styles_rs_content(&self, component_info: &ComponentInfo) -> String {
        let component_name = &component_info.name;
        let pascal_name = self.to_pascal_case(component_name);

        format!(
            "//! {} 组件样式\n\nuse css_in_rust::css;\n\n/// {} 样式\n#[derive(Debug, Clone)]\npub struct {}Styles {{\n    pub base: String,\n}}\n\n/// 生成 {} 组件样式\npub fn generate_{}_styles() -> {}Styles {{\n    let base = css!(r#\"\n        /* {} 基础样式 */\n        .{} {{\n            /* 在这里添加样式 */\n        }}\n    \"#);\n    \n    {}Styles {{ base }}\n}}\n",
            pascal_name, pascal_name, pascal_name, pascal_name, component_name.to_lowercase(), pascal_name, pascal_name, component_name.to_lowercase(), pascal_name
        )
    }

    /// 生成 styles/mod.rs 内容
    fn generate_styles_mod_rs_content(&self, component_info: &ComponentInfo) -> String {
        let mut content = "//! 样式模块\n\nmod base;\n\npub use base::*;\n".to_string();

        if component_info.has_variants {
            content.push_str("\nmod variants;\npub use variants::*;\n");
        }

        content
    }

    /// 生成基础样式内容
    fn generate_base_styles_content(&self, component_info: &ComponentInfo) -> String {
        let component_name = &component_info.name;
        let pascal_name = self.to_pascal_case(component_name);

        format!(
            "//! {} 基础样式\n\nuse css_in_rust::css;\n\n/// 生成基础样式\npub fn generate_base_styles() -> String {{\n    css!(r#\"\n        /* {} 基础样式 */\n        .{} {{\n            /* 在这里添加基础样式 */\n        }}\n    \"#)\n}}\n",
            pascal_name, pascal_name, component_name.to_lowercase()
        )
    }

    /// 生成变体样式内容
    fn generate_variants_styles_content(&self, component_info: &ComponentInfo) -> String {
        let component_name = &component_info.name;
        let pascal_name = self.to_pascal_case(component_name);

        format!(
            "//! {} 变体样式\n\nuse css_in_rust::css;\n\n/// 生成尺寸变体样式\npub fn generate_size_styles() -> String {{\n    css!(r#\"\n        /* 尺寸变体 */\n        .{}-small {{ /* 小尺寸样式 */ }}\n        .{}-middle {{ /* 中等尺寸样式 */ }}\n        .{}-large {{ /* 大尺寸样式 */ }}\n    \"#)\n}}\n\n/// 生成类型变体样式\npub fn generate_type_styles() -> String {{\n    css!(r#\"\n        /* 类型变体 */\n        .{}-default {{ /* 默认类型样式 */ }}\n        .{}-primary {{ /* 主要类型样式 */ }}\n        .{}-secondary {{ /* 次要类型样式 */ }}\n    \"#)\n}}\n",
            pascal_name, component_name.to_lowercase(), component_name.to_lowercase(), component_name.to_lowercase(), component_name.to_lowercase(), component_name.to_lowercase(), component_name.to_lowercase()
        )
    }

    /// 生成 hooks/mod.rs 内容
    fn generate_hooks_mod_rs_content(&self, component_info: &ComponentInfo) -> String {
        let component_name = &component_info.name;
        let pascal_name = self.to_pascal_case(component_name);

        format!(
            "//! {} 组件 Hooks\n\nuse dioxus::prelude::*;\n\n/// {} 组件状态 Hook\npub fn use_{}_state() -> Signal<bool> {{\n    use_signal(|| false)\n}}\n",
            pascal_name, pascal_name, component_name.to_lowercase()
        )
    }

    /// 生成 utils/mod.rs 内容
    fn generate_utils_mod_rs_content(&self, component_info: &ComponentInfo) -> String {
        let component_name = &component_info.name;
        let pascal_name = self.to_pascal_case(component_name);

        format!(
            "//! {} 组件工具函数\n\n/// {} 工具函数\npub fn {}_helper() -> String {{\n    \"helper\".to_string()\n}}\n",
            pascal_name, pascal_name, component_name.to_lowercase()
        )
    }

    /// 转换为 PascalCase
    fn to_pascal_case(&self, s: &str) -> String {
        s.split('_')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect()
    }

    /// 创建目录结构
    fn create_structure(
        &self,
        structure: &DirectoryStructure,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let component_dir = self.output_dir.join(&structure.component_name);

        // 创建主目录
        fs::create_dir_all(&component_dir)?;

        // 创建子目录
        for dir in &structure.directories {
            let dir_path = component_dir.join(dir);
            fs::create_dir_all(&dir_path)?;
        }

        // 创建文件
        for file in &structure.files {
            let file_path = component_dir.join(&file.path);

            // 确保父目录存在
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent)?;
            }

            // 写入文件内容
            fs::write(&file_path, &file.content)?;
        }

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let matches = Command::new("组件结构生成器")
        .version("1.0")
        .author("Ant Design Dioxus Team")
        .about("根据组件复杂度生成新的目录结构")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("DIR")
                .help("输入的组件目录")
                .default_value("src/components"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("DIR")
                .help("输出目录")
                .default_value("output/components"),
        )
        .arg(
            Arg::new("component")
                .short('c')
                .long("component")
                .value_name("NAME")
                .help("指定单个组件名称"),
        )
        .arg(
            Arg::new("analyze")
                .short('a')
                .long("analyze")
                .help("仅分析组件，不生成结构")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let input_dir = PathBuf::from(matches.get_one::<String>("input").unwrap());
    let output_dir = PathBuf::from(matches.get_one::<String>("output").unwrap());

    let generator = StructureGenerator::new(input_dir.clone(), output_dir);

    if let Some(component_name) = matches.get_one::<String>("component") {
        // 处理单个组件
        let component_path = input_dir.join(component_name);
        if !component_path.exists() {
            return Err(format!("组件目录不存在: {:?}", component_path).into());
        }

        let component_info = generator.analyze_component(&component_path)?;
        println!("📊 组件分析结果: {:#?}", component_info);

        if !matches.get_flag("analyze") {
            let structure = generator.generate_structure(&component_info);
            generator.create_structure(&structure)?;
            println!("✅ 成功生成组件结构: {}", component_name);
        }
    } else {
        // 批量处理所有组件
        let mut components = Vec::new();

        for entry in fs::read_dir(&input_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let component_info = generator.analyze_component(&path)?;
                components.push(component_info);
            }
        }

        // 按等级分组统计
        let mut level_stats = HashMap::new();
        for component in &components {
            *level_stats
                .entry(format!("{:?}", component.level))
                .or_insert(0) += 1;
        }

        println!("📊 组件分析统计:");
        for (level, count) in &level_stats {
            println!("   {}: {} 个组件", level, count);
        }

        if !matches.get_flag("analyze") {
            println!("\n🚀 开始生成组件结构...");

            for component_info in &components {
                let structure = generator.generate_structure(component_info);
                match generator.create_structure(&structure) {
                    Ok(_) => println!("✅ 成功生成: {}", component_info.name),
                    Err(e) => println!("❌ 生成失败: {} - {}", component_info.name, e),
                }
            }
        }
    }

    Ok(())
}
