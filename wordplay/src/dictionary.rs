use crate::char_freq::CharFreq;
use crate::normalized_word::NormalizedWord;
use crate::trie::Trie;

pub struct DictEntry {
    char_freq: CharFreq,
    original: String,
}

pub struct Dictionary {
    trie: Trie<DictEntry>,
}

impl Dictionary {
    pub fn add(&mut self, original: &str) {
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
}

impl Default for Dictionary {
    fn default() -> Dictionary {
        Dictionary {
            trie: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dictionary_builds() {
        let dict: Dictionary = Default::default();
    }
}
