---
title: Complete Rust Mastery Roadmap
created: 2025-05-14
tags: [rust, roadmap, programming]
---

# 🦀 complete rust mastery roadmap

> A fully structured, in-depth roadmap to becoming a systems-level Rust expert, ideal for Obsidian.

**🎯 Goal:** Learn every concept by building a real program for it — no passive reading, each phase ends with working code.

**🧵 How this roadmap works:** Many programs deliberately build on earlier ones. The Phase 3 calculator gets better errors in Phase 6, becomes a workspace in Phase 7, gets fuzzed in Phase 14. The Phase 4 `ToJson` trait gets its derive macro in Phase 12. The Phase 3 payment state machine gets a compile-time typestate upgrade in Phase 15. One growing codebase teaches more than fifteen throwaway ones.

## 📊 Progress

| Phase | Status | Code |
|-------|--------|------|
| 1. Rust Basics & Mental Model | ✅ Done | [`1-Basics/`](./1-Basics/) |
| 2. Ownership, Borrowing & Lifetimes | ✅ Done | [`2-Ownership-Burrowing-Lifetimes/`](./2-Ownership-Burrowing-Lifetimes/) |
| 3. Custom Data Types | 🚧 In progress | [`3-custom-data-types/`](./3-custom-data-types/) |
| 4. Traits & Generics | ⬜ Not started | [`4-traits-and-generics/`](./4-traits-and-generics/) |
| 5. Closures & Iterators | ⬜ Not started | [`5-closures-and-iterators/`](./5-closures-and-iterators/) |
| 6. Error Handling | ⬜ Not started | [`6-error-handling/`](./6-error-handling/) |
| 7. Modules, Crates & Cargo | ⬜ Not started | [`7-modules-crates-cargo/`](./7-modules-crates-cargo/) |
| 8. Collections & Strings | ⬜ Not started | [`8-collections-and-strings/`](./8-collections-and-strings/) |
| 9. Smart Pointers | ⬜ Not started | [`9-smart-pointers/`](./9-smart-pointers/) |
| 10. Concurrency | ⬜ Not started | [`10-concurrency/`](./10-concurrency/) |
| 11. Async | ⬜ Not started | [`11-async/`](./11-async/) |
| 12. Macros | ⬜ Not started | [`12-macros/`](./12-macros/) |
| 13. Unsafe & FFI | ⬜ Not started | [`13-unsafe-ffi/`](./13-unsafe-ffi/) |
| 14. Testing & Performance | ⬜ Not started | [`14-testing-benchmarking-performance/`](./14-testing-benchmarking-performance/) |
| 15. API Design & Patterns | ⬜ Not started | [`15-api-design-patterns/`](./15-api-design-patterns/) |
| 16. Capstone | ⬜ Not started | [`16-capstone/`](./16-capstone/) |

---

## 📘 Phase 1: Getting Started — _Rust Basics & Mental Model_ ✅

**Why this phase matters:**  
This phase sets the foundation. Rust's syntax looks familiar, but its _model_ is different. Skip this, and everything else feels painful.

**What you’ll learn:**

- Variables, mutability
- Scalar & compound types
- Functions, expressions
- `if`, loops, `match`
- Basic collections: Vec, tuple, array

**Teaches you:**

- Rust is expression-oriented
- Immutability by default helps safety

**Milestone:**  
Write a simple CLI calculator or guessing game.

---

## 🧪 Phase 2: Ownership, Borrowing & Lifetimes — _The Rust Mindshift_ ✅

**Why this phase matters:**  
Ownership is Rust’s core innovation. Mastering this means unlocking safe systems programming.

**What you’ll learn:**

- Move semantics, `Copy`, `Clone`
- Borrowing: `&T`, `&mut T`
- Lifetimes: `'a`, elision rules

**Teaches you:**

- How the compiler guarantees memory safety
- Understanding the “borrow checker”

**Milestone:**  
Write a file parser that borrows from a buffer.

---

