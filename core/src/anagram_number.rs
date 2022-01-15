use std::convert::TryFrom;

use crate::{char_map::CharMap, normalized_word::NormalizedWord};

type UnsignedAnag = u128;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct AnagramNumber(UnsignedAnag);

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AnagramComparison {
    Exact,
    Unrelated,
    Subset,
    Superset,
}

impl AnagramNumber {
    pub fn compare(&self, other: AnagramNumber) -> AnagramComparison {
        match (*self, other) {
            (a, b) if a == b => AnagramComparison::Exact,
            (a, b) if a < b && b.0 % a.0 == 0 => AnagramComparison::Superset,
            (a, b) if a > b && a.0 % b.0 == 0 => AnagramComparison::Subset,
            _ => AnagramComparison::Unrelated,
        }
    }
}

// more common letters are lower, to increase the maximum supported english word length
const PRIMES_MAP: CharMap<UnsignedAnag> = CharMap::new([
    5,   // A
    71,  // B
    41,  // C
    29,  // D
    2,   // E
    47,  // F
    61,  // G
    23,  // H
    11,  // I
    97,  // J
    79,  // K
    31,  // L
    43,  // M
    13,  // N
    7,   // O
    67,  // P
    89,  // Q
    19,  // R
    17,  // S
    3,   // T
    37,  // U
    73,  // V
    59,  // W
    83,  // X
    53,  // Y
    101, // Z
]);

#[derive(Debug, PartialEq)]
pub struct AnagramNumberOverflow;

impl<'a> TryFrom<&'a NormalizedWord> for AnagramNumber {
    type Error = AnagramNumberOverflow;

    fn try_from(word: &'a NormalizedWord) -> Result<Self, Self::Error> {
        let mut x: UnsignedAnag = 1;
        for &c in word.iter_chars() {
            x = x
                .checked_mul(*PRIMES_MAP.get(c))
                .ok_or(AnagramNumberOverflow)?
        }
        Ok(AnagramNumber(x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;
    use AnagramComparison::*;

    fn get_anag_num(str: &str) -> AnagramNumber {
        (&NormalizedWord::from_str_safe(str)).try_into().unwrap()
    }

    #[test]
    fn small_words_are_equal() {
        let a = get_anag_num("CAT");
        let b = get_anag_num("ACT");

        let res = a.compare(b);

        assert_eq!(res, Exact)
    }

    #[test]
    fn small_words_are_not_equal() {
        let a = get_anag_num("BAT");
        let b = get_anag_num("ACT");

        let res = a.compare(b);

        assert_eq!(res, Unrelated)
    }

    #[test]
    fn word_is_anagram_superset() {
        let a = get_anag_num("AT");
        let b = get_anag_num("CAT");

        let res = a.compare(b);

        assert_eq!(res, Superset)
    }

    #[test]
    fn word_is_anagram_subset() {
        let a = get_anag_num("CAT");
        let b = get_anag_num("AT");

        let res = a.compare(b);

        assert_eq!(res, Subset)
    }

    #[test]
    fn nineteen_letter_word_supported() {
        let n = get_anag_num("zzzzzzzzzzzzzzzzzzz");

        assert_eq!(n, AnagramNumber(120810895044353150938886048668570711901))
    }

    #[test]
    fn worst_case_twenty_letter_word_unsupported() {
        let n: Result<AnagramNumber, _> =
            (&NormalizedWord::from_str_safe("zzzzzzzzzzzzzzzzzzzz")).try_into();

        assert_eq!(n, Err(AnagramNumberOverflow))
    }
}
