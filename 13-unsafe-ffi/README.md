# Phase 13 — Unsafe & FFI

`unsafe` doesn't turn off the borrow checker — it lets you make promises the compiler
can't verify. A broken promise is undefined behavior *even if every test passes*. The
craft: smallest possible unsafe core, wrapped in a sound safe API, verified with Miri.

Non-negotiable rule for this phase: **every test runs under Miri**
(`rustup +nightly component add miri`, then `cargo +nightly miri test`).
Miri catches the lies your tests can't.

Before coding, list in a note file the five things `unsafe` permits, and a personal UB
catalog: dangling pointers, aliasing `&mut`, invalid values, data races, out-of-bounds.
You'll check items off as you personally trigger each one under Miri.

---

## Program 1 — ⭐ `Vec` from scratch (milestone)

**Goal:** Build `MyVec<T>` on raw allocation — what every heap collection is made of. Follow the Rustonomicon's "Implementing Vec" chapter as your guide; type it yourself, understand every line.

**Requirements:**
- Raw parts: pointer, length, capacity. Allocation via `std::alloc`, `NonNull<T>` for the
  pointer (learn why not `*mut T` directly — niche optimization).
- `push` (with growth: amortized doubling), `pop`, `get`, `Drop` (every element dropped,
  memory freed — Miri will tell the truth).
- `Deref<Target = [T]>` so slicing, iteration, and indexing come free.
- Handle zero-sized types or explicitly panic on them with a comment (the Rustonomicon
  covers why ZSTs break naive pointer math).
- Tests: push/pop roundtrip, growth across capacity boundaries, drop counting (element
  type that increments a counter on drop — assert exact drop count), all under Miri.
- Deliberately introduce one bug (e.g. forget to call `drop_in_place` on pop), run Miri,
  paste its diagnosis in a comment, revert.

**What you'll learn:** `std::alloc`, `NonNull`, `ptr::read/write/drop_in_place`, growth
strategy, `Drop` correctness, `Deref` to slice, Miri as a lie detector.

---

## Program 2 — Doubly linked list

**Goal:** The structure that broke your Phase 9 LRU design — now with raw pointers, done right. Guide: "Learn Rust With Entirely Too Many Linked Lists" (the unsafe deque chapters).

**Requirements:**
- `push_front`, `push_back`, `pop_front`, `pop_back`, `len`, iterator (at least by-ref).
- Raw pointers (`NonNull<Node<T>>`) — not `Rc<RefCell<...>>`; that was the Phase 9 lesson,
  this is the other side.
- `Drop` frees every node (Miri-verified, no leaks).
- Aliasing discipline: a comment at each unsafe block stating the invariant that makes it
  sound ("head is either null or points to a live node whose prev is null", etc.). These
  invariant comments are the actual exercise.
- Bonus, big: rebuild the Phase 9 LRU cache with O(1) get/put on top of your list +
  `HashMap` of node pointers. This is the payoff the Phase 9 README promised.

**What you'll learn:** aliasing rules in practice, `NonNull`, invariant thinking, why Rust
made this hard on purpose, `PhantomData` (the list needs it — discover why).

---

## Program 3 — Wrap a C library

**Goal:** Real FFI: consume a C library and export a safe, sound, RAII Rust API where no raw pointer escapes.

**Requirements:**
- Target: `zlib` (simpler) or `sqlite3` (more real) — both are on every Linux box.
- Declare the `extern "C"` functions you need by hand first (3–5 functions); THEN compare
  against what `bindgen` generates from the header — note the differences.
- Safe wrapper type owning the C handle: created in `new()`, released in `Drop`,
  impossible to double-free or use-after-free from safe code.
- C error codes become `Result<_, YourError>`; C strings handled via `CStr`/`CString`
  (document the ownership rule at each boundary: who allocates, who frees).
- The public API contains ZERO `unsafe` and zero raw pointers — a user of your module
  can't cause UB no matter what they call in what order. Write the test that tries.
- Round-trip proof: compress-then-decompress bytes match (zlib), or create-insert-query
  works (sqlite).

**What you'll learn:** `extern "C"`, `#[repr(C)]`, linking, `CStr`/`CString`, ownership
across the language boundary, bindgen, RAII wrappers, sound API design over unsafe cores.

---

## Done when

- [ ] Every unsafe block in the folder has an invariant comment above it
- [ ] Miri passes on everything — and you've watched it catch a real bug at least once
- [ ] You can state the five unsafe superpowers and your UB catalog from memory
