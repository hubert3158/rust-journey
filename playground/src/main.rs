//! MiniLedger — one tiny, deliberately overloaded tour of core Rust.
//! Std-only, single file. Read top to bottom; every section is labeled.
//! Run it:        rustc miniledger.rs && ./miniledger
//! Run the test:  rustc --test miniledger.rs && ./miniledger

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

// === 1. const & static (compile-time values) ===
const FEE_BPS: i64 = 50; // 0.50% fee, expressed in basis points
static APP_NAME: &str = "MiniLedger";

// === 2. module + visibility + newtype (tuple struct) + operator overload ===
mod money {
    use std::fmt;
    use std::ops::Add;

    // A "newtype": a tuple struct wrapping one i64. Derives hand it free behavior.
    // Copy => cheap value semantics (assigning does NOT move it away).
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct Cents(pub i64);

    // Operator overloading: implementing std::ops::Add lets us write `a + b`.
    impl Add for Cents {
        type Output = Cents; // associated type required by the Add trait
        fn add(self, rhs: Cents) -> Cents {
            Cents(self.0 + rhs.0)
        }
    }

    // Implementing Display controls how `{}` prints the type.
    impl fmt::Display for Cents {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "${}.{:02}", self.0 / 100, (self.0 % 100).abs())
        }
    }
}
use money::Cents;

// === 3. enum where each variant carries different data ===
#[derive(Debug, Clone)]
enum Txn {
    Deposit {
        account: String,
        cents: i64,
    },
    Withdraw {
        account: String,
        cents: i64,
    },
    Transfer {
        from: String,
        to: String,
        cents: i64,
    },
}

// === 4. custom error type + Display + Error + From (this is what powers `?`) ===
#[derive(Debug)]
enum LedgerError {
    InsufficientFunds {
        account: String,
        needed: i64,
        have: i64,
    },
    BadAmount(std::num::ParseIntError),
}

impl fmt::Display for LedgerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LedgerError::InsufficientFunds {
                account,
                needed,
                have,
            } => {
                write!(f, "{account}: need {needed}, have {have}")
            }
            LedgerError::BadAmount(e) => write!(f, "bad amount: {e}"),
        }
    }
}
impl Error for LedgerError {} // opt into the std error trait (uses Display + Debug)

// `?` auto-converts a ParseIntError into our error through this From impl.
impl From<std::num::ParseIntError> for LedgerError {
    fn from(e: std::num::ParseIntError) -> Self {
        LedgerError::BadAmount(e)
    }
}

// A small fallible helper: `?` on parse() triggers the From impl above.
fn parse_amount(s: &str) -> Result<i64, LedgerError> {
    let cents: i64 = s.trim().parse()?; // ParseIntError -> LedgerError, automatically
    Ok(cents)
}

// === 5. trait: supertrait bound + required method + default method (dyn-friendly) ===
trait Rail: fmt::Debug {
    // `: fmt::Debug` is a supertrait requirement
    fn name(&self) -> &str; // required: every impl must supply it
    fn settle(&self, c: Cents) -> String {
        // default: you get it free, can override
        format!("[{}] settled {c}", self.name())
    }
}

#[derive(Debug)]
struct Ach;
#[derive(Debug)]
struct Wire {
    priority: bool,
}

impl Rail for Ach {
    fn name(&self) -> &str {
        "ACH"
    }
}
impl Rail for Wire {
    fn name(&self) -> &str {
        "WIRE"
    }
    fn settle(&self, c: Cents) -> String {
        // overriding the default method
        let tag = if self.priority { "PRIORITY" } else { "std" };
        format!("[WIRE/{tag}] settled {c}")
    }
}

// === 6. a struct that holds a borrow needs an explicit lifetime ===
struct Report<'a> {
    title: &'a str,     // borrowed: cannot outlive the &str it points at
    lines: Vec<String>, // owned
}
impl<'a> Report<'a> {
    fn print(&self) {
        println!("== {} ==", self.title);
        for line in &self.lines {
            println!("  {line}");
        }
    }
}

// === 7. the core data: a struct with methods, &self vs &mut self ===
#[derive(Debug, Default)]
struct Ledger {
    balances: HashMap<String, i64>, // owned String keys -> i64 cents
}

