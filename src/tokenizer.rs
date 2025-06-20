use unicode_normalization::UnicodeNormalization;
use lazy_static::lazy_static;
use std::collections::HashSet;

lazy_static! {
    static ref STOP_WORDS: HashSet<&'static str> = {
        ["e","de","o","a","os","as","para","em","um","uma","com"]
            .iter().copied().collect()
    };
}


pub fn tokenize(text: &str) -> Vec<String> {
    let cleaned: String = text
        .nfkd()
        .filter(|c| c.is_ascii())
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c
            } else if c.is_ascii_whitespace() {
                c
            } else {
                ' '
            }
        })
        .collect();
    cleaned
        .to_lowercase()
        .split_whitespace()
        .filter(|w| !STOP_WORDS.contains(*w))
        .map(str::to_string)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokenize() {
        let toks = tokenize("Mouse-Gamer! 100% TOP");
        assert_eq!(toks, vec!["mouse","gamer","100","top"]);
    }

    #[test]
    fn test_stop_words_and_accents() {
        let toks = tokenize("O Teclado Mecânico e o Cabo USB");
        assert_eq!(toks, vec!["teclado","mecanico","cabo","usb"]);
    }
}


