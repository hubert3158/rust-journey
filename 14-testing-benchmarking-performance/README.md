# Phase 14 — Testing, Benchmarking & Performance

Professional Rust is measured Rust. Property tests find bugs your examples can't imagine;
fuzzers find the ones you can't; benchmarks and flamegraphs replace folklore with data.
No new toy programs this phase — your own earlier projects are the victims. That's the
point: real testing skills only show on code you didn't write for the test.

Discipline for the whole phase: **measure → change → re-measure.** Any optimization
without a before/after number didn't happen.

---

## Program 1 — Property-test the money & calculator libs

**Goal:** Stop testing examples, start testing LAWS. `proptest` generates thousands of inputs and shrinks failures to minimal cases for you.

**Requirements:**
- Phase 3 `Money`/`Cents`/`Dollars` laws, each as a proptest:
  - roundtrip: cents → dollars → cents is identity, for ALL amounts
  - addition commutes and is associative
  - no panic on any pair of u64 inputs (this one probably FAILS — your Add can overflow;
    decide the fix: checked math returning `Result`, or saturating — and defend it)
- Phase 3/6 calculator laws:
  - printing an expression tree and re-parsing it yields an equal tree (build the printer
    if missing — roundtrip laws are the highest-value property there is)
  - evaluation never panics for ANY generated tree (division needs thought)
- Let proptest shrink at least one real failure; commit the regression file
  (`proptest-regressions/`) — that's a feature, not noise.
- One custom `Strategy` composing generators (e.g. building random expression trees with
  bounded depth).

**What you'll learn:** proptest, invariant/law thinking, shrinking, custom strategies,
regression corpora, the overflow bug you didn't know you had.

---

## Program 2 — ⭐ Fuzz the calculator (milestone)

**Goal:** Feed random garbage to your parser until it confesses. If Phase 6's error refit was honest, it survives. It won't at first — they never do.

**Requirements:**
- `cargo-fuzz` target: raw `&[u8]` → attempt UTF-8 → feed `evaluate()`. The ONLY
  acceptable outcomes are `Ok` or your error type — any panic is a bug.
- Run at least 10 minutes of fuzzing (longer overnight run: bonus).
- EVERY crash found becomes: (1) a minimized input file, (2) a permanent regression test
  in `calc-core`, (3) a fix. Keep a `CRASHES.md` log — input, cause, fix — your personal
  bug museum.
- Add one fuzz target for the Phase 8 log-line parser too (second parser, second
  perspective).
- Note corpus + coverage basics: why does the fuzzer keep inputs that "do something new"?

**What you'll learn:** cargo-fuzz/libFuzzer, panic-freedom as a testable contract,
crash triage → regression test workflow, why parsers are the classic fuzz target.

---

## Program 3 — Optimize the search engine

**Goal:** The full performance loop on your biggest program: benchmark, profile, fix the top hotspot, prove the win.

**Requirements:**
- `criterion` benchmarks for Phase 8 search-engine: index-build time on a fixed corpus
  (vendor a folder of text files so runs are comparable) and query latency. Learn
  `black_box` and why criterion runs warmups.
- Baseline numbers recorded BEFORE touching anything (`RESULTS.md`: machine, corpus
  size, numbers).
- Flamegraph the index build (`cargo flamegraph`). Read it. Name the top hotspot in
  writing before fixing it. Usual suspects you'll likely find: per-word `String`
  allocation in the tokenizer, clone-happy entry keys, re-reading files.
- Fix ONE thing at a time; re-benchmark after each; keep a table of deltas. Stop when a
  change wins less than 5%.
- Compare debug vs release numbers once (then never benchmark debug again). Try one
  release-profile experiment (`lto = true`, `codegen-units = 1`) and record the effect.
- Add integration-level test doubles somewhere meaningful: the file-reading layer behind
  a trait so index logic tests run on in-memory strings — no disk, no fixtures. (This is
  Rust's answer to mocking frameworks: just traits.)

**What you'll learn:** criterion, `black_box`, flamegraph reading, allocation hunting,
release profiles/LTO, one-change-at-a-time discipline, test doubles via traits.

---

## Done when

- [ ] A proptest-shrunk failure taught you a bug example-based tests missed
- [ ] The calculator survives sustained fuzzing with zero panics
- [ ] `RESULTS.md` shows a measured, honest speedup with the flamegraphs to back it
