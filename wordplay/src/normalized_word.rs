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
    pub chars: Vec<NormalizedChar>,
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
}
