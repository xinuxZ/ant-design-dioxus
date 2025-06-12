//! 样式迁移工具
//!
//! 自动将组件的 style.css 文件转换为 CSS-in-Rust 格式

use ant_design_tools_common::*;
use clap::{Arg, Command};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use log;
use serde_json;
use toml;
use colorized::*;

#[derive(Debug, Clone)]
struct CssRule {
    selector: String,
    properties: HashMap<String, String>,
}

#[derive(Debug, Clone)]
struct ComponentStyle {
    component_name: String,
    base_styles: Vec<CssRule>,
    variant_styles: HashMap<String, Vec<CssRule>>,
    state_styles: HashMap<String, Vec<CssRule>>,
}

pub struct StyleMigrator {
    css_content: String,
    component_name: String,
}

impl StyleMigrator {
    pub fn new(css_content: String, component_name: String) -> Self {
        Self {
            css_content,
            component_name,
        }
    }

    /// 解析 CSS 内容
    fn parse_css(&self) -> Result<ComponentStyle, Box<dyn std::error::Error>> {
        let mut component_style = ComponentStyle {
            component_name: self.component_name.clone(),
            base_styles: Vec::new(),
            variant_styles: HashMap::new(),
            state_styles: HashMap::new(),
        };

        // 移除注释
        let css_without_comments = self.remove_comments(&self.css_content);

        // 解析 CSS 规则
        let rules = self.parse_css_rules(&css_without_comments)?;

        // 分类规则
        for rule in rules {
            self.classify_rule(rule, &mut component_style);
        }

        Ok(component_style)
    }

    /// 移除 CSS 注释
    fn remove_comments(&self, css: &str) -> String {
        let comment_regex = Regex::new(r"/\*[\s\S]*?\*/").unwrap();
        comment_regex.replace_all(css, "").to_string()
    }

    /// 解析 CSS 规则
    fn parse_css_rules(&self, css: &str) -> Result<Vec<CssRule>, Box<dyn std::error::Error>> {
        let mut rules = Vec::new();

        // 匹配 CSS 规则的正则表达式
        let rule_regex = Regex::new(r"([^{}]+)\{([^{}]*)\}").unwrap();

        for cap in rule_regex.captures_iter(css) {
            let selector = cap[1].trim().to_string();
            let properties_str = &cap[2];

            let mut properties = HashMap::new();

            // 解析属性
            let prop_regex = Regex::new(r"([^:;]+):\s*([^;]+);").unwrap();
            for prop_cap in prop_regex.captures_iter(properties_str) {
                let property = prop_cap[1].trim().to_string();
                let value = prop_cap[2].trim().to_string();
                properties.insert(property, value);
            }

            if !properties.is_empty() {
                rules.push(CssRule {
                    selector,
                    properties,
                });
            }
        }

        Ok(rules)
    }

    /// 分类 CSS 规则
    fn classify_rule(&self, rule: CssRule, component_style: &mut ComponentStyle) {
        let selector = &rule.selector;

        // 检查是否是状态样式（:hover, :focus, :active, :disabled 等）
        if selector.contains(":hover") {
            component_style
                .state_styles
                .entry("hover".to_string())
                .or_insert_with(Vec::new)
                .push(rule);
        } else if selector.contains(":focus") {
            component_style
                .state_styles
                .entry("focus".to_string())
                .or_insert_with(Vec::new)
                .push(rule);
        } else if selector.contains(":active") {
            component_style
                .state_styles
                .entry("active".to_string())
                .or_insert_with(Vec::new)
                .push(rule);
        } else if selector.contains(":disabled") {
            component_style
                .state_styles
                .entry("disabled".to_string())
                .or_insert_with(Vec::new)
                .push(rule);
        }
        // 检查是否是变体样式（包含 size、type、variant 等关键词）
        else if selector.contains("-large")
            || selector.contains("-small")
            || selector.contains("-middle")
        {
            let variant_key = if selector.contains("-large") {
                "size_large"
            } else if selector.contains("-small") {
                "size_small"
            } else {
                "size_middle"
            };

            component_style
                .variant_styles
                .entry(variant_key.to_string())
                .or_insert_with(Vec::new)
                .push(rule);
        } else if selector.contains("-primary")
            || selector.contains("-default")
            || selector.contains("-dashed")
            || selector.contains("-text")
            || selector.contains("-link")
        {
            let variant_key = if selector.contains("-primary") {
                "type_primary"
            } else if selector.contains("-dashed") {
                "type_dashed"
            } else if selector.contains("-text") {
                "type_text"
            } else if selector.contains("-link") {
                "type_link"
            } else {
                "type_default"
            };

            component_style
                .variant_styles
                .entry(variant_key.to_string())
                .or_insert_with(Vec::new)
                .push(rule);
        }
        // 其他情况归类为基础样式
        else {
            component_style.base_styles.push(rule);
        }
    }

