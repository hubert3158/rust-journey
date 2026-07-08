# Phase 3 — Custom Data Types (Structs & Enums)

Rust isn't OOP. You model logic with structs, enums, and pattern matching — and the goal
of this phase is one idea: **make illegal states unrepresentable.** Every program below is
that idea in a different costume.

Specs describe **what**, never **how**. Prove each program with your own `#[test]`s before
moving on (like you already did for Money).

---

## Program 1 — Money handler 🚧 (in progress — finish these)

**Goal:** A small library for handling money correctly — dollars and cents can't be mixed up.

**Already done:** `Dollars`/`Cents` newtypes, conversions both ways, `Add` with carry,
derives, two tests. Good.

**Remaining requirements to call it finished:**
- Kill the illegal state: right now `Dollars(1, 150)` can exist (150 cents in the cents
  slot). Make it unrepresentable from outside: constructor normalizes (`Dollars::new(1, 150)`
  → `Dollars(2, 50)`) and arithmetic can never produce it. A test constructs the worst
  input and asserts the invariant.
- `Display` so `{}` prints `$3.02` (two-digit cents, always). `Debug` stays derived.
- Parse from user text: `"12.34"` / `"$12.34"` / `"7"` → money, with a proper error value
  for garbage (`"12.345"`, `"abc"`, `""`) — no panics. (This is `Option`/`Result` practice
  before Program 3 makes you build your own.) Implement it as the **`FromStr` trait** —
  that's the std hook that makes `"12.34".parse::<Money>()` work like it does for numbers.
- Overflow question: what does `Cents(u64::MAX) + Cents(1)` do in your code today?
  Write the test, observe, then decide the policy (checked method returning an error, or
  document the panic). Phase 14 will property-test this exact thing — beat it to the punch.

**What you'll learn:** tuple structs & newtypes, invariants enforced by constructors,
`Display` vs `Debug`, fallible parsing, derives (`Copy`/`Clone`/`PartialEq`) — and the
habit of asking "what values can this type hold that it shouldn't?"

---

## Program 2 — Payment state machine

**Goal:** Track one payment through its life — every stage carries exactly its own data, and impossible events are rejected.

**Requirements:**
- One enum, stages as variants, each carrying only what belongs to it:
  scheduled (run date), submitted (submitted-at, rail used), settled (settled-at),
  returned (reason code + human message). A settled payment CANNOT hold a return reason —
  the type forbids it.
- Events as a second enum: `Submit`, `Settle`, `Return { reason }`, `Cancel`.
- One function: current state + event → `Result<NewState, TransitionError>`. Illegal
  combos (settle a returned payment, cancel a settled one) return a specific error naming
  both the state and the event — never panic.
- The `match` must be exhaustive with NO catch-all `_` arm for state+event pairs — when
  you add a new variant later, the compiler must point at every place that needs thought.
  (Feel free to add a `PartialRefund` variant at the end to experience exactly that.)
- Every state also carries shared info (id, amount, currency) in one `PaymentInfo` struct —
  use **field init shorthand** when building it and **struct update syntax** (`..old`) when
  producing the next state's copy-with-one-change.
- Pattern-matching vocabulary — use each at least once, naturally:
  - destructuring variant fields in match arms
  - a match **guard** (e.g. returns after a cutoff date route differently)
  - `if let` for a one-case peek, `let else` for an early return
  - an `@` binding (e.g. capture a reason code while matching a range of codes)
  - `matches!` for a one-line boolean state check (e.g. `fn is_terminal(&self) -> bool`)
  - a **slice pattern** on the event history: `[first, .., last]` to report the first and
    latest event in one match
- Somewhere you'll match on `&Payment` and bind a `String` field without writing `ref` —
  stop there and write a 2-line comment on **match ergonomics** (binding modes): why the
  compiler gives you `&String` automatically and what the old explicit form looked like.
- Small CLI or test-driven driver that walks a payment through a full happy path and at
  least three rejected transitions.

