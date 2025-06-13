//!
//! Typography组件的国际化支持
//! 提供多语言文本翻译和本地化功能

use crate::locale::Locale;
use std::collections::HashMap;

/// Typography组件的翻译键
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

/// 获取Typography组件的翻译文本
pub fn get_typography_translations(locale: &Locale) -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    match locale {
        Locale::ZhCN => {
            translations.insert(TypographyI18nKeys::COPY.to_string(), "复制".to_string());
            translations.insert(TypographyI18nKeys::COPIED.to_string(), "已复制!".to_string());
            translations.insert(TypographyI18nKeys::EDIT.to_string(), "编辑".to_string());
            translations.insert(TypographyI18nKeys::EDIT_TEXT.to_string(), "编辑文本".to_string());
            translations.insert(TypographyI18nKeys::SAVE.to_string(), "保存".to_string());
            translations.insert(TypographyI18nKeys::CONFIRM.to_string(), "确认".to_string());
            translations.insert(TypographyI18nKeys::CANCEL.to_string(), "取消".to_string());
            translations.insert(TypographyI18nKeys::EXPAND.to_string(), "展开".to_string());
            translations.insert(TypographyI18nKeys::COLLAPSE.to_string(), "收起".to_string());
            translations.insert(TypographyI18nKeys::EDIT_HELP.to_string(), "按回车键确认，按Esc键取消".to_string());
            translations.insert(TypographyI18nKeys::COPY_FAILED.to_string(), "复制失败".to_string());
            translations.insert(TypographyI18nKeys::CHAR_LIMIT.to_string(), "字符数限制".to_string());
        },
        Locale::ZhTW => {
            translations.insert(TypographyI18nKeys::COPY.to_string(), "複製".to_string());
            translations.insert(TypographyI18nKeys::COPIED.to_string(), "已複製!".to_string());
            translations.insert(TypographyI18nKeys::EDIT.to_string(), "編輯".to_string());
            translations.insert(TypographyI18nKeys::EDIT_TEXT.to_string(), "編輯文本".to_string());
            translations.insert(TypographyI18nKeys::SAVE.to_string(), "保存".to_string());
            translations.insert(TypographyI18nKeys::CONFIRM.to_string(), "確認".to_string());
            translations.insert(TypographyI18nKeys::CANCEL.to_string(), "取消".to_string());
            translations.insert(TypographyI18nKeys::EXPAND.to_string(), "展開".to_string());
            translations.insert(TypographyI18nKeys::COLLAPSE.to_string(), "收起".to_string());
            translations.insert(TypographyI18nKeys::EDIT_HELP.to_string(), "按回車鍵確認，按Esc鍵取消".to_string());
            translations.insert(TypographyI18nKeys::COPY_FAILED.to_string(), "複製失敗".to_string());
            translations.insert(TypographyI18nKeys::CHAR_LIMIT.to_string(), "字符數限制".to_string());
        },
        Locale::Ja => {
            translations.insert(TypographyI18nKeys::COPY.to_string(), "コピー".to_string());
            translations.insert(TypographyI18nKeys::COPIED.to_string(), "コピーしました!".to_string());
            translations.insert(TypographyI18nKeys::EDIT.to_string(), "編集".to_string());
            translations.insert(TypographyI18nKeys::EDIT_TEXT.to_string(), "テキストを編集".to_string());
            translations.insert(TypographyI18nKeys::SAVE.to_string(), "保存".to_string());
            translations.insert(TypographyI18nKeys::CONFIRM.to_string(), "確認".to_string());
            translations.insert(TypographyI18nKeys::CANCEL.to_string(), "キャンセル".to_string());
            translations.insert(TypographyI18nKeys::EXPAND.to_string(), "展開".to_string());
            translations.insert(TypographyI18nKeys::COLLAPSE.to_string(), "折りたたみ".to_string());
            translations.insert(TypographyI18nKeys::EDIT_HELP.to_string(), "Enterキーで確認、Escキーでキャンセル".to_string());
            translations.insert(TypographyI18nKeys::COPY_FAILED.to_string(), "コピーに失敗しました".to_string());
            translations.insert(TypographyI18nKeys::CHAR_LIMIT.to_string(), "文字数制限".to_string());
        },
        Locale::Ko => {
            translations.insert(TypographyI18nKeys::COPY.to_string(), "복사".to_string());
            translations.insert(TypographyI18nKeys::COPIED.to_string(), "복사됨!".to_string());
            translations.insert(TypographyI18nKeys::EDIT.to_string(), "편집".to_string());
            translations.insert(TypographyI18nKeys::CONFIRM.to_string(), "확인".to_string());
            translations.insert(TypographyI18nKeys::CANCEL.to_string(), "취소".to_string());
            translations.insert(TypographyI18nKeys::EXPAND.to_string(), "펼치기".to_string());
            translations.insert(TypographyI18nKeys::COLLAPSE.to_string(), "접기".to_string());
            translations.insert(TypographyI18nKeys::EDIT_HELP.to_string(), "Enter키로 확인, Esc키로 취소".to_string());
            translations.insert(TypographyI18nKeys::COPY_FAILED.to_string(), "복사 실패".to_string());
            translations.insert(TypographyI18nKeys::CHAR_LIMIT.to_string(), "문자 수 제한".to_string());
        },
        Locale::Fr => {
            translations.insert(TypographyI18nKeys::COPY.to_string(), "Copier".to_string());
            translations.insert(TypographyI18nKeys::COPIED.to_string(), "Copié!".to_string());
            translations.insert(TypographyI18nKeys::EDIT.to_string(), "Modifier".to_string());
            translations.insert(TypographyI18nKeys::CONFIRM.to_string(), "Confirmer".to_string());
            translations.insert(TypographyI18nKeys::CANCEL.to_string(), "Annuler".to_string());
            translations.insert(TypographyI18nKeys::EXPAND.to_string(), "Développer".to_string());
            translations.insert(TypographyI18nKeys::COLLAPSE.to_string(), "Réduire".to_string());
            translations.insert(TypographyI18nKeys::EDIT_HELP.to_string(), "Appuyez sur Entrée pour confirmer, Échap pour annuler".to_string());
            translations.insert(TypographyI18nKeys::COPY_FAILED.to_string(), "Échec de la copie".to_string());
            translations.insert(TypographyI18nKeys::CHAR_LIMIT.to_string(), "Limite de caractères".to_string());
        },
        Locale::De => {
            translations.insert(TypographyI18nKeys::COPY.to_string(), "Kopieren".to_string());
            translations.insert(TypographyI18nKeys::COPIED.to_string(), "Kopiert!".to_string());
            translations.insert(TypographyI18nKeys::EDIT.to_string(), "Bearbeiten".to_string());
            translations.insert(TypographyI18nKeys::CONFIRM.to_string(), "Bestätigen".to_string());
            translations.insert(TypographyI18nKeys::CANCEL.to_string(), "Abbrechen".to_string());
            translations.insert(TypographyI18nKeys::EXPAND.to_string(), "Erweitern".to_string());
            translations.insert(TypographyI18nKeys::COLLAPSE.to_string(), "Einklappen".to_string());
            translations.insert(TypographyI18nKeys::EDIT_HELP.to_string(), "Enter zum Bestätigen, Esc zum Abbrechen".to_string());
            translations.insert(TypographyI18nKeys::COPY_FAILED.to_string(), "Kopieren fehlgeschlagen".to_string());
            translations.insert(TypographyI18nKeys::CHAR_LIMIT.to_string(), "Zeichenbegrenzung".to_string());
        },
        Locale::Es => {
            translations.insert(TypographyI18nKeys::COPY.to_string(), "Copiar".to_string());
            translations.insert(TypographyI18nKeys::COPIED.to_string(), "¡Copiado!".to_string());
            translations.insert(TypographyI18nKeys::EDIT.to_string(), "Editar".to_string());
            translations.insert(TypographyI18nKeys::CONFIRM.to_string(), "Confirmar".to_string());
            translations.insert(TypographyI18nKeys::CANCEL.to_string(), "Cancelar".to_string());
            translations.insert(TypographyI18nKeys::EXPAND.to_string(), "Expandir".to_string());
            translations.insert(TypographyI18nKeys::COLLAPSE.to_string(), "Contraer".to_string());
            translations.insert(TypographyI18nKeys::EDIT_HELP.to_string(), "Presiona Enter para confirmar, Esc para cancelar".to_string());
            translations.insert(TypographyI18nKeys::COPY_FAILED.to_string(), "Error al copiar".to_string());
            translations.insert(TypographyI18nKeys::CHAR_LIMIT.to_string(), "Límite de caracteres".to_string());
        },
        Locale::Ru => {
            translations.insert(TypographyI18nKeys::COPY.to_string(), "Копировать".to_string());
            translations.insert(TypographyI18nKeys::COPIED.to_string(), "Скопировано!".to_string());
            translations.insert(TypographyI18nKeys::EDIT.to_string(), "Редактировать".to_string());
            translations.insert(TypographyI18nKeys::EDIT_TEXT.to_string(), "Редактировать текст".to_string());
            translations.insert(TypographyI18nKeys::SAVE.to_string(), "Сохранить".to_string());
            translations.insert(TypographyI18nKeys::CONFIRM.to_string(), "Подтвердить".to_string());
            translations.insert(TypographyI18nKeys::CANCEL.to_string(), "Отмена".to_string());
            translations.insert(TypographyI18nKeys::EXPAND.to_string(), "Развернуть".to_string());
            translations.insert(TypographyI18nKeys::COLLAPSE.to_string(), "Свернуть".to_string());
            translations.insert(TypographyI18nKeys::EDIT_HELP.to_string(), "Нажмите Enter для подтверждения, Esc для отмены".to_string());
            translations.insert(TypographyI18nKeys::COPY_FAILED.to_string(), "Ошибка копирования".to_string());
            translations.insert(TypographyI18nKeys::CHAR_LIMIT.to_string(), "Ограничение символов".to_string());
        },
        _ => {
            // 默认英文
            translations.insert(TypographyI18nKeys::COPY.to_string(), "Copy".to_string());
            translations.insert(TypographyI18nKeys::COPIED.to_string(), "Copied!".to_string());
            translations.insert(TypographyI18nKeys::EDIT.to_string(), "Edit".to_string());
            translations.insert(TypographyI18nKeys::EDIT_TEXT.to_string(), "Edit text".to_string());
            translations.insert(TypographyI18nKeys::SAVE.to_string(), "Save".to_string());
            translations.insert(TypographyI18nKeys::CONFIRM.to_string(), "Confirm".to_string());
            translations.insert(TypographyI18nKeys::CANCEL.to_string(), "Cancel".to_string());
            translations.insert(TypographyI18nKeys::EXPAND.to_string(), "Expand".to_string());
            translations.insert(TypographyI18nKeys::COLLAPSE.to_string(), "Collapse".to_string());
            translations.insert(TypographyI18nKeys::EDIT_HELP.to_string(), "Press Enter to confirm, Esc to cancel".to_string());
            translations.insert(TypographyI18nKeys::COPY_FAILED.to_string(), "Copy failed".to_string());
            translations.insert(TypographyI18nKeys::CHAR_LIMIT.to_string(), "Character limit".to_string());
        }
    }
    
    translations
}

/// Typography组件的国际化Hook
pub fn use_typography_i18n() -> impl Fn(&str) -> String {
    use crate::locale::use_translate;
    use_translate()
}