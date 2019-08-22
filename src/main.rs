use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use rand::prelude::*;

fn readdict(dictfile: &str) -> Vec<String> {
    let path = Path::new(dictfile);
    let file = match File::open(path) {
        Err(why) => panic!("Cant open {}: {}", path.display(), why.description()),
        Ok(file) => file
    };

    let mut words: Vec<String> = Vec::with_capacity(10000);
    for line in io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            words.push(line);
        }
    }

    words
}

fn subset(dict: &Vec<String>, len: usize) -> Vec<&String> {
    let mut words: Vec<&String> = Vec::new();

    /*
    for w in dict.iter() {
        if w.len() == len {
            words.push(&w);
        }
    }
    */
    for word in dict.iter().filter(|w| {
            w.len() == len &&
            w.chars().all(char::is_alphabetic) &&
            //w.chars().next().unwrap().is_lowercase()
            w.chars().nth(0).unwrap().is_lowercase()
        } ) {
        words.push(word);
    }

    words
}

fn guess(word: &String) -> bool {
    let mut lives = 5;
    let mut letters: Vec<char> = Vec::new();

    while lives > 0 {
        let display: String = word.chars().map(|c| {
            match letters.contains(&c) {
                true => c,
                false => '?'
            }
        }).collect();

        if display == *word {
            println!("You guesses it!");
            return true;
        }

        println!("Guess the word: {}", display);
        println!("Enter character [Tries={}]:", lives);

        let mut buf = String::new();
        match io::stdin().read_line(&mut buf) {
            Ok(_) => {},
            Err(_) => {
                    println!("Error reading from stdin...");
                    continue;
            }
        }

        let buf = buf.trim();
        if buf.len() != 1 {
            println!("Enter a single character please");
            continue;
        }
        let guess = buf.chars().next().unwrap();
        println!("You guessed {}", guess);

        letters.push(guess);

        if ! word.contains(guess) { lives -= 1 }
    }

    println!("Sorry - you used all your lives. The word was: {}", word);

    false
}

fn main() {
    println!("Reading dictionary");
    let dict = readdict("/usr/share/dict/linux.words");
    println!("Dictionary contains {} words", dict.len());

    loop {
        println!("Enter a required word length (q to quit)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read stdin");
        input = input.trim().to_string();
        println!("Input length {}", input.len());
        if input.len() == 1 && input.starts_with("q") { break; }

        let wordlen: usize;
        match input.trim().parse() {
            Ok(v) => wordlen = v,
            Err(_) => {
                println!("Not a valid number! Try again.");
                continue;
            }
        }

        let subdict = subset(&dict, wordlen);
        if subdict.len() == 0 {
            println!("No words of that length found!");
            continue;
        }

        let i = rand::thread_rng().gen_range(0, subdict.len());
        println!("{} of these are {} letter words", subdict.len(), wordlen);
        println!("Here is a randon one: {}\n", subdict[i]);

        guess(subdict[i]);
    }
}
