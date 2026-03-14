use crate::language_detect::detector::detect_language;
use crate::translator::translate_with_fallback;
use serde::Serialize;

/// Translation response with language information.
#[derive(Serialize)]
pub struct TranslationResult {
    /// Translated text
    text: String,
    /// Source language code (e.g., "zh", "en", "other")
    from: String,
    /// Target language code
    to: String,
}

/// Translate text based on detected language.
/// - Chinese -> English
/// - English -> Chinese
/// - Other -> Chinese
///
/// Uses fallback chain: Baidu → DeepL → Google → Mock
#[tauri::command]
pub async fn translate_text(text: String) -> Result<TranslationResult, String> {
    if text.trim().is_empty() {
        return Ok(TranslationResult {
            text: String::new(),
            from: String::new(),
            to: String::new(),
        });
    }

    let detected_lang = detect_language(&text);

    let (from, to) = match detected_lang.as_str() {
        "zh" => ("zh".to_string(), "en".to_string()),
        "en" => ("en".to_string(), "zh".to_string()),
        _ => ("other".to_string(), "zh".to_string()),
    };

    let translated = translate_with_fallback(&text, &from, &to)
        .await
        .map_err(|e| {
            #[cfg(debug_assertions)]
            log::error!("Translation failed: {}", e);
            e.to_string()
        })?;

    Ok(TranslationResult {
        text: translated,
        from,
        to,
    })
}
