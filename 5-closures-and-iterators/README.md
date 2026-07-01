# Phase 5 — Closures & Iterators (Functional Rust)

The most-used abstractions in real Rust code. `Fn`/`FnMut`/`FnOnce` is just ownership
applied to functions — by the end of this phase that sentence should feel obvious.

Rule for this phase: **no `for` loops with index counters.** If you catch yourself writing
`for i in 0..v.len()`, find the iterator way.

---

## Program 1 — Build your own iterator adapters

**Goal:** Reimplement the core adapters yourself so that `map`/`filter` stop being magic.

**Requirements:**
- `MyMap` — a struct wrapping any iterator + a transform; implements `Iterator` itself.
- `MyFilter` — same idea, keeps only items passing a predicate.
- `Pairs` — yields overlapping pairs of the underlying iterator: `[1,2,3]` → `(1,2), (2,3)`.
- An extension trait so they chain off any iterator naturally:
  `vec.iter().my_map(|x| x * 2).my_filter(|x| x > 2)`.
- Everything must stay lazy: prove with a test that the transform closure does NOT run
  until someone calls `.next()` or `.collect()` (hint: closure sets a flag / pushes to a log).
- Bonus: implement `IntoIterator` for one of your own Phase 3/4 types
  (e.g. iterate a `PriorityQueue<T>` in priority order).

**What you'll learn:** implementing `Iterator` (associated `Item`, `next`), storing closures
in structs (generic params with `Fn` bounds), extension traits, laziness, `IntoIterator`.

---

## Program 2 — Event bus

**Goal:** A subscribe/dispatch system — closures stored as data, called later.

**Requirements:**
- `subscribe(event_name, callback)` — callback is any closure taking `&Event`.
- `emit(event)` — runs every callback registered for that event's name, in order.
- Callbacks must be able to MUTATE captured state (e.g. a counter closure that counts how
  many times "user_login" fired). Getting this to compile teaches you `Fn` vs `FnMut` —
  let the compiler errors drive; read each one fully before fixing.
- `unsubscribe` support: subscribing returns an id; removing by id works.
- One callback that consumes something it captured (a one-shot). Discover why the bus
  can't hold `FnOnce` the same way, and document what you did about it.

**What you'll learn:** `Box<dyn FnMut>` vs `Box<dyn Fn>` vs `FnOnce`, capture modes
(borrow / mut borrow / move), the `move` keyword, closures living longer than their scope.

---

## Program 3 — Lazy number lab

**Goal:** Infinite sequences that cost nothing until consumed.

**Requirements:**
- A Fibonacci iterator — infinite, no collection backing it.
- A primes iterator — infinite, any method.
- Answer, in one chained expression each (no loops, no intermediate Vecs):
  - first 10 primes greater than 1000
  - sum of all even Fibonacci numbers below 1,000,000
  - first prime that is a palindrome AND greater than 10,000
- Each answer verified by a test.

**What you'll learn:** infinite iterators, combinator fluency (`take`, `take_while`, `filter`,
`skip_while`, `find`, `sum`), thinking in pipelines instead of loops.

---

## Program 4 — ⭐ Retry combinator (milestone)

**Goal:** A tiny, real utility: run a fallible operation up to N times. Small program, maximum concept density. You will reuse it (async version) in Phase 11.

**Requirements:**
- `retry(times, op)` where `op` is any closure returning `Result<T, E>`.
- Returns first `Ok`, or the LAST error if all attempts fail.
- The closure must be allowed to mutate captured state (an attempt counter outside).
  Pick the right `Fn*` bound; write a comment on why the other two bounds are wrong here.
- A variant `retry_with_backoff` that also takes a second closure computing the delay
  from the attempt number (just compute, no real sleeping needed — return the delays used).
- Tests: succeeds first try, succeeds on 3rd, exhausts all attempts, counter state visible after.

**What you'll learn:** choosing `Fn`/`FnMut`/`FnOnce` bounds deliberately, generics + closures
+ `Result` in one signature, returning values vs errors from higher-order functions.

---

## Done when

- [ ] You can say what `FnOnce`, `FnMut`, `Fn` each mean in one ownership sentence
- [ ] You know why adapters are lazy and what `collect()` actually does
- [ ] `for i in 0..len` is gone from your muscle memory
