use crate::anagram_number::AnagramNumber;
use crate::char_freq::CharFreq;
use crate::char_match::CharMatch;
use crate::normalized_word::NormalizedWord;
use crate::trie::{Trie, TriePrefix, TrieSearch};
use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

#[derive(Debug, Clone, PartialEq)]
pub struct DictEntry {
    pub char_freq: CharFreq,
    pub anag_num: Option<AnagramNumber>,
    pub original: String,
}

#[derive(Debug, Clone, PartialEq)]

pub struct DictIterItem<'a> {
    pub normalized: NormalizedWord,
    pub char_freq: &'a CharFreq,
    pub anag_num: Option<AnagramNumber>,
    pub original: &'a String,
}

impl<'a> From<(NormalizedWord, &'a DictEntry)> for DictIterItem<'a> {
    fn from((normalized, entry): (NormalizedWord, &'a DictEntry)) -> Self {
        DictIterItem {
            normalized,
            char_freq: &entry.char_freq,
            anag_num: entry.anag_num,
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
        let anag_num = AnagramNumber::try_from(&normalized).ok();
        let entry = DictEntry {
            char_freq,
            anag_num,
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

    pub fn iter_search(&self, search: DictSearch) -> impl Iterator<Item = DictIterItem> {
        let trie_search = search.to_trie_search();
        self.trie.iter_search(trie_search).map(|x| x.into())
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum StringMatchElement {
    Char(CharMatch),
    Any,
}

impl From<char> for StringMatchElement {
    fn from(ch: char) -> StringMatchElement {
        match ch {
            '*' => StringMatchElement::Any,
            x => StringMatchElement::Char(x.into()),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct StringMatch {
    elements: Vec<StringMatchElement>,
}

impl StringMatch {
    pub fn from_pattern(pattern: &str) -> StringMatch {
        StringMatch {
            elements: pattern.chars().map(|c| c.into()).collect(),
        }
    }
}

impl StringMatch {
    pub fn to_prefix(&self) -> (TriePrefix, &[StringMatchElement]) {
        let mut char_match = Vec::new();
        let mut i = 0;
        for c in self.elements.iter() {
            match c {
                StringMatchElement::Char(cm) => char_match.push(*cm),
                StringMatchElement::Any => break,
            }
            i += 1;
        }

        (TriePrefix::new(char_match), &self.elements[i..])
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct DictSearch {
    matches: Option<StringMatch>,
    anagram: Option<AnagramNumber>,
    min_length: Option<usize>,
    max_length: Option<usize>,
}

impl DictSearch {
    pub fn from_pattern(pattern: &str) -> DictSearch {
        let string_match = StringMatch::from_pattern(pattern);
        DictSearch {
            matches: Some(string_match),
            ..Default::default()
        }
    }

    pub fn to_trie_search(&self) -> TrieSearch {
        let prefix = self
            .matches
            .as_ref()
            .map(|m| m.to_prefix().0)
            .unwrap_or_default();

        TrieSearch::new(prefix, self.min_length, self.max_length)
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
