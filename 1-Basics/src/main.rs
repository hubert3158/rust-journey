use std::{
    io::{Write, stdin, stdout},
    string, usize,
};

fn main() {
    // number_guessing_game()
    // calculator()
    //temperature_converter()
    // report_card_generator()
    personal_finance_tracker()
}

// 1. Generate a random number between 1–100
// 2. Ask the user to guess it
// 3. Respond with **“Too low”**, **“Too high”**, or **“Correct!”**
// 4. Repeat until correct
#[allow(dead_code)]
fn number_guessing_game() {
    let rand_value: usize = rand::random_range(1..=100);
    let mut entered_value: String = String::new();
    std::io::stdin()
        .read_line(&mut entered_value)
        .expect("Oh noooo");

    let mut unwrapped: usize = match entered_value.trim().parse() {
        Ok(t) => t,
        Err(_) => {
            println!("{}", "Not a number");
            return;
        }
    };
    println! {"You typed: {}",unwrapped}

    while unwrapped != rand_value {
        if unwrapped > rand_value {
            println!("{}", "Too High")
        } else {
            println!("{}", "Too Low")
        }

        entered_value.clear();

        std::io::stdin()
            .read_line(&mut entered_value)
            .expect("Oh noooo");

        unwrapped = match entered_value.trim().parse() {
            Ok(t) => t,
            Err(_) => {
                println!("{}", "Not a number");
                return;
            }
        };
    }
    println!("{}", "Congratulations")
}

// **🎯 Goal:** Build a simple calculator that supports `+`, `-`, `*`, `/`.
//
// 1. Prompt for two numbers
// 2. Prompt for an operator (`+`, `-`, `*`, `/`)
// 3. Print the result using `match`
// 4. Loop until the user types `exit`
#[allow(dead_code)]
fn calculator() {
    let mut first_number: String = String::new();
    let mut second_number: String = String::new();
    let mut operator: String = String::new();

    let exit: bool = false;

    while !exit {
        println!("{}", "Please provide first number");
        std::io::stdin()
            .read_line(&mut first_number)
            .expect("OOps ");
        if first_number.trim().eq_ignore_ascii_case("exit") {
            break;
        }

        let first_number_int: f32 = match first_number.trim().parse() {
            Ok(t) => t,
            Err(_) => {
                println!("Please provide a digit");
                first_number.clear();
                second_number.clear();
                operator.clear();
                continue;
            }
        };

        println!("{}", "Please provide second number");
        std::io::stdin()
            .read_line(&mut second_number)
            .expect("OOps ");

        if second_number.trim().eq_ignore_ascii_case("exit") {
            break;
        }
        let second_number_int: f32 = match second_number.trim().parse() {
            Ok(t) => t,
            Err(_) => {
                println!("Please provide a digit");
                first_number.clear();
                second_number.clear();
                operator.clear();
                continue;
            }
        };

        println!("{}", "Please provide operator");
        std::io::stdin().read_line(&mut operator).expect("OOps ");
        if operator.trim().eq_ignore_ascii_case("exit") {
            break;
        }

        match operator.trim() {
            "+" => {
                println!(
                    "Calculating {}{}{} and result is:{}",
                    first_number_int,
                    operator.trim(),
                    second_number_int,
                    first_number_int + second_number_int
                );
            }
            "-" => {
                println!(
                    "Calculating {}{}{} and result is:{}",
                    first_number_int,
                    operator.trim(),
                    second_number_int,
                    first_number_int - second_number_int
                );
            }
            "*" => {
                println!(
                    "Calculating {}{}{} and result is:{}",
                    first_number_int,
                    operator.trim(),
                    second_number_int,
                    first_number_int * second_number_int
                );
            }
            "/" => {
                if second_number_int == 0.0 {
                    println!("oohoo there , cant divide by 0");
                } else {
                    println!(
                        "Calculating {}{}{} and result is:{}",
                        first_number_int,
                        operator.trim(),
                        second_number_int,
                        first_number_int / second_number_int
                    );
                }
            }
            _ => {
                println!("no match");
            }
        }

        first_number.clear();
        second_number.clear();
        operator.clear();
    }
}

// **🎯 Goal:** Convert temperature between Celsius and Fahrenheit.
//
// 1. Ask for temperature value
// 2. Ask for direction (`C→F` or `F→C`)
// 3. Display the result
// 4. Loop until exit
//
// **Formulae**
//
// ```text
// F = C × 9/5 + 32
// C = (F − 32) × 5/9
// ```
// ````
#[allow(dead_code)]
fn temperature_converter() {
    let mut temp_value: String = String::new();
    let mut option: String = String::new();
    println!("Please provide temperature value");
    stdin().read_line(&mut temp_value).expect("Invlid input");
    let temp_int = temp_value.trim().parse::<f32>().expect("noo");
    println!("What is the direction?");
    println!("1) C → F");
    println!("2) F → C");
    stdin().read_line(&mut option).expect("Invalid input");

    match option.trim() {
        "1" => {
            println!("{}°F", (temp_int * (9.0 / 5.0)) + 32.0)
        }
        "2" => {
            println!("{}°C", ((temp_int - 32.0) * (5.0 / 9.0)).trunc())
        }
        _ => {
            println!("Wrong option bro")
        }
    }
}

