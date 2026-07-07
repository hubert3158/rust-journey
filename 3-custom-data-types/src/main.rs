//! Do them in order. Prove each one works with your own test cases before moving on.
//! Full specs + done-when checklist: ../README.md
//! The phase's one big idea: make illegal states unrepresentable.
//!

use jiff::{Zoned, civil::Date};

pub fn main() {
    println!("custom-data-types started");
    // money_handler();
    payment_demo();
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

#[derive(Debug, PartialEq, Eq, Clone)]
enum Payment {
    Scheduled {
        run_date: Date,
    },
    Submitted {
        submitted_at: Date,
        rail_used: String,
    },
    Settled {
        setteled_at: Date,
    },
    Returned {
        reason_code: ReasonCode,
        human_message: String,
    },
    Canceled {
        cancelled_reason: String,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum ReasonCode {
    R01,
    R08,
}

impl std::fmt::Display for ReasonCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({})",
            match self {
                ReasonCode::R01 => "Network",
                ReasonCode::R08 => "AboveLimit",
            }
        )
    }
}

enum Events {
    Submitted { rail_used: String },
    Settle,
    Return { reason: ReasonCode },
    Cancel,
}
type TransitonError = String;

impl Payment {
    fn status(&self) {
        match self {
            Payment::Scheduled { run_date } => println!("Scheduled, Run Date: {}", run_date),
            Payment::Submitted {
                submitted_at,
                rail_used,
            } => println!(
                "Submitted , Submitted at:{}, Rail Used:{}",
                submitted_at, rail_used
            ),
            Payment::Settled { setteled_at } => println!("Settled, Settled at: {}", setteled_at),
            Payment::Returned {
                reason_code,
                human_message,
            } => println!(
                "Returned, Reason Code:{}, Human Message:{}",
                reason_code, human_message
            ),
            Payment::Canceled { cancelled_reason } => {
                println!("Canceled, Cancelled Reason:{}", cancelled_reason)
            }
        }
    }
    fn progress(&mut self, event: Events) -> Result<(), TransitonError> {
        match self {
            Payment::Scheduled { run_date } => match event {
                Events::Submitted { rail_used } => {
                    let today = Zoned::now().date();

                    if today > *run_date {
                        Err("Time already passed".to_string())
                    } else if *run_date > today {
                        Err("Not yet time to submit".to_string())
                    } else {
                        *self = Payment::Submitted {
                            submitted_at: Zoned::now().date(),
                            rail_used,
                        };
                        Ok(())
                    }
                }
                Events::Settle => Err(String::from(
                    "You cannot directly settled a scheduled payment bro",
                )),
                Events::Return { reason: _ } => Err(String::from(
                    "You cannot directly settled a scheduled payment bro",
                )),
                Events::Cancel => {
                    *self = Payment::Canceled {
                        cancelled_reason: "Canceled by a user".to_string(),
                    };
                    Ok(())
                }
            },
            Payment::Submitted {
                submitted_at: _,
                rail_used: _,
            } => match event {
                Events::Submitted { rail_used: _ } => Result::Ok(()),
                Events::Settle => {
                    *self = Payment::Settled {
                        setteled_at: Zoned::now().date(),
                    };
                    Ok(())
                }
                Events::Return { reason } => {
                    *self = Payment::Returned {
                        reason_code: reason,
                        human_message: "Payment returned".to_string(),
                    };
                    Ok(())
                }
                Events::Cancel => {
                    *self = Payment::Canceled {
                        cancelled_reason: "successfully Canceled".to_string(),
                    };
                    Ok(())
                }
            },
            Payment::Settled { setteled_at: _ } => match event {
                Events::Submitted { rail_used: _ } => {
                    Err("Not allowed to chage from settled to submitted".to_string())
                }
                Events::Settle => Ok(()),
                Events::Return { reason } => {
                    *self = Payment::Returned {
                        reason_code: reason,
                        human_message: " Late return, Money refunded ".to_string(),
                    };
                    Ok(())
                }
                Events::Cancel => Err("Sorry cannot cancel a settled payment".to_string()),
            },
            Payment::Returned {
                reason_code: _,
                human_message: _,
            } => match event {
                Events::Submitted { rail_used: _ } => {
                    Err("Cant submite returned payment".to_string())
                }
                Events::Settle => Err("Cant settle returned payment".to_string()),
                Events::Return { reason: _ } => Ok(()),
                Events::Cancel => Err("Sorry cannot cancel a returned payment".to_string()),
            },
            Payment::Canceled {
                cancelled_reason: _,
            } => match event {
                Events::Submitted { rail_used: _ } => {
                    Err("sorry not allowed to change from canceled to returned".to_string())
                }
                Events::Settle => Err("sorry cancelled payment cannot be settled".to_string()),
                Events::Return { reason: _ } => {
                    Err("Canceled payments cant be returned".to_string())
                }
                Events::Cancel => Ok(()),
            },
        }
    }
}

fn payment_demo() {
    //payment state 1
    let mut payment = Payment::Scheduled {
        run_date: Zoned::now().date(),
    };

    //illegal progresson scheduled -> return
    let return_val = payment.progress(Events::Return {
        reason: ReasonCode::R01,
    });

    if let Err(string) = &return_val {
        println!("{}", string);
    };

    payment.status();

    //cloning payment state  1 and cancelling it
    let return_val = payment.clone().progress(Events::Cancel);
    match return_val {
        Ok(_) => println!("Cancelled the cloned submitted payment"),
        Err(e) => {
            eprintln!("Sorry, couldn't Cancel, {}", e)
        }
    }

    let return_val = payment.progress(Events::Submitted {
        rail_used: "ACH".to_string(),
    });

    let Ok(_) = return_val else {
        println!("Progressed from scheduled to submitted");
        return;
    };

    payment.status();

    let return_val = payment.clone().progress(Events::Return {
        reason: ReasonCode::R01,
    });
    match return_val {
        Ok(_) => println!("cloned payment is returned"),
        Err(e) => eprintln!("issue returning{}", e),
    }
    let return_val = payment.progress(Events::Settle);
    match return_val {
        Ok(_) => println!("payment settled"),
        Err(e) => eprintln!("issue settling payment{}", e),
    }
    payment.status();
}

#[cfg(test)]
mod test {
    use super::*;

    // Helper: reduces repetition and makes each test focus on the transition,
    // not on constructing the starting state.
    fn scheduled() -> Payment {
        Payment::Scheduled {
            run_date: Zoned::now().date(),
        }
    }

    #[test]
    fn scheduled_to_submitted() {
        let mut payment = scheduled();
        let event = Events::Submitted {
            rail_used: "ACH".to_string(),
        };

        payment.progress(event).unwrap();

        assert_eq!(
            payment,
            Payment::Submitted {
                submitted_at: Zoned::now().date(),
                rail_used: "ACH".to_string(),
            }
        );
    }

    #[test]
    fn submitted_to_settled() {
        let mut payment = Payment::Submitted {
            submitted_at: Zoned::now().date(),
            rail_used: "ACH".to_string(),
        };
        let event = Events::Settle;

        payment.progress(event).unwrap();

        assert_eq!(
            payment,
            Payment::Settled {
                setteled_at: Zoned::now().date(),
            }
        );
    }

    #[test]
    fn full_lifecycle() {
        let mut payment = scheduled();
        payment
            .progress(Events::Submitted {
                rail_used: "ACH".to_string(),
            })
            .unwrap();
        payment.progress(Events::Settle).unwrap();

        assert_eq!(
            payment,
            Payment::Settled {
                setteled_at: Zoned::now().date(),
            }
        );
    }
}

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
