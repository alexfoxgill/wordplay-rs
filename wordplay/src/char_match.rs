use crate::normalized_word::NormalizedChar;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum CharMatch {
    Only(NormalizedChar),
    Any,
}

impl From<char> for CharMatch {
    fn from(ch: char) -> Self {
        match ch {
            ' ' | '.' | '?' => CharMatch::Any,
            _ => CharMatch::Only(NormalizedChar::from_char(ch).expect("Unknown search char")),
        }
    }
}

impl CharMatch {
    pub fn matches(&self, ch: &NormalizedChar) -> bool {
        match self {
            CharMatch::Only(exp) => exp == ch,
            CharMatch::Any => true,
        }
    }
}
