# Phase 8 — Collections & Strings (std in depth)

Fluency phase. The entry API, the right collection for the job, and UTF-8 truth. Strings
are where most newcomers get bitten: Rust strings are bytes, valid UTF-8, and `s[0]`
doesn't compile — by the end you'll know exactly why.

Rule for this phase: before using a collection, say (in a comment) why not the alternatives.
`HashMap` vs `BTreeMap` vs `Vec` of pairs is a *decision*, not a default.

---

## Program 1 — ⭐ Mini search engine (milestone)

**Goal:** Index a folder of text files, answer ranked multi-word queries. The biggest program you've written so far.

**Requirements:**
- Index every `.txt`/`.md` file under a folder (recursively) into an inverted index:
  for each word, which documents contain it and how many times.
- Tokenization: lowercase, strip punctuation, skip a small stop-word list ("the", "a"...).
- Query REPL: `rust ownership` returns documents containing ALL words, ranked by total
  count — top 10 via your Phase 4 `PriorityQueue` (this is why you built it).
- Result line shows: rank, filename, score, and one matching line as a snippet.
- Second index mode `--prefix`: also answer prefix queries (`owner*`). Pick the right
  structure for prefix lookup (hint: one of the sorted collections has range queries)
  and defend the choice in a comment.
- Index build must be a separate step from querying (build once, query many).

**What you'll learn:** entry API under real load, nested collections
(`HashMap<String, Vec<(DocId, u32)>>`), `BTreeMap` range queries, choosing structures,
file walking, tokenization, ranking.

---

## Program 2 — Log analyzer

**Goal:** Time-ordered data → time-ordered structure. Range queries and top-N.

**Requirements:**
- Parse lines like `2026-07-01T14:32:11 ERROR db timeout after 3021ms` (make your own
  sample file generator — a fn that writes ~10k plausible lines; no `rand` needed, a simple
  counter-based pseudo-pattern is fine).
- Bucket into a `BTreeMap` keyed by minute; each bucket counts lines per level
  (INFO/WARN/ERROR).
- Answer: "errors between 14:00 and 15:00" using a RANGE query on the map — no full scan.
- Keep the 10 slowest requests (the `ms` values) using `BinaryHeap` — without ever storing
  all 10k durations (cap the heap at 10; note the min-heap-via-`Reverse` trick you'll need).
- Output a per-hour summary table, sorted naturally by the map's ordering.

**What you'll learn:** `BTreeMap` + `range()`, `BinaryHeap` and `Reverse`, streaming top-N,
simple parsing without regex, when sorted beats hashed.

---

## Program 3 — Markov text generator

**Goal:** Statistical text generation — pure entry-API workout.

**Requirements:**
- Feed it any large text (a book from Project Gutenberg).
- Build word-pair frequencies: for each word, which words follow it and how often —
  built ENTIRELY with `entry().or_insert_with(...)` / `or_default()`. No `contains_key`
  followed by `insert` anywhere (that pattern is the anti-lesson).
- Generate N sentences: start from a capitalized word, repeatedly pick the most frequent
  follower (deterministic is fine), stop at a period or 25 words.
- Also report: 10 most common words, count of distinct words, the word with most distinct
  followers — each as a single iterator chain.

**What you'll learn:** entry API until reflex, `HashMap<String, HashMap<String, u32>>`,
iterator chains over maps, sorting map contents (`Vec` collect + `sort_by_key`).

---

## Program 4 — Unicode inspector

**Goal:** Small CLI that ends every UTF-8 misconception you have, permanently.

**Requirements:**
- Takes a string arg, prints a table: each `char`, its byte length, its byte values.
- Prints THREE lengths: `s.len()` (bytes), `s.chars().count()`, and grapheme count
  (use the `unicode-segmentation` crate).
- Feed it: `"hello"`, `"héllo"`, `"नमस्ते"`, `"👨‍👩‍👧‍👦"` — record the three lengths of each in a
  comment. The family emoji is one grapheme, several chars, many bytes: explain.
- Demonstrate (in a test) that slicing at a non-boundary byte index panics, and show the
  safe alternatives (`char_indices`, `get(..)` returning `Option`).
- Reverse a string correctly (by grapheme) and incorrectly (by byte) — show both outputs.

**What you'll learn:** UTF-8 model for real: bytes vs chars vs graphemes, why indexing is
forbidden, boundary-safe slicing, `char_indices`.

---

## Done when

- [ ] Search engine answers queries on a real folder in the blink of an eye
- [ ] You pick HashMap/BTreeMap/Vec/VecDeque/BinaryHeap by trade-off, not habit
- [ ] You can explain the three string lengths of "👨‍👩‍👧‍👦" to someone else
