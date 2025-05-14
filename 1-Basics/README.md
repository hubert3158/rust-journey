````markdown
# 🦀 Rust Mastery: Phase 1 Completion Checklist

## ✅ Essential Rust Concepts (Phase 1)

- [ ] Understand `let`, `mut`, and shadowing
- [ ] Learn the difference between `i32`, `f64`, `char`, `bool`, etc.
- [ ] Use arrays and tuples confidently
- [ ] Define and call functions with parameters and return values
- [ ] Control flow: `if`, `else`, `loop`, `while`, `for`
- [ ] Use `match` and `if let` for pattern matching
- [ ] Practice with `Vec`, `String`, and slices

---

## 🔨 Suggested Mini Projects

### 📌 1. Guessing Game

**🎯 Goal:** Create a CLI number-guessing game.

1. Generate a random number between 1–100
2. Ask the user to guess it
3. Respond with **“Too low”**, **“Too high”**, or **“Correct!”**
4. Repeat until correct

**Concepts practiced:** `rand`, `loop`, `if`, `io`, variable binding

---

### 📌 2. Basic Calculator CLI

**🎯 Goal:** Build a simple calculator that supports `+`, `-`, `*`, `/`.

1. Prompt for two numbers
2. Prompt for an operator (`+`, `-`, `*`, `/`)
3. Print the result using `match`
4. Loop until the user types `exit`

**Concepts practiced:** control flow, user input, `match`, functions

---

### 📌 3. Temperature Converter (C ↔ F)

**🎯 Goal:** Convert temperature between Celsius and Fahrenheit.

1. Ask for temperature value
2. Ask for direction (`C→F` or `F→C`)
3. Display the result
4. Loop until exit

**Formulae**

```text
F = C × 9/5 + 32
C = (F − 32) × 5/9
```
````

**Concepts practiced:** arithmetic, conditionals, user input, `match`

---

## 🛠️ Challenge Project 1: Student Report Generator

**🎯 Goal:** Build a CLI app that collects student data and generates a formatted grade report.

### Features

- Take student name and **three** subject scores as input
- Compute the average score with a function
- Assign letter grades

```text
A: 90–100
B: 80–89
C: 70–79
D: 60–69
F: <60
```

- Store each student as a tuple inside a `Vec`
- Loop to allow entering multiple students (type `exit` to finish)
- On exit, display the full report, e.g.:

```text
## Name      | Average | Grade
Subash       |  91.67  |  A
```

**Concepts practiced:** variables (`let`, `mut`, shadowing), scalars, tuples, vectors, functions, pattern matching, control flow, string & numeric input handling

---

## 🛠️ Challenge Project 2: CLI Bank Balance Tracker

**🎯 Goal:** Build a basic personal CLI finance tracker.

### Features

1. Ask user for starting balance
2. Loop:

   - Ask **Deposit** or **Withdraw**
   - Take amount
   - Apply operation
   - Show updated balance
   - Block withdrawals that cause negative balance

3. Type `exit` to finish
4. At the end, show a full transaction log

### Example

```text
Start balance: 1000
Action (deposit/withdraw/exit): deposit
Amount: 200
New Balance: 1200

Action: withdraw
Amount: 300
New Balance: 900
...
Final Log:
+200
-300
Final Balance: 900
```

**Concepts practiced:** mutable variables, `match`, control flow, vectors/tuples for logging, functions, error checks

---
