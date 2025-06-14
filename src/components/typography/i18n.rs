//!
//! Typography组件的国际化支持
//! 基于Ant Design v5的ConfigProvider国际化架构
//! 提供与全局locale系统集成的多语言支持

use crate::locale::{Locale, use_translate};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Typography组件的国际化配置
/// 遵循Ant Design官方的locale配置结构
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypographyLocale {
    /// 复制按钮文本
    pub copy: String,
    /// 复制成功提示
    pub copied: String,
    /// 编辑按钮文本
    pub edit: String,
    /// 编辑文本标签
    pub edit_text: String,
    /// 保存按钮
    pub save: String,
    /// 确认编辑按钮
    pub confirm: String,
    /// 取消编辑按钮
    pub cancel: String,
    /// 展开文本
    pub expand: String,
    /// 收起文本
    pub collapse: String,
    /// 编辑帮助文本
    pub edit_help: String,
    /// 复制失败提示
    pub copy_failed: String,
    /// 字符限制提示
    pub char_limit: String,
}

/// Typography组件的翻译键常量
/// 与全局locale系统保持一致的命名规范
pub struct TypographyI18nKeys;

impl TypographyI18nKeys {
    /// 复制按钮文本
    pub const COPY: &'static str = "typography.copy";
    /// 复制成功提示
    pub const COPIED: &'static str = "typography.copied";
    /// 编辑按钮文本
    pub const EDIT: &'static str = "typography.edit";
    /// 编辑文本标签
    pub const EDIT_TEXT: &'static str = "typography.edit_text";
    /// 保存按钮
    pub const SAVE: &'static str = "typography.save";
    /// 确认编辑按钮
    pub const CONFIRM: &'static str = "typography.confirm";
    /// 取消编辑按钮
    pub const CANCEL: &'static str = "typography.cancel";
    /// 展开文本
    pub const EXPAND: &'static str = "typography.expand";
    /// 收起文本
    pub const COLLAPSE: &'static str = "typography.collapse";
    /// 编辑帮助文本
    pub const EDIT_HELP: &'static str = "typography.edit_help";
    /// 复制失败提示
    pub const COPY_FAILED: &'static str = "typography.copy_failed";
    /// 字符限制提示
    pub const CHAR_LIMIT: &'static str = "typography.char_limit";
}

