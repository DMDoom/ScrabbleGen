use std::io::{self, BufRead};
use std::fs::File;
use std::collections::HashSet;

fn main() {
    loop {
        println!("Loading dictionary...");

        // Open file
        let file: File = match File::open("slowa.txt") {
            Result::Ok(num) => num,
            Result::Err(_) => panic!("File not found.")
        };

        // Init reader
        let buf_reader = io::BufReader::new(file);

        // Init map and hash words in the file
        let mut word_bank: HashSet<String> = HashSet::new();
        for line in buf_reader.lines() {
            word_bank.insert(line.unwrap());
        }

        println!("Dictionary loaded.");

        // Read user input
        println!("Input letters: ");
        let mut letters: String = String::new();

        io::stdin()
            .read_line(&mut letters)
            .expect("Unexpected error");

        // Assemble words and print
        let result = assemble(letters, word_bank);
        println!("{:?}", result);
    }
}

// Assemble and return all possible words
fn assemble(letters: String, word_bank: HashSet<String>) -> HashSet<String> {
    // Keeping track of total words found, used letters, and currently assembled word
    let mut result = HashSet::<String>::new();
    let mut used = Vec::<bool>::with_capacity(letters.chars().count());
    let mut chars: Vec<char> = letters.chars().collect();
    let mut word = String::new();

    // Recursion
    dfs(&mut word, &mut chars, &mut used, &mut result, &word_bank);

    return result;
}

// Depth-for-search
fn dfs(word: &mut String, letters: &mut Vec<char>, used: &mut Vec<bool>, result: &mut HashSet<String>, word_bank: &HashSet<String>) {
    if word_bank.contains(word) {
        result.insert(word.clone());
    }

    if word.chars().count() == letters.len() {
        return;
    }

    for (i, letter) in letters.iter().enumerate() {
        if !used[i] {
            word.push(letter.clone());
            used[i] = true;
            dfs(word, letters, used, result, &word_bank); // cannot pass something while im iterating on it, obviously
            // word = word.substring(0, word.length() - 1)
            // used[i] = false
        }
    }
}
