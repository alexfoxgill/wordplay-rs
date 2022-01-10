use crate::normalized_word::NormalizedChar;

#[derive(Debug, PartialEq)]
pub struct CharMap<T> {
    array: [T; 26],
}

impl<T> CharMap<T> {
    pub fn get(&self, ch: NormalizedChar) -> &T {
        &self.array[ch as usize]
    }

    pub fn get_mut(&mut self, ch: NormalizedChar) -> &mut T {
        &mut self.array[ch as usize]
    }

    pub fn set(&mut self, ch: NormalizedChar, t: T) {
        self.array[ch as usize] = t;
    }

    pub fn iter(&self) -> impl Iterator<Item = (NormalizedChar, &T)> {
        self.array.iter().enumerate().map(|(char_int, value)| {
            let char: NormalizedChar = num::FromPrimitive::from_usize(char_int).unwrap();
            (char, value)
        })
    }

    pub fn iter_values(&self) -> impl Iterator<Item = &T> {
        self.array.iter()
    }
}

impl<T: Default> Default for CharMap<T> {
    fn default() -> CharMap<T> {
        let array: [T; 26] = Default::default();
        CharMap { array }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use NormalizedChar::*;

    #[test]
    fn initialises_empty() {
        let map: CharMap<i32> = Default::default();

        map.iter_values().for_each(|&x| assert_eq!(x, 0));
    }

    #[test]
    fn sets_value() {
        let mut map: CharMap<i32> = Default::default();
        map.set(A, 1);

        assert_eq!(map.get(A), &1);
    }

    #[test]
    fn updates_value() {
        let mut map: CharMap<i32> = Default::default();
        let x = map.get_mut(A);
        *x = 1;

        assert_eq!(map.get(A), &1);
    }
}
