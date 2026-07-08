# Phase 9 — Smart Pointers & Interior Mutability

Ownership's rules: one owner, mutation XOR sharing. Smart pointers are the sanctioned
escape hatches — shared ownership (`Rc`), runtime-checked mutation (`RefCell`),
cycle-breaking (`Weak`). Each has a cost; this phase makes you feel each one.

Watch for the moment a design *needs* one of these vs merely *reaches* for one out of
borrow-checker frustration. The second is how Rust code rots.

---

## Program 1 — ⭐ File-tree model (milestone)

**Goal:** The classic shape that forces the whole Rc/RefCell/Weak triad: a tree where children point to parents AND parents point to children.

**Requirements:**
- In-memory directory tree: nodes are files or folders; folders hold children; EVERY node
  can name its parent.
- Operations: create path (`mkdir -p` style), remove subtree, move subtree, print full path
  of any node by walking UP through parents, pretty-print whole tree with indentation.
- Parent links must be `Weak` — first try `Rc` for parents, observe the leak (see below),
  then fix. Keep the broken version in a test comment as a war story.
- While the leak is live, write the 3-line comment that settles a common confusion:
  leaking is **safe** in Rust (`mem::forget` and `Box::leak` are safe functions; safety =
  no UB, not no leaks) — which is exactly why Rc cycles compile fine and are YOUR problem.
- Leak proof: a test that builds a tree, checks `Rc::strong_count`/`weak_count` at key
  points, drops the root, and asserts counts collapse to what you predicted. Predictions
  in comments BEFORE running.
- Cause a `RefCell` double-borrow panic deliberately in a `#[should_panic]` test (e.g.
  iterate children while mutating them), then write the corrected non-panicking version
  next to it.

**What you'll learn:** `Rc<RefCell<Node>>`, `Weak` + `upgrade()`, reference cycles and
leaks, runtime borrow rules (and panics), strong/weak counts, interior mutability's price.

---

## Program 2 — LRU cache

**Goal:** Bounded cache with recency eviction — a design that creaks in safe Rust, on purpose.

**Requirements:**
- `Lru::new(capacity)`, `put(k, v)`, `get(&k) -> Option<&V>`.
- `get` marks the entry most-recent; `put` beyond capacity evicts the least-recent.
- Use `HashMap` + `VecDeque` (or `Vec`) for recency order. It will work but feel clumsy —
  every `get` does a linear scan to move a key to the front. Measure and note it.
- Write, in the README-comment at the top of the file, WHY the clean O(1) design
  (hashmap pointing into a doubly-linked list) fights the borrow checker — you'll build
  exactly that with raw pointers in Phase 13. This program plants that motivation.
- Tests: eviction order, get-refreshes-recency, overwrite same key, capacity 1, capacity 0.

**What you'll learn:** combining collections, ownership tension in cache design, why some
textbook structures don't translate to safe Rust directly, honest cost accounting.

---

## Program 3 — Undo/redo engine

**Goal:** Document history without copying the whole document per edit — sharing via `Rc`.

**Requirements:**
- A "document" is a list of lines. Editing produces a NEW version; undo/redo walk versions.
- Naive version first: full clone per edit. Then the real one: versions share unchanged
  lines via `Rc<str>` (or `Rc<String>`) so an edit clones only the line list, not every line.
- Prove sharing works: after 100 edits to line 5 of a 1000-line doc, assert
  `Rc::strong_count` on an untouched line is 101 — the lines themselves were never copied.
- Operations: `edit(line_no, text)`, `undo()`, `redo()`; new edit after undo truncates the
  redo branch.
- Bonus: `Cow<str>` variant of the edit API — accept borrowed text, clone only when stored.

**What you'll learn:** `Rc` for cheap structural sharing, clone-what-exactly thinking,
`Rc<str>` vs `String`, `Cow`, persistent-data-structure intuition.

---

## Program 4 — Config singleton

**Goal:** Global, lazily-initialized, read-mostly state — the modern way. Five-minute program, permanent pattern.

**Requirements:**
- A global `CONFIG: LazyLock<Config>` loaded on first touch (reuse your Phase 6 loader
  logic or a stub).
- Prove single initialization: loader function increments an `AtomicU32` counter; touch
  the config from several threads; counter must read 1.
- Add one mutable-at-runtime field the right way (e.g. `LazyLock<RwLock<...>>`) and one
  set-exactly-once value with `OnceLock`.
- Comment: why `static mut` is the wrong answer (you'll formally meet it in Phase 13).
- Bonus: the third way to get `&'static Config` — `Box::leak(Box::new(config))`. Try it,
  note when it's legitimate (config lives for the whole program anyway) vs when `LazyLock`
  is simply better.

**What you'll learn:** `LazyLock`, `OnceLock`, global state without `unsafe`, lazy init,
a first taste of `RwLock` before Phase 10.

---

## Done when

- [ ] You can draw (on paper) who points to whom in the file-tree, strong vs weak
- [ ] You reach for `&`/`&mut` first, `Rc<RefCell>` only when ownership is truly shared
- [ ] You know what each of `Box`/`Rc`/`Arc`/`Cell`/`RefCell`/`Weak` costs
