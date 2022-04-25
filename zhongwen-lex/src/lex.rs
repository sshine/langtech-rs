use std::cmp::min;
use std::collections::HashSet;

pub fn lex(input: &str, dictionary: &HashSet<String>) -> Vec<String> {
    let longest_word = dictionary
        .iter()
        .map(|s| s.chars().count())
        .max()
        .unwrap_or(0);
    let input_chars = str::chars(input).collect::<Vec<_>>();
    let mut tokens = vec![];

    let mut cur_pos = 0;
    while cur_pos < input_chars.len() {
        match longest_prefix(cur_pos, &input_chars, dictionary, longest_word) {
            Some(word) => {
                cur_pos += word.chars().count();
                tokens.push(word);
            }
            None => cur_pos += 1,
        }
    }

    tokens
}

fn longest_prefix<'a>(
    cur_start: usize,
    input_chars: &[char],
    dictionary: &HashSet<String>,
    longest_word: usize,
) -> Option<String> {
    let end_of_search = min(input_chars.len(), cur_start + longest_word);
    let mut cur_end = cur_start + 1;

    println!("longest_word = {}", longest_word);
    let mut best_match = None;

    while cur_end < end_of_search {
        let word: String = input_chars[cur_start..cur_end].into_iter().collect();
        println!(": {}", word);
        if dictionary.contains(&word) {
            best_match = Some(word);
        }
        cur_end += 1;
    }

    best_match
}

#[cfg(test)]
mod lex_test {
    use super::lex;
    use crate::dict;

    #[test]
    fn lex_test() {
        let hsk_1 = dict::from_file("../data/hsk/hsk30-1.txt").unwrap();
        let input = "你好，朋友！";
        let expected = vec!["你", "好", "朋友"];
        let actual = lex(input, &hsk_1);

        assert_eq!(expected, actual);
    }
}
