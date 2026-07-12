//! Do them in order. Prove each one works with your own test cases before moving on.
//! Full specs + done-when checklist: ../README.md
//! The phase's one big idea: make illegal states unrepresentable.
//!

#![allow(unused_imports)]

use jiff::{Zoned, civil::Date};

pub fn main() {
    println!("custom-data-types started");
    // money_handler();
    // payment_demo();
    program3();
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

// #[derive(Debug, PartialEq, Clone, Copy)]
// struct Dollars(u64, u64);
// #[derive(Debug, PartialEq, Clone, Copy)]
// struct Cents(u64);
//
// pub fn money_handler() {
//     println!("Converting cent amount to dollars and back");
//
//     let cents = Cents(213);
//     println!("{:?}", cents);
//     let cents1 = cents + Cents(5);
//     println!("added 5 cents -> {:?}", cents1);
//     println!("cents1 -> {:?}", cents);
//     let dollars = cents.to_dollars();
//     println!("cents to dollars -> {:?}", dollars);
//     println!("added 5 dollars -> {:?}", dollars.add(Dollars::new(5)));
//     let cents = dollars.to_cents();
//     println!("dollars to cents ->{:?}", cents);
// }
//
// impl Dollars {
//     fn new(val: u64) -> Self {
//         Dollars(val, 0)
//     }
//     fn to_cents(self) -> Cents {
//         Cents(self.0 * 100 + self.1)
//     }
// }
//
// impl Cents {
//     fn to_dollars(self) -> Dollars {
//         Dollars(self.0 / 100, self.0 % 100)
//     }
// }
//
// use std::ops::Add;
//
// impl Add for Dollars {
//     type Output = Dollars;
//     fn add(self, val: Dollars) -> Dollars {
//         Dollars(
//             (self.0 + val.0) + ((self.1 + val.1) / 100),
//             (self.1 + val.1) % 100,
//         )
//     }
// }
// impl Add for Cents {
//     type Output = Cents;
//     fn add(self, val: Cents) -> Cents {
//         Cents(self.0 + val.0)
//     }
// }
//
// #[test]
// fn dollars_carry() {
//     assert_eq!(Dollars(1, 12).add(Dollars(1, 90)), Dollars(3, 2));
// }
// #[test]
// fn cents_addition() {
//     assert_eq!(Cents(90).add(Cents(12)), Cents(102));
// }

// ===== PROGRAM 2 — Payment state machine =====
// Track one payment through its life — every stage carries exactly its own data,
// and impossible events are rejected.
// Features it must have:
//   - One enum, stages as variants, each carrying only what belongs to it:
//     scheduled (run date), submitted (submitted-at, rail used), settled (settled-at),
//     returned (reason code + human message). A settled payment CANNOT hold a return
//     reason — the type forbids it.
//   - Events as a second enum: Submit, Settle, Return { reason }, Cancel.
//   - One function: current state + event -> Result<NewState, TransitionError>. Illegal
//     combos (settle a returned payment, cancel a settled one) return a specific error
//     naming both the state and the event — never panic.
//   - The match must be exhaustive with NO catch-all `_` arm for state+event pairs —
//     when a new variant appears later, the compiler must point at every place that
//     needs thought. (Add a PartialRefund variant at the end to experience exactly that.)
//   - Pattern-matching vocabulary — use each at least once, naturally:
//       * destructuring variant fields in match arms
//       * a match guard (e.g. returns after a cutoff date route differently)
//       * `if let` for a one-case peek, `let else` for an early return
//       * an `@` binding (e.g. capture a reason code while matching a range of codes)
//   - Test-driven driver: one full happy path + at least three rejected transitions.
// What you'll learn:
//   data-carrying enums, exhaustive matching as a refactoring tool, the full pattern
//   syntax (guards, @, if let, let else), errors as values.
//   (Phase 15 rebuilds this with typestate so illegal transitions won't even compile —
//   keep this version for the side-by-side comparison.)

// #[derive(Debug, PartialEq, Eq, Clone)]
// enum Payment {
//     Scheduled {
//         run_date: Date,
//     },
//     Submitted {
//         submitted_at: Date,
//         rail_used: String,
//     },
//     Settled {
//         setteled_at: Date,
//     },
//     Returned {
//         return_code: RetrunCode,
//         human_message: String,
//     },
//     Canceled {
//         cancelled_reason: String,
//     },
//     PartialRefund(),
// }
//
// impl Payment {
//     fn status(&self) {
//         match self {
//             Payment::Scheduled { run_date } => println!("Scheduled, Run Date: {}", run_date),
//             Payment::Submitted {
//                 submitted_at,
//                 rail_used,
//             } => println!(
//                 "Submitted , Submitted at:{}, Rail Used:{}",
//                 submitted_at, rail_used
//             ),
//             Payment::Settled { setteled_at } => println!("Settled, Settled at: {}", setteled_at),
//             Payment::Returned {
//                 return_code,
//                 human_message,
//             } => println!(
//                 "Returned, Reason Code:{}, Human Message:{}",
//                 return_code, human_message
//             ),
//             Payment::Canceled { cancelled_reason } => {
//                 println!("Canceled, Cancelled Reason:{}", cancelled_reason)
//             }
//             Payment::PartialRefund() => println!("Partially refunded"),
//         }
//     }
//     fn progress(self, event: Events, today: &Date) -> Result<Payment, TransitonError> {
//         match self {
//             Payment::Scheduled { run_date } => match &event {
//                 Events::Submitted { rail_used } if *today == run_date => Ok(Payment::Submitted {
//                     submitted_at: Zoned::now().date(),
//                     rail_used: rail_used.to_string(),
//                 }),
//                 Events::Submitted { rail_used: _ } => Err(TransitonError::new(self, event)),
//                 Events::Settle => Err(TransitonError::new(self, event)),
//                 Events::Return { return_code: _ } => Err(TransitonError::new(self, event)),
//                 Events::Cancel { reason } => Ok(Payment::Canceled {
//                     cancelled_reason: reason.to_string(),
//                 }),
//                 Events::PartialRefund => Err(TransitonError::new(self, event)),
//             },
//             Payment::Submitted {
//                 submitted_at: _,
//                 rail_used: _,
//             } => match &event {
//                 Events::Submitted { rail_used: _ } => Result::Ok(self),
//                 Events::Settle => Ok(Payment::Settled {
//                     setteled_at: Zoned::now().date(),
//                 }),
//                 Events::Return { return_code } => match return_code {
//                     RetrunCode::R01 => Ok(Payment::Returned {
//                         return_code: RetrunCode::R01,
//                         human_message: "Insufficient fund".to_string(),
//                     }),
//                     RetrunCode::R08 => Ok(Payment::Returned {
//                         return_code: RetrunCode::R08,
//                         human_message: "Payment Stopped".to_string(),
//                     }),
//                 },
//                 Events::Cancel { reason } => Ok(Payment::Canceled {
//                     cancelled_reason: reason.to_string(),
//                 }),
//                 Events::PartialRefund => Err(TransitonError::new(self, event)),
//             },
//             Payment::Settled { setteled_at: _ } => match &event {
//                 Events::Submitted { rail_used: _ } => Err(TransitonError::new(self, event)),
//                 Events::Settle => Ok(self),
//                 Events::Return {
//                     return_code: reason,
//                 } => Ok(Payment::Returned {
//                     return_code: reason.clone(), //INFO, clone why
//                     human_message: " Late return, Money refunded ".to_string(),
//                 }),
//                 Events::Cancel { reason: _ } => Err(TransitonError::new(self, event)),
//                 Events::PartialRefund => Ok(Payment::PartialRefund()),
//             },
//             Payment::Returned {
//                 return_code: _,
//                 human_message: _,
//             } => match &event {
//                 Events::Submitted { rail_used: _ } => Err(TransitonError::new(self, event)),
//                 Events::Settle => Err(TransitonError::new(self, event)),
//                 Events::Return { return_code: _ } => Ok(self),
//                 Events::Cancel { reason: _ } => Err(TransitonError::new(self, event)),
//                 Events::PartialRefund => Err(TransitonError::new(self, event)),
//             },
//             Payment::Canceled {
//                 cancelled_reason: _,
//             } => match event {
//                 Events::Submitted { rail_used: _ } => Err(TransitonError::new(self, event)),
//                 Events::Settle => Err(TransitonError::new(self, event)),
//                 Events::Return { return_code: _ } => Err(TransitonError::new(self, event)),
//                 Events::Cancel { reason: _ } => Ok(self),
//                 Events::PartialRefund => Err(TransitonError::new(self, event)),
//             },
//             Payment::PartialRefund() => Err(TransitonError::new(self, event)),
//         }
//     }
// }
//
// #[derive(Debug, Clone, PartialEq, Eq)]
// enum RetrunCode {
//     R01,
//     R08,
// }
// impl fmt::Display for RetrunCode {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             RetrunCode::R01 => write!(f, "R01"),
//             RetrunCode::R08 => write!(f, "R08"),
//         }
//     }
// }
//
// #[derive(Debug)]
// enum Events {
//     Submitted { rail_used: String },
//     Settle,
//     Return { return_code: RetrunCode },
//     PartialRefund,
//     Cancel { reason: String },
// }
//
// #[derive(Debug)]
// struct TransitonError {
//     state: Payment,
//     event: Events,
// }
//
// impl TransitonError {
//     /// Creates a new [`TransitonError`].
//     fn new(state: Payment, event: Events) -> Self {
//         Self { state, event }
//     }
// }
//
// trait Prnt {
//     fn print(&self);
// }
// impl Prnt for Result<Payment, TransitonError> {
//     fn print(&self) {
//         match &self {
//             Ok(p) => println!("{p:#?}"),
//             Err(e) => println!("State: \n{:#?}\n Event: \n{:#?}\n", e.state, e.event),
//         }
//     }
// }
//
// fn payment_demo() {
//     let today = Zoned::now().date();
//
//     //scheduling payment
//     let scheduled_payment = Payment::Scheduled { run_date: today };
//     scheduled_payment.status();
//
//     //payment schedule -> submitted
//     println!("payment schedule -> submitted");
//     let return_val = scheduled_payment.progress(
//         Events::Submitted {
//             rail_used: "ACH".to_string(),
//         },
//         &today,
//     );
//     return_val.print();
//
//     //payment submitted -> settled
//     println!("payment submitted-> settled");
//     let submitted_payment = Payment::Submitted {
//         submitted_at: today,
//         rail_used: "ACH".into(),
//     };
//     let return_val = submitted_payment.progress(Events::Settle, &today);
//     return_val.print();
//     if let Ok(Payment::Settled { setteled_at }) = return_val {
//         println!("settled on {}", setteled_at);
//     }
//
//     //payment settled -> partial refund
//     println!("payment settled -> partial refund");
//     let settled_payment = Payment::Settled { setteled_at: today };
//     let return_val = settled_payment.progress(Events::PartialRefund, &today);
//     return_val.print();
//
//     //payment submitted -> returned
//     println!("payment submitted -> returned");
//     let submitted_payment = Payment::Submitted {
//         submitted_at: today,
//         rail_used: "ACH".to_string(),
//     };
//     let return_val = submitted_payment.progress(
//         Events::Return {
//             return_code: RetrunCode::R08,
//         },
//         &today,
//     );
//     return_val.print();
//
//     //payment submitted -> cancel
//     println!("payment submitted -> cancel");
//     let submitted_payment = Payment::Submitted {
//         submitted_at: today,
//         rail_used: "ACH".to_string(),
//     };
//     let return_val = submitted_payment.progress(
//         Events::Cancel {
//             reason: "Coz i wanted to ".to_string(),
//         },
//         &today,
//     );
//     return_val.print();
//
//     //payment Canceled -> return
//     println!("payment settled -> return");
//     let cancelled_payment = Payment::Canceled {
//         cancelled_reason: ("created a cancelled payment".to_string()),
//     };
//     let return_val = cancelled_payment.progress(
//         Events::Return {
//             return_code: RetrunCode::R01,
//         },
//         &today,
//     );
//     return_val.print();
// }
//
// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn scheduled_to_submitted() {
//         let today = Zoned::now().date();
//
//         //scheduling payment
//         let scheduled_payment = Payment::Scheduled { run_date: today };
//         scheduled_payment.status();
//
//         //payment schedule -> submitted
//         println!("payment schedule -> submitted");
//         let return_val = scheduled_payment.progress(
//             Events::Submitted {
//                 rail_used: "ACH".to_string(),
//             },
//             &today,
//         );
//
//         assert_eq!(
//             return_val.unwrap(),
//             Payment::Submitted {
//                 submitted_at: today,
//                 rail_used: "ACH".to_string()
//             }
//         )
//     }
//
//     #[test]
//     fn submitted_to_settled() {
//         //payment submitted -> settled
//         println!("payment submitted-> settled");
//         let today = Zoned::now().date();
//         let submitted_payment = Payment::Submitted {
//             submitted_at: today,
//             rail_used: "ACH".into(),
//         };
//         let return_val = submitted_payment.progress(Events::Settle, &today);
//         assert_eq!(return_val.unwrap(), Payment::Settled { setteled_at: today })
//     }
//
//     #[test]
//     fn settled_to_partialrefund() {
//         // //payment settled -> partial refund
//         println!("payment settled -> partial refund");
//         let today = Zoned::now().date();
//         let settled_payment = Payment::Settled { setteled_at: today };
//         let return_val = settled_payment.progress(Events::PartialRefund, &today);
//         return_val.print();
//         assert_eq!(return_val.unwrap(), Payment::PartialRefund());
//     }
//
//     #[test]
//     fn submitted_to_returned() {
//         // //payment submitted -> returned
//         println!("payment submitted -> returned");
//         let today = Zoned::now().date();
//         let submitted_payment = Payment::Submitted {
//             submitted_at: today,
//             rail_used: "ACH".to_string(),
//         };
//         let return_val = submitted_payment.progress(
//             Events::Return {
//                 return_code: RetrunCode::R08,
//             },
//             &today,
//         );
//         assert!(return_val.is_ok());
//     }
//
//     #[test]
//     fn submitted_to_cancelled() {
//         //payment submitted -> cancel
//         println!("payment submitted -> cancel");
//         let today = Zoned::now().date();
//         let submitted_payment = Payment::Submitted {
//             submitted_at: today,
//             rail_used: "ACH".to_string(),
//         };
//         let return_val = submitted_payment.progress(
//             Events::Cancel {
//                 reason: "Coz i wanted to ".to_string(),
//             },
//             &today,
//         );
//         assert_eq!(
//             return_val.unwrap(),
//             Payment::Canceled {
//                 cancelled_reason: "Coz i wanted to ".to_string()
//             }
//         )
//     }
//
//     #[test]
//     fn canceled_to_returned_err() {
//         let today = Zoned::now().date();
//         //payment Canceled -> return
//         println!("payment settled -> return");
//         let cancelled_payment = Payment::Canceled {
//             cancelled_reason: ("created a cancelled payment".to_string()),
//         };
//         let return_val = cancelled_payment.progress(
//             Events::Return {
//                 return_code: RetrunCode::R01,
//             },
//             &today,
//         );
//         assert!(return_val.is_err())
//     }
//
//     #[test]
//     fn canceled_to_partialrefund_err() {
//         let today = Zoned::now().date();
//         //payment cancelled-> refund
//         println!("payment cancelled-> refund");
//         let cancelled_payment = Payment::Canceled {
//             cancelled_reason: ("created a cancelled payment".to_string()),
//         };
//         let return_val = cancelled_payment.progress(Events::PartialRefund, &today);
//         assert!(return_val.is_err())
//     }
//
//     #[test]
//     fn scheduled_to_settled_err() {
//         let today = Zoned::now().date();
//         //payment scheduled -> settled
//         println!("payment scheduled -> settled");
//         let cancelled_payment = Payment::Scheduled { run_date: today };
//         let return_val = cancelled_payment.progress(Events::Settle, &today);
//         assert!(return_val.is_err())
//     }
// }

// ===== PROGRAM 3 — Your own Option and Result =====
// Build Maybe<T> and Outcome<T, E> from scratch — discover that Rust's two most-used
// types are just enums, and their combinators are just match.
// Features it must have:
//   - Maybe<T>: your version of "value or nothing".
//     Outcome<T, E>: "success or failure".
//   - Methods on both, each a few lines of match:
//       * transform the inner value when present (map)
//       * chain a fallible step so failures short-circuit (and_then)
//       * fall back to a default (unwrap_or / unwrap_or_else)
//       * convert between them: Maybe -> Outcome by supplying an error (ok_or)
//   - A worked example end-to-end: parse config text "port=8080" -> find key ->
//     parse number -> validate range, chained, one failure path.
//   - Compare against std: same example with real Option/Result and `?`. Then answer
//     in a comment: what does `?` desugar to, roughly, in terms of YOUR Outcome?
//   - Generic syntax is new here (enum Maybe<T>) — fine and on purpose. Phase 4
//     formalizes generics; this is your preview by necessity.
// What you'll learn:
//   generics by building the two types you'll use daily forever, combinators from the
//   inside, why `?` isn't magic, enum + impl + match fluency.

fn program3() {
    enum Maybe<T> {
        Ok(T),
        Err(),
    }

    enum Outcome<T, E> {
        Ok(T),
        Err(E),
    }

    fn test<T>(x: T) -> Option<T> {
        return Some(x);
    }
    fn test1<T, E>(x: T) -> Result<T, E> {
        return Ok(x);
    }
}
// ===== PROGRAM 4 — JSON value in memory =====
// Represent ANY JSON document as one Rust type, then pretty-print it back out.
// Features it must have:
//   - One enum covering every JSON shape: null, bool, number, string, array, object.
//     Nesting to any depth — arrays in objects in arrays.
//   - Somewhere the compiler will refuse a naive definition ("recursive type has
//     infinite size"). Hit that error, read it fully, fix it. That error is the whole
//     lesson about why heap indirection exists.
//   - Pretty-printer: proper indentation, one field per line, correct comma placement
//     (no trailing comma), strings quoted. Compact one-line printer as a second mode.
//   - Build a small document by hand (your CV as JSON: name, skills array, nested
//     address) and snapshot-test the pretty output against an expected string.
//   - NO parsing in this program — text -> value comes in Program 5 for math; a JSON
//     parser is a great optional stretch AFTER that.
// What you'll learn:
//   recursive types and Box, modeling "any of N shapes" with enums, recursive functions
//   over recursive data, Vec vs Box (when does the enum need which?).
//   (This type is load-bearing later: Phase 4's serde-lite outputs it, Phase 12's
//   json! macro constructs it.)

// ===== PROGRAM 5 — ⭐ Expression calculator (capstone, pulls in everything above) =====
// Read a math expression as text, compute the result. The text -> structure -> result
// pipeline behind every parser, interpreter, and compiler.
// Features it must have:
//   - Supports + - * /, parentheses, decimal numbers, unary minus (-3 + 2).
//   - Correct precedence and associativity: 2 + 3 * 4 = 14, 2 - 3 - 4 = -5,
//     (2 + 3) * 4 = 20.
//   - Two distinct phases in code, with a named type between each:
//     raw text -> tokens (an enum) -> expression tree (a recursive enum, Program 4's
//     lesson applied) -> number. No evaluating while parsing.
//   - Errors are values with meaning, not panics: stray symbol, unbalanced parens,
//     unexpected end, division by zero — each its own variant. (Phase 6 upgrades these
//     with byte positions and caret rendering; leave room.)
//   - REPL loop: expression in, result or error out, never dies.
//   - Extension test: once everything passes, add % (or ^) — count how many places the
//     compiler forces you to touch. That number is the payoff of exhaustive enums.
//   - Tests: the precedence table above, nesting, every error variant, weird-but-legal
//     input (" ( 1 ) ", --3 — decide and document).
// What you'll learn:
//   everything in this phase combined — tokenizing, recursive trees with Box, exhaustive
//   matching across a pipeline, Option/Result under real load, custom error enums.
//   (Long life ahead: Phase 6 refits its errors, Phase 7 splits it into a workspace,
//   Phase 14 fuzzes it.)
