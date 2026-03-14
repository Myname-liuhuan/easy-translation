use crate::translator::{TranslateError, Translator};
use reqwest::Client;
use serde::Deserialize;
use std::env;

const DEFAULT_API_URL: &str = "https://api-free.deepl.com/v2/translate";

/// DeepL translation service.
pub struct DeepLTranslator {
    client: Client,
    api_key: Option<String>,
    api_url: String,
}

#[derive(Deserialize)]
struct DeepLResponse {
    translations: Vec<DeepLTranslation>,
}

#[derive(Deserialize)]
struct DeepLTranslation {
    text: String,
}

impl DeepLTranslator {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            api_key: env::var("DEEPL_API_KEY").ok(),
            api_url: env::var("DEEPL_API_URL").unwrap_or_else(|_| DEFAULT_API_URL.to_string()),
        }
    }

    /// Convert language code to DeepL format.
    fn to_deepl_lang(lang: &str) -> &str {
        match lang {
            "zh" => "ZH",      // Chinese
            "en" => "EN",      // English
            "ja" => "JA",      // Japanese
            "ko" => "KO",      // Korean
            "fr" => "FR",      // French
            "de" => "DE",      // German
            "es" => "ES",      // Spanish
            "pt" => "PT",      // Portuguese
            "it" => "IT",      // Italian
            "ru" => "RU",      // Russian
            _ => lang,
        }
    }
}

#[async_trait::async_trait]
impl Translator for DeepLTranslator {
    fn name(&self) -> &'static str {
        "DeepL"
    }

    fn is_available(&self) -> bool {
        self.api_key.is_some()
    }

    async fn translate(&self, from: &str, to: &str, text: &str) -> Result<String, TranslateError> {
        let api_key = self.api_key.as_ref().ok_or_else(|| {
            TranslateError::ConfigError("DEEPL_API_KEY not configured".to_string())
        })?;

        let source_lang = if from == "other" {
            None // Let DeepL auto-detect
        } else {
            Some(Self::to_deepl_lang(from).to_string())
        };

        let target_lang = Self::to_deepl_lang(to);

        let response = self
            .client
            .post(&self.api_url)
            .query(&[
                ("auth_key", api_key.as_str()),
                ("text", text),
                ("target_lang", target_lang),
            ])
            .query(&source_lang.as_ref().map(|s| ("source_lang", s.as_str())))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(TranslateError::ApiError(format!(
                "DeepL API error: {} - {}",
                status, body
            )));
        }

        let result: DeepLResponse = response.json().await?;

        result
            .translations
            .first()
            .map(|t| t.text.clone())
            .ok_or_else(|| TranslateError::ApiError("No translation returned".to_string()))
    }
}

impl Default for DeepLTranslator {
    fn default() -> Self {
        Self::new()
    }
}
