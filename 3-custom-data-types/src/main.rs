//! Phase 3 projects — build each from scratch. Features only; figure out the "how" yourself.
//! Do them in order. Prove each one works with your own test cases before moving on.
//!
#![allow(unused_variables)]
pub fn main() {
    println!("custom-data-types started");
    money_handler();
}

// ===== PROGRAM 1 — Money handler =====
// A small library for handling money correctly.
// Features it must have:
//   - Dollars and cents are two separate kinds of value you can't accidentally mix up.
//   - Convert a cents amount into dollars (and back).
//   - Add two amounts of the same kind together.
//   - Combining two incompatible kinds should be impossible — caught before the program runs.
// What you'll learn:
//   tuple structs and the newtype (wrapper) pattern, attaching methods to your own types,
//   deriving common traits (Debug, Clone, Copy, PartialEq), and using the type system for safety.

pub fn money_handler() {
    #[derive(Debug)]
    struct Dollars(u64, u64);
    #[derive(Debug)]
    struct Cents(u64);

    trait MoneyAdd<T> {
        fn add(&self, val: T) -> T;
    }

    impl MoneyAdd<Dollars> for Dollars {
        fn add(&self, val: Dollars) -> Dollars {
            Dollars(self.0 + val.0, self.1)
        }
    }
    impl MoneyAdd<Cents> for Cents {
        fn add(&self, val: Cents) -> Cents {
            Cents(self.0 + val.0)
        }
    }

    impl Dollars {
        fn new(val: u64) -> Self {
            Dollars(val, 0)
        }
        fn to_cents(&self) -> Cents {
            Cents(self.0 * 100 + self.1)
        }
    }

    impl Cents {
        fn to_dollars(&self) -> Dollars {
            Dollars(self.0 / 100, self.0 % 100)
        }
    }

    println!("Converting cent amount to dollars and back");

    let cent_amount = 213;
    let cents = Cents(213);
    println!("{:?}", cents);
    let cents = cents.add(Cents(5));
    println!("added 5 cents -> {:?}", cents);
    let dollars = cents.to_dollars();
    println!("cents to dollars -> {:?}", dollars);
    println!("added 5 dollars -> {:?}", dollars.add(Dollars::new(5)));
    let cents = dollars.to_cents();
    println!("dollars to cents ->{:?}", cents);
}

// ===== PROGRAM 2 — Payment tracker =====
// Track a single payment through its whole life.
// Features it must have:
//   - A payment is always in exactly one stage (e.g. scheduled, submitted, settled, returned).
//   - Each stage carries its own info: a scheduled one knows its run date, a returned one
//     knows the reason it bounced, etc. A stage can't hold info that doesn't belong to it.
//   - Given the current stage and an event that happens, produce the next stage —
//     or reject the event if it makes no sense for the current stage.
// What you'll learn:
//   enums whose variants carry their own data, exhaustive pattern matching, modeling states
//   so that illegal ones can't even be written, and returning errors instead of crashing.

// ===== PROGRAM 3 — Your own "maybe" and "success-or-failure" values =====
// Build, from scratch, your own version of the two values Rust leans on everywhere:
// one that means "there might be a value or there might be nothing", and one that means
// "this either succeeded with a value or failed with an error".
// Features it must have:
//   - Transform the inner value when one is present, leaving the empty/failed case alone.
//   - Chain one operation onto another so failures short-circuit.
//   - Fall back to a default when there's nothing / it failed.
// What you'll learn:
//   how Option and Result actually work (they're just enums), generics, writing the transform
//   and chaining helpers yourself, and what the error-propagation shortcut really does underneath.

// ===== PROGRAM 4 — JSON value in memory =====
// Represent any JSON value in memory, then print it back out as nicely-formatted text.
// Features it must have:
//   - Handle every JSON shape: null, true/false, numbers, strings, arrays, objects.
//   - Handle nesting to any depth — arrays inside objects inside arrays, etc.
//   - Pretty-print with proper indentation.
// What you'll learn:
//   recursive types (a type defined in terms of itself), nested data modeling, walking a
//   structure with recursive pattern matching, and when a growable collection is enough vs.
//   when you need explicit heap allocation.

// ===== PROGRAM 5 — Calculator (capstone, pulls in everything above) =====
// Read a math expression typed as text and compute its result.
// Features it must have:
//   - Support + - * / and parentheses, with correct precedence (e.g. 2 + 3 * 4 = 14).
//   - Turn the raw text into something structured before evaluating it.
//   - Report a clear, specific error for bad input (stray symbol, unbalanced parens,
//     divide by zero) instead of crashing.
//   - Once it works, add a new operator and notice how much the structure guides you.
// What you'll learn:
//   everything above combined — data-carrying enums, a recursive tree type with heap-allocated
//   nodes, every flavor of pattern matching, the maybe/result types in real use, and custom
//   error types. This is the text -> structure -> result pipeline behind every parser.