/// 获取Typography组件的默认国际化配置
/// 基于Ant Design官方的locale配置结构
pub fn get_typography_locale(locale: &Locale) -> TypographyLocale {
    match locale {
        Locale::ZhCN => TypographyLocale {
            copy: "复制".to_string(),
            copied: "已复制!".to_string(),
            edit: "编辑".to_string(),
            edit_text: "编辑文本".to_string(),
            save: "保存".to_string(),
            confirm: "确认".to_string(),
            cancel: "取消".to_string(),
            expand: "展开".to_string(),
            collapse: "收起".to_string(),
            edit_help: "按回车键确认，按Esc键取消".to_string(),
            copy_failed: "复制失败".to_string(),
            char_limit: "字符数限制".to_string(),
        },
        Locale::ZhTW => TypographyLocale {
            copy: "複製".to_string(),
            copied: "已複製!".to_string(),
            edit: "編輯".to_string(),
            edit_text: "編輯文本".to_string(),
            save: "保存".to_string(),
            confirm: "確認".to_string(),
            cancel: "取消".to_string(),
            expand: "展開".to_string(),
            collapse: "收起".to_string(),
            edit_help: "按回車鍵確認，按Esc鍵取消".to_string(),
            copy_failed: "複製失敗".to_string(),
            char_limit: "字符數限制".to_string(),
        },
        Locale::Ja => TypographyLocale {
            copy: "コピー".to_string(),
            copied: "コピーしました!".to_string(),
            edit: "編集".to_string(),
            edit_text: "テキストを編集".to_string(),
            save: "保存".to_string(),
            confirm: "確認".to_string(),
            cancel: "キャンセル".to_string(),
            expand: "展開".to_string(),
            collapse: "折りたたみ".to_string(),
            edit_help: "Enterキーで確認、Escキーでキャンセル".to_string(),
            copy_failed: "コピーに失敗しました".to_string(),
            char_limit: "文字数制限".to_string(),
        },
        Locale::Ko => TypographyLocale {
            copy: "복사".to_string(),
            copied: "복사됨!".to_string(),
            edit: "편집".to_string(),
            edit_text: "편집 텍스트".to_string(),
            save: "저장".to_string(),
            confirm: "확인".to_string(),
            cancel: "취소".to_string(),
            expand: "펼치기".to_string(),
            collapse: "접기".to_string(),
            edit_help: "Enter키로 확인, Esc키로 취소".to_string(),
            copy_failed: "복사 실패".to_string(),
            char_limit: "문자 수 제한".to_string(),
        },
        Locale::Fr => TypographyLocale {
             copy: "Copier".to_string(),
             copied: "Copié!".to_string(),
             edit: "Modifier".to_string(),
             edit_text: "Modifier le texte".to_string(),
             save: "Enregistrer".to_string(),
             confirm: "Confirmer".to_string(),
             cancel: "Annuler".to_string(),
            expand: "Développer".to_string(),
            collapse: "Réduire".to_string(),
            edit_help: "Appuyez sur Entrée pour confirmer, Échap pour annuler".to_string(),
            copy_failed: "Échec de la copie".to_string(),
            char_limit: "Limite de caractères".to_string(),
        },
        Locale::De => TypographyLocale {
            copy: "Kopieren".to_string(),
            copied: "Kopiert!".to_string(),
            edit: "Bearbeiten".to_string(),
            edit_text: "Text bearbeiten".to_string(),
            save: "Speichern".to_string(),
            confirm: "Bestätigen".to_string(),
            cancel: "Abbrechen".to_string(),
            expand: "Erweitern".to_string(),
            collapse: "Einklappen".to_string(),
            edit_help: "Enter zum Bestätigen, Esc zum Abbrechen".to_string(),
            copy_failed: "Kopieren fehlgeschlagen".to_string(),
            char_limit: "Zeichenbegrenzung".to_string(),
        },
        Locale::Es => TypographyLocale {
            copy: "Copiar".to_string(),
            copied: "¡Copiado!".to_string(),
            edit: "Editar".to_string(),
            edit_text: "Editar texto".to_string(),
            save: "Guardar".to_string(),
            confirm: "Confirmar".to_string(),
            cancel: "Cancelar".to_string(),
            expand: "Expandir".to_string(),
            collapse: "Contraer".to_string(),
            edit_help: "Presiona Enter para confirmar, Esc para cancelar".to_string(),
            copy_failed: "Error al copiar".to_string(),
            char_limit: "Límite de caracteres".to_string(),
        },
        Locale::Ru => TypographyLocale {
            copy: "Копировать".to_string(),
            copied: "Скопировано!".to_string(),
            edit: "Редактировать".to_string(),
            edit_text: "Редактировать текст".to_string(),
            save: "Сохранить".to_string(),
            confirm: "Подтвердить".to_string(),
            cancel: "Отмена".to_string(),
            expand: "Развернуть".to_string(),
            collapse: "Свернуть".to_string(),
            edit_help: "Нажмите Enter для подтверждения, Esc для отмены".to_string(),
            copy_failed: "Ошибка копирования".to_string(),
            char_limit: "Ограничение символов".to_string(),
        },
        _ => {
            // 默认英文
            TypographyLocale {
                copy: "Copy".to_string(),
                copied: "Copied!".to_string(),
                edit: "Edit".to_string(),
                edit_text: "Edit text".to_string(),
                save: "Save".to_string(),
                confirm: "Confirm".to_string(),
                cancel: "Cancel".to_string(),
                expand: "Expand".to_string(),
                collapse: "Collapse".to_string(),
                edit_help: "Press Enter to confirm, Esc to cancel".to_string(),
                copy_failed: "Copy failed".to_string(),
                char_limit: "Character limit".to_string(),
            }
        }
    }
}

