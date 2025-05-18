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
    let mut no_of_lines = 0;
    let mut first_and_last_words: Vec<(String, String)> = Vec::new();
    let mut total_characters = 0;
    let mut longest_word = "";
    loop {
        let mut input_line = String::new();
        stdin()
            .read_line(&mut input_line)
            .expect("Something went wrong");
        if input_line.trim() == "exit" {
            break;
        }
        no_of_lines += 1;
        add_to_paragraph(&mut paragraph, input_line.trim());
        add_first_and_last_words(&mut first_and_last_words, String::from(input_line.trim()));
        count_characters(input_line.trim(), &mut total_characters);
    }
    println!("Report");
    println!("Total no of Lines: {}", no_of_lines);
    for words_tuple in first_and_last_words {
        println!("{}, {}", words_tuple.0, words_tuple.1);
    }
    println!("Total characters: {}", total_characters);
    fn count_characters(input_line: &str, total_characters: &mut i32) {
        input_line.chars().for_each(|c| {
            if c != ' ' {
                *total_characters += 1;
            }
        });
    }

    fn add_to_paragraph(paragraph: &mut String, input_line: &str) {
        let formatted_string = format!("{}{}", paragraph, input_line);
        *paragraph = formatted_string;
    }
    fn add_first_and_last_words(words_list: &mut Vec<(String, String)>, input_line: String) {
        let words: Vec<_> = input_line.split(' ').collect();
        let words_size = words.len();
        if words_size > 0 {
            let first_letter: String = words
                .get(0)
                .expect("couldnt get the first item")
                .to_string();
            let last_letter: String = words
                .get(words_size - 1)
                .expect("couldnt get the first item")
                .to_string();
            words_list.push((first_letter, last_letter));
        } else {
            words_list.push((String::new(), String::new()));
        }
    }
}
