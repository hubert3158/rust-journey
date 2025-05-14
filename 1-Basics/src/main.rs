fn main() {
    number_guessing_game()
}

// 1. Generate a random number between 1–100
// 2. Ask the user to guess it
// 3. Respond with **“Too low”**, **“Too high”**, or **“Correct!”**
// 4. Repeat until correct
fn number_guessing_game() {
    let rand_value = rand::random_range(1..=100);
    println! {"{}",rand_value};
    let mut entered_value: String = String::new();
    let test = std::io::stdin().read_line(&mut entered_value);
    println! {"You typed {}",test.unwrap()}
}
