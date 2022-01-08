#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
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
    pub fn from_char(ch: char) -> Option<NormalizedChar> {
        let ascii_ch = ch.to_ascii_uppercase();
        if 'A' <= ascii_ch && ascii_ch <= 'Z' {
            let u8_ch = (ascii_ch as u8) - ('A' as u8);
            return num::FromPrimitive::from_u8(u8_ch);
        }

        None
    }
}

#[derive(Debug, PartialEq)]
pub struct NormalizedWord {
    chars: Vec<NormalizedChar>,
}

impl NormalizedWord {
    pub fn new(chars: Vec<NormalizedChar>) -> NormalizedWord {
        NormalizedWord { chars }
    }

    pub fn from_str(str: &str) -> NormalizedWord {
        NormalizedWord {
            chars: str.chars().filter_map(NormalizedChar::from_char).collect(),
        }
    }

    pub fn iter_chars<'a>(&'a self) -> std::slice::Iter<'a, NormalizedChar> {
        self.chars.iter()
    }

    pub fn is_palindrome(self) -> bool {
        if self.chars.len() == 0 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use NormalizedChar::*;

    #[test]
    fn normalized_word_creates_from_ascii_uppercase() {
        let nw = NormalizedWord::from_str("ABC");

        let expected = NormalizedWord::new(vec![A, B, C]);

        assert_eq!(nw, expected);
    }

    #[test]
    fn normalized_word_creates_from_ascii_lowercase() {
        let nw = NormalizedWord::from_str("abc");

        let expected = NormalizedWord::new(vec![A, B, C]);

        assert_eq!(nw, expected);
    }

    #[test]
    fn normalized_word_ignores_non_letters() {
        let nw = NormalizedWord::from_str("A1B2C3");

        let expected = NormalizedWord::new(vec![A, B, C]);

        assert_eq!(nw, expected);
    }

    fn mk(str: &str) -> NormalizedWord {
        NormalizedWord::from_str(str)
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
}
