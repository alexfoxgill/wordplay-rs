use crate::char_map::CharMap;
use crate::char_match::CharMatch;
use crate::normalized_word::*;
use std::collections::VecDeque;
use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::ops::RangeInclusive;

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
        self.add(&NormalizedWord::from_str_safe(str), value)
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
        TrieIter::new(self, Default::default())
    }

    pub fn iter_range(&self, range: RangeInclusive<usize>) -> TrieIter<T> {
        let search = TrieSearch {
            prefix: TriePrefix::any_with_length(*range.start()),
            max_depth: Some(*range.end()),
        };
        TrieIter::new(self, search)
    }

    pub fn iter_search(&self, search: TrieSearch) -> TrieIter<T> {
        TrieIter::new(self, search)
    }
}

impl<'a, T> Extend<(&'a NormalizedWord, T)> for Trie<T> {
    fn extend<It: IntoIterator<Item = (&'a NormalizedWord, T)>>(&mut self, iter: It) {
        for (nw, v) in iter {
            self.add(nw, v);
        }
    }
}

impl<'a, T> FromIterator<(&'a NormalizedWord, T)> for Trie<T> {
    fn from_iter<It: IntoIterator<Item = (&'a NormalizedWord, T)>>(iter: It) -> Self {
        let mut trie: Trie<T> = Default::default();
        trie.extend(iter);
        trie
    }
}

impl<'a, T> Extend<(&'a str, T)> for Trie<T> {
    fn extend<It: IntoIterator<Item = (&'a str, T)>>(&mut self, iter: It) {
        for (nw, v) in iter {
            self.add_string(nw, v);
        }
    }
}

