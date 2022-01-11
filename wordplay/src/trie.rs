use crate::char_map::CharMap;
use crate::normalized_word::*;
use std::collections::VecDeque;

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

    pub fn add_string(&mut self, str: &str, value: T) {
        self.add(&NormalizedWord::from_str(str), value)
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

    pub fn iter(&self) -> impl Iterator<Item = (NormalizedWord, &T)> {
        TrieIter::new(self)
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

pub struct TrieIter<'a, T> {
    current_word: NormalizedWord,
    node_queue: VecDeque<(NormalizedWord, &'a Trie<T>)>,
    terminal_queue: VecDeque<&'a T>,
}

impl<'a, T> TrieIter<'a, T> {
    fn new(root: &'a Trie<T>) -> TrieIter<'a, T> {
        let mut node_queue: VecDeque<(NormalizedWord, &Trie<T>)> = Default::default();
        node_queue.push_back((Default::default(), root));

        TrieIter {
            current_word: Default::default(),
            node_queue,
            terminal_queue: Default::default(),
        }
    }

    fn visit(&mut self, word: NormalizedWord, node: &'a Trie<T>) {
        self.terminal_queue.extend(node.terminals.iter());

        let nodes = node.children.iter_rev().filter_map(|(ch, node_opt)| {
            if let Some(x) = node_opt {
                let mut child_word = word.clone();
                child_word.push(ch);
                Some((child_word, x.as_ref()))
            } else {
                None
            }
        });

        self.node_queue.extend(nodes);

        self.current_word = word;
    }
}

impl<'a, T> Iterator for TrieIter<'a, T> {
    type Item = (NormalizedWord, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(term) = self.terminal_queue.pop_front() {
            return Some((self.current_word.clone(), term));
        }

        if let Some((word, node)) = self.node_queue.pop_back() {
            self.visit(word, node);
            return self.next();
        }

        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_empty() {
        let trie: Trie<i32> = Default::default();

        for child in trie.children.iter_values() {
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

    #[test]
    fn add_multiple() {
        let mut trie: Trie<i32> = Default::default();

        let nw = NormalizedWord::from_str("ABC");
        trie.add(&nw, 1);
        trie.add(&nw, 2);

        let res = trie.get(&nw);

        assert_eq!(res, Some(&vec![1, 2]))
    }

    #[test]
    fn iterate() {
        let mut trie: Trie<i32> = Default::default();

        trie.add_string("A", 1);
        trie.add_string("AB", 2);
        trie.add_string("B", 3);

        let res: Vec<_> = trie.iter().collect();

        assert_eq!(
            res,
            vec![
                (NormalizedWord::from_str("A"), &1),
                (NormalizedWord::from_str("AB"), &2),
                (NormalizedWord::from_str("B"), &3)
            ]
        )
    }
}
