//! 质量检查工具
//!
//! 验证重构后的组件质量和一致性

// 引用自身库，避免未使用依赖警告
use ant_design_tools_common::*;
use clap::{Arg, Command};
use colorized;
use env_logger;
use log;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use toml;
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityReport {
    pub component_name: String,
    pub overall_score: f64,
    pub checks: Vec<QualityCheck>,
    pub suggestions: Vec<String>,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityCheck {
    pub name: String,
    pub description: String,
    pub passed: bool,
    pub score: f64,
    pub details: Option<String>,
}

#[derive(Debug, Clone)]
pub struct QualityChecker {
    components_dir: PathBuf,
    strict_mode: bool,
}

impl QualityChecker {
    fn new(components_dir: PathBuf, strict_mode: bool) -> Self {
        Self {
            components_dir,
            strict_mode,
        }
    }

    /// 检查单个组件质量
    pub fn check_component(
        &self,
        component_path: &Path,
    ) -> Result<QualityReport, Box<dyn std::error::Error>> {
        // 验证组件路径是否在指定的组件目录下
        if !component_path.starts_with(&self.components_dir) {
            return Err(format!(
                "组件路径 {:?} 不在指定的组件目录 {:?} 下",
                component_path, self.components_dir
            )
            .into());
        }

        let component_name = component_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let mut checks = Vec::new();
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let mut suggestions = Vec::new();

        // 1. 检查目录结构
        checks.push(self.check_directory_structure(component_path, &mut errors, &mut warnings)?);

        // 2. 检查文件命名规范
        checks.push(self.check_file_naming(component_path, &mut errors, &mut warnings)?);

        // 3. 检查代码质量
        checks.push(self.check_code_quality(component_path, &mut errors, &mut warnings)?);

        // 4. 检查样式规范
        checks.push(self.check_style_standards(component_path, &mut errors, &mut warnings)?);

        // 5. 检查文档完整性
        checks.push(self.check_documentation(component_path, &mut errors, &mut warnings)?);

        // 6. 检查类型安全
        checks.push(self.check_type_safety(component_path, &mut errors, &mut warnings)?);

        // 7. 检查性能优化
        checks.push(self.check_performance(component_path, &mut errors, &mut warnings)?);

        // 8. 检查无障碍性
        checks.push(self.check_accessibility(component_path, &mut errors, &mut warnings)?);

        // 生成建议
        self.generate_suggestions(&checks, &mut suggestions);

        // 计算总分
        let overall_score = self.calculate_overall_score(&checks);

        Ok(QualityReport {
            component_name,
            overall_score,
            checks,
            suggestions,
            errors,
            warnings,
        })
    }

    /// 检查目录结构
    fn check_directory_structure(
        &self,
        component_path: &Path,
        errors: &mut Vec<String>,
        warnings: &mut Vec<String>,
    ) -> Result<QualityCheck, Box<dyn std::error::Error>> {
        let mut score = 100.0;
        let mut details = Vec::new();
        let mut passed = true;

        // 检查必需文件
        let required_files = ["mod.rs"];
        for file in &required_files {
            let file_path = component_path.join(file);
            if !file_path.exists() {
                errors.push(format!("缺少必需文件: {}", file));
                score -= 20.0;
                passed = false;
            }
        }

        // 检查是否还有旧的 style.css 文件
        let old_style_file = component_path.join("style.css");
        if old_style_file.exists() {
            warnings.push("发现旧的 style.css 文件，应该已迁移到 CSS-in-Rust".to_string());
            score -= 10.0;
        }

        // 检查样式文件
        let styles_rs = component_path.join("styles.rs");
        let styles_dir = component_path.join("styles");
        if !styles_rs.exists() && !styles_dir.exists() {
            errors.push("缺少样式文件 (styles.rs 或 styles/ 目录)".to_string());
            score -= 15.0;
            passed = false;
        }

        // 检查目录结构层次
        let depth = self.calculate_directory_depth(component_path);
        if depth > 3 {
            warnings.push(format!("目录层次过深 ({}), 建议不超过 3 层", depth));
            score -= 5.0;
        }

        details.push(format!("目录深度: {}", depth));
        details.push(format!("文件数量: {}", self.count_files(component_path)));

        Ok(QualityCheck {
            name: "目录结构".to_string(),
            description: "检查组件目录结构是否符合规范".to_string(),
            passed,
            score,
            details: Some(details.join("; ")),
        })
    }

    /// 检查文件命名规范
    fn check_file_naming(
        &self,
        component_path: &Path,
        errors: &mut Vec<String>,
        warnings: &mut Vec<String>,
    ) -> Result<QualityCheck, Box<dyn std::error::Error>> {
        let mut score = 100.0;
        let mut passed = true;
        let mut details = Vec::new();

        // 检查文件命名规范
        let snake_case_regex = Regex::new(r"^[a-z][a-z0-9_]*\.rs$").unwrap();
        let valid_extensions = ["rs", "md", "toml"];

        for entry in WalkDir::new(component_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                let file_name = entry.file_name().to_str().unwrap();
                let path = entry.path();

                // 检查扩展名
                if let Some(extension) = path.extension() {
                    let ext_str = extension.to_str().unwrap();
                    if !valid_extensions.contains(&ext_str) {
                        warnings.push(format!("不推荐的文件扩展名: {}", file_name));
                        score -= 2.0;
                    }
                }

                // 检查 Rust 文件命名
                if file_name.ends_with(".rs") && !snake_case_regex.is_match(file_name) {
                    errors.push(format!("文件命名不符合 snake_case 规范: {}", file_name));
                    score -= 10.0;
                    passed = false;
                }

                // 检查是否有大写字母
                if file_name.chars().any(|c| c.is_uppercase()) {
                    errors.push(format!("文件名包含大写字母: {}", file_name));
                    score -= 5.0;
                    passed = false;
                }
            }
        }

        details.push(format!(
            "检查了 {} 个文件",
            self.count_files(component_path)
        ));

        Ok(QualityCheck {
            name: "文件命名".to_string(),
            description: "检查文件命名是否符合 Rust 规范".to_string(),
            passed,
            score,
            details: Some(details.join("; ")),
        })
    }

    /// 检查代码质量
    fn check_code_quality(
        &self,
        component_path: &Path,
        errors: &mut Vec<String>,
        warnings: &mut Vec<String>,
    ) -> Result<QualityCheck, Box<dyn std::error::Error>> {
        let mut score = 100.0;
        let mut passed = true;
        let mut details = Vec::new();

        let mut total_lines = 0;
        let mut documented_functions = 0;
        let mut total_functions = 0;
        let mut has_tests = false;

        for entry in WalkDir::new(component_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file()
                && entry.path().extension() == Some(std::ffi::OsStr::new("rs"))
            {
                let content = fs::read_to_string(entry.path())?;
                total_lines += content.lines().count();

                // 检查函数文档
                let function_regex = Regex::new(r"(?m)^\s*(?:pub\s+)?fn\s+\w+").unwrap();
                let doc_regex = Regex::new(r"(?m)^\s*///").unwrap();

                let functions: Vec<_> = function_regex.find_iter(&content).collect();
                total_functions += functions.len();

                // 简单检查：如果函数前面有文档注释
                for func_match in functions {
                    let before_func = &content[..func_match.start()];
                    if doc_regex.is_match(&before_func.lines().last().unwrap_or("")) {
                        documented_functions += 1;
                    }
                }

                // 检查是否有测试
                if content.contains("#[test]") || content.contains("#[cfg(test)]") {
                    has_tests = true;
                }

                // 检查代码复杂度（简单指标）
                let complexity_indicators = ["if ", "match ", "for ", "while ", "loop "];
                let complexity_count: usize = complexity_indicators
                    .iter()
                    .map(|indicator| content.matches(indicator).count())
                    .sum();

                if complexity_count > 50 {
                    warnings.push(format!(
                        "文件 {} 复杂度较高",
                        entry.file_name().to_str().unwrap()
                    ));
                    score -= 5.0;
                }

                // 检查行长度
                for (line_num, line) in content.lines().enumerate() {
                    if line.len() > 120 {
                        warnings.push(format!(
                            "{}:{} 行长度超过 120 字符",
                            entry.file_name().to_str().unwrap(),
                            line_num + 1
                        ));
                        score -= 1.0;
                    }
                }
            }
        }

        // 文档覆盖率检查
        let doc_coverage = if total_functions > 0 {
            (documented_functions as f64 / total_functions as f64) * 100.0
        } else {
            100.0
        };

        if doc_coverage < 80.0 {
            warnings.push(format!("文档覆盖率较低: {:.1}%", doc_coverage));
            score -= 10.0;
        }

        if !has_tests && self.strict_mode {
            warnings.push("缺少测试代码".to_string());
            score -= 15.0;
        }

        details.push(format!("总行数: {}", total_lines));
        details.push(format!("函数数量: {}", total_functions));
        details.push(format!("文档覆盖率: {:.1}%", doc_coverage));
        details.push(format!("包含测试: {}", has_tests));

        Ok(QualityCheck {
            name: "代码质量".to_string(),
            description: "检查代码质量、文档和测试覆盖率".to_string(),
            passed,
            score,
            details: Some(details.join("; ")),
        })
    }

    /// 检查样式规范
    fn check_style_standards(
        &self,
        component_path: &Path,
        errors: &mut Vec<String>,
        warnings: &mut Vec<String>,
    ) -> Result<QualityCheck, Box<dyn std::error::Error>> {
        let mut score = 100.0;
        let mut passed = true;
        let mut details = Vec::new();

        // 检查是否使用 CSS-in-Rust
        let mut uses_css_in_rust = false;
        let mut has_old_css = false;

        for entry in WalkDir::new(component_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                let file_name = entry.file_name().to_str().unwrap();

                if file_name.ends_with(".css") {
                    has_old_css = true;
                    errors.push(format!("发现旧的 CSS 文件: {}", file_name));
                    score -= 20.0;
                    passed = false;
                }

                if file_name.ends_with(".rs") {
                    let content = fs::read_to_string(entry.path())?;
                    if content.contains("css_in_rust") || content.contains("css!") {
                        uses_css_in_rust = true;
                    }
                }
            }
        }

        if !uses_css_in_rust && !has_old_css {
            warnings.push("未发现样式实现".to_string());
            score -= 10.0;
        }

        // 检查样式组织
        let styles_rs = component_path.join("styles.rs");
        let styles_dir = component_path.join("styles");

        if styles_dir.exists() {
            // 检查样式模块化
            let base_rs = styles_dir.join("base.rs");
            let variants_rs = styles_dir.join("variants.rs");
            let mod_rs = styles_dir.join("mod.rs");

            if !mod_rs.exists() {
                errors.push("styles 目录缺少 mod.rs 文件".to_string());
                score -= 10.0;
                passed = false;
            }

            if base_rs.exists() {
                details.push("包含基础样式模块".to_string());
            }

            if variants_rs.exists() {
                details.push("包含变体样式模块".to_string());
            }
        } else if styles_rs.exists() {
            details.push("使用单文件样式组织".to_string());
        }

        Ok(QualityCheck {
            name: "样式规范".to_string(),
            description: "检查样式实现是否符合 CSS-in-Rust 规范".to_string(),
            passed,
            score,
            details: Some(details.join("; ")),
        })
    }

    /// 检查文档完整性
    fn check_documentation(
        &self,
        component_path: &Path,
        errors: &mut Vec<String>,
        warnings: &mut Vec<String>,
    ) -> Result<QualityCheck, Box<dyn std::error::Error>> {
        let mut score = 100.0;
        let mut passed = true;
        let mut details = Vec::new();

        // 检查 README 或文档文件
        let readme_files = ["README.md", "readme.md", "TODO.md"];
        let mut has_readme = false;

        for readme in &readme_files {
            if component_path.join(readme).exists() {
                has_readme = true;
                break;
            }
        }

        if !has_readme {
            warnings.push("缺少 README 或文档文件".to_string());
            score -= 10.0;
        }

        // 检查组件文档注释
        let mod_rs = component_path.join("mod.rs");
        if mod_rs.exists() {
            let content = fs::read_to_string(&mod_rs)?;

            if !content.contains("//!") {
                warnings.push("mod.rs 缺少模块级文档注释".to_string());
                score -= 5.0;
            }

            // 检查组件示例
            if !content.contains("```") && !content.contains("# Example") {
                warnings.push("缺少使用示例".to_string());
                score -= 5.0;
            }
        }

        details.push(format!("包含文档文件: {}", has_readme));

        Ok(QualityCheck {
            name: "文档完整性".to_string(),
            description: "检查组件文档是否完整".to_string(),
            passed,
            score,
            details: Some(details.join("; ")),
        })
    }

    /// 检查类型安全
    fn check_type_safety(
        &self,
        component_path: &Path,
        errors: &mut Vec<String>,
        warnings: &mut Vec<String>,
    ) -> Result<QualityCheck, Box<dyn std::error::Error>> {
        let mut score = 100.0;
        let mut passed = true;
        let mut details = Vec::new();

        let mut has_props = false;
        let mut uses_proper_types = false;
        let mut unsafe_count = 0;

        for entry in WalkDir::new(component_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file()
                && entry.path().extension() == Some(std::ffi::OsStr::new("rs"))
            {
                let content = fs::read_to_string(entry.path())?;

                // 检查 Props 定义
                if content.contains("Props") {
                    has_props = true;
                }

                // 检查类型定义
                if content.contains("enum") || content.contains("struct") {
                    uses_proper_types = true;
                }

                // 检查 unsafe 使用
                unsafe_count += content.matches("unsafe").count();

                // 检查 unwrap 使用
                let unwrap_count = content.matches(".unwrap()").count();
                if unwrap_count > 5 {
                    warnings.push(format!(
                        "文件 {} 过度使用 unwrap()",
                        entry.file_name().to_str().unwrap()
                    ));
                    score -= 5.0;
                }
            }
        }

        if unsafe_count > 0 {
            warnings.push(format!("发现 {} 处 unsafe 代码", unsafe_count));
            score -= unsafe_count as f64 * 2.0;
        }

        details.push(format!("包含 Props: {}", has_props));
        details.push(format!("使用类型定义: {}", uses_proper_types));
        details.push(format!("unsafe 代码数量: {}", unsafe_count));

        Ok(QualityCheck {
            name: "类型安全".to_string(),
            description: "检查类型定义和安全性".to_string(),
            passed,
            score,
            details: Some(details.join("; ")),
        })
    }

    /// 检查性能优化
    fn check_performance(
        &self,
        component_path: &Path,
        _errors: &mut Vec<String>,
        warnings: &mut Vec<String>,
    ) -> Result<QualityCheck, Box<dyn std::error::Error>> {
        let mut score = 100.0;
        let mut passed = true;
        let mut details = Vec::new();

        let mut uses_memo = false;
        let mut uses_callback = false;
        let mut clone_count = 0;

        for entry in WalkDir::new(component_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file()
                && entry.path().extension() == Some(std::ffi::OsStr::new("rs"))
            {
                let content = fs::read_to_string(entry.path())?;

                if content.contains("use_memo") {
                    uses_memo = true;
                }

                if content.contains("use_callback") {
                    uses_callback = true;
                }

                clone_count += content.matches(".clone()").count();
            }
        }

        if clone_count > 10 {
            warnings.push(format!("过度使用 clone(): {} 次", clone_count));
            score -= 5.0;
        }

        details.push(format!("使用 memo: {}", uses_memo));
        details.push(format!("使用 callback: {}", uses_callback));
        details.push(format!("clone 使用次数: {}", clone_count));

        Ok(QualityCheck {
            name: "性能优化".to_string(),
            description: "检查性能优化措施".to_string(),
            passed,
            score,
            details: Some(details.join("; ")),
        })
    }

    /// 检查无障碍性
    fn check_accessibility(
        &self,
        component_path: &Path,
        errors: &mut Vec<String>,
        warnings: &mut Vec<String>,
    ) -> Result<QualityCheck, Box<dyn std::error::Error>> {
        let mut score = 100.0;
        let mut passed = true;
        let mut details = Vec::new();

        let mut has_aria_attrs = false;
        let mut has_role_attrs = false;
        let mut has_alt_text = false;

        for entry in WalkDir::new(component_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file()
                && entry.path().extension() == Some(std::ffi::OsStr::new("rs"))
            {
                let content = fs::read_to_string(entry.path())?;

                if content.contains("aria-") {
                    has_aria_attrs = true;
                }

                if content.contains("role:") {
                    has_role_attrs = true;
                }

                if content.contains("alt:") {
                    has_alt_text = true;
                }
            }
        }

        if !has_aria_attrs && self.strict_mode {
            warnings.push("缺少 ARIA 属性支持".to_string());
            score -= 10.0;
        }

        details.push(format!("ARIA 属性: {}", has_aria_attrs));
        details.push(format!("Role 属性: {}", has_role_attrs));
        details.push(format!("Alt 文本: {}", has_alt_text));

        Ok(QualityCheck {
            name: "无障碍性".to_string(),
            description: "检查无障碍性支持".to_string(),
            passed,
            score,
            details: Some(details.join("; ")),
        })
    }

    /// 生成改进建议
    fn generate_suggestions(&self, checks: &[QualityCheck], suggestions: &mut Vec<String>) {
        for check in checks {
            if check.score < 80.0 {
                match check.name.as_str() {
                    "目录结构" => {
                        suggestions.push("建议重新组织目录结构，确保符合组件等级规范".to_string());
                    }
                    "样式规范" => {
                        suggestions
                            .push("建议完成 CSS-in-Rust 迁移，移除旧的 CSS 文件".to_string());
                    }
                    "代码质量" => {
                        suggestions.push("建议增加函数文档注释和单元测试".to_string());
                    }
                    "类型安全" => {
                        suggestions.push("建议减少 unwrap() 使用，增加错误处理".to_string());
                    }
                    "性能优化" => {
                        suggestions.push("建议使用 use_memo 和 use_callback 优化性能".to_string());
                    }
                    "无障碍性" => {
                        suggestions.push("建议添加 ARIA 属性和无障碍性支持".to_string());
                    }
                    _ => {}
                }
            }
        }
    }

    /// 计算总分
    fn calculate_overall_score(&self, checks: &[QualityCheck]) -> f64 {
        if checks.is_empty() {
            return 0.0;
        }

        let total_score: f64 = checks.iter().map(|check| check.score).sum();
        total_score / checks.len() as f64
    }

    /// 计算目录深度
    fn calculate_directory_depth(&self, path: &Path) -> usize {
        let mut max_depth = 0;

        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            let depth = entry.depth();
            if depth > max_depth {
                max_depth = depth;
            }
        }

        max_depth
    }

    /// 统计文件数量
    fn count_files(&self, path: &Path) -> usize {
        WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .count()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let matches = Command::new("质量检查工具")
        .version("1.0")
        .author("Ant Design Dioxus Team")
        .about("检查组件质量和一致性")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("DIR")
                .help("组件目录路径")
                .default_value("src/components"),
        )
        .arg(
            Arg::new("component")
                .short('c')
                .long("component")
                .value_name("NAME")
                .help("指定单个组件名称"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("输出报告文件路径"),
        )
        .arg(
            Arg::new("strict")
                .short('s')
                .long("strict")
                .help("启用严格模式")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .value_name("FORMAT")
                .help("输出格式 (json|text)")
                .default_value("text"),
        )
        .get_matches();

    let input_dir = PathBuf::from(matches.get_one::<String>("input").unwrap());
    let strict_mode = matches.get_flag("strict");
    let format = matches.get_one::<String>("format").unwrap();

    let checker = QualityChecker::new(input_dir.clone(), strict_mode);

    if let Some(component_name) = matches.get_one::<String>("component") {
        // 检查单个组件
        let component_path = input_dir.join(component_name);
        if !component_path.exists() {
            return Err(format!("组件目录不存在: {:?}", component_path).into());
        }

        let report = checker.check_component(&component_path)?;

        if format == "json" {
            let json = serde_json::to_string_pretty(&report)?;
            if let Some(output_file) = matches.get_one::<String>("output") {
                fs::write(output_file, json)?;
                println!("✅ 报告已保存到: {}", output_file);
            } else {
                println!("{}", json);
            }
        } else {
            print_report(&report);
        }
    } else {
        // 批量检查所有组件
        let mut all_reports = Vec::new();

        for entry in fs::read_dir(&input_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                match checker.check_component(&path) {
                    Ok(report) => {
                        all_reports.push(report);
                    }
                    Err(e) => {
                        eprintln!(
                            "❌ 检查失败: {} - {}",
                            path.file_name().unwrap().to_str().unwrap(),
                            e
                        );
                    }
                }
            }
        }

        if format == "json" {
            let json = serde_json::to_string_pretty(&all_reports)?;
            if let Some(output_file) = matches.get_one::<String>("output") {
                fs::write(output_file, json)?;
                println!("✅ 报告已保存到: {}", output_file);
            } else {
                println!("{}", json);
            }
        } else {
            print_summary(&all_reports);
        }
    }

    Ok(())
}