    /// 生成 CSS-in-Rust 代码
    fn generate_rust_code(&self, component_style: &ComponentStyle) -> String {
        let mut code = String::new();

        // 添加文件头部注释
        code.push_str(&format!(
            "//! {} 组件样式\n",
            component_style.component_name
        ));
        code.push_str("//! \n");
        code.push_str("//! 此文件由样式迁移工具自动生成，请勿手动修改\n\n");

        // 添加导入
        code.push_str("use css_in_rust::css;\n\n");

        // 生成样式结构体
        code.push_str(&self.generate_style_structs(component_style));

        // 生成样式生成器
        code.push_str(&self.generate_style_generator(component_style));

        // 生成样式函数
        code.push_str(&self.generate_style_functions(component_style));

        code
    }

    /// 生成样式结构体
    fn generate_style_structs(&self, component_style: &ComponentStyle) -> String {
        let component_name = &component_style.component_name;
        let struct_name = format!("{}Styles", self.to_pascal_case(component_name));

        let mut code = String::new();

        // 主样式结构体
        code.push_str(&format!("/// {} 组件样式\n", component_name));
        code.push_str("#[derive(Debug, Clone)]\n");
        code.push_str(&format!("pub struct {} {{\n", struct_name));
        code.push_str("    pub base: String,\n");

        if !component_style.variant_styles.is_empty() {
            code.push_str("    pub variants: VariantStyles,\n");
        }

        if !component_style.state_styles.is_empty() {
            code.push_str("    pub states: StateStyles,\n");
        }

        code.push_str("}\n\n");

        // 变体样式结构体
        if !component_style.variant_styles.is_empty() {
            code.push_str("/// 变体样式\n");
            code.push_str("#[derive(Debug, Clone)]\n");
            code.push_str("pub struct VariantStyles {\n");

            for variant_key in component_style.variant_styles.keys() {
                code.push_str(&format!("    pub {}: String,\n", variant_key));
            }

            code.push_str("}\n\n");
        }

        // 状态样式结构体
        if !component_style.state_styles.is_empty() {
            code.push_str("/// 状态样式\n");
            code.push_str("#[derive(Debug, Clone)]\n");
            code.push_str("pub struct StateStyles {\n");

            for state_key in component_style.state_styles.keys() {
                code.push_str(&format!("    pub {}: String,\n", state_key));
            }

            code.push_str("}\n\n");
        }

        code
    }

    /// 生成样式生成器
    fn generate_style_generator(&self, component_style: &ComponentStyle) -> String {
        let component_name = &component_style.component_name;
        let generator_name = format!("{}StyleGenerator", self.to_pascal_case(component_name));

        let mut code = String::new();

        code.push_str(&format!("/// {} 样式生成器\n", component_name));
        code.push_str("#[derive(Debug, Clone)]\n");
        code.push_str(&format!("pub struct {} {{\n", generator_name));
        code.push_str("    // 在这里添加样式生成器的字段\n");
        code.push_str("}\n\n");

        code.push_str(&format!("impl {} {{\n", generator_name));
        code.push_str("    pub fn new() -> Self {\n");
        code.push_str("        Self {}\n");
        code.push_str("    }\n\n");

        code.push_str(&format!(
            "    pub fn generate(&self) -> {} {{\n",
            format!("{}Styles", self.to_pascal_case(component_name))
        ));
        code.push_str(&format!(
            "        generate_{}_styles()\n",
            component_name.to_lowercase()
        ));
        code.push_str("    }\n");
        code.push_str("}\n\n");

        code
    }

