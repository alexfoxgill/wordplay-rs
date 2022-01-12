use crate::char_freq::CharFreq;
use crate::normalized_word::NormalizedWord;
use crate::trie::{Trie, TrieSearch};

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

#[derive(Debug, Clone, PartialEq)]
pub struct DictEntry {
    pub char_freq: CharFreq,
    pub original: String,
}

#[derive(Debug, Clone, PartialEq)]

pub struct DictIterItem<'a> {
    pub normalized: NormalizedWord,
    pub char_freq: &'a CharFreq,
    pub original: &'a String,
}

impl<'a> From<(NormalizedWord, &'a DictEntry)> for DictIterItem<'a> {
    fn from((normalized, entry): (NormalizedWord, &'a DictEntry)) -> Self {
        DictIterItem {
            normalized,
            char_freq: &entry.char_freq,
            original: &entry.original,
        }
    }
}

#[derive(Default)]
pub struct Dictionary {
    trie: Trie<DictEntry>,
}

impl Dictionary {
    pub fn from_file(file: File) -> Dictionary {
        let reader = BufReader::new(file);
        let lines = reader.lines().map(|l| l.unwrap());
        let mut dict: Dictionary = Default::default();
        for line in lines {
            dict.insert(&line);
        }
        dict
    }

    pub fn insert(&mut self, original: &str) {
        let normalized = NormalizedWord::from_str_safe(original);
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

    pub fn iter(&self) -> impl Iterator<Item = DictIterItem> {
        self.trie.iter().map(|x| x.into())
    }

    pub fn iter_search(&self, search: TrieSearch) -> impl Iterator<Item = DictIterItem> {
        self.trie.iter_search(search).map(|x| x.into())
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

        let nw = NormalizedWord::from_str_safe("test");
        let res = dict.find(&nw);
        assert!(res.is_some())
    }

    #[test]
    fn extend() {
        let mut dict: Dictionary = Default::default();
        dict.extend(vec!["test", "foo"]);

        let nw = NormalizedWord::from_str_safe("test");
        let res = dict.find(&nw);
        assert!(res.is_some());

        let nw = NormalizedWord::from_str_safe("foo");
        let res = dict.find(&nw);
        assert!(res.is_some())
    }

    #[test]
    fn from_iter() {
        let dict = Dictionary::from_iter(vec!["test", "foo"]);

        let nw = NormalizedWord::from_str_safe("test");
        let res = dict.find(&nw);
        assert!(res.is_some());

        let nw = NormalizedWord::from_str_safe("foo");
        let res = dict.find(&nw);
        assert!(res.is_some())
    }
}