// **🎯 Goal:** Build a CLI app that collects student data and generates a formatted grade report.
//
// ### Features
//
// - Take student name and **three** subject scores as input
// - Compute the average score with a function
// - Assign letter grades
//
// ```text
// A: 90–100
// B: 80–89
// C: 70–79
// D: 60–69
// F: <60
// ```
//
// - Store each student as a tuple inside a `Vec`
// - Loop to allow entering multiple students (type `exit` to finish)
// - On exit, display the full report, e.g.:
//
// ```text
// ## Name      | Average | Grade
// Subash       |  91.67  |  A
// ```

#[allow(dead_code)]
fn report_card_generator() {
    let mut vec = Vec::<(String, String, String)>::new();

    loop {
        // get name
        let mut name = String::new();
        println!("Please state your name");
        stdin().read_line(&mut name).expect("Input Error");
        if name.trim() == "exit" {
            break;
        }

        //get scores
        let mut subject_one = String::new();
        println!("subject 1 score");
        stdin().read_line(&mut subject_one).expect("Input Error");
        let subject_one_int = subject_one.trim().parse::<f32>().expect("Oh noooo");

        let mut subject_two = String::new();
        println!("subject 2 score");
        stdin().read_line(&mut subject_two).expect("Input Error");
        let subject_two_int = subject_two
            .trim()
            .parse::<f32>()
            .expect("Something went wrong");

        let mut subject_three = String::new();
        println!("subject 3 score");
        stdin().read_line(&mut subject_three).expect("Input Error");
        let subject_three_int = subject_three
            .trim()
            .parse::<f32>()
            .expect("Something went wrong");

        let average: f32 = (subject_one_int + subject_two_int + subject_three_int) / 3.0;
        let average_str = format!("{:.2}", average);

        let grade = match average {
            90.0..100.0 => "A",
            80.0..89.0 => "B",
            70.0..79.0 => "C",
            60.0..69.0 => "D",
            0.0..60.0 => "F",
            _ => "NaN",
        };

        let student_info = (name, average_str, grade.to_string());
        vec.push(student_info);
    }

    println!("{:^10} | {:^10} | {:^10}", "Name", "Average", "Grade");
    for info in vec {
        println!("{:^10} | {:^10} | {:^10}", info.0.trim(), info.1, info.2);
    }
}

// **🎯 Goal:** Build a basic personal CLI finance tracker.
//
// ### Features
//
// 1. Ask user for starting balance
// 2. Loop:
//
//    - Ask **Deposit** or **Withdraw**
//    - Take amount
//    - Apply operation
//    - Show updated balance
//    - Block withdrawals that cause negative balance
//
// 3. Type `exit` to finish
// 4. At the end, show a full transaction log
//
// ### Example
//
// ```text
// Start balance: 1000
// Action (deposit/withdraw/exit): deposit
// Amount: 200
// New Balance: 1200
//
// Action: withdraw
// Amount: 300
// New Balance: 900
// ...
// Final Log:
// +200
// -300
// Final Balance: 900
//

#[allow(dead_code)]
fn personal_finance_tracker() {
    let mut balance = string::String::new();
    print!("Start balance: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut balance).expect("invalid");
    let mut balance_int = balance.trim().parse::<usize>().expect("Not a number");
    let mut log = Vec::<String>::new();

    loop {
        let mut action = string::String::new();
        let mut amount = string::String::new();
        print!("Action (deposit/withdraw/exit): ");
        stdout().flush().unwrap();
        stdin().read_line(&mut action).expect("invalid input");
        if action.trim() == "exit" {
            break;
        }
        print!("Amount: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut amount).expect("invalid input");
        let amount_int = amount.trim().parse::<usize>().expect("Not a integer");

        match action.trim() {
            "deposit" => {
                println!("New balance: {}", balance_int + amount_int);
                balance_int = balance_int + amount_int;
                let log_val = format!("+{}", amount_int);
                log.push(log_val);
            }
            "withdraw" => {
                println!("New balance: {}", balance_int - amount_int);
                balance_int = balance_int - amount_int;
                let log_val = format!("-{}", amount_int);
                log.push(log_val);
            }
            _ => println!("what?"),
        }
    }

    println!("Final Log:");
    for val in log {
        println!("{}", val)
    }
    println!("Final Balance {}", balance_int);
}
