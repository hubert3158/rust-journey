#![allow(unused_imports)]
// #![allow(unused_variables)]
use clipboard_rs::{Clipboard, ClipboardContext};
use serde::Deserialize;
use std::{
    borrow::Cow,
    fs::File,
    io::{Read, Write, stdin, stdout},
    ops::Add,
    slice::SliceIndex,
    thread::sleep,
    time::Duration,
};

use serde_json::{Map, Number};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    text_indexer()?;
    task_management_app()?;
    paragraph_highlighter()?;
    file_parser()?;
    Ok(())
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
fn text_indexer() -> Result<(), Box<dyn std::error::Error>> {
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
                .first()
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
    Ok(())
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
fn task_management_app() -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks: Vec<String> = Vec::new();
    let mut option: String = String::new();
    loop {
        println!("Please select an option:\n1)Add Task\n2)Update Task\n3)List Task\n4)Exit");
        stdin().read_line(&mut option).expect("Couldnt add a task");
        match option.trim() {
            "1" => {
                add_tasks(&mut tasks);
            }
            "2" => {
                update_task(&mut tasks);
            }

            "3" => {
                list_tasks(&tasks);
            }
            "4" => {
                break;
            }
            _ => {
                println!("Wrong option")
            }
        }

        option.clear();
    }
    fn update_task(tasks: &mut [String]) {
        println!("which task do you want to update?");
        let mut task_no = String::new();
        stdin().read_line(&mut task_no).expect("Couldnt add a task");
        let task_no_int: usize = task_no.trim().parse().expect("Not a valid option");
        if task_no_int > tasks.len() {
            println!("Invalid number selected");
            return;
        }
        println!("Enter a task you want to replace it with");
        let mut new_task = String::new();
        stdin()
            .read_line(&mut new_task)
            .expect("Couldnt add a task");
        tasks[task_no_int - 1] = new_task.trim().to_string();
        println!("Successfully updated");
    }

    fn add_tasks(tasks: &mut Vec<String>) {
        let mut new_task = String::new();
        println!("please enter a one line task");
        stdin()
            .read_line(&mut new_task)
            .expect("Couldnt add a task");
        tasks.push(new_task.trim().to_string());
    }

    fn list_tasks(tasks: &[String]) {
        // let mut count = 1;
        // for (each_task) in tasks {
        //     println!("{}) {}", count, each_task);
        //     count += 1;
        // }
        for (count, task) in tasks.iter().enumerate() {
            println!("{} {}", count + 1, task);
        }
    }
    Ok(())
}
//
// ### 📌 3. Paragraph Highlighter
//
// **🎯 Goal:** Accept a paragraph and a keyword, and show keyword-highlighted results.
//
// #### Your app must:
//
// - Accept a paragraph of text from the user
// - Accept a search keyword
// - Display the paragraph with all occurrences of the keyword highlighted (e.g., `**word**`)
// - Support previewing the sentence in which the keyword appears
// - Let the user copy out only those matched lines
//
// 📘 _You'll use_:
// slices of `String`, borrowed data for rendering, and return-by-reference helpers. Keyword matching will push you into handling string views and lifetimes naturally.
//
//
//
fn paragraph_highlighter() -> Result<(), Box<dyn std::error::Error>> {
    println!("Please enter the paragraph");
    let mut paragraph = String::new();
    stdin()
        .read_line(&mut paragraph)
        .expect("failed to read the paragraph");

    let mut search_keyword = String::new();
    stdin()
        .read_line(&mut search_keyword)
        .expect("failed to read search_keyword");

    let key_size: usize = search_keyword.len() - 1;
    search_keyword.remove(key_size);
    highlighter(&search_keyword, &paragraph);
    preview(paragraph, &search_keyword);

    fn highlighter(search_keyword: &str, paragraph: &str) {
        println!("Highlight:");
        let new_val = String::from("**").add(search_keyword).add("**");
        println!("new value is -> {new_val}");
        let final_paragraph: String = paragraph.replace(search_keyword, &new_val);
        println!("Final paragraph is {}", final_paragraph);
        println!("\n");
    }

    fn preview(paragraph: String, search_keyword: &String) {
        let a: Vec<&str> = paragraph.split(".").collect();
        let mut is_found = false;
        println!("preview:");
        let mut copied_paragraph: Vec<&str> = Vec::new();
        for (count, x) in a.iter().enumerate() {
            if x.find(search_keyword).is_some() {
                is_found = true;
                copied_paragraph.push(x);
                println!("{}:{}", count, x);
            }
        }
        if !is_found {
            println!("Match not found breh");
        }

        println!("\n");

        let context = ClipboardContext::new().unwrap();
        let copied_paragraph_string = copied_paragraph.join(".");
        context.set_text(copied_paragraph_string).unwrap();
        sleep(Duration::from_secs(5));
    }
    Ok(())
}

//     3. **Milestone:**
// Write a file parser that borrows from a buffer.

pub fn file_parser() -> Result<(), Box<dyn std::error::Error>> {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct Me<'a> {
        name: Cow<'a, str>,
        age: u32,
        height: f32,
        email: Cow<'a, str>,
    }
    let mut file: File = File::open("../files/example_json_simple.json")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    println!("content:{}", buf);
    let json_ser: Me = serde_json::from_str(buf.as_str()).unwrap();
    println!(
        "Value of name:{},age:{},height:{},email:{}",
        json_ser.name, json_ser.age, json_ser.height, json_ser.email
    );
    Ok(())
}
