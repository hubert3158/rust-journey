# 🦀 Rust Mastery: Phase 2 Completion Checklist

## ✅ Ownership, Borrowing & Lifetimes (Phase 2)

- [ ] Understand how values are moved and reused safely
- [ ] Learn when and how to clone or copy data
- [ ] Use shared and mutable references in useful ways
- [ ] Build functions that return borrowed data
- [ ] Design structs that hold references with lifetimes
- [ ] Prevent invalid access by respecting borrowing rules
- [ ] Grasp lifetime elision and when to be explicit

---

## 🔨 Suggested Mini Projects

### 📌 1. Text Formatter CLI

**🎯 Goal:** Create a CLI tool that formats text for a report.

1. Accept multi-line input from the user
2. Clean it: trim whitespace, lowercase all lines
3. Store and print formatted lines
4. Add a summary: total characters, lines, and first/last word of each line

**What you’ll learn while building:**

- Ownership of strings across lines
- Borrowing lines without cloning
- Handling `String` vs `&str` clearly
- Returning references from helper functions

---

### 📌 2. Editable Todo List

**🎯 Goal:** Build a terminal todo app where the user can add, list, edit, and delete tasks.

1. Tasks are stored in a `Vec<String>`
2. When editing a task, allow updating it without cloning everything
3. When listing tasks, pass them as borrowed references
4. On exit, show summary with longest/shortest task

**What you'll practice:**

- Borrowing for printing vs modifying
- Mutably updating elements inside collections
- Ownership transfer vs shared reference in edits

---

### 📌 3. Keyword Highlighter

**🎯 Goal:** Accept a paragraph and highlight a keyword.

1. Let the user input a paragraph and a keyword
2. Search the keyword without reallocating strings
3. Print each line with the keyword bolded (just wrap with `**...**`)

**Concepts reinforced:**

- Borrowing slices of strings
- Returning highlighted borrowed slices
- Avoiding unnecessary `clone`s or new allocations

---

## 🛠️ Challenge Project 1: Contact Book (Borrowed Data)

**🎯 Goal:** Build a CLI contact manager that keeps lightweight records using borrowed data.

### Features

- Accept name and email for each contact
- Store as `&str` references to a main buffer (for memory efficiency)
- Show the contact list any time
- Validate for duplicate emails

**Why it matters:**

- You'll learn struct lifetimes naturally
- You’ll see how to store borrowed references inside collections
- You’ll practice lifetimes for both struct and functions

---

## 🛠️ Challenge Project 2: Journal Viewer with Line Access

**🎯 Goal:** Build a CLI app that loads a journal file and lets the user view, search, or extract lines.

### Features

1. Load entire file into memory (`String`)
2. Let user:
   - View specific line by index
   - Search for lines containing a keyword
   - Extract a range of lines (by reference)

**This forces you to:**

- Borrow slices from the original buffer
- Respect borrowing rules when returning line refs
- Use lifetime annotations in helper functions

---

## 🛠️ Challenge Project 3: Smart Notes with Preview

**🎯 Goal:** Build a notes app that shows a preview of each note without copying its content.

### Features

1. Let user add full notes (`String`)
2. Preview shows only the first sentence (as `&str`)
3. View all notes or only previews
4. Optional: allow updating a note’s content

**You'll be dealing with:**

- Slicing strings and returning references
- Handling both mutable and immutable references
- Designing preview functions with lifetimes

---

## 🛠️ Challenge Project 4: CLI Library System (Books & Borrowers)

**🎯 Goal:** Simulate a small library where users can borrow books.

### Features

1. Each `Book` has a title and author (`String`)
2. Track who borrowed each book using a struct like:

```rust
struct BorrowRecord<'a> {
    book_title: &'a str,
    borrower_name: &'a str,
}
```

3. Show who borrowed what
4. Ensure books are returned before re-borrowed

**Teaches you:**

- Struct lifetimes
- Ownership of records vs borrowing for tracking
- Real-life borrow enforcement

---
