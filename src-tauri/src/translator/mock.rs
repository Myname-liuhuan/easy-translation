use crate::translator::Translator;

/// Mock translator for testing and development.
pub struct MockTranslator;

impl Translator for MockTranslator {
    fn translate(&self, from: &str, to: &str, text: &str) -> String {
        // Mock implementation - returns formatted placeholder
        // In the future, this will be replaced with real API calls
        match (from, to) {
            ("zh", "en") => format!("[Translation: ZH→EN] {}", text),
            ("en", "zh") => format!("[Translation: EN→ZH] {}", text),
            _ => format!("[Translation: {}→{}] {}", from.to_uppercase(), to.to_uppercase(), text),
        }
    }
}
