use std::io;

fn main() {
    println!("🚀 Welcome to the Rust Basics Tour!");

    // 🟦 Variables, Mutability, and Shadowing
    let x = 5;
    println!("x is immutable: {}", x);

    let mut y = 10;
    println!("y before mutation: {}", y);
    y = 20;
    println!("y after mutation: {}", y);

    let x = x + 1; // Shadowing
    println!("x after shadowing: {}", x);

    // 🟧 Scalar and Compound Types
    let a: i32 = 42;
    let b: f64 = 3.14;
    let c: bool = true;
    let d: char = '🦀';
    println!(
        "Scalars -> int: {}, float: {}, bool: {}, char: {}",
        a, b, c, d
    );

    let tup: (i32, f64, char) = (500, 6.4, 'R');
    let (t1, t2, t3) = tup;
    println!("Tuple destructured: {}, {}, {}", t1, t2, t3);

    let arr = [1, 2, 3, 4, 5];
    println!("Array element [2]: {}", arr[2]);

    // 🟨 Functions
    greet("Subash");
    let sum = add(7, 3);
    println!("7 + 3 = {}", sum);

    // 🟥 Control Flow
    let number = 7;
    if number < 10 {
        println!("Number is less than 10");
    } else {
        println!("Number is 10 or more");
    }

    // 🌀 Loops
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 3 {
            println!("loop: broke at 3");
            break;
        }
    }

    let mut w = 0;
    while w < 3 {
        println!("while: w = {}", w);
        w += 1;
    }

    for val in arr.iter() {
        println!("for loop: val = {}", val);
    }

    // 🎯 Match Statement
    let day = 3;
    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        _ => println!("Other day"),
    }

    // 📦 Vectors
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);

    for num in &numbers {
        println!("Vector element: {}", num);
    }

    // 📝 Basic IO
    println!("Enter your name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read");
    println!("Hello, {}", name.trim());
}

fn greet(name: &str) {
    println!("👋 Hello, {}!", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
