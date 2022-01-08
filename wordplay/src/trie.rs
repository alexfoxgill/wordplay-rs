use crate::char_map::CharMap;
use crate::normalized_word::*;

#[derive(Debug, PartialEq)]
pub struct Trie<T> {
    children: CharMap<Option<Box<Trie<T>>>>,
    terminals: Vec<T>,
}

impl<T> Trie<T> {
    pub fn empty() -> Trie<T> {
        Default::default()
    }

    fn get_or_create_mut(&mut self, child: NormalizedChar) -> &mut Trie<T> {
        let relation: &mut Option<Box<Trie<T>>> = self.children.get_mut(child);
        if relation.is_none() {
            *relation = Some(Box::new(Trie::empty()));
        }

        let boxed: &mut Box<Trie<T>> = relation.as_mut().unwrap();
        let res: &mut Trie<T> = &mut *boxed;
        res
    }

    pub fn add(&mut self, key: &NormalizedWord, value: T) {
        let mut node: &mut Trie<T> = self;
        for &ch in key.iter_chars() {
            node = node.get_or_create_mut(ch);
        }

        node.terminals.push(value)
    }

    pub fn get(&self, key: &NormalizedWord) -> Option<&Vec<T>> {
        let mut node: &Trie<T> = self;
        for &ch in key.iter_chars() {
            let child = node.children.get(ch);
            match child {
                None => return None,
                Some(x) => node = x,
            }
        }

        Some(&node.terminals)
    }
}

impl<T> Default for Trie<T> {
    fn default() -> Trie<T> {
        Trie {
            children: Default::default(),
            terminals: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_empty() {
        let trie: Trie<i32> = Default::default();

        for child in trie.children.iter() {
            assert_eq!(child, &None)
        }

        assert_eq!(trie.terminals.len(), 0)
    }

    #[test]
    fn add_single() {
        let mut trie: Trie<i32> = Default::default();

        let nw = NormalizedWord::from_str("ABC");
        trie.add(&nw, 1);

        let res = trie.get(&nw);

        assert_eq!(res, Some(&vec![1]))
    }
}
