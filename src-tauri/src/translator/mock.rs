use crate::translator::{TranslateError, Translator};

/// Mock translator for testing and development.
pub struct MockTranslator;

impl MockTranslator {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Translator for MockTranslator {
    fn name(&self) -> &'static str {
        "Mock"
    }

    fn is_available(&self) -> bool {
        true
    }

    async fn translate(&self, from: &str, to: &str, text: &str) -> Result<String, TranslateError> {
        // Mock implementation - returns formatted placeholder
        Ok(match (from, to) {
            ("zh", "en") => format!("[Mock ZH→EN] {}", text),
            ("en", "zh") => format!("[Mock EN→ZH] {}", text),
            _ => format!("[Mock {}→{}] {}", from.to_uppercase(), to.to_uppercase(), text),
        })
    }
}

impl Default for MockTranslator {
    fn default() -> Self {
        Self::new()
    }
}
