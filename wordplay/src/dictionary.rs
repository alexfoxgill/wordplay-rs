use crate::char_freq::CharFreq;
use crate::normalized_word::NormalizedWord;
use crate::trie::Trie;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

pub struct DictEntry {
    char_freq: CharFreq,
    original: String,
}

pub struct Dictionary {
    trie: Trie<DictEntry>,
}

impl Dictionary {
    pub fn insert(&mut self, original: &str) {
        let normalized = NormalizedWord::from_str(original);
        let char_freq = CharFreq::from(&normalized);
        let entry = DictEntry {
            char_freq,
            original: String::from(original),
        };
        self.trie.add(&normalized, entry);
    }

    pub fn find(&self, word: &NormalizedWord) -> Option<&Vec<DictEntry>> {
        self.trie.get(word)
    }

    pub fn iter(&self) -> impl Iterator<Item = (NormalizedWord, &DictEntry)> {
        self.trie.iter()
    }
}

impl Default for Dictionary {
    fn default() -> Dictionary {
        Dictionary {
            trie: Default::default(),
        }
    }
}

impl<'a> Extend<&'a str> for Dictionary {
    fn extend<T: IntoIterator<Item = &'a str>>(&mut self, iter: T) {
        for str in iter {
            self.insert(str);
        }
    }
}

impl<'a> FromIterator<&'a str> for Dictionary {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut dict: Dictionary = Default::default();
        dict.extend(iter);
        dict
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut dict: Dictionary = Default::default();
        dict.insert("test");

        let nw = NormalizedWord::from_str("test");
        let res = dict.find(&nw);
        assert!(res.is_some())
    }

    #[test]
    fn extend() {
        let mut dict: Dictionary = Default::default();
        dict.extend(vec!["test", "foo"]);

        let nw = NormalizedWord::from_str("test");
        let res = dict.find(&nw);
        assert!(res.is_some());

        let nw = NormalizedWord::from_str("foo");
        let res = dict.find(&nw);
        assert!(res.is_some())
    }

    #[test]
    fn from_iter() {
        let dict = Dictionary::from_iter(vec!["test", "foo"]);

        let nw = NormalizedWord::from_str("test");
        let res = dict.find(&nw);
        assert!(res.is_some());

        let nw = NormalizedWord::from_str("foo");
        let res = dict.find(&nw);
        assert!(res.is_some())
    }

    // #[test]
    fn large_file() {
        let file = File::open("data/enable.txt").unwrap();
        let mut reader = BufReader::new(file);
        let lines = reader.lines().map(|l| l.unwrap());
        let mut dict: Dictionary = Default::default();
        for line in lines {
            dict.insert(&line);
        }
    }
}
