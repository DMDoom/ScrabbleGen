use std::io::{self, BufRead};
use std::fs::File;
use std::collections::HashSet;

struct Assembler {
    word_bank: HashSet<String>
}

impl Assembler {
    fn build_assembler() -> Assembler {
        Assembler {
            word_bank: Self::load_dictionary()
        }
    }
    
    // Load word dictionary to memory
    fn load_dictionary() -> HashSet<String> {
        println!("Opening file...");

        let file: File = match File::open("slowa.txt") {
            Result::Ok(num) => num,
            Result::Err(_) => panic!("File not found.")
        };
    
        println!("Loading dictionary...");

        let buf_reader = io::BufReader::new(file);

        let mut word_bank: HashSet<String> = HashSet::new();
        for line in buf_reader.lines() {
            word_bank.insert(line.unwrap());
        }
        
        println!("Dictionary loaded.");
        return word_bank;
    }
}

fn main() {
    let assembler: Assembler = Assembler::build_assembler();

    loop {
        // Read user input
        println!("Input letters: ");
        let mut letters: String = String::new();

        io::stdin()
            .read_line(&mut letters)
            .expect("Unexpected error");

        // Remove trailing formatting that windows appends to strings
        letters.truncate(letters.chars().count() - 2);

        if letters.chars().count() > 10 {
            println!("Too many letters, try again");
            continue;
        }

        // Assemble words and print
        let result = find_words(letters, &assembler.word_bank);
        println!("{:?}", result);
    }
}

// Assemble and return all possible words
fn find_words(letters: String, word_bank: &HashSet<String>) -> HashSet<String> {
    // Keeping track of total words found, used letters, and currently assembled word
    let mut result = HashSet::<String>::new();
    let mut chars: Vec<char> = letters.chars().collect();
    let mut used = vec![false; letters.chars().count()];
    let mut word = String::new();

    // Recursion
    dfs(&mut word, &mut chars, &mut used, &mut result, &word_bank);

    return result;
}

fn dfs(word: &mut String, letters: &mut Vec<char>, used: &mut Vec<bool>, result: &mut HashSet<String>, word_bank: &HashSet<String>) {
    if word_bank.contains(word) {
        result.insert(word.clone());
    }

    if word.chars().count() == letters.len() {
        return;
    }

    for (i, letter) in letters.clone().iter().enumerate() {
        if !used[i] {
            word.push(letter.clone());
            used[i] = true;
            dfs(word, letters, used, result, &word_bank);
            word.pop();
            used[i] = false;
        }
    }
}
