#![allow(unused)]

fn main() {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 10, y: 20 };

    let full_point @ Point { y, .. } = p;
    {
        println!("The y coordinate is {}", y);
        println!("The whole point is {:?}", full_point);
    }

    let age = 19;

    match age {
        n @ 13..=19 => println!("Teenager of age: {}", n),
        _ => println!("Not a teenager"),
    }

    let my_option = Some(String::from("Hello"));

    if let Some(ref s @ ref string) = my_option {
        println!("Got a reference to the string: {}", s);
    }
    //     match my_option {
    //     Some(ref s @ ref string) => {
    //         println!("Got a reference to the string: {}", s);
    //     }
    //     _ => {}
    // }
}
