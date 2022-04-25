use crate::dict;
use std::collections::{HashMap, HashSet};
use std::io::Error;
use HSKLevel::*;

#[derive(Debug, Copy, Clone)]
pub enum HSKLevel {
    HSK1,
    HSK2,
    HSK3,
    HSK4,
    HSK5,
    HSK6,
    HSK7_9,
}

pub fn hsk_30_dictionary() -> Result<(HashSet<String>, HashMap<String, HSKLevel>), Error> {
    let levels_dicts = [
        (HSK1, dict::from_file("../data/hsk/hsk30-1.txt")),
        (HSK2, dict::from_file("../data/hsk/hsk30-2.txt")),
        (HSK3, dict::from_file("../data/hsk/hsk30-3.txt")),
        (HSK4, dict::from_file("../data/hsk/hsk30-4.txt")),
        (HSK5, dict::from_file("../data/hsk/hsk30-5.txt")),
        (HSK6, dict::from_file("../data/hsk/hsk30-6.txt")),
        (HSK7_9, dict::from_file("../data/hsk/hsk30-7-9.txt")),
    ];

    let mut all_words = HashSet::<String>::default();
    let mut level_dict = HashMap::<String, HSKLevel>::default();
    for (level, dict) in levels_dicts {
        let dict = dict?;
        for word in dict.iter() {
            level_dict.insert(word.clone(), level);
        }
        all_words.extend(dict);
    }

    Ok((all_words, level_dict))
}
