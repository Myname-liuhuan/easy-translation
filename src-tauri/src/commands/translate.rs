use crate::language_detect::detector::detect_language;
use crate::translator::translate_with_fallback;

/// Translate text based on detected language.
/// - Chinese -> English
/// - English -> Chinese
/// - Other -> Chinese
///
/// Uses fallback chain: DeepL → Google → Baidu → Mock
#[tauri::command]
pub async fn translate_text(text: String) -> Result<String, String> {
    if text.trim().is_empty() {
        return Ok(String::new());
    }

    let detected_lang = detect_language(&text);

    let (from, to) = match detected_lang.as_str() {
        "zh" => ("zh", "en"),
        "en" => ("en", "zh"),
        _ => ("other", "zh"),
    };

    translate_with_fallback(&text, from, to)
        .await
        .map_err(|e| {
            log::error!("Translation failed: {}", e);
            e.to_string()
        })
}
