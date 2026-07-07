enum Message {
    Move { x: i32, y: i32 },
}

fn main() {
    let msg = Message::Move { x: 3, y: 12 };

    match msg {
        // bind the whole y value while also checking it's in range
        Message::Move { x, y: xy @ 10..=20 } => {
            println!("x={x}, y in range: {xy}");
        }
        Message::Move { x, y } => println!("x={x}, y={y}"),
    }
}
