pub mod baidu;
pub mod deepl;
pub mod google;
pub mod mock;

use thiserror::Error;

/// Translation service errors.
#[derive(Error, Debug)]
pub enum TranslateError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("API returned error: {0}")]
    ApiError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

}

/// Translator trait for swappable translation implementations.
#[async_trait::async_trait]
pub trait Translator: Send + Sync {
    /// Get the name of this translator.
    fn name(&self) -> &'static str;

    /// Check if this translator is properly configured and available.
    fn is_available(&self) -> bool;

    /// Translate text from source language to target language.
    async fn translate(&self, from: &str, to: &str, text: &str) -> Result<String, TranslateError>;
}

/// Translate text using fallback chain: Baidu → DeepL → Google → Mock.
///
/// # Arguments
/// * `text` - The text to translate
/// * `from` - Source language code (e.g., "zh", "en")
/// * `to` - Target language code (e.g., "zh", "en")
///
/// # Returns
/// The translated text, or an error if all services fail.
pub async fn translate_with_fallback(text: &str, from: &str, to: &str) -> Result<String, TranslateError> {
    let translators: Vec<Box<dyn Translator>> = vec![
        Box::new(baidu::BaiduTranslator::new()),
        Box::new(deepl::DeepLTranslator::new()),
        Box::new(google::GoogleTranslator::new()),
    ];

    for translator in translators {
        if !translator.is_available() {
            continue;
        }

        match translator.translate(from, to, text).await {
            Ok(result) => return Ok(result),
            Err(e) => {
                #[cfg(debug_assertions)]
                log::warn!("Translator {} failed: {}", translator.name(), e);
                let _ = e; // suppress unused variable warning in release
            }
        }
    }

    // Fallback to mock translator if all real services fail
    #[cfg(debug_assertions)]
    log::warn!("All translation services failed, falling back to mock");

    let mock = mock::MockTranslator::new();
    mock.translate(from, to, text).await
}

/// Initialize the translation module.
/// Loads environment variables from .env file if present.
pub fn init() {
    // Try multiple possible locations for .env file
    let env_paths = [
        "src-tauri/.env",
        ".env",
        "../.env",
        "../../src-tauri/.env",
    ];

    for path in &env_paths {
        if std::path::Path::new(path).exists() {
            if dotenvy::from_path(path).is_ok() {
                return;
            }
        }
    }

    // Last resort: try dotenvy's default behavior
    let _ = dotenvy::dotenv();
}
