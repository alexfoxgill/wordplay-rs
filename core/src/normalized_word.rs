use std::{
    ops::Index,
    slice::{Iter, SliceIndex},
};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, FromPrimitive, EnumIter, PartialOrd, Ord)]
pub enum NormalizedChar {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

pub const ALPHABET_SIZE: usize = 26;

impl NormalizedChar {
    pub fn all() -> impl Iterator<Item = NormalizedChar> {
        NormalizedChar::iter()
    }

    pub fn from_char(ch: char) -> Option<NormalizedChar> {
        let ascii_ch = ch.to_ascii_uppercase();
        if ('A'..='Z').contains(&ascii_ch) {
            let u8_ch = (ascii_ch as u8) - b'A';
            return num::FromPrimitive::from_u8(u8_ch);
        }

        use NormalizedChar::*;

        let nc = match ch {
            'á' | 'Á' | 'â' | 'Â' | 'ä' | 'Ä' | 'à' | 'À' | 'ã' | 'Ã' | 'å' | 'Å' => A,
            'ç' | 'Ç' => C,
            'é' | 'É' | 'ê' | 'Ê' | 'ë' | 'Ë' | 'è' | 'È' => E,
            'í' | 'Í' | 'î' | 'Î' | 'ï' | 'Ï' | 'ì' | 'Ì' => I,
            'ñ' | 'Ñ' => N,
            'ó' | 'Ó' | 'ô' | 'Ô' | 'ö' | 'Ö' | 'ò' | 'Ò' | 'õ' | 'Õ' => O,
            'ú' | 'Ú' | 'û' | 'Û' | 'ü' | 'Ü' | 'ù' | 'Ù' => U,
            'ý' | 'Ý' => Y,
            _ => return None,
        };

        Some(nc)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, PartialOrd, Ord)]
pub struct NormalizedWord {
    chars: Vec<NormalizedChar>,
}

impl NormalizedWord {
    pub fn new(chars: Vec<NormalizedChar>) -> NormalizedWord {
        NormalizedWord { chars }
    }

    pub fn from_str_safe(str: &str) -> NormalizedWord {
        NormalizedWord {
            chars: str.chars().filter_map(NormalizedChar::from_char).collect(),
        }
    }

    pub fn len(&self) -> usize {
        self.chars.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push(&mut self, ch: NormalizedChar) {
        self.chars.push(ch)
    }

    pub fn iter_chars(&self) -> Iter<NormalizedChar> {
        self.chars.iter()
    }

    pub fn is_palindrome(self) -> bool {
        if self.is_empty() {
            return true;
        }

        let mut i = 0;
        let mut j = self.chars.len() - 1;
        while i < j {
            if self.chars[i] != self.chars[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }

        true
    }
}

impl From<&str> for NormalizedWord {
    fn from(str: &str) -> Self {
        NormalizedWord::from_str_safe(str)
    }
}

impl Default for NormalizedWord {
    fn default() -> NormalizedWord {
        NormalizedWord::new(Default::default())
    }
}

impl<Idx> Index<Idx> for NormalizedWord
where
    Idx: SliceIndex<[NormalizedChar]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.chars[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use NormalizedChar::*;

    #[test]
    fn creates_from_ascii_uppercase() {
        let nw = NormalizedWord::from_str_safe("ABC");

        let expected = NormalizedWord::new(vec![A, B, C]);

        assert_eq!(nw, expected);
    }

    #[test]
    fn creates_from_ascii_lowercase() {
        let nw = NormalizedWord::from_str_safe("abc");

        let expected = NormalizedWord::new(vec![A, B, C]);

        assert_eq!(nw, expected);
    }

    #[test]
    fn ignores_non_letters() {
        let nw = NormalizedWord::from_str_safe("A1B2C3");

        let expected = NormalizedWord::new(vec![A, B, C]);

        assert_eq!(nw, expected);
    }

    #[test]
    fn marshalls_accented_letters() {
        [
            ("áÁâÂäÄàÀãÃåÅ", "AAAAAAAAAAAA"),
            ("çÇ", "CC"),
            ("éÉêÊëËèÈ", "EEEEEEEE"),
            ("íÍîÎïÏìÌ", "IIIIIIII"),
            ("ñÑ", "NN"),
            ("óÓôÔöÖòÒõÕ", "OOOOOOOOOO"),
            ("úÚûÛüÜùÙ", "UUUUUUUU"),
            ("ýÝ", "YY"),
        ]
        .iter()
        .for_each(|(str, expected)| {
            assert_eq!(
                NormalizedWord::from_str_safe(str),
                NormalizedWord::from_str_safe(expected)
            )
        })
    }

    fn mk(str: &str) -> NormalizedWord {
        NormalizedWord::from_str_safe(str)
    }

    #[test]
    fn is_palindrome_returns_true_for_empty() {
        let nw = mk("");

        assert!(nw.is_palindrome())
    }

    #[test]
    fn is_palindrome_returns_true_for_single() {
        let nw = mk("A");

        assert!(nw.is_palindrome())
    }

    #[test]
    fn is_palindrome_returns_true_for_double() {
        let nw = mk("AA");

        assert!(nw.is_palindrome())
    }

    #[test]
    fn is_palindrome_returns_true_for_triple() {
        let nw = mk("AAA");

        assert!(nw.is_palindrome())
    }

    #[test]
    fn is_palindrome_returns_true_for_even() {
        let nw = mk("ABBA");

        assert!(nw.is_palindrome())
    }

    #[test]
    fn is_palindrome_returns_true_for_odd() {
        let nw = mk("ABA");

        assert!(nw.is_palindrome())
    }

    #[test]
    fn is_palindrome_returns_false_for_even() {
        let nw = mk("ABBC");

        assert!(!nw.is_palindrome())
    }

    #[test]
    fn is_palindrome_returns_false_for_odd() {
        let nw = mk("ABC");

        assert!(!nw.is_palindrome())
    }

    #[test]
    fn chars_can_be_iterated() {
        let len = NormalizedChar::all().count();

        assert_eq!(len, ALPHABET_SIZE)
    }
}
