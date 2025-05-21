use std::io::stdin;
fn main() {
    // text_indexer();
    task_management_app();
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
    let mut longest_word = String::new();
    let mut line_with_longest_word = String::new();
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
        save_longest_word_and_sentence(
            input_line.trim(),
            &mut longest_word,
            &mut line_with_longest_word,
        );
    }
    println!("Report");
    println!("Total no of Lines: {}", no_of_lines);
    for words_tuple in first_and_last_words {
        println!("{}, {}", words_tuple.0, words_tuple.1);
    }
    println!("Total characters: {}", total_characters);
    println!("Longest word: {}", longest_word);
    println!("Longest line: {}", line_with_longest_word);

    fn count_characters(input_line: &str, total_characters: &mut i32) {
        input_line.chars().for_each(|c| {
            if c != ' ' {
                *total_characters += 1;
            }
        });
    }

    fn save_longest_word_and_sentence(
        input_line: &str,
        longest_word: &mut String,
        full_line_with_longest_word: &mut String,
    ) {
        let values: Vec<_> = input_line.split(' ').collect();
        for value in values {
            if value.chars().count() > longest_word.chars().count() {
                *longest_word = String::from(value);
                *full_line_with_longest_word = String::from(input_line);
            }
        }
    }

    fn add_to_paragraph(paragraph: &mut String, input_line: &str) {
        let formatted_string = format!("{}{}", paragraph, input_line);
        *paragraph = formatted_string;
    }
    fn add_first_and_last_words(words_list: &mut Vec<(String, String)>, input_line: String) {
        let words: Vec<_> = input_line.split(' ').collect();
        let words_size = words.len();

        //optmized way to do
        // if let Some((first, last)) = words.first().zip(words.last()) {
        //     words_list.push((first.to_string(), last.to_string()));
        // }

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

// ### 📌 2. Todo Manager CLI (Full CRUD + Analytics)
//
// **🎯 Goal:** Build a task management app for the terminal.
//
// #### Your app must:
//
// - Add, list, edit, and delete tasks
// - Support viewing all tasks
// - Support editing by replacing a portion of a task
// - Support searching tasks by keyword
// - Highlight the keyword in search results
// - Show the task with:
//
//   - the most words
//   - the fewest characters
//
// - Generate previews for each task (first sentence or 10 words)
// - Support sorting tasks alphabetically and by length
//
// 📘 _This will cover_:
// shared vs mutable references, mutable borrowing of items in a `Vec`, slicing, lifetimes in return values, and writing functions with reference-based input/output.
//
#[allow(dead_code)]
fn task_management_app() {
    let mut tasks: Vec<String> = Vec::new();

    let mut new_task = String::new();
    let mut new_task_edit = String::from("this is a test");

    stdin()
        .read_line(&mut new_task)
        .expect("Couldnt add a task");

    add_tasks(&mut tasks, new_task);

    list_tasks(&tasks);

    stdin()
        .read_line(&mut new_task_edit)
        .expect("Couldnt add a task");

    fn add_tasks(tasks: &mut Vec<String>, new_task: String) {
        tasks.push(new_task.trim().to_string());
    }

    fn list_tasks(tasks: &Vec<String>) {
        let mut count = 1;
        for each_task in tasks {
            println!("{}). {}", count, each_task);
            count += 1;
        }
    }
}
