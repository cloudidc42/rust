# 🦀 Rust Quick Reference Guide
# คู่มืออ้างอิงด่วนสำหรับ Rust

## 📚 สารบัญ
- [Variables & Data Types](#variables--data-types)
- [Functions](#functions)
- [Control Flow](#control-flow)
- [Ownership & Borrowing](#ownership--borrowing)
- [Structs & Enums](#structs--enums)
- [Collections](#collections)
- [Error Handling](#error-handling)
- [Common Patterns](#common-patterns)

---

## Variables & Data Types

### ตัวแปร (Variables)
```rust
let x = 5;              // immutable
let mut y = 10;         // mutable
const MAX: u32 = 100;   // constant
static NAME: &str = "Rust"; // static

// Shadowing
let x = x + 1;
let x = x * 2;
```

### Integer Types
```rust
i8, i16, i32, i64, i128, isize    // signed
u8, u16, u32, u64, u128, usize    // unsigned

let x: i32 = -100;
let y: u8 = 255;
```

### Float Types
```rust
f32, f64  // floating-point

let x: f32 = 3.14;
let y: f64 = 2.71828;  // default
```

### Boolean & Character
```rust
let t: bool = true;
let f: bool = false;

let c: char = 'A';
let emoji: char = '😀';
```

### Tuples
```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;  // destructuring
let first = tup.0;     // access by index
```

### Arrays
```rust
let arr = [1, 2, 3, 4, 5];
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let zeros = [0; 10];  // [0, 0, ..., 0]

let first = arr[0];
let len = arr.len();
```

### Strings
```rust
let s1: &str = "Hello";           // string slice
let s2: String = String::from("Hello"); // owned string
let s3 = "Hello".to_string();

// String operations
s2.push_str(", World!");
let len = s2.len();
let chars = s2.chars();
```

---

## Functions

### Basic Functions
```rust
fn function_name() {
    // code
}

fn with_params(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x + y);
}

fn with_return(x: i32, y: i32) -> i32 {
    x + y  // no semicolon = return
}

fn multiple_return() -> (i32, i32) {
    (1, 2)
}
```

### Closures
```rust
let add = |x, y| x + y;
let result = add(5, 3);

// With type annotations
let multiply = |x: i32, y: i32| -> i32 { x * y };
```

---

## Control Flow

### if/else
```rust
if condition {
    // code
} else if other_condition {
    // code
} else {
    // code
}

// if as expression
let number = if condition { 5 } else { 6 };
```

### match
```rust
match value {
    1 => println!("One"),
    2 | 3 => println!("Two or Three"),
    4..=10 => println!("Four to Ten"),
    _ => println!("Something else"),
}

// match with return
let result = match value {
    0 => "zero",
    1 => "one",
    _ => "many",
};
```

### loop
```rust
loop {
    // infinite loop
    if condition {
        break;
    }
}

// with return value
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;
    }
};
```

### while
```rust
while condition {
    // code
}
```

### for
```rust
for i in 0..10 {  // 0 to 9
    println!("{}", i);
}

for i in 0..=10 { // 0 to 10
    println!("{}", i);
}

for item in array {
    println!("{}", item);
}

for (i, item) in array.iter().enumerate() {
    println!("{}: {}", i, item);
}
```

---

## Ownership & Borrowing

### Ownership Rules
1. Each value has one owner
2. Only one owner at a time
3. Value dropped when owner goes out of scope

```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 moved to s2, s1 invalid

let s3 = s2.clone();  // deep copy
```

### References & Borrowing
```rust
// Immutable reference
let s = String::from("hello");
let len = calculate_length(&s);

fn calculate_length(s: &String) -> usize {
    s.len()
}

// Mutable reference
let mut s = String::from("hello");
change(&mut s);

fn change(s: &mut String) {
    s.push_str(", world");
}
```

### Slices
```rust
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];

let arr = [1, 2, 3, 4, 5];
let slice = &arr[1..3];  // [2, 3]
```

---

## Structs & Enums

### Structs
```rust
// Define struct
struct User {
    username: String,
    email: String,
    age: u32,
}

// Create instance
let user = User {
    username: String::from("alice"),
    email: String::from("alice@example.com"),
    age: 25,
};

// Access fields
println!("{}", user.username);

// Implement methods
impl User {
    fn new(username: String, email: String) -> User {
        User { username, email, age: 0 }
    }

    fn describe(&self) {
        println!("{}: {}", self.username, self.email);
    }
}
```

### Tuple Structs
```rust
struct Point(i32, i32);
let p = Point(10, 20);
println!("{}, {}", p.0, p.1);
```

### Enums
```rust
enum Direction {
    North,
    South,
    East,
    West,
}

let dir = Direction::North;

// Enums with data
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Pattern matching
match dir {
    Direction::North => println!("Going north"),
    Direction::South => println!("Going south"),
    _ => println!("Going somewhere"),
}
```

### Option
```rust
let some_number: Option<i32> = Some(5);
let no_number: Option<i32> = None;

match some_number {
    Some(n) => println!("Number: {}", n),
    None => println!("No number"),
}

// if let
if let Some(n) = some_number {
    println!("Number: {}", n);
}
```

---

## Collections

### Vec (Vector)
```rust
let mut v: Vec<i32> = Vec::new();
v.push(1);
v.push(2);
v.push(3);

let v = vec![1, 2, 3];  // macro

let third = &v[2];
let third = v.get(2);  // returns Option

for i in &v {
    println!("{}", i);
}
```

### String
```rust
let mut s = String::new();
let s = String::from("hello");
let s = "hello".to_string();

s.push_str(" world");
s.push('!');

let s3 = s1 + &s2;  // s1 moved
let s3 = format!("{} {}", s1, s2);  // doesn't take ownership
```

### HashMap
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Red"), 50);

let score = scores.get("Blue");  // returns Option<&V>

for (key, value) in &scores {
    println!("{}: {}", key, value);
}

// Update
scores.entry(String::from("Blue")).or_insert(0);
```

---

## Error Handling

### Option
```rust
fn divide(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        None
    } else {
        Some(x / y)
    }
}

let result = divide(10, 2);
match result {
    Some(n) => println!("Result: {}", n),
    None => println!("Cannot divide by zero"),
}
```

### Result
```rust
use std::fs::File;
use std::io::Error;

fn open_file(name: &str) -> Result<File, Error> {
    File::open(name)
}

match open_file("file.txt") {
    Ok(file) => println!("File opened"),
    Err(e) => println!("Error: {}", e),
}

// Shorthand with ?
fn read_file() -> Result<String, Error> {
    let content = std::fs::read_to_string("file.txt")?;
    Ok(content)
}
```

### panic!
```rust
panic!("Something went wrong!");

// Only for unrecoverable errors
if x < 0 {
    panic!("x must be positive");
}
```

---

## Common Patterns

### Iterators
```rust
let v = vec![1, 2, 3, 4, 5];

// map
let doubled: Vec<_> = v.iter().map(|x| x * 2).collect();

// filter
let evens: Vec<_> = v.iter().filter(|&x| x % 2 == 0).collect();

// fold
let sum: i32 = v.iter().fold(0, |acc, x| acc + x);

// chain
let v1 = vec![1, 2];
let v2 = vec![3, 4];
let combined: Vec<_> = v1.iter().chain(v2.iter()).collect();
```

### Pattern Matching
```rust
// Destructuring
let (x, y) = (1, 2);
let [first, second, ..] = [1, 2, 3, 4, 5];

// Match guards
match number {
    n if n < 0 => println!("Negative"),
    n if n > 0 => println!("Positive"),
    _ => println!("Zero"),
}
```

### Lifetimes
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

### Traits
```rust
trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
}
```

### Generics
```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

---

## คำสั่งที่ใช้บ่อย

```bash
# สร้างโปรเจคใหม่
cargo new project_name
cargo new --lib library_name

# Build & Run
cargo build
cargo run
cargo build --release

# Testing
cargo test
cargo test test_name

# Documentation
cargo doc --open

# Linting & Formatting
cargo clippy
cargo fmt

# Update dependencies
cargo update

# Check without building
cargo check
```

---

**Happy Coding! 🦀**
