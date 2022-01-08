use ascii::{AsciiChar, AsciiStr};

#[derive(Debug)]
struct CharFreq {
    pub freqs: [u8; 26],
}

impl CharFreq {
    fn from(word: &AsciiStr) -> CharFreq {
        let a = AsciiChar::A as u8;
        let z = AsciiChar::Z as u8;
        let mut freqs: [u8; 26] = [0; 26];
        for ch in word.chars() {
            let c8 = ch as u8;
            if a <= c8 && c8 <= z {
                let idx = c8 - a;
                freqs[idx as usize] += 1;
            }
        }
        CharFreq { freqs }
    }
}

#[cfg(test)]
mod tests {
    fn to_charfreq(word: &str) -> crate::CharFreq {
        use crate::CharFreq;
        let asc = ascii::AsciiStr::from_ascii(word).unwrap();
        CharFreq::from(asc)
    }

    #[test]
    fn charfreq_counts_A_once() {
        let freqs = to_charfreq("A");
        let mut expected: [u8; 26] = [0; 26];
        expected[0] = 1;
        assert_eq!(freqs.freqs, expected);
    }

    #[test]
    fn charfreq_counts_Z_once() {
        let freqs = to_charfreq("Z");
        let mut expected: [u8; 26] = [0; 26];
        expected[25] = 1;
        assert_eq!(freqs.freqs, expected);
    }
}
