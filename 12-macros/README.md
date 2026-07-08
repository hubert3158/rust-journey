# Phase 12 — Macros & Metaprogramming

Code that writes code, at compile time. Rule one before every program here: could a
function, generic, or trait do this? If yes, the macro is wrong. Macros earn their place
for exactly three jobs: variadic interfaces, DSL syntax, and derives.

This phase closes two loops: your Phase 3 JSON type gets literal syntax, and your Phase 4
`ToJson` trait gets its `#[derive]` — you'll have built the serde experience from the inside.

---

## Program 1 — `json!` literal macro

**Goal:** Declarative macro (`macro_rules!`) turning JSON-looking syntax into your Phase 3 JSON value type at compile time.

**Requirements:**
- All of these must compile and produce the right value:
  ```rust
  json!(null)
  json!(42)
  json!("hello")
  json!([1, 2, 3])
  json!({"name": "subash", "tags": ["rust", "cli"], "meta": {"level": 3}})
  json!([])          // empty array
  json!({})          // empty object
  ```
- Arbitrary nesting depth — the macro must recurse into itself for array elements and
  object values.
- Expressions as values must work: `json!({"sum": 1 + 2})`.
- Trailing commas accepted: `json!([1, 2, 3,])`.
- Inspect your expansions with `cargo expand` at least once; paste one expansion into a
  comment.
- Tests comparing macro output against hand-built values.

**What you'll learn:** `macro_rules!` fragment specifiers (`expr`, `tt`, `literal`),
repetition `$(...),*`, macro recursion, trailing-comma handling, hygiene, `cargo expand`.

---

## Program 2 — proc-macro-workshop: Builder

**Goal:** The canonical guided introduction to procedural macros. Do NOT improvise this one — clone `github.com/dtolnay/proc-macro-workshop` and work the `builder` project's test suite in order.

**Requirements:**
- Work through the numbered tests of the `builder` project; each test unlocks the next
  concept (parsing the input struct with `syn`, generating code with `quote`, handling
  `Option` fields, attributes, error spans).
- Get at least through test 07 (attribute handling). Further is bonus.
- Keep a running `NOTES.md` in this folder: one line per test about what it forced you
  to learn — this becomes your proc-macro cheat sheet.

**What you'll learn:** proc-macro crate setup (`proc-macro = true`), `syn` (parsing Rust
syntax trees), `quote!` (generating code), `TokenStream`, spans and good error messages,
derive-macro architecture.

---

## Program 3 — ⭐ `#[derive(ToJson)]` (milestone)

**Goal:** Close the Phase 4 loop: structs get your serialization for free, exactly like serde does it.

**Requirements:**
- `#[derive(ToJson)]` on a named-field struct generates the `ToJson` impl: each field
  serialized under its own name, calling the field's own `ToJson` (so nesting and
  `Vec`/`HashMap` fields work via your Phase 4 impls).
- Must handle: multiple field types, nested derived structs, generic structs
  (`struct Wrapper<T: ToJson>` — the derive must add the bound).
- One attribute: `#[to_json(rename = "otherName")]` on a field changes the output key.
- Deriving on an enum or tuple struct: produce a CLEAN compile error pointing at the
  right span, not a panic. Test it with `trybuild` (compile-fail tests).
- Round-trip proof: a struct with your derive, serialized, parsed back by your Phase 3
  JSON parser (if you built one) or checked against expected text.

**What you'll learn:** the full derive workflow end-to-end on YOUR trait, `syn` field
iteration, generated bounds on generics, custom attributes, `trybuild` compile-fail
testing, respect for what serde does.

---

## Closing the loop — meet the real serde (1 evening, no code)

You've now built serde's architecture yourself: trait (Phase 4) + derive (here) + literal
macro (here). Cash it in: skim serde's docs and map every feature to what you built —
`#[serde(rename)]` ↔ your `to_json(rename)`, derive bounds on generics ↔ what you added
by hand, `Serialize` ↔ `ToJson`. Then use REAL `serde_json` once: derive
`Serialize`/`Deserialize` on a struct, round-trip it. From now on you use serde like
everyone else — the difference is you know exactly what it generates.

---

## Done when

- [ ] `json!` handles anything you throw at it, nested and trailing-comma'd
- [ ] You can read `syn`/`quote` code without fear
- [ ] `#[derive(ToJson)]` works on a generic struct with a renamed field
- [ ] serde's docs read like a description of code you've already written
