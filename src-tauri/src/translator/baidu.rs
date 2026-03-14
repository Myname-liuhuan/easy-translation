use crate::translator::{TranslateError, Translator};
use reqwest::Client;
use serde::Deserialize;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

const API_URL: &str = "https://fanyi-api.baidu.com/api/trans/vip/translate";

/// Baidu translation service.
pub struct BaiduTranslator {
    client: Client,
    app_id: Option<String>,
    secret_key: Option<String>,
}

#[derive(Deserialize)]
struct BaiduResponse {
    #[serde(default)]
    trans_result: Option<Vec<BaiduTransResult>>,
    #[serde(default)]
    error_code: Option<String>,
    #[serde(default)]
    error_msg: Option<String>,
}

#[derive(Deserialize)]
struct BaiduTransResult {
    dst: String,
}

impl BaiduTranslator {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            app_id: env::var("BAIDU_APP_ID").ok(),
            secret_key: env::var("BAIDU_SECRET_KEY").ok(),
        }
    }

    /// Generate MD5 hash for Baidu API signature.
    fn md5(input: &str) -> String {
        let digest = md5::compute(input.as_bytes());
        format!("{:x}", digest)
    }

    /// Generate sign for Baidu API.
    fn generate_sign(app_id: &str, query: &str, salt: &str, secret_key: &str) -> String {
        let sign_str = format!("{}{}{}{}", app_id, query, salt, secret_key);
        Self::md5(&sign_str)
    }

    /// Convert language code to Baidu format.
    fn to_baidu_lang(lang: &str) -> &str {
        match lang {
            "zh" => "zh",   // Chinese
            "en" => "en",   // English
            "ja" => "jp",   // Japanese
            "ko" => "kor",  // Korean
            "fr" => "fra",  // French
            "de" => "de",   // German
            "es" => "spa",  // Spanish
            "pt" => "pt",   // Portuguese
            "it" => "it",   // Italian
            "ru" => "ru",   // Russian
            _ => "auto",    // Auto-detect
        }
    }
}

#[async_trait::async_trait]
impl Translator for BaiduTranslator {
    fn name(&self) -> &'static str {
        "Baidu"
    }

    fn is_available(&self) -> bool {
        self.app_id.is_some() && self.secret_key.is_some()
    }

    async fn translate(&self, from: &str, to: &str, text: &str) -> Result<String, TranslateError> {
        let app_id = self.app_id.as_ref().ok_or_else(|| {
            TranslateError::ConfigError("BAIDU_APP_ID not configured".to_string())
        })?;
        let secret_key = self.secret_key.as_ref().ok_or_else(|| {
            TranslateError::ConfigError("BAIDU_SECRET_KEY not configured".to_string())
        })?;

        let salt = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string();

        let sign = Self::generate_sign(app_id, text, &salt, secret_key);

        let source_lang = if from == "other" { "auto" } else { Self::to_baidu_lang(from) };
        let target_lang = Self::to_baidu_lang(to);

        let response = self
            .client
            .get(API_URL)
            .query(&[
                ("q", text),
                ("from", source_lang),
                ("to", target_lang),
                ("appid", app_id),
                ("salt", &salt),
                ("sign", &sign),
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(TranslateError::ApiError(format!(
                "Baidu API error: {} - {}",
                status, body
            )));
        }

        let result: BaiduResponse = response.json().await?;

        // Check for API error
        if let Some(error_code) = result.error_code {
            return Err(TranslateError::ApiError(format!(
                "Baidu API error {}: {}",
                error_code,
                result.error_msg.unwrap_or_default()
            )));
        }

        result
            .trans_result
            .and_then(|r| r.first().map(|t| t.dst.clone()))
            .ok_or_else(|| TranslateError::ApiError("No translation returned".to_string()))
    }
}

impl Default for BaiduTranslator {
    fn default() -> Self {
        Self::new()
    }
}
