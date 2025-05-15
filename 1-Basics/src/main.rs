fn main() {
    // number_guessing_game()
    calculator()
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
