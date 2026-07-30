use std::collections::{HashMap, LinkedList};

fn main() {
    let list = [1, 2];
    let tu = (1, 2, "a");
    let tuple = (1, 2);
    let vec = Vec::from([1, 2]); /* -> con memeory */
    let vec = LinkedList::from([1, 2]); /* -> non cont memeory */
    let list: [(i32, i32); 2] = [(1, 2), (1, 3)]; /* --> whut? */
    let vec: Vec<(i32, i32)> = Vec::from([(1, 2), (1, 3)]); /* --> whut? */
    let mut map = HashMap::new();
    map.insert("a", "b");
    let x = map.get("a").unwrap();
    let x = {
        let a = 1;
        a + 1
    }; // block — evaluates to 2
    //
    #[derive()]
    struct S {
        field: String,
    }

    impl S {
        fn new(field: String) -> Self {
            Self { field }
        }
    }

    let s1 = S {
        field: "test".to_string(),
    };

    let x = "1,2,3";
    let x = x.split_at(1);
    println!("{:?}", x);
    let (x, y) = (9, 2);
    println!("{}", x);
}

// before 29 , to offer out -> alo
