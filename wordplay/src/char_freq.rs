use ascii::{AsciiChar, AsciiStr};

#[derive(Debug, PartialEq)]
pub struct CharFreq {
    pub freqs: [u8; 26],
}

impl CharFreq {
    pub fn new(freqs: [u8; 26]) -> CharFreq {
        CharFreq { freqs }
    }

    pub fn new_empty() -> CharFreq {
        CharFreq { freqs: [0; 26] }
    }

    pub fn set(&mut self, ch: AsciiChar, value: u8) {
        let idx = (ch as u8) - (AsciiChar::A as u8);
        self.freqs[idx as usize] = value;
    }

    pub fn update(&mut self, ch: AsciiChar, f: fn(u8) -> u8) {
        let idx = (ch as u8) - (AsciiChar::A as u8);
        let curr = self.freqs[idx as usize];
        self.freqs[idx as usize] = f(curr);
    }

    pub fn from(word: &AsciiStr) -> CharFreq {
        let a = AsciiChar::A as u8;
        let z = AsciiChar::Z as u8;
        let mut res = CharFreq::new_empty();
        for ch in word.chars() {
            let c8 = ch as u8;
            if a <= c8 && c8 <= z {
                res.update(ch, |x| x + 1);
            }
        }
        res
    }

    pub fn compare(self, other: &CharFreq) -> CharFreqComparisonResult {
        use CharFreqComparison::*;
        let mut comp = Same;
        let mut diff: [u8; 26] = [0; 26];
        for i in 0..26 {
            let a = self.freqs[i];
            let b = other.freqs[i];

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
                diff[i] = b - a;
            }

            if a > b {
                if comp == Subset {
                    return CharFreqComparisonResult::Unrelated;
                }
                if comp == Same {
                    comp = Superset;
                }
                diff[i] = a - b;
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
    use crate::char_freq::CharFreq;
    use crate::char_freq::CharFreqComparisonResult::*;
    use ascii::AsciiStr;

    fn to_charfreq(word: &str) -> CharFreq {
        let asc = AsciiStr::from_ascii(word).unwrap();
        CharFreq::from(asc)
    }

    #[test]
    fn charfreq_counts_a_once() {
        let freqs = to_charfreq("A");
        let mut expected = CharFreq::new_empty();
        expected.freqs[0] = 1;
        assert_eq!(freqs, expected);
    }

    #[test]
    fn charfreq_counts_z_once() {
        let freqs = to_charfreq("Z");
        let mut expected = CharFreq::new_empty();
        expected.freqs[25] = 1;
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
