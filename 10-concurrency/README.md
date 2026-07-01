# Phase 10 — Concurrency (Fearless Multithreading)

Rust's boldest claim: data races are compile errors. The guarantee comes from two marker
traits — `Send` and `Sync`. Before writing any code, answer on paper: why is `Rc` not
`Send`? Why is `RefCell` not `Sync`? Keep your answers; check them after Program 1.

Every program here: run it many times. Concurrency bugs are probabilistic; one clean run
proves nothing (in Rust, the compiler already proved the data-race part — feel the
difference between what it can and can't prove: deadlocks are still on you).

---

## Program 1 — ⭐ Thread pool from scratch (milestone)

**Goal:** Fixed pool of workers executing arbitrary jobs — the Rust Book's web-server pool, but build it BEFORE looking, then compare against theirs.

**Requirements:**
- `ThreadPool::new(n)` spawns n workers; `execute(job)` accepts any closure to run
  (work out yourself what bounds the closure needs — the compiler will tell you; write a
  comment translating each bound: what does `Send` mean HERE, what does `'static` mean HERE).
- Jobs come through a channel; workers loop receiving. Discover the "one receiver, many
  workers" problem and solve it (the solution involves something from Phase 9).
- Graceful shutdown on drop: no new jobs, workers finish current job, all joined. Prove
  with output ordering in a test.
- Demonstrate `Send`/`Sync` errors on purpose: try to `execute` a closure capturing an
  `Rc` — paste the compiler error in a comment and explain it in one sentence.
- Compare with `std::thread::scope`: one example where scoped threads borrow a local
  `Vec` directly — no `Arc`, no `'static` — and note when scope beats a pool.

**What you'll learn:** `thread::spawn`/join, `Send + 'static` bounds decoded,
channel + shared-receiver pattern, `Arc<Mutex<Receiver>>`, graceful shutdown, scoped threads.

---

## Program 2 — Parallel grep

**Goal:** Real work distributed across your own pool — and honest measurement.

**Requirements:**
- Search a pattern in all files under a directory (reuse Phase 8's file walker) using the
  Program 1 pool; results (file, line number, line) stream back over a channel while
  searching continues.
- Print results as they arrive; final summary line: files scanned, matches, elapsed.
- Single-threaded flag `--serial`. Benchmark both on a big folder (your whole projects
  dir): record numbers in a comment. Try pool sizes 1, 2, 4, 8 — where does it stop
  helping and why (IO-bound vs CPU-bound: which is this?).
- Poison one file (no read permission): the pool must report the error and keep going —
  one bad file can't kill the run.

**What you'll learn:** work distribution, streaming results via channels, ownership moving
into jobs, measuring instead of assuming, error isolation across threads.

---

## Program 3 — Metrics counter shootout

**Goal:** Same job, three synchronization strategies, one table of numbers — atomics and contention made visible.

**Requirements:**
- 8 threads, each incrementing a shared counter 1,000,000 times. Three implementations:
  1. `Arc<Mutex<u64>>`
  2. `AtomicU64` (`fetch_add` — start with `Ordering::Relaxed`; comment on why Relaxed
     is fine for a pure counter but wouldn't be for a flag protecting data)
  3. Per-thread local counters, summed at the join
- Time all three; print a comparison table. Expect the ranking to surprise you; explain it
  (contention, cache-line bouncing).
- Add a `AtomicBool` stop-flag variant: threads spin until a coordinator flips it —
  your first acquire/release pairing. One comment: what could go wrong with Relaxed here.
- Sanity checks: every variant must total exactly 8,000,000.

**What you'll learn:** atomics, `Ordering` with working intuition (Relaxed vs
Acquire/Release vs SeqCst), contention costs, why sharing less beats syncing better.

---

## Program 4 — Rayon retrofit

**Goal:** Learn when the library beats your hand-rolled pool: data parallelism in one line.

**Requirements:**
- Take the Phase 8 search-engine index build and parallelize it with rayon's `par_iter`
  (per-file tokenization in parallel, merge at the end).
- The merge step forces a decision: parallel-then-merge vs `Mutex` around one shared map.
  Try both; measure both; keep the winner and record the numbers.
- Also rayon-ify one pure-CPU task (e.g. count primes below 10M using your Phase 5 primes
  logic) and record the speedup vs serial.
- One paragraph comment: when do you reach for rayon vs channels+threads vs (later) async?

**What you'll learn:** rayon `par_iter`, parallel map-reduce shape, merge strategies,
choosing the right concurrency tool per problem class.

---

## Done when

- [ ] You can explain `Send` vs `Sync` and why `Rc`/`RefCell` lack them — without notes
- [ ] Your pool shuts down cleanly under test, every run
- [ ] You have real numbers showing where parallelism helps and where it doesn't
