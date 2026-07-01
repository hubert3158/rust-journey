# Phase 11 — Async Rust (Non-Blocking IO)

Threads' complement: massive concurrency for IO-bound work. Steepest concepts in Rust
live here — `Future`, `Waker`, `Pin`, cancellation-by-drop. Order matters in this phase:
build the toy executor FIRST. Everything after stops being magic.

Sharpest edge to internalize: **dropping a future cancels it.** Every `select!`, every
timeout, every abandoned task — code just stops running at its last `.await`. Keep that
in your head through all three programs.

---

## Program 1 — Toy executor (no tokio, no async-std, ~150 lines)

**Goal:** Hand-build the machinery `async`/`await` compiles into, once, so you know what the runtime actually does.

**Requirements:**
- A hand-written `Future` (impl the trait yourself): a `Delay` that completes after a
  deadline. `poll` returns `Pending` before, `Ready` after.
- First version may busy-poll (executor loops over `poll`). Second version must use the
  `Waker` properly: a thread parks; a timer thread calls `wake()`; the executor only polls
  when woken. Feel the difference — that gap IS why runtimes exist.
- A tiny executor: takes one top-level future, polls it to completion using a `Waker` you
  construct (std's `Wake` trait on an `Arc` is the sane route).
- Then run an `async fn` that awaits two `Delay`s in sequence on YOUR executor —
  proof that async/await sugar runs on anything implementing the contract.
- Comments narrating the flow: who calls poll, who holds the waker, who wakes whom.

**What you'll learn:** the `Future` trait for real, `Poll`, `Waker`/`Wake`, why futures are
lazy state machines, what a runtime fundamentally is. *(Pin appears here too — note where
the compiler makes you touch it and read just enough to know why.)*

---

## Program 2 — Site health checker

**Goal:** The daily-driver async skill set: bounded concurrency, timeouts, retries, structured results. Tokio from here on.

**Requirements:**
- Input: a text file of ~100 URLs. Check all CONCURRENTLY — but never more than 10 in
  flight (`tokio::sync::Semaphore`).
- Per-request: 5s timeout (`tokio::time::timeout`) and up to 3 attempts — port your
  Phase 5 retry combinator to async (takes a closure returning a future; getting the
  bounds right is the exercise).
- Each result: URL, status, latency, attempts used. Collect via `JoinSet` (task panics
  must not kill the run — handle the join error path).
- Live progress line while running (`watch` channel or a counter task + `select!`).
- Summary table at the end sorted by latency; non-zero exit if any URL failed.
- One deliberate experiment: drop a future mid-flight (via `select!` racing a short
  timeout) and log that its post-await code never ran — cancellation made visible.

**What you'll learn:** `tokio::spawn`/`JoinSet`, Semaphore-bounded concurrency, timeouts,
`select!`, cancellation semantics, async closures/bounds, `Send + 'static` on tasks.

---

## Program 3 — ⭐ Chat server + TUI client (milestone)

**Goal:** The full async server shape — then a real client for it, growing your `TUI-app/` seed into something alive.

**Requirements — server:**
- Tokio TCP server: many clients; each connection = one task; first line is the nickname.
- Every message fans out to all other clients via `tokio::sync::broadcast`; joins and
  leaves are announced.
- Per-connection task `select!`s over: socket reads, broadcast receives, shutdown signal.
- Graceful shutdown on Ctrl-C (`watch` channel): all clients get a "server closing"
  message, connections close cleanly, no task left dangling.
- Slow-client policy: broadcast lag (`RecvError::Lagged`) must be handled — decide drop vs
  disconnect and comment the choice.

**Requirements — client:**
- Ratatui TUI: message pane + input line (this is what `TUI-app/` was waiting for).
- The hard part: terminal events are blocking, sockets are async — solve the bridge
  (dedicated input thread + channel into the async side is the classic shape).
- `/quit` works; server disconnect shows a message instead of a crash.

**What you'll learn:** async TCP, task-per-connection architecture, broadcast/watch
channels, `select!` loops, graceful shutdown, bridging sync and async worlds, backpressure
decisions.

---

## Done when

- [ ] You can narrate what happens between `.await` and the task resuming — wakers included
- [ ] Two terminal windows are chatting through your server
- [ ] "Dropping a future cancels it" changed how you write async code
