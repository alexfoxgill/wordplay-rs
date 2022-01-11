use crate::char_freq::CharFreq;
use crate::normalized_word::NormalizedWord;
use crate::trie::Trie;

pub struct CorpusEntry {
    char_freq: CharFreq,
    original: String,
}

pub struct Corpus {
    trie: Trie<CorpusEntry>,
}

impl Corpus {
    pub fn add(&mut self, original: String) {
        let normalized = NormalizedWord::from_str(&original);
        let char_freq = CharFreq::from(&normalized);
        let entry = CorpusEntry {
            char_freq,
            original,
        };
        self.trie.add(&normalized, entry);
    }

    pub fn find(&self, word: &NormalizedWord) -> Option<&Vec<CorpusEntry>> {
        self.trie.get(word)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn corpus_builds() {
        assert_eq!(1, 1)
    }
}