/// 打印单个组件报告
fn print_report(report: &QualityReport) {
    println!("\n📊 {} 组件质量报告", report.component_name);
    println!("总分: {:.1}/100.0", report.overall_score);

    let grade = match report.overall_score {
        score if score >= 90.0 => "🟢 优秀",
        score if score >= 80.0 => "🟡 良好",
        score if score >= 70.0 => "🟠 一般",
        _ => "🔴 需要改进",
    };
    println!("等级: {}", grade);

    println!("\n📋 检查项目:");
    for check in &report.checks {
        let status = if check.passed { "✅" } else { "❌" };
        println!("  {} {} - {:.1}/100.0", status, check.name, check.score);
        if let Some(details) = &check.details {
            println!("     {}", details);
        }
    }

    if !report.errors.is_empty() {
        println!("\n🚨 错误:");
        for error in &report.errors {
            println!("  • {}", error);
        }
    }

    if !report.warnings.is_empty() {
        println!("\n⚠️  警告:");
        for warning in &report.warnings {
            println!("  • {}", warning);
        }
    }

    if !report.suggestions.is_empty() {
        println!("\n💡 建议:");
        for suggestion in &report.suggestions {
            println!("  • {}", suggestion);
        }
    }
}

/// 打印汇总报告
fn print_summary(reports: &[QualityReport]) {
    println!("\n📊 组件质量汇总报告");
    println!("检查组件数量: {}", reports.len());

    let avg_score: f64 =
        reports.iter().map(|r| r.overall_score).sum::<f64>() / reports.len() as f64;
    println!("平均分数: {:.1}/100.0", avg_score);

    let mut grade_counts = HashMap::new();
    for report in reports {
        let grade = match report.overall_score {
            score if score >= 90.0 => "优秀",
            score if score >= 80.0 => "良好",
            score if score >= 70.0 => "一般",
            _ => "需要改进",
        };
        *grade_counts.entry(grade).or_insert(0) += 1;
    }

    println!("\n📈 等级分布:");
    for (grade, count) in &grade_counts {
        println!("  {}: {} 个组件", grade, count);
    }

    // 显示分数最低的组件
    let mut sorted_reports = reports.to_vec();
    sorted_reports.sort_by(|a, b| a.overall_score.partial_cmp(&b.overall_score).unwrap());

    println!("\n🔴 需要优先改进的组件:");
    for report in sorted_reports.iter().take(5) {
        if report.overall_score < 80.0 {
            println!(
                "  {} - {:.1}/100.0",
                report.component_name, report.overall_score
            );
        }
    }
}
