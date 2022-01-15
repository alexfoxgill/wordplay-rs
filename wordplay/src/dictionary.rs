use crate::anagram_number::AnagramNumber;
use crate::char_freq::CharFreq;
use crate::char_match::CharMatch;
use crate::normalized_word::{NormalizedChar, NormalizedWord};
use crate::trie::{Trie, TriePrefix, TrieSearch};
use std::convert::{TryFrom, TryInto};
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
        let prefix = search.prefix.unwrap_or_default();
        let trie_search = TrieSearch::new(prefix, search.max_length);
        let anag = search.anagram;

        self.trie
            .iter_search(trie_search)
            .map(DictIterItem::from)
            .filter(move |x| {
                match anag {
                    None => (),
                    exp if x.anag_num == exp => return true,
                    _ => return false,
                }

                true
            })
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

#[derive(Debug, PartialEq, Default)]
pub struct DictSearch {
    prefix: Option<TriePrefix>,
    anagram: Option<AnagramNumber>,
    max_length: Option<usize>,
}

impl DictSearch {
    pub fn from_pattern(pattern: &str) -> DictSearch {
        let prefix = TriePrefix::from_pattern(pattern);
        let max_length = prefix.len();
        DictSearch {
            prefix: Some(prefix),
            max_length: Some(max_length),
            ..Default::default()
        }
    }

    pub fn anagram_of(str: &str) -> DictSearch {
        let word = NormalizedWord::from_str_safe(str);
        let anagram: AnagramNumber = (&word).try_into().unwrap();
        let len = word.len();
        let prefix = TriePrefix::new(vec![CharMatch::Any; len]);
        DictSearch {
            prefix: Some(prefix),
            anagram: Some(anagram),
            max_length: Some(len),
        }
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

    #[test]
    fn search_anagram() {
        let dict = Dictionary::from_iter(vec!["cat", "bat", "bait", "at"]);

        let search = DictSearch::anagram_of("tab");
        let res: Vec<_> = dict.iter_search(search).map(|x| x.original).collect();

        assert_eq!(res, vec!["bat"])
    }
}
