# Phase 15 — API Design & Patterns

Everything before this was mechanics. This phase is taste: encoding invariants in types
so misuse doesn't compile. The recurring question — for every design here, ask it out
loud: **"can a user of this API hold it wrong?"** If yes, can the type system forbid it?

Skim the official Rust API Guidelines (rust-lang.github.io/api-guidelines/) before
starting; Program 4 applies the checklist for real.

---

## Program 1 — ⭐ Typestate payment machine (milestone)

**Goal:** Rebuild Phase 3's payment tracker so the state lives in the TYPE. Invalid transitions stop being runtime errors and become code that won't compile.

**Requirements:**
- States as types: `Payment<Scheduled>`, `Payment<Submitted>`, `Payment<Settled>`,
  `Payment<Returned>` (zero-sized marker types + one generic struct).
- Transitions consume self and return the next state:
  `Payment<Scheduled>::submit(self) -> Payment<Submitted>` — after submitting, the old
  scheduled value is GONE (move semantics as workflow enforcement).
- `settle()` simply does not exist on `Payment<Scheduled>` — prove it with a compile-fail
  test (`trybuild`, from Phase 12): a file that calls it and MUST fail to compile, with
  the error text asserted.
- Per-state data still applies: only `Payment<Returned>` carries a return reason, only
  `Payment<Scheduled>` carries a run date (associated data on the marker types).
- Shared behavior (e.g. `id()`, `amount()`) written once, available in every state.
- The honest comparison, in this folder's notes: enum version (Phase 3) vs typestate —
  when is each right? (Hint: what happens to typestate when states come from user input
  or a database at runtime? Write the answer down.)

**What you'll learn:** typestate pattern, zero-sized marker types, `PhantomData`,
move-based transitions, compile-fail testing, the runtime-vs-compile-time state trade-off.

---

## Program 2 — Request builder

**Goal:** Builder pattern, then typestate-hardened: `send()` doesn't exist until required fields are set. Infallible by construction — no `Option` checks, no runtime "missing url" error.

**Requirements:**
- Stage 1, classic builder: `Request::builder().url(...).method(...).header(...).timeout(...).build()`
  returning `Result` (missing url = error). Chaining must work; headers accumulate.
- Stage 2, typestate builder: two type parameters tracking "url set?" / "method set?" —
  `send()`/`build()` implemented ONLY for the fully-set state. The missing-url failure
  mode is now unrepresentable; `build()` returns `Request`, not `Result`.
- Ergonomics: setters accept `impl Into<String>` / `impl AsRef<str>` so callers pass
  `&str` or `String` freely; one comment on when to take `&str` vs `String` vs generic.
- `#[must_use]` on the builder — an unused builder chain should warn.
- Compare both stages in notes: what did typestate cost in code size and error-message
  quality? (Typestate compile errors are famously cryptic — experience one.)

**What you'll learn:** builder pattern, typestate on builders, `impl Into<T>` parameter
design, `#[must_use]`, API ergonomics vs type-safety trade-offs.

---

## Program 3 — Transaction guard

**Goal:** RAII beyond memory: a guard whose `Drop` rolls back — commit must be explicit. The pattern behind `MutexGuard`, database transactions, and every "scoped" API.

**Requirements:**
- For the MiniLedger (playground) or your own ledger: `ledger.begin() -> Transaction<'_>`;
  operations go through the transaction, not the ledger.
- Explicit `commit(self)` applies changes. If the guard drops WITHOUT commit (early
  return, `?` bailing out, panic) — nothing is applied. All three paths tested, including
  the panic one (`catch_unwind` in the test).
- While a transaction is live, the ledger itself must be untouchable — enforce with
  borrows (`&mut self` in `begin`), not runtime flags. Compile-fail test proving it.
- Decide and document: what does `commit` return if the underlying apply fails? (The
  Drop-can't-return-errors problem — meet it, pick a policy, defend it.)
- Bonus: a `#[must_use]` on `Transaction` so silently dropping one warns.

**What you'll learn:** RAII guards, `Drop` for logical cleanup, lifetimes as exclusivity
enforcement, panic-safety, the Drop-and-errors dilemma every real guard API faces.

---

## Program 4 — API audit of `calc-core`

**Goal:** Apply the official checklist to a real crate of yours. Auditing your own API teaches more than reading ten guideline docs.

**Requirements:**
- Walk `calc-core` (Phase 7) through the Rust API Guidelines checklist, section by
  section; keep an `AUDIT.md` with a verdict per item (pass / fixed / consciously skipped
  + why).
- Minimum fixes to hunt for: naming conventions (`as_`/`to_`/`into_` prefixes used
  correctly), every public type implements the expected std traits (`Debug`, `Clone`,
  `PartialEq`, `Display` where sensible — C-COMMON-TRAITS), no public fields that encode
  invariants, `#[non_exhaustive]` on the error enum (future variants without semver
  break), `#[must_use]` where dropping a result is a bug.
- Sealed-trait exercise: if `calc-core` exposes any trait users shouldn't implement,
  seal it (public trait, private supertrait); write the compile-fail test showing a
  downstream impl is impossible.
- Version bump reasoning: list which of your fixes would be semver-major if the crate
  were published, and why.

**What you'll learn:** the API Guidelines by application, std-trait completeness,
`#[non_exhaustive]`, sealed traits, semver thinking, reading your own API as a stranger.

---

## Done when

- [ ] A wrong payment transition is a compile error with a test proving it
- [ ] You can articulate when typestate is worth it and when an enum is
- [ ] `AUDIT.md` exists and changed real code
