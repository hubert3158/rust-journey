# Phase 16 — Capstone & Specialization

Depth now beats breadth. Pick ONE capstone and finish it — a substantial project that
forces phases 4–15 to work together teaches more than three more tutorials. "Finish"
means: tested, documented, `check.sh`-clean, README with usage, and one honest
`LESSONS.md` about what fought back.

Whichever you pick, the same bar applies:
- workspace layout with a small public API (Phase 7)
- typed errors, no stray unwraps (Phase 6)
- property tests or a fuzz target on the parsing surface (Phase 14)
- a benchmark with recorded numbers for the hot path (Phase 14)
- at least one deliberately designed API following your audit checklist (Phase 15)
- **real-ecosystem surface**: CLI args via `clap` (derive API — you'll recognize your
  Phase 12 skills in what it generates), config/persistence via real `serde`, and
  `tracing` spans on the hot path instead of `println!` debugging. You built the
  educational clones; a master also ships with the tools everyone shares.

---

## Option A — ⭐ Mini-Redis (recommended)

**What:** An async key-value server speaking (a subset of) the real Redis protocol —
`GET`, `SET`, `DEL`, expiry, `SUBSCRIBE`/`PUBLISH`. The real `redis-cli` should be able
to talk to it.

**Guided path:** tokio's official mini-redis tutorial (tokio.rs → tutorial). Work it,
then extend beyond it — the extensions are where the learning is:
- key expiry (a background task + the right data structure for "what expires next")
- `Arc<Mutex<...>>` state first; then measure and shard it (N maps by key hash) —
  benchmark the difference under concurrent load
- graceful shutdown that drains in-flight commands
- a `MONITOR`-style command streaming all traffic to an admin connection

**Exercises:** async (11), concurrency (10), protocol parsing (3/6), collections (8),
API design (15), benchmarks (14).

---

## Option B — Bitcask key-value store

**What:** A PERSISTENT KV store — append-only log files on disk, in-memory index of
key → file offset, crash recovery by replaying the log, background compaction.

**Guided path:** the PNA Rust course (github.com/pingcap/talent-plan), or the Bitcask
paper (it's short) + your own design.

**The interesting problems:** file format design (length-prefixed records, checksums),
recovery correctness (kill -9 the process mid-write in a test; it must recover),
compaction without stopping reads, `fsync` decisions and what they cost.

**Exercises:** files & bytes (8), error design (6), unsafe-adjacent thinking about
durability, property tests on the storage layer (14), benchmarking write paths (14).

---

## Option C — Interpreter (Lox or your own language)

**What:** Your Phase 3 calculator grown to a full language: variables, scopes, functions,
closures, control flow. The straight-line continuation of everything you built.

**Guided path:** *Crafting Interpreters* (craftinginterpreters.com — free online), porting
the Java tree-walk interpreter to Rust. The port is the exercise: Java's GC'd object graphs
force you into Rust decisions (`Rc<RefCell<Environment>>` for closures — Phase 9 pays off).

**The interesting problems:** environments and closures (this is where it gets real),
error recovery in the parser (report many errors, not just the first), the
Rust-vs-inheritance modeling of the AST (enums beat visitor patterns — or do they?).

**Exercises:** enums & recursion (3), traits (4), closures conceptually (5), errors (6),
Rc/RefCell (9), fuzzing the parser (14).

---

## Option D — Full TUI app

**What:** Grow `TUI-app/` into a real application — a task manager / kanban with
persistence, or a dashboard for your Option A/B server. Ratatui, async data layer,
config, themes, mouse + keyboard.

**Guided path:** ratatui book + its `examples/` directory (the async template especially).

**The interesting problems:** the render-loop-vs-async-events architecture (Phase 11's
bridge problem, permanently), state management as the app grows (message/update pattern —
Elm architecture), responsive layout, testing a TUI (snapshot-test the rendered buffer —
`insta` from Phase 14).

**Exercises:** async bridging (11), state architecture (15), snapshot testing (14),
config loading (6), everything ergonomic you've learned.

---

## After the capstone — specialization tracks

| Track | Focus | Entry point |
|-------|-------|-------------|
| 🧵 Systems | Embedded, OS, memory | `no_std`, embedded-hal, phil-opp's *Writing an OS in Rust* |
| 🌐 Backend | Web APIs, async | axum + sqlx + tracing; deploy something real |
| 🕹️ Game Dev | Bevy, ECS | Bevy book — ECS is a genuinely different architecture |
| 🌍 WASM | Browser Rust | wasm-bindgen first, then Leptos/Yew |
| 🧠 Tooling | Compilers, LSPs | a formatter/linter for Option C's language |

Also worth knowing: **CodeCrafters** (build your own Redis/git/shell/SQLite, in Rust) and
**PNGme** (hide messages in PNGs — smaller warm-up capstone) if you want more guided reps
before specializing.
