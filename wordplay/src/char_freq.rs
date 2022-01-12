use crate::char_map::CharMap;
use crate::normalized_word::*;
use strum::IntoEnumIterator;

type UFreq = u8;

#[derive(Debug, PartialEq)]
pub struct CharFreq {
    freqs: CharMap<UFreq>,
}

impl CharFreq {
    pub fn new(freqs: CharMap<UFreq>) -> CharFreq {
        CharFreq { freqs }
    }

    pub fn new_empty() -> CharFreq {
        CharFreq::new(Default::default())
    }

    pub fn get(&self, ch: NormalizedChar) -> UFreq {
        *self.freqs.get(ch)
    }

    pub fn set(&mut self, ch: NormalizedChar, value: UFreq) {
        self.freqs.set(ch, value)
    }

    pub fn update(&mut self, ch: NormalizedChar, f: fn(UFreq) -> UFreq) {
        self.set(ch, f(self.get(ch)))
    }

    pub fn from(word: &NormalizedWord) -> CharFreq {
        let mut res = CharFreq::new_empty();
        for &ch in word.iter_chars() {
            res.update(ch, |x| x + 1);
        }
        res
    }

    pub fn compare(self, other: &CharFreq) -> CharFreqComparisonResult {
        use CharFreqComparison::*;
        let mut comp = Same;
        let mut diff: CharMap<UFreq> = Default::default();
        for ch in NormalizedChar::iter() {
            let a = self.get(ch);
            let b = other.get(ch);

            if a == b {
                continue;
            }

            if a < b {
                if comp == Superset {
                    return CharFreqComparisonResult::Unrelated;
                }
                if comp == Same {
                    comp = Subset;
                }
                diff.set(ch, b - a);
            }

            if a > b {
                if comp == Subset {
                    return CharFreqComparisonResult::Unrelated;
                }
                if comp == Same {
                    comp = Superset;
                }
                diff.set(ch, a - b);
            }
        }

        match comp {
            Same => CharFreqComparisonResult::Same,
            Subset => CharFreqComparisonResult::Subset {
                diff: CharFreq::new(diff),
            },
            Superset => CharFreqComparisonResult::Superset {
                diff: CharFreq::new(diff),
            },
        }
    }
}

#[derive(Debug, PartialEq)]
enum CharFreqComparison {
    Same,
    Subset,
    Superset,
}

#[derive(Debug, PartialEq)]
pub enum CharFreqComparisonResult {
    Same,
    Unrelated,
    Subset { diff: CharFreq },
    Superset { diff: CharFreq },
}

#[cfg(test)]
mod tests {
    use super::*;
    use CharFreqComparisonResult::*;
    use NormalizedChar::*;

    fn to_charfreq(word: &str) -> CharFreq {
        let asc = NormalizedWord::from_str_safe(word);
        CharFreq::from(&asc)
    }

    #[test]
    fn charfreq_counts_a_once() {
        let freqs = to_charfreq("A");
        let mut expected = CharFreq::new_empty();
        expected.freqs.set(A, 1);
        assert_eq!(freqs, expected);
    }

    #[test]
    fn charfreq_counts_z_once() {
        let freqs = to_charfreq("Z");
        let mut expected = CharFreq::new_empty();
        expected.freqs.set(Z, 1);
        assert_eq!(freqs, expected);
    }

    #[test]
    fn charfreq_ignores_non_letter() {
        let freqs = to_charfreq("@");
        let expected = CharFreq::new_empty();
        assert_eq!(freqs, expected);
    }

    #[test]
    fn charfreq_comparison_identifies_same() {
        let a = to_charfreq("CAT");
        let b = to_charfreq("CAT");

        let res = a.compare(&b);

        assert_eq!(res, Same);
    }

    #[test]
    fn charfreq_comparison_identifies_same_scrambled() {
        let a = to_charfreq("CAT");
        let b = to_charfreq("ACT");

        let res = a.compare(&b);

        assert_eq!(res, Same);
    }

    #[test]
    fn charfreq_comparison_identifies_subset() {
        let a = to_charfreq("AT");
        let b = to_charfreq("CAT");

        let diff = to_charfreq("C");

        let res = a.compare(&b);

        assert_eq!(res, Subset { diff })
    }

    #[test]
    fn charfreq_comparison_identifies_subset_repeated() {
        let a = to_charfreq("ANNA");
        let b = to_charfreq("BANANA");

        let diff = to_charfreq("BA");

        let res = a.compare(&b);

        assert_eq!(res, Subset { diff })
    }

    #[test]
    fn charfreq_comparison_identifies_superset() {
        let a = to_charfreq("CAT");
        let b = to_charfreq("AT");

        let diff = to_charfreq("C");

        let res = a.compare(&b);

        assert_eq!(res, Superset { diff })
    }

    #[test]
    fn charfreq_comparison_identifies_superset_repeated() {
        let a = to_charfreq("BANANA");
        let b = to_charfreq("ANNA");

        let diff = to_charfreq("BA");

        let res = a.compare(&b);

        assert_eq!(res, Superset { diff })
    }

    #[test]
    fn charfreq_comparison_identifies_unrelated() {
        let a = to_charfreq("CAT");
        let b = to_charfreq("BAT");

        let res = a.compare(&b);

        assert_eq!(res, Unrelated)
    }
}
