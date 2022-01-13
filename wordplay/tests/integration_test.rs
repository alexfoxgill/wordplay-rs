#[macro_use]
extern crate lazy_static;
use std::fs::File;

use wordplay::{dictionary::Dictionary, trie::TrieSearch};

lazy_static! {
    static ref DICT: Dictionary = {
        let file = File::open("data/enable.txt").unwrap();
        Dictionary::from_file(file)
    };
}

#[test]
fn file_loads_all_lines() {
    let dict_size = DICT.iter().count();
    assert_eq!(dict_size, 172820)
}

#[test]
fn find_matching_words() {
    let search = TrieSearch::from_prefix("?ana").with_min(6).with_max(6);

    let mut iter = DICT.iter_search(search).map(|x| &x.original[..]);

    assert!(iter.any(|x| x == "banana"));
}

#[test]
fn all_words_have_anagram_num() {
    for x in DICT.iter() {
        assert!(x.anag_num.is_some());
    }
}
