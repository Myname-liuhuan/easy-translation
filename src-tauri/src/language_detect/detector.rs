use whatlang::{detect, Lang};

/// Detect the language of the given text.
/// Returns "zh" for Chinese, "en" for English, "other" for other languages.
pub fn detect_language(text: &str) -> String {
    let info = match detect(text) {
        Some(info) => info,
        None => return "other".to_string(),
    };

    match info.lang() {
        Lang::Cmn => "zh".to_string(),
        Lang::Eng => "en".to_string(),
        _ => "other".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_chinese() {
        assert_eq!(detect_language("你好世界"), "zh");
        assert_eq!(detect_language("这是一个测试"), "zh");
    }

    #[test]
    fn test_detect_english() {
        assert_eq!(detect_language("Hello world"), "en");
        assert_eq!(detect_language("This is a test"), "en");
    }

    #[test]
    fn test_detect_other() {
        assert_eq!(detect_language("Bonjour le monde"), "other");
        assert_eq!(detect_language("Hola mundo"), "other");
    }
}
