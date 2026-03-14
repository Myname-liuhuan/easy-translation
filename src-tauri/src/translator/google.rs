use crate::translator::{TranslateError, Translator};
use reqwest::Client;
use serde::Deserialize;
use std::env;

const API_URL: &str = "https://translation.googleapis.com/language/translate/v2";

/// Google Cloud Translation service.
pub struct GoogleTranslator {
    client: Client,
    api_key: Option<String>,
}

#[derive(Deserialize)]
struct GoogleResponse {
    data: GoogleData,
}

#[derive(Deserialize)]
struct GoogleData {
    translations: Vec<GoogleTranslation>,
}

#[derive(Deserialize)]
struct GoogleTranslation {
    #[serde(rename = "translatedText")]
    translated_text: String,
}

impl GoogleTranslator {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            api_key: env::var("GOOGLE_API_KEY").ok(),
        }
    }

    /// Convert language code to Google format.
    fn to_google_lang(lang: &str) -> &str {
        match lang {
            "zh" => "zh-CN", // Simplified Chinese
            "en" => "en",
            "ja" => "ja",
            "ko" => "ko",
            "fr" => "fr",
            "de" => "de",
            "es" => "es",
            "pt" => "pt",
            "it" => "it",
            "ru" => "ru",
            _ => lang,
        }
    }
}

#[async_trait::async_trait]
impl Translator for GoogleTranslator {
    fn name(&self) -> &'static str {
        "Google"
    }

    fn is_available(&self) -> bool {
        self.api_key.is_some()
    }

    async fn translate(&self, from: &str, to: &str, text: &str) -> Result<String, TranslateError> {
        let api_key = self.api_key.as_ref().ok_or_else(|| {
            TranslateError::ConfigError("GOOGLE_API_KEY not configured".to_string())
        })?;

        let source = if from == "other" {
            None // Let Google auto-detect
        } else {
            Some(Self::to_google_lang(from).to_string())
        };

        let target = Self::to_google_lang(to);

        let mut request = self
            .client
            .post(API_URL)
            .query(&[("key", api_key.as_str()), ("q", text), ("target", target)]);

        if let Some(ref source_lang) = source {
            request = request.query(&[("source", source_lang.as_str())]);
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(TranslateError::ApiError(format!(
                "Google API error: {} - {}",
                status, body
            )));
        }

        let result: GoogleResponse = response.json().await?;

        result
            .data
            .translations
            .first()
            .map(|t| t.translated_text.clone())
            .ok_or_else(|| TranslateError::ApiError("No translation returned".to_string()))
    }
}

impl Default for GoogleTranslator {
    fn default() -> Self {
        Self::new()
    }
}