## 🧱 Phase 3: Custom Data Types — _Modeling with Structs & Enums_ 🚧

**Why this phase matters:**  
Rust isn't OOP. You model real-world logic using structs, enums, and pattern matching.

**What you’ll learn:**

- `struct`, tuple struct, newtype
- `enum`, nested types
- Pattern matching: `match`, `if let`, `while let`, `let else`
- `Option`, `Result`

**Teaches you:**

- Business logic as type-safe models
- The phase's one big idea: **make illegal states unrepresentable**

**Programs (full specs in [`3-custom-data-types/README.md`](./3-custom-data-types/README.md)):**

1. **Money handler** 🚧 — newtypes, constructor-enforced invariants, `Display`, fallible parsing *(in progress — finish-line items listed in the folder README)*
2. **Payment state machine** — data-carrying enums, exhaustive matching, full pattern syntax (guards, `@`, `if let`, `let else`) *(Phase 15 rebuilds it with typestate)*
3. **Your own Option/Result** — build `Maybe<T>`/`Outcome<T, E>` from scratch; combinators and `?` stop being magic
4. **JSON value in memory** — recursive enums, `Box`, pretty-printing *(used later by Phase 4 serde-lite and Phase 12's `json!` macro)*
5. ⭐ **Expression calculator** (capstone) — text → tokens → tree → result *(lives on: Phase 6 error refit, Phase 7 workspace, Phase 14 fuzz target)*

---

## 🔄 Phase 4: Traits & Generics — _Abstraction in Rust_

**Why this phase matters:**  
This is Rust's answer to inheritance and interfaces — but with a twist: you must understand *when the compiler generates code* (generics → monomorphization, zero cost) versus *when it looks things up at runtime* (`dyn Trait` → vtable). Almost every API you'll ever use is built from the concepts in this phase.

**What you’ll learn:**

- Defining traits, required vs default methods, supertraits
- Generic functions, structs, and `impl` blocks; trait bounds, `where` clauses, turbofish
- Associated types vs generic type parameters (when to use which)
- Trait objects: `dyn Trait`, object safety rules, `Box<dyn Trait>` vs `&dyn Trait`
- Static vs dynamic dispatch — trade-offs, monomorphization, vtables
- The standard conversion traits: `From`/`Into`, `TryFrom`/`TryInto`, `AsRef`, `Borrow`
- `Display` and `Debug` by hand; derive semantics of `PartialEq`, `Eq`, `Hash`, `Ord`, `Default`
- Operator overloading (`std::ops`)
- Blanket implementations (`impl<T: Display> MyTrait for T`)
- Coherence and the orphan rule — why you can't impl foreign traits on foreign types
- `impl Trait` in argument and return position
- Const generics basics (`struct Matrix<const N: usize>`)

**Programs to build:**

1. **Serde-lite** — your own `ToJson` trait with impls for primitives, `Vec<T: ToJson>`, `HashMap<String, T>`, and one blanket impl. Reuse your Phase 3 JSON value type as the output. *(Teaches: trait design, generics, associated types, the orphan rule when you try to impl for foreign types. In Phase 12 you'll write `#[derive(ToJson)]` for it.)*
2. **Dispatch lab** — a text-processing pipeline (`trait Stage { fn run(&self, input: String) -> String }`) implemented twice: once with generics (`Vec` of one concrete type, chained statically) and once with `Vec<Box<dyn Stage>>`. Print `std::mem::size_of` of things; observe what compiles and what object safety rejects. *(Teaches: static vs dynamic dispatch viscerally.)*
3. **Units & conversions library** — redo your Phase 1 temperature converter properly: `Celsius`, `Fahrenheit`, `Kelvin` newtypes wired together with `From`/`TryFrom` (negative Kelvin must fail), `Display`, and `Add`. *(Teaches: the conversion trait web, fallible conversions.)*
4. ⭐ **Generic priority task queue** — a `PriorityQueue<T>` that works for any `T: Ord`, plus a variant accepting a custom comparator. Used again in Phase 8. *(Teaches: bounds, generic data structures, `Ord`/`PartialOrd` semantics.)*

---

## ⚡ Phase 5: Closures & Iterators — _Functional Rust_

**Why this phase matters:**  
Closures and iterators are the most-used abstractions in real Rust code — more than traits you define yourself. The `Fn`/`FnMut`/`FnOnce` distinction *is* ownership applied to functions, and iterator chains are how idiomatic Rust replaces most loops. The original roadmap skipped this entirely; it's a required phase.

**What you’ll learn:**

- Closure capture modes: by reference, by mutable reference, by move; the `move` keyword
- `Fn` vs `FnMut` vs `FnOnce` — what each means and when the compiler picks which
- Taking closures as parameters (generic bounds) and returning them (`impl Fn`, `Box<dyn Fn>`)
- Function pointers (`fn`) vs closures
- Implementing `Iterator` (associated `Item`, `next()`) and `IntoIterator`
- Why adapters are lazy; `iter()` vs `iter_mut()` vs `into_iter()`
- The combinator vocabulary: `map`, `filter`, `filter_map`, `fold`, `zip`, `chain`, `take_while`, `flat_map`, `collect` into different containers
- Writing extension traits (`trait IteratorExt: Iterator`)

**Programs to build:**

1. **Build your own adapters** — implement `MyMap`, `MyFilter`, and `Pairs` (yields overlapping pairs) as structs implementing `Iterator`, exposed via an extension trait so you can write `v.iter().my_map(...)`. *(Teaches: how the real adapters work inside — nothing about iterators is magic afterwards.)*
2. **Event bus** — register callbacks (`Box<dyn FnMut(&Event)>`) for named events, then dispatch. Try to make a callback mutate outside state and watch the borrow checker teach you `FnMut` vs `Fn`. *(Teaches: closures as stored data, capture semantics.)*
3. **Lazy number lab** — an infinite Fibonacci/primes generator as an `Iterator`, then chains like "first 10 primes that are palindromes". Nothing computes until `collect`/`next`. *(Teaches: laziness, infinite sequences, combinator fluency.)*
4. ⭐ **Retry combinator** — `fn retry<T, E>(times: u32, op: impl FnMut() -> Result<T, E>) -> Result<T, E>`. Small, but forces closures + generics + `Result` together. You'll reuse it in Phase 11.

---

## 🛠️ Phase 6: Error Handling — _Fallible Logic the Right Way_

**Why this phase matters:**  
Rust has no exceptions. The `?` operator, the `Error` trait, and `From`-powered conversions form one coherent machine — and library code and application code handle errors *differently*. Knowing which style to use where is a professional skill.

**What you’ll learn:**

- `panic!` vs `Result` — what each is for; when `unwrap`/`expect` is actually fine
- The `?` operator desugared: how `From` conversions power it (you saw this in MiniLedger)
- Designing custom error enums; implementing `Display` + `std::error::Error`
- Error source chains (`source()`), wrapping underlying errors
- `Box<dyn Error>` — when it's enough
- `thiserror` for libraries vs `anyhow` (with `.context()`) for applications — and why
- `Option`/`Result` combinators: `map`, `and_then`, `ok_or`, `unwrap_or_else`, `transpose`
- Process exit codes; reporting errors to users vs logs

**Programs to build:**

1. **CSV → JSON converter CLI** — read a CSV, emit JSON, with a hand-rolled error enum (`IoError`, `ParseError { line, col }`, `SchemaError`) implementing `Error` + `From` so `?` flows everywhere. No `thiserror` yet — do it manually once. *(Teaches: the full error machine by hand.)*
2. **Layered config loader** — merge defaults ← config file ← env vars; errors must say *which layer and which key* failed, with `source()` preserving the underlying cause. Do this one with `thiserror` for the lib part and `anyhow` in `main`. *(Teaches: the two-crate error style used in real projects.)*
3. ⭐ **Calculator error refit** — go back to your Phase 3 calculator: replace every panic with a specific error carrying the byte position, and print a caret (`^`) under the offending character. *(Teaches: error design as UX; upgrading existing code.)*

---

## 🔍 Phase 7: Modules, Crates & Cargo — _Scalable Project Architecture_

**Why this phase matters:**  
Everything so far fits in one `main.rs`. Real projects don't. Module visibility, workspace layout, features, and documentation are what make a codebase navigable — and rustdoc + doctests are Rust's secret weapon for keeping docs honest.

**What you’ll learn:**

- The module tree: `mod`, file/folder mapping, `use`, paths (`crate::`, `super::`)
- Visibility levels: `pub`, `pub(crate)`, `pub(super)` — designing a minimal public API
- Re-exports (`pub use`) to shape a crate's façade
- lib crate vs bin crate; `src/bin/`, `examples/`, `tests/` directories
- Workspaces: shared `Cargo.lock`, path dependencies between member crates
- Cargo features (optional deps, conditional compilation with `#[cfg(feature = "...")]`)
- rustdoc: `///`, `//!`, intra-doc links, **doctests** (examples that run as tests)
- semver, `cargo publish --dry-run`, `cargo tree`, `cargo add`
- Toolchain hygiene: `clippy` (fix every lint once, learn why), `rustfmt`, `cargo audit`

**Programs to build:**

1. ⭐ **Calculator workspace** — split the Phase 3/6 calculator into a workspace: `calc-core` (lib: lexer, parser, evaluator as separate modules with a deliberately small public API) + `calc-cli` (bin). Every public item gets a doc comment with a runnable doctest. *(Teaches: everything above on code you already understand.)*
2. **Feature flag exercise** — add a `bigint` feature to `calc-core` that swaps `f64` for a big-decimal type behind `#[cfg]`. *(Teaches: conditional compilation, optional dependencies.)*
3. **Toolchain pass** — run `cargo clippy -- -W clippy::pedantic` over your whole repo; fix or consciously `allow` each lint with a comment saying why. *(Teaches: idiom, fast.)*

---

## 📦 Phase 8: Collections & Strings — _The Standard Library in Depth_

**Why this phase matters:**  
Fluency here is what makes Rust feel fast to write instead of painful. Strings especially: UTF-8 handling is where most newcomers get bitten, and the `HashMap` entry API and `mem::take`-style tricks are daily tools.

**What you’ll learn:**

- `Vec` internals: capacity vs length, reallocation, `retain`, `drain`, `swap_remove`
- `HashMap`: the **entry API** (`entry().or_insert_with()`), what types can be keys (`Hash + Eq`)
- `BTreeMap` (sorted keys, range queries), `HashSet`/`BTreeSet`, `VecDeque`, `BinaryHeap`
- Choosing the right collection — and what the wrong choice costs
- `String` vs `&str` vs `Cow<str>` — when each; string slicing is byte-indexed!
- UTF-8 reality: `chars()` vs `bytes()` vs grapheme clusters; why `s[0]` doesn't compile
- Sorting: `sort_by_key`, `sort_unstable`, `binary_search`; why floats aren't `Ord` (`total_cmp`)
- `std::mem::take` / `swap` / `replace` — moving out from behind `&mut`

**Programs to build:**

1. ⭐ **Mini search engine** — index a folder of text files into an inverted index (`HashMap<String, Vec<(DocId, Count)>>`), answer multi-word queries ranked by count (use your Phase 4 priority queue). *(Teaches: entry API, nested collections, ranking, string processing at scale.)*
2. **Log analyzer** — parse timestamped log lines, bucket into `BTreeMap<Timestamp, Stats>`, answer range queries ("errors between 14:00 and 15:00"), keep top-N slowest with `BinaryHeap`. *(Teaches: BTreeMap ranges, heap, parsing.)*
3. **Markov text generator** — build word-pair frequency tables from input text, generate sentences. Almost entirely entry-API manipulation. *(Teaches: HashMap fluency until it's reflex.)*
4. **Unicode inspector** — CLI that takes a string and prints its bytes, chars, and lengths under each definition; feed it emoji and accented text. *(Teaches: UTF-8 truth, once and permanently.)*

---

## 🔒 Phase 9: Smart Pointers & Interior Mutability

**Why this phase matters:**  
Ownership says: one owner, mutation XOR sharing. Smart pointers are the escape hatches — shared ownership (`Rc`), runtime-checked mutation (`RefCell`), cycle-breaking (`Weak`). Knowing *which hatch* and *what it costs* is the difference between fighting the borrow checker and designing around it.

**What you’ll learn:**

- `Box<T>`: heap allocation, recursive types, `Box<dyn Trait>`
- `Rc<T>`/`Arc<T>`: shared ownership, refcounts, `Rc::clone` vs `.clone()` convention
- **`Weak<T>`** — reference cycles leak; parent pointers must be weak
- `Cell<T>` vs `RefCell<T>`: interior mutability, runtime borrow panics (cause one deliberately)
- `OnceCell` / `LazyLock`: lazy initialization, the modern replacement for `lazy_static`
- `Deref`/`DerefMut` coercion — why `&Box<String>` works where `&str` is wanted
- `Drop`: RAII, drop order, why you can't call `.drop()` directly
- `Cow<str>` revisited: clone-on-write APIs
- Stack vs heap, `size_of`, what `Rc<RefCell<T>>` actually costs

**Programs to build:**

1. ⭐ **File-tree model** — an in-memory directory tree where children hold `Rc<RefCell<Node>>` and each node has a `Weak` parent pointer. Support add/remove/move-subtree and full-path printing (walk up via parents). Prove no leak: check `Rc::strong_count` after drops. *(Teaches: the whole Rc/RefCell/Weak triad on the classic problem shape.)*
2. **LRU cache** — fixed capacity, `get` refreshes recency, evict least-recent on insert. `HashMap` + `VecDeque` is fine; note where the design creaks (this is why Phase 13's raw pointers exist). *(Teaches: combining collections, ownership tension in cache design.)*
3. **Undo/redo engine** — a document with edit history; snapshots via `Rc` sharing so unchanged text isn't copied. *(Teaches: persistent-ish data, sharing to avoid clones.)*
4. **Config singleton** — global, lazily-initialized, read-mostly config with `LazyLock`. *(Teaches: modern global state, five minutes, done.)*

---

## ⚙️ Phase 10: Concurrency — _Fearless Multithreading_

**Why this phase matters:**  
Rust's boldest claim: data races are compile errors. That guarantee flows from two marker traits — `Send` and `Sync` — that the original roadmap didn't even mention. Understand those and every confusing "cannot be sent between threads" error becomes readable.

**What you’ll learn:**

- `thread::spawn`, `JoinHandle`, why closures need `move`
- **`Send` and `Sync`** — what they mean, which types lack them (`Rc`, `RefCell`) and *why*
- `std::thread::scope` — borrowing from the stack across threads (kills most `Arc` boilerplate)
- `Arc<Mutex<T>>`, lock poisoning, `RwLock`, contention and deadlock (make one on purpose)
- Channels: `mpsc`, ownership transfer through channels, worker patterns
- Atomics: `AtomicU64`/`AtomicBool`, `Ordering` (`Relaxed` vs `SeqCst` — working intuition)
- `Condvar` for wait/notify
- `rayon`: `par_iter`, when data parallelism beats hand-rolled threads

**Programs to build:**

1. ⭐ **Thread pool from scratch** — fixed workers pulling jobs (`Box<dyn FnOnce() + Send>`) off a channel; graceful shutdown. This is the Rust Book's web-server pool — build it without looking first, then compare. *(Teaches: channels + closures + Send bounds in one design.)*
2. **Parallel grep** — walk a directory tree, search files across your thread pool, stream results back over a channel. Compare wall-time vs single-threaded on a big folder. *(Teaches: real work distribution, ownership across threads.)*
3. **Metrics counter shootout** — hammer a shared counter from 8 threads three ways: `Mutex<u64>`, `AtomicU64`, and per-thread counters merged at the end. Time them. *(Teaches: atomics, contention costs, `Ordering` in practice.)*
4. **Rayon retrofit** — parallelize your Phase 8 search-engine indexing with `par_iter`; measure speedup. *(Teaches: when the library beats your hand-rolled pool.)*

---

## ⚡ Phase 11: Async Rust — _Non-Blocking IO_

**Why this phase matters:**  
Async Rust is threads' complement: massive concurrency for IO-bound work. It's also where Rust's steepest concepts live — `Future`, `Pin`, cancellation-by-drop. Building a toy executor once makes all of it stop being magic.

**What you’ll learn:**

- What a `Future` is: `poll`, `Waker`, why async fns are lazy state machines
- `async`/`await`, `tokio` runtime (multi-thread vs current-thread)
- `tokio::spawn`, `JoinHandle`, `JoinSet`; `Send + 'static` bounds on tasks and why
- **Cancellation: dropping a future cancels it** — the async model's sharpest edge
- `select!`, timeouts (`tokio::time`), graceful shutdown patterns
- Async channels: `mpsc`, `oneshot`, `broadcast`, `watch` — each has a distinct job
- `Semaphore` for bounding concurrency; `spawn_blocking` for CPU work
- Streams (`futures::Stream`) as async iterators
- `Pin`/`Unpin` — working understanding of why self-referential futures need it
- Async traits (native since Rust 1.75), when you still need `Box<dyn Future>`

**Programs to build:**

1. **Toy executor** — hand-write a `Future` (a timer), a `Waker`, and a tiny single-threaded executor that polls it. ~150 lines, no tokio. *(Teaches: what `.await` actually compiles into; demystifies everything after.)*
2. **Site health checker** — check 100 URLs concurrently, max 10 in flight (`Semaphore`), per-request timeout, retries via your Phase 5 retry combinator (now async). Summarize results. *(Teaches: bounded concurrency, timeouts, cancellation — the daily-driver async skills.)*
3. ⭐ **Chat server** — tokio TCP: clients connect, messages fan out via `broadcast`, clean disconnects, `Ctrl-C` graceful shutdown via `watch`. Then add a TUI client with ratatui (grow your existing `TUI-app/`). *(Teaches: the full async server shape — tasks, channels, select, shutdown.)*

---

## 🧬 Phase 12: Macros & Metaprogramming

**Why this phase matters:**  
Macros generate code at compile time — Rust's answer to boilerplate without runtime cost. Rule one: reach for macros only after functions, generics, and traits can't do it. This phase also closes two loops you opened earlier.

**What you’ll learn:**

- `macro_rules!`: fragment specifiers (`expr`, `ident`, `ty`, `tt`), repetition (`$(...),*`), hygiene
- When macros are wrong (most of the time) and right (variadics, DSLs, derives)
- Proc-macro crates: the three kinds (derive, attribute, function-like)
- `syn` (parse Rust code) + `quote` (generate it)
- Testing macros: `cargo expand`, `trybuild` for compile-fail tests

**Programs to build:**

1. **`json!` literal macro** — `macro_rules!` macro producing your Phase 3 JSON value type from literal syntax: `json!({"a": [1, 2, null]})`. *(Teaches: recursion + repetition in declarative macros, on your own type.)*
2. **proc-macro-workshop: Builder** — do the `derive(Builder)` project from dtolnay's proc-macro-workshop (github.com/dtolnay/proc-macro-workshop). It's the canonical guided path; don't improvise this one. *(Teaches: syn/quote properly, with test-driven checkpoints.)*
3. ⭐ **`#[derive(ToJson)]`** — write the derive macro for your Phase 4 `ToJson` trait: structs get field-by-field serialization for free. *(Teaches: the full derive workflow end-to-end on your own trait — the serde experience from the inside.)*

---

## 🧨 Phase 13: Unsafe & FFI — _Systems Programming Unleashed_

**Why this phase matters:**  
`unsafe` doesn't turn off the borrow checker — it lets you make promises the compiler can't verify. Breaking a promise is undefined behavior even if tests pass. The skill is building small unsafe cores wrapped in sound safe APIs — and using Miri to catch the lies.

**What you’ll learn:**

- Exactly what `unsafe` permits (5 abilities) and what it never changes
- Raw pointers (`*const`/`*mut`), aliasing rules, why `&mut` exclusivity still binds you
- A working catalog of UB: dangling pointers, invalid values, data races, aliasing violations
- `MaybeUninit`, `NonNull`, `std::ptr` routines; why `transmute` is a last resort
- **Miri** (`cargo +nightly miri test`) — run it on everything in this phase
- Unsafe traits: what implementing `Send`/`Sync` manually asserts
- FFI: `extern "C"`, `#[repr(C)]`, `CString`/`CStr`, who owns what across the boundary
- `bindgen` for consuming C headers; `cbindgen` for exposing Rust to C

**Programs to build:**

1. ⭐ **`Vec` from scratch** — raw allocation, growth, `Drop`, `Deref` to slice. Follow the Rustonomicon's "Implementing Vec" chapter. Every test runs under Miri. *(Teaches: what every heap collection is made of; allocation, pointers, drop correctness.)*
2. **Doubly linked list** — with raw pointers, guided by "Learn Rust With Entirely Too Many Linked Lists" (final chapters). Fixes the design creak you felt in the Phase 9 LRU cache. *(Teaches: aliasing discipline, NonNull, why Rust makes this hard on purpose.)*
3. **Wrap a C library** — safe Rust API over `zlib` or `sqlite3`: RAII handles, errors mapped to `Result`, no raw pointer escapes the module. *(Teaches: real FFI, ownership across languages, sound API design over unsafe cores.)*

---

## 🔬 Phase 14: Testing, Benchmarking & Performance

**Why this phase matters:**  
Professional Rust is measured Rust. Property tests and fuzzers find the bugs example tests can't imagine; benchmarks and flamegraphs replace performance folklore with data. Your own earlier projects are the perfect victims.

**What you’ll learn:**

- Test organization: unit (`#[cfg(test)]`), integration (`tests/`), doctests; `#[should_panic]`
- Test doubles via traits (swap the real dependency for a fake — no mocking framework)
- **Property-based testing** with `proptest`: invariants and roundtrip laws, shrinking
- **Fuzzing** with `cargo-fuzz`: throwing garbage at parsers until they confess
- Snapshot testing with `insta` for text output
- `criterion` benchmarks; why microbenchmarks lie (and `black_box`)
- Profiling: `cargo flamegraph`, reading a flame graph, allocation hunting
- Release profile: `opt-level`, `lto`, `codegen-units`; debug vs release gap
- `#[derive(Debug)]` vs real observability: `tracing` basics

**Programs to build:**

1. **Property-test the money lib** — Phase 3 `Money`: roundtrip law (`cents → dollars → cents` is identity), addition commutes, no overflow surprises. Let proptest shrink a failure for you. *(Teaches: invariant thinking, the proptest workflow.)*
2. ⭐ **Fuzz the calculator** — `cargo-fuzz` target feeding random bytes to your parser. Every crash becomes a regression test. If Phase 6's error refit was honest, it survives; it probably won't at first. *(Teaches: fuzzing loop, hardening real code.)*
3. **Optimize the search engine** — benchmark Phase 8 indexing with criterion, flamegraph it, fix the top hotspot (allocation in a loop? clone-happy code?), prove the speedup with numbers. *(Teaches: measure → fix → re-measure discipline.)*

---

## 🏛️ Phase 15: API Design & Patterns — _Writing Rust That Feels Like Rust_

**Why this phase matters:**  
Everything so far is mechanics. This phase is taste: encoding invariants so misuse doesn't compile. It's what separates "knows Rust" from "designs good Rust" — and it's almost entirely missing from beginner roadmaps.

**What you’ll learn:**

- Newtype pattern as API armor (you started this in Phase 3 — now systematize it)
- Builder pattern; **typestate**: encoding a state machine in the type system so invalid transitions are compile errors
- RAII guards: types whose `Drop` releases/rolls back (like `MutexGuard`)
- Sealed traits (public trait, private supertrait — no downstream impls)
- Extension traits for ergonomics
- API flexibility: accepting `impl AsRef<str>` / `IntoIterator`; `&str` vs `String` parameters
- `#[must_use]`, `#[non_exhaustive]`, semver hazards of public fields
- Skim the official Rust API Guidelines checklist and apply it

**Programs to build:**

1. ⭐ **Typestate payment machine** — rebuild Phase 3's payment tracker so state lives in the *type*: `Payment<Scheduled> → Payment<Submitted> → Payment<Settled>`, and `settle()` simply doesn't exist on a scheduled payment. Compare both versions: runtime-checked enum vs compile-time typestate, and when each is right. *(Teaches: the crown-jewel pattern, on a problem you already solved the other way.)*
2. **Request builder** — an HTTP-request builder where `.send()` only exists once a URL is set (typestate again, builder flavor). *(Teaches: infallible builders, method-chaining ergonomics.)*
3. **Transaction guard** — RAII guard for the MiniLedger: changes commit on explicit `.commit()`, roll back automatically on drop. *(Teaches: Drop-driven correctness, guard API shape.)*
4. **API audit** — take `calc-core` (Phase 7) through the API Guidelines checklist; fix naming, add `#[must_use]`, hide what shouldn't be public. *(Teaches: the checklist by application, not reading.)*

---

## 🌐 Phase 16: Capstone & Specialization — _Pick Your Path_

**Why this phase matters:**  
Depth now beats breadth. One substantial project that forces phases 4–15 to work together teaches more than three more tutorials.

**Recommended capstone (pick one):**

| Capstone | What it exercises | Guided path |
|----------|-------------------|-------------|
| ⭐ **Mini-Redis** — async KV server speaking a real protocol | async, parsing, concurrency, testing | tokio's official mini-redis tutorial |
| **Bitcask KV store** — persistent log-structured storage engine | files, serialization, indexing, crash recovery | PNA Rust course (talent-plan) |
| **Interpreter** — Lox or your own language | the Phase 3 calculator grown to full size: closures, scopes, GC-ish design | *Crafting Interpreters* (port to Rust) |
| **Full TUI app** — grow `TUI-app/` into a real task manager: async data layer, config, themes | ratatui, async, state architecture, polish | ratatui book + examples |

**Then specialize:**

| Track | Focus Area | Entry point |
|-------|------------|-------------|
| 🧵 Systems | Embedded, OS, memory | `no_std`, embedded-hal, Writing an OS in Rust (phil-opp) |
| 🌐 Backend | Web APIs, async | axum + sqlx + tracing; deploy something real |
| 🕹️ Game Dev | Bevy, ECS | Bevy book; ECS is a genuinely different architecture |
| 🌍 WASM | Leptos, Yew | wasm-bindgen first, then a framework |
| 🧠 Tooling | Compilers, LSPs, proc macros | write a formatter or linter for a toy language |

---

**🧠 Resources worth their time:**

- _The Rust Programming Language_ (the Book) — phases 1–10 reference
- **Rustlings** + **Exercism** — drill between phases
- _Rust for Rustaceans_ (Gjengset) — read alongside phases 8–15; also his YouTube deep-dives
- _Learn Rust With Entirely Too Many Linked Lists_ — phases 9 & 13
- _The Rustonomicon_ — phase 13 scripture
- **dtolnay/proc-macro-workshop** — phase 12
- **tokio mini-redis tutorial** — phases 11 & 16
- Rust API Guidelines — phase 15
