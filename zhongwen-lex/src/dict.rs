use std::collections::HashSet;
use std::fs;
use std::io::Error;

/// A "common" CJK codepoint
///
/// https://stackoverflow.com/questions/1366068/whats-the-complete-range-for-chinese-characters-in-unicode
pub fn is_common_cjk_codepoint(codepoint: char) -> bool {
    ('\u{4E00}'..'\u{9FFF}').contains(&codepoint)
}

/// Read all Chinese words separated by whitespace from a file into a HashSet
pub fn from_file(path: &str) -> Result<HashSet<String>, Error> {
    Ok(fs::read_to_string(path)?
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<HashSet<String>>())
}

#[cfg(test)]
mod lex_test {
    use super::is_common_cjk_codepoint;

    #[test]
    fn is_common_cjk_codepoint_test() {
        assert!(is_common_cjk_codepoint('你'));
        assert!(!is_common_cjk_codepoint('a'));
    }
}
