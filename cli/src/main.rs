use itertools::Itertools;
use std::{
    cmp::Ordering,
    io::{self, stdin},
};

use wordplay_core::{
    anagram_number::AnagramNumber,
    dict_enable,
    dictionary::{DictEntry, DictIterItem, DictSearch, Dictionary, WordPredicate},
    normalized_word::NormalizedWord,
    trie::TrieSearch,
};

fn read_line() -> io::Result<String> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().into())
}

fn parse_line(str: &str) -> Option<Command> {
    if str == "q" || str == "quit" {
        return Some(Command::Quit);
    }

    if let Some(stripped) = str.strip_prefix("f ") {
        let mut prefix: String = "".into();
        let mut predicates: Vec<WordPredicate> = vec![];
        let mut sort: Option<Sort> = None;
        for cmd in stripped.split(',') {
            let cmd_parts: Vec<_> = cmd.trim().split(' ').collect();
            match cmd_parts.as_slice() {
                ["p", p] => prefix = String::from(*p),
                ["a", a] => {
                    let nw = NormalizedWord::from_str_safe(a);
                    let anag = AnagramNumber::try_from(&nw).unwrap();
                    predicates.push(WordPredicate::AnagramOf(anag))
                }
                ["a+", a] => {
                    let nw = NormalizedWord::from_str_safe(a);
                    let anag = AnagramNumber::try_from(&nw).unwrap();
                    predicates.push(WordPredicate::SuperanagramOf(anag))
                }
                ["a-", a] => {
                    let nw = NormalizedWord::from_str_safe(a);
                    let anag = AnagramNumber::try_from(&nw).unwrap();
                    predicates.push(WordPredicate::SubanagramOf(anag))
                }
                ["sort", s] => {
                    sort = Some(match *s {
                        "len" => Sort(SortAspect::Length, SortDirection::Ascending),
                        "len-" => Sort(SortAspect::Length, SortDirection::Descending),
                        "alph" => Sort(SortAspect::Alphabetical, SortDirection::Ascending),
                        "alph-" => Sort(SortAspect::Alphabetical, SortDirection::Descending),
                        _ => continue,
                    })
                }
                _ => (),
            }
        }
        return Some(Command::Find {
            prefix,
            predicate: WordPredicate::All(predicates),
            sort,
        });
    }

    None
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum SortAspect {
    Length,
    Alphabetical,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum SortDirection {
    Ascending,
    Descending,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Sort(SortAspect, SortDirection);

impl Sort {
    pub fn compare(&self, a: &DictIterItem, b: &DictIterItem) -> Ordering {
        let ordering = match self.0 {
            SortAspect::Length => a.normalized.len().cmp(&b.normalized.len()),
            SortAspect::Alphabetical => a.normalized.cmp(&b.normalized),
        };
        match self.1 {
            SortDirection::Ascending => ordering,
            SortDirection::Descending => ordering.reverse(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Command {
    Find {
        prefix: String,
        predicate: WordPredicate,
        sort: Option<Sort>,
    },
    Quit,
}

fn read_command() -> Option<Command> {
    println!("Enter command");
    let line = read_line().unwrap();
    parse_line(&line)
}

fn present<'a, It: Iterator<Item = DictIterItem<'a>>>(iter: It) {
    let res = iter.take(5);
    for x in res {
        println!("{}", x.original);
    }
}

fn command_loop(dict: Dictionary) {
    use Command::*;
    loop {
        let command = read_command();
        match command {
            Some(Quit) => {
                println!("Bye!");
                break;
            }
            Some(Find {
                prefix,
                predicate,
                sort,
            }) => {
                println!("Finding...");
                let trie_search = TrieSearch::from_prefix(&prefix);
                let search = DictSearch::new(Some(trie_search), predicate);
                let results = dict.iter_search(search);
                match sort {
                    Some(sort) => {
                        let sorted = results.sorted_by(|a, b| sort.compare(a, b));
                        present(sorted)
                    }
                    None => present(results),
                }
            }
            None => {
                println!("Unrecognised command")
            }
        }
    }
}

fn main() {
    println!("Loading...");
    let enable = dict_enable();
    command_loop(enable);
}
