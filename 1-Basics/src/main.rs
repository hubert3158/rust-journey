fn main() {
    // number_guessing_game()
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
fn calculator() {}