impl<'a, T> FromIterator<(&'a str, T)> for Trie<T> {
    fn from_iter<It: IntoIterator<Item = (&'a str, T)>>(iter: It) -> Self {
        let mut trie: Trie<T> = Default::default();
        trie.extend(iter);
        trie
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

#[derive(Debug, PartialEq, Default, Clone)]
pub struct TriePrefix {
    chars: Vec<CharMatch>,
}

impl TriePrefix {
    pub fn new(chars: Vec<CharMatch>) -> Self {
        Self { chars }
    }

    pub fn any_with_length(len: usize) -> Self {
        Self {
            chars: vec![CharMatch::Any; len],
        }
    }

    pub fn len(&self) -> usize {
        self.chars.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn from_pattern(str: &str) -> Self {
        TriePrefix {
            chars: str.chars().map(CharMatch::from).collect(),
        }
    }

    pub fn get_char_restriction(&self, depth: usize) -> CharMatch {
        if depth < self.chars.len() {
            self.chars[depth]
        } else {
            CharMatch::Any
        }
    }
}

#[derive(Debug, PartialEq, Default, Clone)]
pub struct TrieSearch {
    prefix: TriePrefix,
    max_depth: Option<usize>,
}

impl TrieSearch {
    pub fn new(prefix: TriePrefix, max_depth: Option<usize>) -> Self {
        Self { prefix, max_depth }
    }

    pub fn from_prefix(str: &str) -> Self {
        TrieSearch {
            prefix: TriePrefix::from_pattern(str),
            ..Default::default()
        }
    }

    pub fn exactly(str: &str) -> Self {
        let search = TrieSearch::from_prefix(str);
        let len = search.prefix.len();
        search.with_max(len)
    }

    pub fn with_max(&self, max: usize) -> Self {
        TrieSearch {
            max_depth: Some(max),
            ..self.clone()
        }
    }

    pub fn below_max(&self, depth: usize) -> bool {
        self.max_depth.map_or(true, |m| depth < m)
    }

    pub fn get_char_restriction(&self, depth: usize) -> CharMatch {
        self.prefix.get_char_restriction(depth)
    }
}

pub struct TrieIter<'a, T> {
    search: TrieSearch,
    node_queue: VecDeque<(NormalizedWord, &'a Trie<T>)>,
    terminal_queue: VecDeque<(NormalizedWord, &'a T)>,
}

impl<'a, T> TrieIter<'a, T> {
    fn new(root: &'a Trie<T>, search: TrieSearch) -> TrieIter<'a, T> {
        let mut node_queue: VecDeque<_> = Default::default();
        node_queue.push_back((Default::default(), root));

        TrieIter {
            search,
            node_queue,
            terminal_queue: Default::default(),
        }
    }

    fn visit(&mut self, word: NormalizedWord, node: &'a Trie<T>) {
        let depth = word.len();

        let prefix_len = self.search.prefix.len();

        if prefix_len <= depth {
            self.terminal_queue
                .extend(node.terminals.iter().map(|t| (word.clone(), t)));
        }

        if self.search.below_max(depth) {
            let char_restriction = self.search.get_char_restriction(depth);

            let nodes = node
                .children
                .iter()
                .filter(|(ch, _)| char_restriction.matches(ch))
                .filter_map(|(ch, node_opt)| {
                    if let Some(x) = node_opt {
                        let mut child_word = word.clone();
                        child_word.push(ch);
                        Some((child_word, x.as_ref()))
                    } else {
                        None
                    }
                })
                .rev();

            self.node_queue.extend(nodes);
        }
    }
}

impl<'a, T> Iterator for TrieIter<'a, T> {
    type Item = (NormalizedWord, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(term) = self.terminal_queue.pop_front() {
            return Some(term);
        }

        if let Some((word, node)) = self.node_queue.pop_back() {
            self.visit(word, node);
            return self.next();
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_empty() {
        let trie: Trie<i32> = Default::default();

        assert_eq!(trie.iter().count(), 0)
    }

    #[test]
    fn add_single() {
        let mut trie: Trie<i32> = Default::default();

        let nw = "ABC".into();
        trie.add(&nw, 1);

        let res = trie.get(&nw);

        assert_eq!(res, Some(&vec![1]))
    }

    #[test]
    fn add_multiple() {
        let mut trie: Trie<i32> = Default::default();

        let nw = "ABC".into();
        trie.add(&nw, 1);
        trie.add(&nw, 2);

        let res = trie.get(&nw);

        assert_eq!(res, Some(&vec![1, 2]))
    }

    #[test]
    fn iterate_single() {
        let trie = Trie::from_iter(vec![("A", 1)]);

        let res: Vec<_> = trie.iter().collect();

        assert_eq!(res, [("A".into(), &1),])
    }

    #[test]
    fn iterate_multiple() {
        let trie = Trie::from_iter(vec![("A", 1), ("AB", 2), ("B", 3)]);

        let res: Vec<_> = trie.iter().collect();

        assert_eq!(res, [("A".into(), &1), ("AB".into(), &2), ("B".into(), &3)])
    }

    #[test]
    fn iterate_many() {
        let trie = Trie::from_iter(vec![("A", 1), ("AB", 2), ("B", 3), ("CDE", 4), ("CDE", 5)]);

        let res: Vec<_> = trie.iter().collect();

        assert_eq!(
            res,
            [
                ("A".into(), &1),
                ("AB".into(), &2),
                ("B".into(), &3),
                ("CDE".into(), &4),
                ("CDE".into(), &5),
            ]
        )
    }

    #[test]
    fn iterate_bound() {
        let trie = Trie::from_iter(vec![("A", 1), ("AB", 2), ("ABC", 3)]);

        let res: Vec<_> = trie.iter_range(2..=2).collect();

        assert_eq!(res, [("AB".into(), &2)])
    }

    #[test]
    fn iterate_prefix_search() {
        let trie = Trie::from_iter(vec![("BAT", ()), ("CAR", ()), ("CAT", ())]);

        let search = TrieSearch::from_prefix("CA");
        let res: Vec<_> = trie.iter_search(search).collect();

        assert_eq!(res, [("CAR".into(), &()), ("CAT".into(), &())])
    }

    #[test]
    fn iterate_prefix_exclude_shorter() {
        let trie = Trie::from_iter(vec![("C", ()), ("CAR", ())]);

        let search = TrieSearch::from_prefix("CA");
        let res: Vec<_> = trie.iter_search(search).collect();

        assert_eq!(res, [("CAR".into(), &()),])
    }

    #[test]
    fn iterate_wildcard_match() {
        let trie = Trie::from_iter(vec![("BAT", ()), ("CAR", ()), ("COT", ())]);

        let search = TrieSearch::from_prefix("?A");
        let res: Vec<_> = trie.iter_search(search).collect();

        assert_eq!(res, [("BAT".into(), &()), ("CAR".into(), &())])
    }
}