impl Ledger {
    fn new() -> Self {
        Ledger {
            ..Default::default()
        } // struct-update syntax + Default
    }

    fn balance(&self, who: &str) -> i64 {
        // get() returns Option<&i64>; copied() -> Option<i64>; unwrap_or supplies a default
        self.balances.get(who).copied().unwrap_or(0)
    }

    // &mut self: an exclusive borrow. Result makes the op fallible.
    fn apply(&mut self, txn: &Txn) -> Result<(), LedgerError> {
        match txn {
            Txn::Deposit { account, cents } => {
                *self.balances.entry(account.clone()).or_insert(0) += cents;
            }
            Txn::Withdraw { account, cents } => {
                let have = self.balance(account);
                if have < *cents {
                    return Err(LedgerError::InsufficientFunds {
                        account: account.clone(),
                        needed: *cents,
                        have,
                    });
                }
                *self.balances.get_mut(account).unwrap() -= cents;
            }
            Txn::Transfer { from, to, cents } => {
                // reuse apply() + `?` to bubble an error up out of the transfer
                self.apply(&Txn::Withdraw {
                    account: from.clone(),
                    cents: *cents,
                })?;
                self.apply(&Txn::Deposit {
                    account: to.clone(),
                    cents: *cents,
                })?;
            }
        }
        Ok(())
    }

    // Explicit lifetime: the returned &str borrows from `self`'s map keys.
    // (Elision would let you omit 'a here; written out so you can see it.)
    fn richest<'a>(&'a self) -> Option<(&'a str, i64)> {
        self.balances
            .iter()
            .max_by_key(|&(_, &bal)| bal)
            .map(|(name, &bal)| (name.as_str(), bal))
    }
}

// === 8. custom iterator: associated type `Item`, required `next`, free combinators ===
struct Countdown {
    n: u32,
}
impl Iterator for Countdown {
    type Item = u32; // the associated type the Iterator trait demands
    fn next(&mut self) -> Option<u32> {
        if self.n == 0 {
            None
        } else {
            self.n -= 1;
            Some(self.n + 1)
        }
    }
}

// === 9. generics + trait bound + where-clause + impl-Trait return ===
fn describe_all<R>(rails: &[R]) -> Vec<String>
where
    R: Rail, // bound: R must implement Rail (static dispatch / monomorphized)
{
    rails.iter().map(|r| r.settle(Cents(100))).collect()
}

fn fee_schedule() -> impl Iterator<Item = i64> {
    // returns an opaque iterator type
    (1_i64..=3).map(|tier| tier * FEE_BPS)
}

// === 10. closures: an `Fn` taken as a generic parameter ===
fn apply_fee<F: Fn(i64) -> i64>(amount: i64, f: F) -> i64 {
    f(amount)
}