**What you'll learn:** data-carrying enums, exhaustive matching as a refactoring tool,
the full pattern syntax (guards, `@`, `if let`, `let else`, `matches!`, slice patterns,
binding modes), struct update syntax, errors as values.
*(Phase 15 rebuilds this with typestate so illegal transitions won't even compile —
keep your version, you'll compare them side by side.)*

---

## Program 3 — Your own Option and Result

**Goal:** Build `Maybe<T>` and `Outcome<T, E>` from scratch — discover that Rust's two most-used types are just enums, and their combinators are just `match`.

**Requirements:**
- `Maybe<T>`: your version of "value or nothing". `Outcome<T, E>`: "success or failure".
- Methods on both, each a few lines of `match`:
  - transform the inner value when present (`map`)
  - chain a fallible step so failures short-circuit (`and_then`)
  - fall back to a default (`unwrap_or` / `unwrap_or_else`)
  - convert between them: `Maybe → Outcome` by supplying an error (`ok_or`)
- A worked example using them end-to-end: e.g. parse config text "port=8080" → find key →
  parse number → validate range, chained, one failure path.
- Compare against std: write the same example with real `Option`/`Result` and `?`. Then
  answer in a comment: what does `?` desugar to, roughly, in terms of YOUR `Outcome`?
- Generic syntax is new here (`enum Maybe<T>`) — that's fine and on purpose. Phase 4
  formalizes generics; this is your preview by necessity.

**What you'll learn:** generics by building the two types you'll use daily forever,
combinators from the inside, why `?` isn't magic, `enum` + `impl` + `match` fluency.

---

## Program 4 — JSON value in memory

**Goal:** Represent ANY JSON document as one Rust type, then pretty-print it back out.

**Requirements:**
- One enum covering every JSON shape: null, bool, number, string, array, object.
  Nesting to any depth must work — arrays in objects in arrays.
- Somewhere in this type the compiler will refuse a naive definition ("recursive type has
  infinite size"). Hit that error, read it fully, fix it. That error is the whole lesson
  about why heap indirection exists.
- Pretty-printer: proper indentation, one field per line, correct comma placement (no
  trailing comma), strings quoted. Compact one-line printer as a second mode.
- Build a small document by hand (your CV as JSON: name, skills array, nested address)
  and snapshot-test the pretty output against an expected string.
- NO parsing in this program — text → value comes later (Program 5 does it for math;
  a JSON parser is a great optional stretch AFTER Program 5).

**What you'll learn:** recursive types and `Box`, modeling "any of N shapes" with enums,
recursive functions over recursive data, `Vec` vs `Box` (when does the enum need which?).
*(This type is load-bearing later: Phase 4's serde-lite outputs it, Phase 12's `json!`
macro constructs it.)*

---

## Program 5 — ⭐ Expression calculator (capstone)

**Goal:** Read a math expression as text, compute the result. The text → structure → result pipeline behind every parser, interpreter, and compiler.

**Requirements:**
- Supports `+ - * /`, parentheses, decimal numbers, unary minus (`-3 + 2`).
- Correct precedence and associativity: `2 + 3 * 4` = 14, `2 - 3 - 4` = -5,
  `(2 + 3) * 4` = 20.
- Two distinct phases in code, with a named type between each:
  raw text → tokens (an enum) → expression tree (a recursive enum, Program 4's lesson
  applied) → number. No evaluating while parsing.
- Errors are values with meaning, not panics: stray symbol, unbalanced parens, unexpected
  end, division by zero — each its own variant. (Phase 6 will upgrade these with byte
  positions and caret rendering; leave room.)
- REPL loop: expression in, result or error out, never dies.
- Extension test: once everything passes, add `%` (or `^`) — count how many places the
  compiler forces you to touch. That number is the payoff of exhaustive enums.
- Tests: the precedence table above, nesting, every error variant, weird-but-legal input
  (`" ( 1 ) "`, `--3` — decide and document).

**What you'll learn:** everything in this phase combined — tokenizing, recursive trees
with `Box`, exhaustive matching across a pipeline, `Option`/`Result` under real load,
custom error enums. *(This program has a long life: Phase 6 refits its errors, Phase 7
splits it into a workspace, Phase 14 fuzzes it.)*

---

## Done when

- [ ] All five programs pass their tests
- [ ] "Make illegal states unrepresentable" means something concrete to you — you can
      point at three places you did it
- [ ] You can write `enum` + `impl` + exhaustive `match` without looking anything up
- [ ] The calculator survives your nastiest inputs with errors, not panics
