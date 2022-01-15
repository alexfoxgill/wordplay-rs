use std::io::{self, stdin};

use wordplay_core::{
    dict_enable,
    dictionary::{DictSearch, Dictionary},
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

    let tokenized: Vec<_> = str.split(' ').collect();

    if tokenized.get(0) == Some(&"anag") {
        let &fodder = tokenized.get(1)?;
        return Some(Command::Anagram(fodder.into()));
    }

    None
}

#[derive(Debug, PartialEq, Clone)]
pub enum Command {
    Anagram(String),
    Quit,
}

fn read_command() -> Option<Command> {
    println!("Enter command");
    let line = read_line().unwrap();
    parse_line(&line)
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
            Some(Anagram(word)) => {
                println!("Finding anagrams...");
                let search = DictSearch::anagram_of(&word);
                let res = dict.iter_search(search).take(5);
                for x in res {
                    println!("{}", x.original);
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
