# Phase 7 — Modules, Crates & Cargo

Everything so far fits in one `main.rs`. Real projects don't. This phase is architecture:
module trees, visibility as API design, workspaces, features, and docs that can't rot
(doctests run as tests).

This phase has no new "programs" from scratch — it restructures what you already built.
That's deliberate: you learn layout by laying out code you understand.

---

## Program 1 — ⭐ Calculator workspace (milestone)

**Goal:** Turn the Phase 3+6 calculator into a real multi-crate project shaped like something you'd publish.

**Requirements:**
- A cargo **workspace** with two members:
  - `calc-core` — a library crate. Internal modules: `lexer`, `parser`, `eval`, `error`
    (separate files). The public API is deliberately tiny: aim for one entry function
    (`evaluate(&str) -> Result<Value, CalcError>`) plus the error type. Everything else
    `pub(crate)` or private.
  - `calc-cli` — a binary crate depending on `calc-core` by path. Owns the REPL loop,
    arg handling, error printing (the caret rendering from Phase 6 lives HERE, not in core —
    decide why and write it down).
- Re-export so users write `use calc_core::evaluate`, never `use calc_core::eval::evaluate`
  (`pub use` façade).
- **Every public item documented**: `///` with at least one runnable example each —
  `cargo test` must run your doc examples and pass. Crate-level docs (`//!`) explain the
  crate in 5 lines with an example.
- `cargo doc --open` — read your own docs as a stranger would. Fix what confuses you.
- An `examples/` directory with one example program (`cargo run --example quick`).
- Integration test in `calc-core/tests/` exercising ONLY the public API — if the test needs
  a private item, your API is wrong, not the test.

**What you'll learn:** workspaces, lib vs bin split, module tree + file layout, `pub` vs
`pub(crate)` as design pressure, `pub use` façades, rustdoc + doctests, integration tests,
`examples/`.

---

## Program 2 — Feature flag exercise

**Goal:** Add an optional capability behind a Cargo feature — the mechanism every serious crate uses.

**Requirements:**
- A `history` feature on `calc-core`: when enabled, the evaluator records every expression
  + result and exposes `history()`; when disabled that code doesn't even compile into the
  crate (`#[cfg(feature = "history")]`).
- Feature is OFF by default. `calc-cli` turns it on via its dependency declaration and adds
  a `:history` REPL command.
- Prove both build states: `cargo build -p calc-core` and
  `cargo build -p calc-core --features history` both succeed;
  `cargo test --all-features` passes.
- Bonus: make the feature pull in an optional dependency (e.g. `chrono` to timestamp
  history entries) — optional dep tied to the feature.

**What you'll learn:** cargo features, `#[cfg(...)]` conditional compilation, optional
dependencies, feature hygiene (additive-only rule).

---

## Program 3 — Toolchain pass (whole repo)

**Goal:** Meet the tools that keep Rust codebases honest, by pointing them at everything you've written so far.

**Requirements:**
- `cargo fmt` everything; commit the diff separately so you can read what it changed.
- `cargo clippy --workspace -- -W clippy::pedantic` — for EVERY warning: either fix it, or
  add `#[allow(...)]` with a one-line comment defending the choice. No silent allows.
  Expect clippy to teach you ~10 idioms you didn't know (`.map_or`, `if let` chains,
  needless clones...).
- `cargo audit` on the repo (install via `cargo install cargo-audit`).
- Write a `check.sh` (or `justfile`) that runs fmt-check + clippy + tests in one command —
  your poor man's CI. It must exit non-zero if anything fails.
- Skim `cargo tree` output for the workspace; find one transitive dependency you didn't
  know you had.

**What you'll learn:** clippy as a tutor, rustfmt, cargo-audit, dependency awareness,
the check-script habit that becomes CI later.

---

## Done when

- [ ] `cargo test --workspace` passes, doctests included
- [ ] Public API of `calc-core` is ≤ 5 items and fully documented
- [ ] `./check.sh` is your reflex before every commit
