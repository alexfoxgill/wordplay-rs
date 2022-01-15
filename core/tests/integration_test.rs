#[macro_use]
extern crate lazy_static;
use std::fs::File;

use wordplay_core::dictionary::{DictSearch, Dictionary};

#[cfg(test)]
mod enable_tests {
    use super::*;

    lazy_static! {
        static ref ENABLE: Dictionary = {
            let file = File::open("data/enable.txt").unwrap();
            Dictionary::from_file(file)
        };
    }

    #[test]
    fn file_loads_all_lines() {
        let dict_size = ENABLE.iter().count();
        assert_eq!(dict_size, 172820)
    }

    #[test]
    fn find_matching_words() {
        let search = DictSearch::from_pattern("?ana??");

        let mut iter = ENABLE.iter_search(search).map(|x| &x.original[..]);

        assert!(iter.any(|x| x == "banana"));
    }

    #[test]
    fn all_words_have_anagram_num() {
        for x in ENABLE.iter() {
            assert!(x.anag_num.is_some());
        }
    }
}
