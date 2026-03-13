use crate::language_detect::detector::detect_language;
use crate::translator::{mock::MockTranslator, Translator};

/// Translate text based on detected language.
/// - Chinese -> English
/// - English -> Chinese
/// - Other -> Chinese
#[tauri::command]
pub fn translate_text(text: String) -> Result<String, String> {
    if text.trim().is_empty() {
        return Ok(String::new());
    }

    let detected_lang = detect_language(&text);
    let translator = MockTranslator;

    let (from, to) = match detected_lang.as_str() {
        "zh" => ("zh", "en"),
        "en" => ("en", "zh"),
        _ => ("other", "zh"),
    };

    Ok(translator.translate(from, to, &text))
}
