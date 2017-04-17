#[macro_use]
extern crate clap;
use clap::App;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let filenames: Vec<&str> = matches.values_of("INPUT").unwrap().collect();
    for filename in filenames {
        let file = File::open(filename).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();
        if matches.is_present("characters") {
            println!("{}", char_count(&contents))
        }
        if matches.is_present("words") {
            println!("{}", word_count(&contents))
        }
        if matches.is_present("lines") {
            println!("{}", line_count(&contents))
        }
    }
}

fn char_count(contents: &str) -> usize {
    contents.len()
}

fn word_count(contents: &str) -> usize {
    contents.split_whitespace().count()
}

fn line_count(contents: &str) -> usize {
    contents.split('\n').count()
}