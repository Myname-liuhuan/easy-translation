pub mod mock;

/// Translator trait for swappable translation implementations.
pub trait Translator: Send + Sync {
    /// Translate text from source language to target language.
    fn translate(&self, from: &str, to: &str, text: &str) -> String;
}
