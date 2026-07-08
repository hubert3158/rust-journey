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
- Give one concrete stage a second trait with a method named `run` too (e.g. a `Describe`
  trait). Now a plain `.run()` call is ambiguous — resolve it with **fully qualified
  syntax** (`<T as Stage>::run(...)`). One comment: when does this come up in real code
  (hint: `Iterator::next` vs your own `next`).
- Add a **supertrait**: `trait LoggedStage: Stage` with a default method that calls
  `self.run(...)` and prints timing. Then take a `&dyn LoggedStage` and pass it where a
  `&dyn Stage` is wanted — **trait upcasting** (stable since Rust 1.86). Note what the
  coercion does to the vtable pointer.

**What you'll learn:** static vs dynamic dispatch, monomorphization vs vtables, object
safety rules, `Box<dyn Trait>`, fat pointers, fully qualified syntax, supertraits,
trait upcasting, trait objects' real trade-offs.

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

## Program 5 — Odds-and-ends drill (~1 hour, one playground file)

**Goal:** The Book's "Advanced Types" leftovers — small, but a master knows them cold.

**Requirements:**
- **Type alias:** your `PriorityQueue` results probably repeat a long `Result<..., ...>` —
  define `type QueueResult<T> = ...` and use it. Note: alias ≠ newtype (no new type safety).
- **DSTs & `?Sized`:** write `fn describe<T: ?Sized + std::fmt::Debug>(t: &T)` and call it
  with a `str` and a `[i64]`. Then remove `?Sized` and read the error. Comment: why do
  `str`/`[T]`/`dyn Trait` only live behind pointers, and what does `Sized` (implicit on
  every generic) actually assert?
- **Default generic type parameters:** you already used one — `std::ops::Add` is
  `trait Add<Rhs = Self>`. Prove you can override the default: impl `Add<Cents> for
  Dollars` (mixed-type addition) on your Phase 3 money types. One comment: why does the
  default make the common case (`Dollars + Dollars`) require zero annotations?
- **Associated consts:** give your units trait (or a new `Bounded` trait) a
  `const MIN: Self; const MAX: Self;` — impls provide values, generic code reads
  `T::MAX`. Note how this differs from a method returning the value.
- **`const fn`:** make one of your newtype constructors `const fn` and use it to build a
  `const` value. Try to put something non-const inside and read the error — that boundary
  is the whole concept.
- **`Any` + downcasting (bonus):** store `Vec<Box<dyn Any>>`, put three different types
  in, get one back out with `.downcast_ref::<T>()`. One comment: why is this almost
  always the wrong design in Rust (what did you lose that enums/generics keep?) — and
  name-check where it's legitimate (plugin registries, `anyhow`'s internals).
- **GATs awareness (stretch, read-only):** try to sketch a "lending iterator" whose `next`
  returns a reference into itself — see why the plain `Iterator` trait can't express it,
  and read how a generic associated type (`type Item<'a>`) fixes it. No implementation
  required; a 5-line summary comment is the deliverable. While you're reading, learn to
  *recognize* two unstable features blog posts mention: specialization and TAIT — one
  sentence each, no code.

**What you'll learn:** type aliases, dynamically sized types, `?Sized`, the implicit
`Sized` bound, default generic type params, associated consts, `const fn`, `Any`'s
trade-off, GATs as a signpost for later.

---

## Done when

- [ ] All programs have passing tests
- [ ] You can explain: monomorphization vs vtable, orphan rule, object safety — out loud, no notes
- [ ] You know when to pick `impl Trait` param vs generic vs `Box<dyn Trait>`
- [ ] You can disambiguate two same-named trait methods without looking up the syntax
