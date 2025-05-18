# 🦀 Rust Mastery: Phase 2 Completion Checklist

## ✅ Ownership, Borrowing & Lifetimes (Phase 2)

- [ ] Understand ownership and move semantics
- [ ] Know when values are moved vs copied (`Copy`, `Clone`)
- [ ] Use immutable and mutable references (`&T`, `&mut T`)
- [ ] Respect Rust’s borrowing rules (one mutable or many immutable)
- [ ] Understand and apply lifetimes (`'a`) to function arguments and return types
- [ ] Use structs that borrow data with lifetime annotations
- [ ] Avoid dangling references
- [ ] Understand lifetime elision rules in functions

---

## 🔨 Suggested Mini Projects

### 📌 1. Ownership Tracker CLI

**🎯 Goal:** Visually demonstrate ownership and move semantics.

1. Create a variable `String`, move it, and try accessing it again
2. Print when variables are moved or cloned
3. Optionally: simulate a “borrow checker” with logs

**Concepts practiced:** ownership, move, copy, clone

---

### 📌 2. Mutable vs Immutable Borrower

**🎯 Goal:** Write a program that takes both mutable and immutable references.

1. Define a `String`
2. Pass as `&str` to a function that prints it
3. Then pass as `&mut String` to modify it
4. Show what compiles and what doesn’t

**Concepts practiced:** `&T` vs `&mut T`, borrow rules

---

### 📌 3. String Slice Extractor

**🎯 Goal:** Create a CLI app that slices a string from `start` to `end`.

1. Input a string
2. Ask for start/end index
3. Return a borrowed `&str` slice
4. Try with both owned `String` and borrowed `&str`

**Concepts practiced:** borrowing, slices, lifetimes, elision

---

## 🛠️ Challenge Project 1: Contact Book with Struct Lifetimes

**🎯 Goal:** Create a CLI address book using borrowed data in structs.

### Features

- Define a `Contact<'a>` struct with name/email as `&'a str`
- Input contacts and store them in a `Vec<Contact>`
- Print the full list on exit
- Compare ergonomics to using `String` instead of `&str`

```rust
struct Contact<'a> {
    name: &'a str,
    email: &'a str,
}
```

**Concepts practiced:** struct lifetimes, borrowing, Vec storage, elision vs explicit

---

## 🛠️ Challenge Project 2: Note Taker with Lifetime Return

**🎯 Goal:** Create a simple CLI note-taking app that returns a borrowed note from a list.

### Features

1. Add notes (`&'a str`) into a `Vec`
2. Ask user for an index → return reference to note
3. Enforce valid lifetimes in the return value
4. Try refactoring to own `String` later

**Concepts practiced:** borrowing, function lifetimes, mutable borrow, return references

---

## 🛠️ Challenge Project 3: File Line Borrower

**🎯 Goal:** Build a program that reads a file into a buffer and borrows from it.

### Features

1. Read a file into a single `String` buffer
2. Borrow `&str` lines using `.lines()`
3. Search for a keyword without allocating new strings
4. Highlight matched lines

**Concepts practiced:** lifetimes, borrowing, file I/O, slices, zero-copy thinking

---

## 🛠️ Challenge Project 4: Borrowed Calculator

**🎯 Goal:** Write a calculator where user-defined operations are borrowed closures.

### Features

1. Create closures for `+`, `-`, `*`, `/` operations
2. Store them as borrowed function pointers or closures
3. Call them dynamically during runtime
4. Show borrowed behavior in action

**Concepts practiced:** borrowing, function traits (`Fn`, `FnMut`), references to functions

---
