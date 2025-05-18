use std::io::stdin;

fn main() {
    text_indexer();
}

// ### 📌 1. Text Indexer CLI
//
// **🎯 Goal:** Create a CLI that processes and indexes lines of user input.
//
// #### Your tool must:
//
// - Accept multiple lines of input from the user
// - Store all lines
// - Print a clean report showing:
//
//   - total number of lines
//   - total character count (excluding whitespace)
//   - the longest word
//   - the full line that contains the longest word
//   - a list of first and last words from each line
//
// 📘 _This naturally teaches_:
// ownership of `String`, borrowing lines for analysis, returning references from helpers, slicing strings, and tracking lifetimes.
#[allow(dead_code)]
fn text_indexer() {
    let mut paragraph = String::new();
    let mut first_and_last_words: Vec<String> = Vec::new();
    loop {
        let mut input_line = String::new();
        stdin()
            .read_line(&mut input_line)
            .expect("Something went wrong");
        add_to_paragraph(&mut paragraph, input_line.trim());
        add_first_and_last_words(&mut first_and_last_words, String::from(input_line.trim()))
    }

    fn add_to_paragraph(paragraph: &mut String, input_line: &str) {
        let formatted_string = format!("{}{}", paragraph, input_line);
        *paragraph = formatted_string;
    }
    fn add_first_and_last_words(words_list: &mut Vec<String>, input_line: String) {
        // TODO: understand
        let words: Vec<_> = input_line.split(' ').collect();
        words_list.push(input_line);
    }
}
