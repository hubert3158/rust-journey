Absolutely — here’s your **rewritten Phase 2 Rust checklist**, refined so that the **tasks themselves** push you into every major concept you need to master. No constraints listed — just **rich functionality** that makes the learning unavoidable.

---

# 🦀 Rust Mastery: Phase 2 Completion Checklist

## ✅ Master These Core Concepts

- [ ] Move and reuse values safely across scopes
- [ ] Decide when to clone or borrow
- [ ] Work with both shared (`&T`) and mutable (`&mut T`) references
- [ ] Write functions that return borrowed data
- [ ] Design structs that hold borrowed data using lifetimes
- [ ] Avoid compile errors by respecting borrowing rules
- [ ] Understand lifetime elision and when explicit lifetimes are needed

---

## 🔨 Phase 2 Projects — No Constraints, Just Real Work

---

### 📌 1. Text Indexer CLI

**🎯 Goal:** Create a CLI that processes and indexes lines of user input.

#### Your tool must:

- Accept multiple lines of input from the user
- Store all lines
- Print a clean report showing:

  - total number of lines
  - total character count (excluding whitespace)
  - the longest word
  - the full line that contains the longest word
  - a list of first and last words from each line

📘 _This naturally teaches_:
ownership of `String`, borrowing lines for analysis, returning references from helpers, slicing strings, and tracking lifetimes.

---

### 📌 2. Todo Manager CLI (Full CRUD + Analytics)

**🎯 Goal:** Build a task management app for the terminal.

#### Your app must:

- Add, list, edit, and delete tasks
- Support viewing all tasks
- Support editing by replacing a portion of a task
- Support searching tasks by keyword
- Highlight the keyword in search results
- Show the task with:

  - the most words
  - the fewest characters

- Generate previews for each task (first sentence or 10 words)
- Support sorting tasks alphabetically and by length

📘 _This will cover_:
shared vs mutable references, mutable borrowing of items in a `Vec`, slicing, lifetimes in return values, and writing functions with reference-based input/output.

---

### 📌 3. Paragraph Highlighter

**🎯 Goal:** Accept a paragraph and a keyword, and show keyword-highlighted results.

#### Your app must:

- Accept a paragraph of text from the user
- Accept a search keyword
- Display the paragraph with all occurrences of the keyword highlighted (e.g., `**word**`)
- Support previewing the sentence in which the keyword appears
- Let the user copy out only those matched lines

📘 _You'll use_:
slices of `String`, borrowed data for rendering, and return-by-reference helpers. Keyword matching will push you into handling string views and lifetimes naturally.

---

### 🛠️ Challenge Project 1: Contact Book with Lightweight Data

**🎯 Goal:** Build a lightweight contact manager.

#### Your contact manager must:

- Accept name and email input
- Store the data using one shared `String` buffer
- Store only references in the contact records
- Support:

  - listing all contacts
  - detecting duplicate emails
  - editing an existing contact’s name or email

📘 _This project will force_:
structs with lifetimes, reference storage, and borrowing across multiple layers (input -> storage -> display).

---

### 🛠️ Challenge Project 2: Journal Reader

**🎯 Goal:** Build a journal file reader and navigator.

#### Features:

- Load the entire journal as one big `String`
- Allow:

  - viewing a line by index
  - searching lines by a word
  - extracting and printing a range of lines
  - slicing a specific sentence for preview

📘 _This teaches_:
how to return slices of a large string, borrow lines safely from a single source, use helper functions with lifetime annotations, and prevent invalid access.

---

### 🛠️ Challenge Project 3: Smart Notes with Slicing

**🎯 Goal:** Create a notes system with preview functionality.

#### Features:

- Accept long-form notes
- Store all notes
- Allow:

  - viewing full notes
  - previewing only the first sentence or N words
  - editing a note in-place
  - printing the note with most characters

📘 _Concepts baked in_:
slicing `String` into `&str`, editing through `&mut`, previewing via borrowed views, and safely handling lifetime-bound returns.

---

### 🛠️ Challenge Project 4: CLI Library System

**🎯 Goal:** Simulate book borrowing and returning in a small library system.

#### You must:

- Store books (title, author) using `String`
- Track who borrowed which book using a struct with references like:

```rust
struct BorrowRecord<'a> {
    book_title: &'a str,
    borrower_name: &'a str,
}
```

- Prevent re-borrowing without returning
- Allow listing who borrowed what
- Show available books

📘 _This will make you_:
design structs with lifetimes, return references from collections, and handle borrowing logic in a real-world model with actual constraints.

---

## 🔍 Gap-Check Drill (added after Rust Book audit — ~45 min, playground file)

Phase 2 is done, but four lifetime/borrow details deserve explicit reps. One `gaps.rs`,
prediction comments before running, same rules as Phase 1's drill:

- [ ] **`&'static`** — why does `let s: &'static str = "hello";` work? Where does the
      data live? Write a function returning `&'static str` legitimately (no leaks).
- [ ] **`T: 'a` and `T: 'static` bounds on generics** — write
      `fn hold<T: 'static>(t: T)` and call it with: an owned `String` (works),
      a `&String` to a local (fails). Comment: `T: 'static` means "no borrows of
      shorter-lived data inside T" — NOT "lives forever". This bound reappears in
      Phases 10 & 11 constantly; nail the reading now.
- [ ] **Slices are fat pointers** — print `size_of::<&u8>()`, `size_of::<&[u8]>()`,
      `size_of::<&str>()`. Explain the extra 8 bytes. (Phase 4's `&dyn Trait` is the
      same trick with a vtable instead of a length.)
- [ ] **NLL / reborrowing** — write code that takes two `&mut` to the same value
      *sequentially* (compiles) vs *overlapping* (fails). Comment on when a borrow
      actually ends (last use, not end of scope).

Done when all four run and predictions are graded.

---

## ✅ By finishing these:

You won't just "know about" ownership and borrowing — you'll **live it**. Every task makes you feel the compiler's rules and rewards.

