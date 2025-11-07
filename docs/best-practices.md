# 🏆 Rust Best Practices
# แนวทางการเขียน Rust ที่ดี

## 📋 สารบัญ
- [Code Organization](#code-organization)
- [Naming Conventions](#naming-conventions)
- [Error Handling](#error-handling)
- [Performance](#performance)
- [Safety](#safety)
- [Testing](#testing)
- [Documentation](#documentation)

---

## Code Organization

### โครงสร้างโปรเจค
```
my_project/
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── main.rs           # Binary entry point
│   ├── lib.rs            # Library root
│   ├── bin/              # Multiple binaries
│   └── modules/
│       ├── mod.rs
│       ├── module1.rs
│       └── module2.rs
├── tests/                # Integration tests
├── benches/              # Benchmarks
└── examples/             # Example code
```

### Modules
```rust
// ใช้ modules เพื่อจัดระเบียบโค้ด

// lib.rs
pub mod network;
pub mod database;

// network/mod.rs
pub mod client;
pub mod server;

// ใช้ในโค้ด
use my_project::network::client;
```

---

## Naming Conventions

### ตัวแปรและฟังก์ชัน (snake_case)
```rust
let my_variable = 42;
let user_count = 100;

fn calculate_total() {}
fn get_user_name() {}
```

### Types และ Traits (PascalCase)
```rust
struct UserAccount {}
enum MessageType {}
trait Drawable {}
```

### Constants (SCREAMING_SNAKE_CASE)
```rust
const MAX_CONNECTIONS: u32 = 100;
const DEFAULT_TIMEOUT: u64 = 30;
```

### Lifetimes (lowercase single letter)
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {}
```

---

## Error Handling

### ✅ ใช้ Result สำหรับ recoverable errors
```rust
fn divide(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(x / y)
    }
}

// ใช้ ? operator
fn process_file() -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string("file.txt")?;
    Ok(content)
}
```

### ✅ ใช้ Option สำหรับ optional values
```rust
fn find_user(id: u32) -> Option<User> {
    // Return Some(user) or None
}

// Pattern matching
match find_user(1) {
    Some(user) => println!("Found: {}", user.name),
    None => println!("User not found"),
}
```

### ❌ หลีกเลี่ยง panic! ในโค้ดจริง
```rust
// ❌ ไม่ดี
fn get_element(arr: &[i32], index: usize) -> i32 {
    arr[index]  // อาจ panic!
}

// ✅ ดีกว่า
fn get_element(arr: &[i32], index: usize) -> Option<i32> {
    arr.get(index).copied()
}
```

### ✅ สร้าง custom error types
```rust
#[derive(Debug)]
enum AppError {
    IoError(std::io::Error),
    ParseError(std::num::ParseIntError),
    CustomError(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::IoError(e) => write!(f, "IO error: {}", e),
            AppError::ParseError(e) => write!(f, "Parse error: {}", e),
            AppError::CustomError(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}
```

---

## Performance

### ✅ ใช้ references เมื่อไม่ต้องการ ownership
```rust
// ❌ ไม่ดี (move ownership)
fn print_string(s: String) {
    println!("{}", s);
}

// ✅ ดีกว่า (borrow)
fn print_string(s: &str) {
    println!("{}", s);
}
```

### ✅ ใช้ &str แทน &String
```rust
// ❌
fn process(s: &String) {}

// ✅
fn process(s: &str) {}
```

### ✅ ใช้ iterators แทน loops
```rust
// ❌ ช้ากว่า
let mut sum = 0;
for i in 0..vec.len() {
    sum += vec[i];
}

// ✅ เร็วกว่า
let sum: i32 = vec.iter().sum();
```

### ✅ ใช้ capacity hint สำหรับ Vec
```rust
// ❌
let mut vec = Vec::new();
for i in 0..1000 {
    vec.push(i);
}

// ✅ ดีกว่า (ไม่ต้อง reallocate)
let mut vec = Vec::with_capacity(1000);
for i in 0..1000 {
    vec.push(i);
}
```

### ✅ ใช้ String::clone() เฉพาะเมื่อจำเป็น
```rust
// ❌ clone ไม่จำเป็น
fn get_greeting(name: String) -> String {
    let greeting = name.clone();
    format!("Hello, {}!", greeting)
}

// ✅ ใช้ reference
fn get_greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

---

## Safety

### ✅ ใช้ Option และ Result แทน null/undefined
```rust
fn find_item(items: &[Item], id: u32) -> Option<&Item> {
    items.iter().find(|item| item.id == id)
}
```

### ✅ ใช้ pattern matching อย่างครบถ้วน
```rust
// ✅ ดี - ครอบคลุมทุกกรณี
match value {
    Some(x) => println!("{}", x),
    None => println!("No value"),
}

// ❌ หลีกเลี่ยง - อาจมีกรณีที่ไม่ได้จัดการ
if let Some(x) = value {
    println!("{}", x);
}
// ลืม handle None case
```

### ✅ ใช้ newtype pattern สำหรับ type safety
```rust
// ❌ ใช้ i32 โดยตรง อาจสับสน
fn calculate(user_id: i32, product_id: i32) {}

// ✅ ใช้ newtype
struct UserId(i32);
struct ProductId(i32);

fn calculate(user_id: UserId, product_id: ProductId) {}
```

### ✅ ระวัง integer overflow
```rust
// ❌ อาจ overflow
let x: u8 = 255;
let y = x + 1;  // panic in debug, wrap in release

// ✅ Handle overflow
let x: u8 = 255;
let y = x.checked_add(1);  // returns Option
match y {
    Some(val) => println!("{}", val),
    None => println!("Overflow!"),
}
```

---

## Testing

### ✅ เขียน unit tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_division_by_zero() {
        assert!(divide(10, 0).is_err());
    }

    #[test]
    #[should_panic(expected = "divide by zero")]
    fn test_panic() {
        divide_panic(10, 0);
    }
}
```

### ✅ ใช้ doc tests
```rust
/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// let result = my_crate::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}
```

### ✅ เขียน integration tests
```rust
// tests/integration_test.rs
use my_crate;

#[test]
fn test_full_workflow() {
    let result = my_crate::process_data("input");
    assert_eq!(result, expected_output);
}
```

---

## Documentation

### ✅ เขียน documentation comments
```rust
/// สร้าง User ใหม่
///
/// # Arguments
///
/// * `name` - ชื่อผู้ใช้
/// * `age` - อายุ
///
/// # Examples
///
/// ```
/// let user = User::new("Alice", 25);
/// assert_eq!(user.name, "Alice");
/// ```
///
/// # Panics
///
/// Panics if age is negative (won't happen with u32)
///
/// # Errors
///
/// Returns error if name is empty
pub fn create_user(name: String, age: u32) -> Result<User, Error> {
    // implementation
}
```

### ✅ ใช้ markdown ใน comments
```rust
/// # Main Features
///
/// - Feature 1
/// - Feature 2
///
/// ## Performance
///
/// This function runs in O(n) time.
///
/// ## See Also
///
/// * [`other_function`]
/// * [`AnotherStruct`]
```

---

## Code Style

### ✅ ใช้ rustfmt
```bash
cargo fmt
```

### ✅ ใช้ clippy สำหรับ linting
```bash
cargo clippy
```

### ✅ เขียนโค้ดที่อ่านง่าย
```rust
// ❌ ยากอ่าน
let r = if c { v1 } else { v2 };

// ✅ อ่านง่าย
let result = if is_valid {
    valid_value
} else {
    default_value
};
```

### ✅ แยกฟังก์ชันยาวเกินไป
```rust
// ❌ ฟังก์ชันยาวเกินไป
fn process() {
    // 100 lines of code...
}

// ✅ แยกเป็นฟังก์ชันย่อย
fn process() {
    let data = load_data();
    let validated = validate_data(data);
    let processed = transform_data(validated);
    save_data(processed);
}
```

---

## Ownership Best Practices

### ✅ คิดให้ดีว่าใครควรเป็นเจ้าของ
```rust
// Option 1: Transfer ownership
fn consume(data: Vec<i32>) {
    // data is moved here
}

// Option 2: Borrow immutably
fn read(data: &Vec<i32>) {
    // just read data
}

// Option 3: Borrow mutably
fn modify(data: &mut Vec<i32>) {
    // can modify data
}

// Option 4: Clone (มีค่าใช้จ่าย)
fn keep_original(data: Vec<i32>) {
    let data_copy = data.clone();
    // use data_copy
    // original data still valid
}
```

### ✅ ใช้ Rc/Arc เมื่อจำเป็น
```rust
use std::rc::Rc;

// เมื่อต้องการ multiple owners (single-threaded)
let data = Rc::new(vec![1, 2, 3]);
let data1 = Rc::clone(&data);
let data2 = Rc::clone(&data);

// สำหรับ multi-threaded
use std::sync::Arc;
let shared = Arc::new(vec![1, 2, 3]);
```

---

## Common Mistakes to Avoid

### ❌ String indexing
```rust
// ❌ ไม่ได้ใน Rust
let s = String::from("hello");
let h = s[0];  // Error!

// ✅ ใช้วิธีนี้
let h = s.chars().next();
let h = &s[0..1];  // slice (ระวัง UTF-8)
```

### ❌ Borrowing ผิด
```rust
// ❌
let mut vec = vec![1, 2, 3];
let first = &vec[0];
vec.push(4);  // Error! can't borrow as mutable
println!("{}", first);

// ✅
let mut vec = vec![1, 2, 3];
{
    let first = vec[0];
    println!("{}", first);
}
vec.push(4);  // OK
```

### ❌ ใช้ unwrap() โดยไม่คิด
```rust
// ❌ อาจ panic
let file = File::open("file.txt").unwrap();

// ✅ Handle error
let file = match File::open("file.txt") {
    Ok(f) => f,
    Err(e) => {
        eprintln!("Error: {}", e);
        return;
    }
};

// หรือใช้ ?
let file = File::open("file.txt")?;
```

---

## Summary

✅ **DO:**
- ใช้ Result และ Option สำหรับ error handling
- เขียน documentation และ tests
- ใช้ rustfmt และ clippy
- คิดเรื่อง ownership ให้ดี
- ใช้ iterators แทน loops เมื่อเหมาะสม
- เขียนโค้ดที่อ่านง่าย

❌ **DON'T:**
- ใช้ panic! ในโค้ดจริง
- Clone ทุกอย่างโดยไม่จำเป็น
- เขียนฟังก์ชันยาวเกินไป
- ใช้ unwrap() โดยไม่คิด
- Ignore compiler warnings

---

**Happy Coding! 🦀**
