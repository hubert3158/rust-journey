# Phase 6 — Error Handling

Rust has no exceptions. `?`, the `Error` trait, and `From` conversions form one machine —
and libraries handle errors differently than applications. This phase you do it by hand
once, then learn the two crates everyone actually uses (`thiserror`, `anyhow`) and *why
they're used in different places*.

Ground rule: after this phase, `unwrap`/`expect` in your code needs a justification comment
or a test context. Every panic you write is now a decision, not a habit.

---

## Program 1 — CSV → JSON converter (all by hand, no error crates)

**Goal:** A CLI that reads a CSV file and prints JSON — where every possible failure has a precise, typed error. Do the whole error machine manually exactly once, so the crates never feel like magic.

**Requirements:**
- Reads a CSV path from args; first line is the header; every row becomes a JSON object
  (reuse your Phase 3 JSON type or Phase 4 serde-lite — your choice).
- One error enum covering at least: file not found / unreadable, empty file, a row with
  the wrong number of columns (error carries line number AND expected vs actual count),
  invalid UTF-8.
- Implement `Display` (human-readable message per variant) and `std::error::Error` for it.
- Write `From<std::io::Error>` so `?` lifts IO errors into your enum automatically.
- `main` returns `Result<(), YourError>`; a bad file exits non-zero with a clean one-line
  message — no `RUST_BACKTRACE`, no panic output, ever.
- **stderr discipline:** errors and progress go to stderr (`eprintln!`), JSON to stdout —
  `converter data.csv > out.json` must produce a clean file even when rows warn.
- Row parsing uses the fail-fast collect idiom at least once:
  `rows.map(parse_row).collect::<Result<Vec<_>, _>>()` — one comment on how `collect`
  can target `Result<Vec<T>, E>` at all (it's a `FromIterator` impl, not magic).
- One arm somewhere ends in `unreachable!()` or `todo!()` during development — before
  finishing, note in a comment why these type-check in any position (the never type `!`).
- Tests for each error variant using small fixture files (put them in `files/`).
- Once, deliberately: build with `panic = "abort"` in the profile and observe what changes
  (no unwinding, no `catch_unwind`, smaller binary). Revert; one comment on when servers
  choose abort.

**What you'll learn:** designing error enums, `Display` + `Error` impls, how `From` powers
`?` (the MiniLedger trick, now yours), error data as part of the type, exit codes,
stdout-vs-stderr, collecting `Result`s, the never type, unwind vs abort.

---

## Program 2 — Layered config loader (the two-crate style)

**Goal:** Load app config from three layers — defaults ← config file ← env vars — with errors so good the user never has to guess *which layer, which key*.

**Requirements:**
- Config struct with ~5 fields (port, log level, database url, etc.).
- Layer precedence: env var beats file beats default. Partial file (some keys) is fine.
- Structure it as two parts, like real projects:
  - the loading logic as a *library module* → errors with `thiserror`. Every error names
    the layer and the key: `"invalid value for 'port' in config file: 'abc' is not a number"`.
  - `main` as the *application* → uses `anyhow`, adds context with `.context()`, prints
    the FULL error chain (`{:#}` or walking `source()`).
- One deliberate deep failure (unreadable file inside file-layer inside load) must display
  as a readable chain of causes, outermost to root.
- Document in one comment: why `thiserror` for the lib part and `anyhow` for the app part —
  in your own words.

**What you'll learn:** `thiserror` derive, `anyhow::Context`, error source chains
(`source()`), the library-vs-application error split used across the ecosystem.

---

## Program 3 — ⭐ Calculator error refit (milestone)

**Goal:** Go back to your Phase 3 calculator. Replace every panic and vague error with precise, positioned errors. Error output becomes a feature.

**Requirements:**
- Every error carries the byte offset where the problem is; printing shows the input line
  with a caret underneath:
  ```text
  > 2 + (3 * 4
        ^ unclosed parenthesis opened here
  ```
- Distinct variants at minimum: unexpected character, unclosed paren, unexpected end of
  input, division by zero (with the position of the `/`).
- Lexer, parser, and evaluator each have their own error type; they compose into one
  top-level error via `From`.
- The REPL never dies on bad input — error shown, next prompt appears.
- Old panicking behavior fully gone: run your worst inputs (`"((("`, `"1//2"`, `""`, `"+"`)
  as tests asserting the exact variant returned.

**What you'll learn:** error design as UX, positional error data, composing error types
across layers with `From`, hardening existing code (Phase 14 will fuzz this — bugs you
leave now will be found then).

---

## Done when

- [ ] You can write the `Display`+`Error`+`From` triad from memory
- [ ] You can say when to use `thiserror` vs `anyhow` vs `Box<dyn Error>` and why
- [ ] The calculator survives every hostile input you can invent
