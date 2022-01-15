use std::fs::File;

use dictionary::Dictionary;

extern crate num;
#[macro_use]
extern crate num_derive;

pub mod anagram_number;
pub mod char_freq;
pub mod char_map;
pub mod char_match;
pub mod dictionary;
pub mod normalized_word;
pub mod trie;

pub fn dict_enable() -> Dictionary {
    Dictionary::from_file(File::open("data/enable.txt").unwrap())
}
