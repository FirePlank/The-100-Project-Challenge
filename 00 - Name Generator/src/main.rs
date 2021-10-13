use names::{Generator, Name};
use rand::seq::IteratorRandom;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// A function to return a random line from a file
fn find_word(f_name: &str) -> String {
    let f = File::open(f_name)
        .unwrap_or_else(|e| panic!("(;_;) file not found: {}: {}", f_name, e));
    let f = BufReader::new(f);

    let lines = f.lines().map(|l| l.expect("Couldn't read line"));

    lines
        .choose(&mut rand::thread_rng())
        .expect("File had no lines")
}

// Converts the first letter of a string to uppercase
fn capitalize (s1: String) -> String {
  let mut c = s1.chars();
  match c.next() {
    None => String::new(),
    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
  }
}

fn main() {
    lazy_generator();
    custom_generator();
}

// A lazy implementation of a name generator that uses a dependency to do all the work
fn lazy_generator() {
    let mut generator = Generator::with_naming(Name::Plain);
    println!("The generated name is: {}", generator.next().unwrap());
}

// A home-made name generator
fn custom_generator() {
    let adjective = find_word("adjectives.txt");
    let noun = find_word("nouns.txt");
    println!("The generated name is: {}", capitalize(adjective) + " " + &noun);
}