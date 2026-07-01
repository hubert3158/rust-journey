# Phase 4 — Traits & Generics

Build each program from scratch, in order. Specs describe **what**, never **how**.
Before moving on, prove each works with your own tests (`#[test]`, like you did for Money).

Core question this phase answers: *when does the compiler generate specialized code
(generics → monomorphization, zero cost), and when does it look things up at runtime
(`dyn Trait` → vtable)?* You should be able to explain the difference out loud by the end.

---

## Program 1 — Serde-lite

**Goal:** Your own mini serialization framework: a `ToJson` trait that turns values into JSON text.

**Requirements:**
- Define a trait `ToJson` with one method that produces a JSON string (or, better: produces
  your Phase 3 JSON value type, then reuse its pretty-printer).
- Implement it for: `i64`, `f64`, `bool`, `String`, `&str`.
- Implement it for `Vec<T>` where `T` is anything serializable — nesting must work.
- Implement it for `HashMap<String, T>` — same deal.
- Write ONE blanket impl (e.g. anything that is `Display` gets a default `ToJson`... discover
  why this collides with your other impls, then resolve it. That pain is the lesson).
- Try to implement your trait for `std::time::Duration`, and try to implement `Display`
  (a foreign trait) for `Vec<i64>` (a foreign type). One works, one doesn't. Write a comment
  explaining the orphan rule in your own words.

**What you'll learn:** trait definition, impls for foreign vs local types, generic impls with
bounds, blanket impls and their conflicts, the orphan rule / coherence, associated functions
on traits. *(In Phase 12 you'll write `#[derive(ToJson)]` so structs get this for free.)*

---

## Program 2 — Dispatch lab

**Goal:** The same tiny system built twice — once with static dispatch, once with dynamic — so you feel the difference instead of reading about it.

**Requirements:**
- A trait `Stage` with a method taking a `String` and returning a `String`
  (e.g. stages: lowercase, trim punctuation, reverse words — pick any three).
- Version A: a generic function `run_pipeline<S: Stage>(stages: &[S], input: String)` —
  discover what limitation this has (can you mix different stage types in one slice?).
- Version B: `Vec<Box<dyn Stage>>` — mixed stage types in one pipeline, chained at runtime.
- Add a method to `Stage` that returns `Self` and watch Version B stop compiling.
  Write a comment explaining object safety in your own words, then fix it
  (hint: the fix is a `where Self: Sized` escape hatch or redesign).
- Print `std::mem::size_of_val` of a `&dyn Stage` reference vs a plain `&ConcreteStage`.
  Explain the extra pointer.

**What you'll learn:** static vs dynamic dispatch, monomorphization vs vtables, object
safety rules, `Box<dyn Trait>`, fat pointers, trait objects' real trade-offs.

---

## Program 3 — Units & conversions library

**Goal:** Redo your Phase 1 temperature converter as a *type-safe library* — mixing up units becomes a compile error, like Dollars/Cents but wired into std's conversion traits.

**Requirements:**
- Newtypes: `Celsius(f64)`, `Fahrenheit(f64)`, `Kelvin(f64)`.
- Conversions go through the standard traits: `From<Celsius> for Fahrenheit`, etc. —
  all six directions. Calling code uses `.into()`.
- Kelvin below absolute zero is invalid: constructing Kelvin from a raw `f64` must be
  fallible (`TryFrom<f64>`), returning an error, not panicking.
- Implement `Display` for all three (`"23.5°C"`), and `Add` where it makes sense —
  decide (and document in a comment) whether `Celsius + Celsius` should even exist,
  and why `Celsius + Fahrenheit` must not.
- CLI front end: parse `"72F"` / `"23.5C"` / `"300K"` from stdin, convert to the other two.

**What you'll learn:** `From`/`Into`, `TryFrom` for fallible conversions, `Display` by hand,
operator overloading decisions as API design, newtypes + std traits working together.

---

## Program 4 — ⭐ Generic priority queue (milestone)

**Goal:** A reusable `PriorityQueue<T>` container — generic over what it stores and how it orders. You will reuse this in Phase 8's search engine.

**Requirements:**
- Works for ANY `T: Ord` out of the box: `push(item)`, `pop() -> Option<T>` (highest first),
  `peek() -> Option<&T>`, `len()`, `is_empty()`.
- Internal storage: your choice — but justify it in a comment (sorted Vec? binary heap you
  maintain yourself? Compare push/pop costs of your choice).
- A second constructor accepting a custom ordering so callers can prioritize however they
  want without `T` being `Ord` (you don't know closures deeply yet — a plain
  `fn(&T, &T) -> Ordering` function pointer is fine; Phase 5 will upgrade your view of this).
- Implement `Default`, `Debug` (derive where possible — but note which derives put bounds
  on `T` and check: does your `Debug` derive require `T: Debug`?).
- Tests: ints, strings, and one custom struct (e.g. `Task { urgency: u8, name: String }`)
  with both natural and custom ordering.

**What you'll learn:** generic structs and impl blocks, trait bounds and `where` clauses,
`Ord`/`PartialOrd` semantics, derive bounds on generic types, designing a container API.

---

## Done when

- [ ] All four programs have passing tests
- [ ] You can explain: monomorphization vs vtable, orphan rule, object safety — out loud, no notes
- [ ] You know when to pick `impl Trait` param vs generic vs `Box<dyn Trait>`