    /// 生成样式函数
    fn generate_style_functions(&self, component_style: &ComponentStyle) -> String {
        let component_name = &component_style.component_name;
        let function_name = format!("generate_{}_styles", component_name.to_lowercase());
        let struct_name = format!("{}Styles", self.to_pascal_case(component_name));

        let mut code = String::new();

        code.push_str(&format!("/// 生成 {} 组件样式\n", component_name));
        code.push_str(&format!(
            "pub fn {}() -> {} {{\n",
            function_name, struct_name
        ));

        // 生成基础样式
        code.push_str("    let base = css!(r#\"\n");
        for rule in &component_style.base_styles {
            code.push_str(&self.format_css_rule(rule));
        }
        code.push_str("    \"#);\n\n");

        // 生成变体样式
        if !component_style.variant_styles.is_empty() {
            code.push_str("    let variants = VariantStyles {\n");
            for (variant_key, rules) in &component_style.variant_styles {
                code.push_str(&format!("        {}: css!(r#\"\n", variant_key));
                for rule in rules {
                    code.push_str(&self.format_css_rule(rule));
                }
                code.push_str("        \"#),\n");
            }
            code.push_str("    };\n\n");
        }

        // 生成状态样式
        if !component_style.state_styles.is_empty() {
            code.push_str("    let states = StateStyles {\n");
            for (state_key, rules) in &component_style.state_styles {
                code.push_str(&format!("        {}: css!(r#\"\n", state_key));
                for rule in rules {
                    code.push_str(&self.format_css_rule(rule));
                }
                code.push_str("        \"#),\n");
            }
            code.push_str("    };\n\n");
        }

        // 返回样式结构体
        code.push_str(&format!("    {} {{\n", struct_name));
        code.push_str("        base,\n");
        if !component_style.variant_styles.is_empty() {
            code.push_str("        variants,\n");
        }
        if !component_style.state_styles.is_empty() {
            code.push_str("        states,\n");
        }
        code.push_str("    }\n");
        code.push_str("}\n");

        code
    }

    /// 格式化 CSS 规则
    fn format_css_rule(&self, rule: &CssRule) -> String {
        let mut formatted = String::new();

        formatted.push_str(&format!("        {} {{\n", rule.selector));
        for (property, value) in &rule.properties {
            formatted.push_str(&format!("            {}: {};\n", property, value));
        }
        formatted.push_str("        }\n");

        formatted
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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let matches = Command::new("样式迁移工具")
        .version("1.0")
        .author("Ant Design Dioxus Team")
        .about("将 CSS 文件转换为 CSS-in-Rust 格式")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("输入的 CSS 文件路径")
                .required(false),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("输出的 Rust 文件路径")
                .required(false),
        )
        .arg(
            Arg::new("component")
                .short('c')
                .long("component")
                .value_name("NAME")
                .help("组件名称")
                .required(false),
        )
        .arg(
            Arg::new("batch")
                .short('b')
                .long("batch")
                .help("批量处理所有组件")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("components_dir")
                .short('d')
                .long("dir")
                .value_name("DIR")
                .help("组件目录路径")
                .default_value("src/components"),
        )
        .get_matches();

    if matches.get_flag("batch") {
        // 批量处理模式
        let components_dir = matches.get_one::<String>("components_dir").unwrap();
        batch_migrate(components_dir)?;
    } else {
        // 单个文件处理模式
        let input_file = matches
            .get_one::<String>("input")
            .ok_or("请指定输入文件路径")?;
        let output_file = matches
            .get_one::<String>("output")
            .ok_or("请指定输出文件路径")?;
        let component_name = matches
            .get_one::<String>("component")
            .ok_or("请指定组件名称")?;

        migrate_single_file(input_file, output_file, component_name)?;
    }

    Ok(())
}

/// 批量迁移所有组件
fn batch_migrate(components_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 开始批量迁移样式文件...");

    let mut migrated_count = 0;
    let mut failed_count = 0;

    for entry in WalkDir::new(components_dir)
        .min_depth(1)
        .max_depth(2)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        // 查找 style.css 文件
        if path.file_name() == Some(OsStr::new("style.css")) {
            let component_dir = path.parent().unwrap();
            let component_name = component_dir.file_name().unwrap().to_str().unwrap();

            let output_path = component_dir.join("styles.rs");

            match migrate_single_file(
                path.to_str().unwrap(),
                output_path.to_str().unwrap(),
                component_name,
            ) {
                Ok(_) => {
                    println!("✅ 成功迁移: {}", component_name);
                    migrated_count += 1;
                }
                Err(e) => {
                    println!("❌ 迁移失败: {} - {}", component_name, e);
                    failed_count += 1;
                }
            }
        }
    }

    println!("\n📊 迁移完成统计:");
    println!("   ✅ 成功: {} 个组件", migrated_count);
    println!("   ❌ 失败: {} 个组件", failed_count);

    Ok(())
}

/// 迁移单个文件
fn migrate_single_file(
    input_file: &str,
    output_file: &str,
    component_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // 读取 CSS 文件
    let css_content = fs::read_to_string(input_file)
        .map_err(|e| format!("无法读取文件 {}: {}", input_file, e))?;

    // 创建迁移器
    let migrator = StyleMigrator::new(css_content, component_name.to_string());

    // 解析 CSS
    let component_style = migrator.parse_css()?;

    // 生成 Rust 代码
    let rust_code = migrator.generate_rust_code(&component_style);

    // 写入输出文件
    fs::write(output_file, rust_code)
        .map_err(|e| format!("无法写入文件 {}: {}", output_file, e))?;

    println!("✅ 成功将 {} 迁移到 {}", input_file, output_file);

    Ok(())
}