fn main() -> Result<(), Box<dyn Error>> {
    // main returns Result so `?` works here too

    // --- variable flavors ---
    let immutable = 42; // inferred i32, immutable by default
    let mut counter: u32 = 0; // explicit type + mutable
    let pi: f64 = 3.14159; // float
    let grade: char = 'A'; // char = one Unicode scalar (4 bytes)
    let active: bool = true; // bool
    let owned: String = String::from("syndication"); // owned, heap-allocated
    let view: &str = &owned[0..4]; // borrowed slice into it -> "synd"
    let trio: (u8, &str, f64) = (1, "x", 0.5); // tuple
    let arr: [i64; 3] = [10, 20, 30]; // fixed-size array
    let slice: &[i64] = &arr[1..]; // slice view -> [20, 30]
    let shadowed = immutable; // shadowing:
    let shadowed = shadowed.to_string(); // same name, different type (i32 -> String)

    println!("{APP_NAME}: {immutable} {pi} {grade} {active} {view} {trio:?} {slice:?} {shadowed}");

    // --- Result + `?` + From conversion ---
    let amt = parse_amount(" 4200 ")?; // returns LedgerError, lifted to Box<dyn Error> by main
    println!("parsed amount = {}", Cents(amt));

    // --- newtype + operator overload + Copy ---
    let a = Cents(150);
    let b = Cents(250);
    let total = a + b; // uses our Add impl; a and b stay usable because Cents is Copy
    println!("fee math: {a} + {b} = {total}");

    // --- closures: Fn passed in, FnMut capturing &mut ---
    let with_fee = apply_fee(10_000, |x| x + x * FEE_BPS / 10_000); // Fn closure
    let mut bump = || counter += 1; // FnMut: captures counter by &mut
    bump();
    bump();
    println!("with_fee={with_fee}, counter={counter}");

    // --- custom iterator + standard combinators ---
    let countdown: Vec<u32> = Countdown { n: 3 }.collect();
    let squared_sum: u32 = Countdown { n: 3 }.map(|x| x * x).sum();
    println!("countdown={countdown:?}, squared_sum={squared_sum}");
    println!("fees={:?}", fee_schedule().collect::<Vec<_>>());

    // --- trait objects: dynamic dispatch through Box<dyn Rail> ---
    let rails: Vec<Box<dyn Rail>> = vec![Box::new(Ach), Box::new(Wire { priority: true })];
    for r in &rails {
        println!("{}", r.settle(Cents(999))); // resolved at runtime via a vtable
    }
    // static-dispatch counterpart (one specialized copy generated at compile time):
    println!("{:?}", describe_all(&[Ach, Ach]));

    // --- concurrency: Arc<Mutex<_>> shared state + mpsc channel for receipts ---
    let ledger = Arc::new(Mutex::new(Ledger::new()));
    let batches: Vec<Vec<Txn>> = vec![
        vec![
            Txn::Deposit {
                account: "merchant".into(),
                cents: 100_000,
            },
            Txn::Withdraw {
                account: "merchant".into(),
                cents: 30_000,
            },
        ],
        vec![
            Txn::Deposit {
                account: "alice".into(),
                cents: 50_000,
            },
            Txn::Transfer {
                from: "alice".into(),
                to: "bob".into(),
                cents: 20_000,
            },
            Txn::Withdraw {
                account: "bob".into(),
                cents: 999_999,
            }, // will fail -> Err path
        ],
    ];

    let (tx, rx) = mpsc::channel::<String>();
    let mut handles = Vec::new();

    for (i, batch) in batches.into_iter().enumerate() {
        let ledger = Arc::clone(&ledger); // bump the atomic refcount, move the clone in
        let tx = tx.clone(); // each worker gets its own sender
        let handle = thread::spawn(move || {
            // `move` hands ownership of captures to the thread
            for txn in &batch {
                let mut guard = ledger.lock().unwrap(); // acquire the Mutex
                match guard.apply(txn) {
                    Ok(()) => {
                        let _ = tx.send(format!("worker {i}: ok    {txn:?}"));
                    }
                    Err(e) => {
                        let _ = tx.send(format!("worker {i}: ERR   {e}"));
                    }
                }
            } // `guard` dropped each loop -> Mutex released
        });
        handles.push(handle);
    }
    drop(tx); // drop the original sender so the channel can close

    for h in handles {
        h.join().unwrap(); // wait for every worker to finish
    }

    // `while let` drains the channel until all senders are gone
    let mut receipts = Vec::new();
    while let Ok(msg) = rx.recv() {
        receipts.push(msg);
    }
    receipts.sort();

    // --- pattern matching: destructuring, ranges, guards, `@` binding ---
    let final_ledger = ledger.lock().unwrap();
    if let Some((name, bal)) = final_ledger.richest() {
        let tier = match bal {
            0 => "empty".to_string(),
            n @ 1..=49_999 => format!("small({n})"), // `@` binds the matched value
            50_000..=99_999 => "mid".to_string(),    // inclusive range pattern
            _ => "whale".to_string(),                // catch-all
        };
        println!("richest: {name} = {} ({tier})", Cents(bal));
    }

    // --- build a Report that borrows a &str (the lifetime from section 6 in action) ---
    let report = Report {
        title: APP_NAME,
        lines: receipts,
    };
    report.print();

    Ok(())
}

// === 11. unit test (run with: rustc --test miniledger.rs && ./miniledger) ===
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deposit_then_withdraw() {
        let mut l = Ledger::new();
        l.apply(&Txn::Deposit {
            account: "a".into(),
            cents: 100,
        })
        .unwrap();
        l.apply(&Txn::Withdraw {
            account: "a".into(),
            cents: 40,
        })
        .unwrap();
        assert_eq!(l.balance("a"), 60);
    }
}