/// 获取Typography组件的翻译文本映射
/// 兼容旧版API，建议使用get_typography_locale
#[deprecated(since = "0.2.0", note = "请使用get_typography_locale替代")]
pub fn get_typography_translations(locale: &Locale) -> HashMap<String, String> {
    let typography_locale = get_typography_locale(locale);
    let mut translations = HashMap::new();
    
    translations.insert(TypographyI18nKeys::COPY.to_string(), typography_locale.copy);
    translations.insert(TypographyI18nKeys::COPIED.to_string(), typography_locale.copied);
    translations.insert(TypographyI18nKeys::EDIT.to_string(), typography_locale.edit);
    translations.insert(TypographyI18nKeys::EDIT_TEXT.to_string(), typography_locale.edit_text);
    translations.insert(TypographyI18nKeys::SAVE.to_string(), typography_locale.save);
    translations.insert(TypographyI18nKeys::CONFIRM.to_string(), typography_locale.confirm);
    translations.insert(TypographyI18nKeys::CANCEL.to_string(), typography_locale.cancel);
    translations.insert(TypographyI18nKeys::EXPAND.to_string(), typography_locale.expand);
    translations.insert(TypographyI18nKeys::COLLAPSE.to_string(), typography_locale.collapse);
    translations.insert(TypographyI18nKeys::EDIT_HELP.to_string(), typography_locale.edit_help);
    translations.insert(TypographyI18nKeys::COPY_FAILED.to_string(), typography_locale.copy_failed);
    translations.insert(TypographyI18nKeys::CHAR_LIMIT.to_string(), typography_locale.char_limit);
    
    translations
}

/// Typography组件的国际化Hook
/// 基于ConfigProvider的全局locale配置
/// 遵循Ant Design官方的国际化模式
pub fn use_typography_i18n() -> impl Fn(&str) -> String {
    use_translate()
}

/// 获取Typography组件的国际化配置Hook
/// 返回当前locale下的Typography国际化配置
pub fn use_typography_locale() -> TypographyLocale {
    use crate::locale::use_locale;
    let locale_context = use_locale();
    get_typography_locale(&locale_context.locale)
}

/// Typography组件的国际化文本获取函数
/// 提供类型安全的文本获取方式
pub struct TypographyI18n {
    locale: TypographyLocale,
}

impl TypographyI18n {
    /// 创建新的Typography国际化实例
    pub fn new(locale: &Locale) -> Self {
        Self {
            locale: get_typography_locale(locale),
        }
    }
    
    /// 获取复制按钮文本
    pub fn copy(&self) -> &str {
        &self.locale.copy
    }
    
    /// 获取复制成功提示
    pub fn copied(&self) -> &str {
        &self.locale.copied
    }
    
    /// 获取编辑按钮文本
    pub fn edit(&self) -> &str {
        &self.locale.edit
    }
    
    /// 获取编辑文本标签
    pub fn edit_text(&self) -> &str {
        &self.locale.edit_text
    }
    
    /// 获取保存按钮文本
    pub fn save(&self) -> &str {
        &self.locale.save
    }
    
    /// 获取确认按钮文本
    pub fn confirm(&self) -> &str {
        &self.locale.confirm
    }
    
    /// 获取取消按钮文本
    pub fn cancel(&self) -> &str {
        &self.locale.cancel
    }
    
    /// 获取展开文本
    pub fn expand(&self) -> &str {
        &self.locale.expand
    }
    
    /// 获取收起文本
    pub fn collapse(&self) -> &str {
        &self.locale.collapse
    }
    
    /// 获取编辑帮助文本
    pub fn edit_help(&self) -> &str {
        &self.locale.edit_help
    }
    
    /// 获取复制失败提示
    pub fn copy_failed(&self) -> &str {
        &self.locale.copy_failed
    }
    
    /// 获取字符限制提示
    pub fn char_limit(&self) -> &str {
        &self.locale.char_limit
    }
}

/// Typography组件的国际化Hook（类型安全版本）
/// 返回Typography专用的国际化实例
pub fn use_typography_i18n_safe() -> TypographyI18n {
    use crate::locale::use_locale;
    let locale_context = use_locale();
    TypographyI18n::new(&locale_context.locale)
}