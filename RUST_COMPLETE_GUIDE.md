# 🦀 Rust Complete Learning Guide - 1000 Steps
# คู่มือเรียน Rust อย่างครอบคลุม 1000 ขั้นตอน

---

## 📋 สารบัญ

- [ส่วนที่ 1: ระดับพื้นฐาน (Steps 1-250)](#section-1-basic)
- [ส่วนที่ 2: ระดับกลาง (Steps 251-500)](#section-2-intermediate)
- [ส่วนที่ 3: ระดับสูง (Steps 501-750)](#section-3-advanced)
- [ส่วนที่ 4: ระดับมืออาชีพ (Steps 751-1000)](#section-4-professional)

---

# <a id="section-1-basic"></a>📘 ส่วนที่ 1: ระดับพื้นฐาน (Steps 1-250)

## 🎯 Chapter 1: เริ่มต้นกับ Rust (Steps 1-50)

### Step 1: ทำความรู้จักกับ Rust
**ทฤษฎี:**
- Rust คือภาษาโปรแกรมที่เน้นความปลอดภัย (Safety), ความเร็ว (Speed), และการทำงานพร้อมกัน (Concurrency)
- พัฒนาโดย Mozilla Research และเปิดตัวครั้งแรกในปี 2010
- ไม่มี Garbage Collector แต่ใช้ระบบ Ownership

**จุดเด่นของ Rust:**
- Memory Safety โดยไม่ต้องใช้ Garbage Collector
- Zero-cost abstractions
- Thread Safety ที่ตรวจสอบตอน Compile time
- Performance เทียบเท่า C/C++

---

### Step 2: ติดตั้ง Rust

**Windows:**
```bash
# ดาวน์โหลด rustup-init.exe จาก https://rustup.rs/
# หรือใช้ scoop
scoop install rustup
```

**macOS/Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**ตรวจสอบการติดตั้ง:**
```bash
rustc --version  # compiler version
cargo --version  # package manager version
rustup --version # toolchain manager version
```

---

### Step 3: เครื่องมือที่สำคัญ

**rustc** - Rust Compiler
```bash
# คอมไพล์ไฟล์เดี่ยว
rustc main.rs
```

**cargo** - Package Manager และ Build System
```bash
cargo new my_project    # สร้างโปรเจคใหม่
cargo build            # build โปรเจค
cargo run              # build และรันโปรเจค
cargo test             # รัน tests
cargo check            # ตรวจสอบโค้ดโดยไม่ build
cargo clean            # ลบไฟล์ที่ build
```

**rustup** - Toolchain Manager
```bash
rustup update          # อัพเดท Rust
rustup component add clippy  # ติดตั้ง linter
rustup component add rustfmt # ติดตั้ง formatter
```

---

### Step 4: สร้างโปรเจคแรก

```bash
# สร้างโปรเจคใหม่
cargo new hello_rust
cd hello_rust

# โครงสร้างไฟล์ที่สร้าง:
# hello_rust/
# ├── Cargo.toml       # ไฟล์ config
# └── src/
#     └── main.rs      # ไฟล์โค้ดหลัก
```

**ไฟล์ Cargo.toml:**
```toml
[package]
name = "hello_rust"
version = "0.1.0"
edition = "2021"

[dependencies]
# เพิ่ม dependencies ที่นี่
```

---

### Step 5: Hello, World!

**ไฟล์: src/main.rs**
```rust
// นี่คือ comment บรรทัดเดียว
/* นี่คือ comment
   หลายบรรทัด */

// ฟังก์ชัน main คือจุดเริ่มต้นของโปรแกรม
fn main() {
    // println! คือ macro สำหรับพิมพ์ข้อความ (สังเกตเครื่องหมาย !)
    println!("Hello, World!");
    println!("สวัสดี Rust! 🦀");
}
```

**รันโปรแกรม:**
```bash
cargo run
# Output:
# Hello, World!
# สวัสดี Rust! 🦀
```

---

### Step 6: Comments และ Documentation

```rust
// Comment บรรทัดเดียว

/* Comment หลายบรรทัด
   สามารถเขียนได้หลายบรรทัด
   ใช้สำหรับอธิบายโค้ดที่ซับซ้อน */

/// Documentation comment สำหรับฟังก์ชันด้านล่าง
/// สามารถใช้ markdown ได้
///
/// # Examples
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}

//! Documentation comment สำหรับ module หรือ crate
//! ใช้ที่ต้นไฟล์

fn main() {
    // inline comment ในบรรทัดเดียวกัน
    let x = 5; // ประกาศตัวแปร x
}
```

---

### Step 7: ตัวแปร (Variables) พื้นฐาน

```rust
fn main() {
    // ตัวแปรใน Rust เป็น immutable (ไม่สามารถเปลี่ยนค่าได้) โดยปกติ
    let x = 5;
    println!("ค่าของ x คือ: {}", x);

    // x = 6; // ⚠️ Error! ไม่สามารถเปลี่ยนค่าได้

    // ใช้ mut เพื่อทำให้เปลี่ยนค่าได้
    let mut y = 10;
    println!("ค่าของ y คือ: {}", y);
    y = 15; // ✅ OK
    println!("ค่าใหม่ของ y คือ: {}", y);

    // การระบุชนิดข้อมูล (type annotation)
    let z: i32 = 20;
    println!("ค่าของ z คือ: {}", z);

    // การประกาศหลายตัวแปรพร้อมกัน
    let (a, b, c) = (1, 2, 3);
    println!("a={}, b={}, c={}", a, b, c);
}
```

**Output:**
```
ค่าของ x คือ: 5
ค่าของ y คือ: 10
ค่าใหม่ของ y คือ: 15
ค่าของ z คือ: 20
a=1, b=2, c=3
```

---

### Step 8: Shadowing

```rust
fn main() {
    // Shadowing คือการประกาศตัวแปรชื่อเดิมใหม่
    let x = 5;
    println!("x = {}", x);

    // Shadow ด้วยค่าใหม่
    let x = x + 1; // x ใหม่ = x เก่า + 1
    println!("x = {}", x); // 6

    // Shadow ด้วยชนิดข้อมูลใหม่ (ต่างจาก mut)
    let x = "text"; // เปลี่ยนจาก i32 เป็น &str
    println!("x = {}", x);

    // Shadowing ใน scope
    {
        let x = x.len(); // shadow ใน scope นี้
        println!("ความยาวของ x = {}", x); // 4
    }

    println!("x = {}", x); // กลับเป็น "text"

    // ตัวอย่างการใช้ shadowing กับ type ต่างกัน
    let spaces = "   ";        // &str
    let spaces = spaces.len(); // usize
    println!("จำนวน spaces: {}", spaces);
}
```

---

### Step 9: Constants และ Static

```rust
// Constant: ค่าคงที่ที่ต้องระบุ type และไม่สามารถเปลี่ยนค่าได้
const MAX_POINTS: u32 = 100_000; // ใช้ _ เพื่อแยกหลัก
const PI: f64 = 3.14159265359;

// Static: คล้าย constant แต่มี memory address คงที่
static LANGUAGE: &str = "Rust";
static mut COUNTER: u32 = 0; // ⚠️ unsafe เมื่อเปลี่ยนค่า

fn main() {
    println!("MAX_POINTS = {}", MAX_POINTS);
    println!("PI = {}", PI);
    println!("LANGUAGE = {}", LANGUAGE);

    // const ต่างจาก let:
    // 1. ต้องระบุ type เสมอ
    // 2. ไม่สามารถใช้ mut ได้
    // 3. สามารถประกาศใน global scope
    // 4. ค่าต้องเป็นค่าคงที่ (compile-time constant)

    const SECONDS_IN_HOUR: u32 = 60 * 60;
    println!("วินาทีใน 1 ชั่วโมง: {}", SECONDS_IN_HOUR);
}
```

---

### Step 10: ชนิดข้อมูลพื้นฐาน - Integer

```rust
fn main() {
    // Integer Types (จำนวนเต็ม)

    // Signed integers (มีเครื่องหมาย +/-)
    let a: i8 = -128;           // -128 to 127
    let b: i16 = -32_768;       // -32,768 to 32,767
    let c: i32 = -2_147_483_648; // default type สำหรับ integer
    let d: i64 = -9_223_372_036_854_775_808;
    let e: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;
    let f: isize = -100;        // ขนาดขึ้นกับ architecture (32 หรือ 64 bit)

    println!("Signed: {}, {}, {}, {}, {}, {}", a, b, c, d, e, f);

    // Unsigned integers (ไม่มีเครื่องหมาย, เฉพาะบวก)
    let g: u8 = 255;            // 0 to 255
    let h: u16 = 65_535;        // 0 to 65,535
    let i: u32 = 4_294_967_295;
    let j: u64 = 18_446_744_073_709_551_615;
    let k: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;
    let l: usize = 100;         // ขนาดขึ้นกับ architecture

    println!("Unsigned: {}, {}, {}, {}, {}, {}", g, h, i, j, k, l);

    // Integer Literals (การเขียนเลขในรูปแบบต่างๆ)
    let decimal = 98_222;       // ฐาน 10
    let hex = 0xff;             // ฐาน 16 (Hexadecimal)
    let octal = 0o77;           // ฐาน 8 (Octal)
    let binary = 0b1111_0000;   // ฐาน 2 (Binary)
    let byte = b'A';            // u8 เท่านั้น

    println!("Literals: dec={}, hex={}, oct={}, bin={}, byte={}",
             decimal, hex, octal, binary, byte);

    // การระบุ type suffix
    let x = 57u8;     // u8
    let y = 1000i32;  // i32
    let z = 9999999999i64; // i64

    println!("Type suffix: {}, {}, {}", x, y, z);
}
```

---

### Step 11: ชนิดข้อมูลพื้นฐาน - Floating Point

```rust
fn main() {
    // Floating-point types (จำนวนทศนิยม)

    // f64 เป็น default type (64-bit, double precision)
    let x = 2.0;        // f64
    let y: f32 = 3.0;   // f32 (32-bit, single precision)

    println!("x = {}, y = {}", x, y);

    // ตัวอย่างการคำนวณ
    let sum = x + 5.5;
    let difference = 95.5 - 4.3;
    let product = 4.0 * 30.0;
    let quotient = 56.7 / 32.2;
    let remainder = 43.0 % 5.0; // modulo

    println!("sum = {}", sum);
    println!("difference = {}", difference);
    println!("product = {}", product);
    println!("quotient = {}", quotient);
    println!("remainder = {}", remainder);

    // Scientific notation (วิทยาศาสตร์)
    let large = 1.23e6;    // 1.23 × 10^6 = 1,230,000
    let small = 1.23e-6;   // 1.23 × 10^-6 = 0.00000123

    println!("large = {}", large);
    println!("small = {}", small);

    // พิเศษค่า (Special values)
    let infinity = f64::INFINITY;
    let neg_infinity = f64::NEG_INFINITY;
    let nan = f64::NAN; // Not a Number

    println!("infinity = {}", infinity);
    println!("neg_infinity = {}", neg_infinity);
    println!("nan = {}", nan);

    // ตรวจสอบค่าพิเศษ
    println!("nan is nan? {}", nan.is_nan());
    println!("infinity is infinite? {}", infinity.is_infinite());
}
```

---

### Step 12: ชนิดข้อมูลพื้นฐาน - Boolean

```rust
fn main() {
    // Boolean type มีค่า true หรือ false เท่านั้น
    let t = true;
    let f: bool = false; // with explicit type annotation

    println!("t = {}, f = {}", t, f);

    // การใช้ boolean ในการเปรียบเทียบ
    let is_greater = 5 > 3;
    let is_equal = 10 == 10;
    let is_not_equal = 10 != 5;

    println!("5 > 3: {}", is_greater);
    println!("10 == 10: {}", is_equal);
    println!("10 != 5: {}", is_not_equal);

    // Logical operators (ตัวดำเนินการทางตรรกะ)
    let and_result = true && false; // AND
    let or_result = true || false;  // OR
    let not_result = !true;          // NOT

    println!("true && false = {}", and_result);
    println!("true || false = {}", or_result);
    println!("!true = {}", not_result);

    // Boolean มีขนาด 1 byte
    println!("Boolean size: {} byte", std::mem::size_of::<bool>());

    // การแปลง boolean
    let num = is_greater as u8; // true = 1, false = 0
    println!("true as u8 = {}", num);
}
```

---

### Step 13: ชนิดข้อมูลพื้นฐาน - Character

```rust
fn main() {
    // Character type ใช้ single quotes ''
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart = '💖';
    let thai = 'ก';

    println!("c = {}", c);
    println!("z = {}", z);
    println!("heart = {}", heart);
    println!("thai = {}", thai);

    // char ใน Rust เป็น Unicode Scalar Value (4 bytes)
    // รองรับตัวอักษรทุกภาษา, emoji, และสัญลักษณ์พิเศษ
    println!("char size: {} bytes", std::mem::size_of::<char>());

    // ตัวอย่างตัวอักษรต่างๆ
    let emoji = '😀';
    let japanese = 'あ';
    let chinese = '中';
    let arabic = 'ب';
    let greek = 'Ω';

    println!("Various characters:");
    println!("Emoji: {}", emoji);
    println!("Japanese: {}", japanese);
    println!("Chinese: {}", chinese);
    println!("Arabic: {}", arabic);
    println!("Greek: {}", greek);

    // Escape sequences
    let newline = '\n';      // ขึ้นบรรทัดใหม่
    let tab = '\t';          // tab
    let backslash = '\\';    // backslash
    let single_quote = '\''; // single quote

    println!("Tab{}separated", tab);
    println!("Backslash: {}", backslash);
    println!("Quote: {}", single_quote);
}
```

---

### Step 14: Numeric Operations (การคำนวณ)

```rust
fn main() {
    // ==================== Integer Operations ====================

    // การบวก (Addition)
    let sum = 5 + 10;
    println!("5 + 10 = {}", sum);

    // การลบ (Subtraction)
    let difference = 95 - 4;
    println!("95 - 4 = {}", difference);

    // การคูณ (Multiplication)
    let product = 4 * 30;
    println!("4 * 30 = {}", product);

    // การหาร (Division)
    let quotient = 56 / 32; // integer division (ได้ 1)
    let quotient_f = 56.7 / 32.2; // float division
    println!("56 / 32 = {} (integer division)", quotient);
    println!("56.7 / 32.2 = {}", quotient_f);

    // การหารเอาเศษ (Modulo/Remainder)
    let remainder = 43 % 5;
    println!("43 % 5 = {}", remainder);

    // ==================== Compound Assignment ====================
    let mut x = 10;
    x += 5;  // x = x + 5
    println!("After x += 5: {}", x);

    x -= 3;  // x = x - 3
    println!("After x -= 3: {}", x);

    x *= 2;  // x = x * 2
    println!("After x *= 2: {}", x);

    x /= 4;  // x = x / 4
    println!("After x /= 4: {}", x);

    x %= 3;  // x = x % 3
    println!("After x %= 3: {}", x);

    // ==================== Bitwise Operations ====================
    let a = 0b1100; // 12 in binary
    let b = 0b1010; // 10 in binary

    println!("a = {}, b = {}", a, b);
    println!("a & b = {} (AND)", a & b);        // 0b1000 = 8
    println!("a | b = {} (OR)", a | b);         // 0b1110 = 14
    println!("a ^ b = {} (XOR)", a ^ b);        // 0b0110 = 6
    println!("!a = {} (NOT)", !a);              // NOT
    println!("a << 1 = {} (Left Shift)", a << 1);  // 0b11000 = 24
    println!("a >> 1 = {} (Right Shift)", a >> 1); // 0b0110 = 6

    // ==================== Overflow Handling ====================
    // Debug mode: panic on overflow
    // Release mode: wrapping behavior

    let mut num: u8 = 255;
    // num = num + 1; // จะ panic ใน debug mode

    // ใช้ wrapping methods เพื่อจัดการ overflow
    num = num.wrapping_add(1); // กลับไปเป็น 0
    println!("255.wrapping_add(1) = {}", num);

    // Checked operations (คืนค่า Option)
    let result = 200u8.checked_add(100);
    match result {
        Some(val) => println!("Result: {}", val),
        None => println!("Overflow occurred!"),
    }

    // Saturating operations (หยุดที่ค่า min/max)
    let sat = 250u8.saturating_add(10); // หยุดที่ 255
    println!("250.saturating_add(10) = {}", sat);

    // Overflowing operations (คืนค่าและ bool)
    let (res, overflowed) = 250u8.overflowing_add(10);
    println!("250 + 10 = {}, overflowed: {}", res, overflowed);
}
```

---

### Step 15: Type Casting (การแปลงชนิดข้อมูล)

```rust
fn main() {
    // ==================== Basic Casting ====================

    // Integer to Integer
    let x: i32 = 100;
    let y: i64 = x as i64;  // i32 -> i64 (widening, safe)
    let z: i16 = x as i16;  // i32 -> i16 (narrowing, may lose data)

    println!("x (i32) = {}", x);
    println!("y (i64) = {}", y);
    println!("z (i16) = {}", z);

    // Integer to Float
    let a: i32 = 5;
    let b: f64 = a as f64;
    println!("5 as f64 = {}", b);

    // Float to Integer (ทศนิยมจะถูกตัดทิ้ง)
    let pi: f64 = 3.14159;
    let int_pi: i32 = pi as i32; // ได้ 3
    println!("3.14159 as i32 = {}", int_pi);

    // ==================== Unsafe Casts ====================

    // Unsigned to Signed
    let u: u8 = 255;
    let s: i8 = u as i8; // 255 -> -1 (overflow)
    println!("255u8 as i8 = {}", s);

    // Large to Small (data loss)
    let large: i32 = 300;
    let small: i8 = large as i8; // 300 -> 44 (300 % 256 = 44)
    println!("300i32 as i8 = {}", small);

    // ==================== Boolean Casting ====================
    let is_true = true;
    let num = is_true as u8; // true -> 1, false -> 0
    println!("true as u8 = {}", num);

    // ⚠️ ไม่สามารถแปลง u8 เป็น bool โดยตรง
    // let not_bool: bool = 1 as bool; // Error!
    // ต้องใช้ comparison
    let bool_val = num != 0;
    println!("1 != 0 = {}", bool_val);

    // ==================== Char Casting ====================
    let character = 'A';
    let ascii = character as u8; // char -> u8 (ASCII)
    println!("'A' as u8 = {}", ascii); // 65

    let num_char = 65u8 as char; // u8 -> char
    println!("65u8 as char = '{}'", num_char); // 'A'

    // ==================== Reference Casting ====================
    let x = 10;
    let ptr = &x as *const i32; // reference -> raw pointer
    println!("Pointer address: {:p}", ptr);

    // ==================== Safe Conversions ====================

    // Using From/Into traits (preferred method)
    let x: i32 = 100;
    let y: i64 = i64::from(x); // explicit
    let z: i64 = x.into();     // implicit (type inference required)

    println!("Using From/Into: {}, {}", y, z);

    // TryFrom/TryInto (for fallible conversions)
    use std::convert::TryFrom;

    let big: i32 = 300;
    match i8::try_from(big) {
        Ok(small) => println!("Converted: {}", small),
        Err(e) => println!("Conversion error: {}", e),
    }
}
```

---

### Step 16: Tuples (ทูเพิล)

```rust
fn main() {
    // ==================== Basic Tuples ====================

    // Tuple คือการรวมค่าหลายค่าที่อาจมี type ต่างกันไว้ด้วยกัน
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("Tuple: {:?}", tup);

    // Destructuring (แยกค่าออกมา)
    let (x, y, z) = tup;
    println!("x = {}, y = {}, z = {}", x, y, z);

    // เข้าถึงค่าโดยใช้ index (เริ่มจาก 0)
    let first = tup.0;
    let second = tup.1;
    let third = tup.2;
    println!("tup.0 = {}, tup.1 = {}, tup.2 = {}", first, second, third);

    // ==================== Tuple Types ====================

    // Empty tuple (unit type)
    let unit: () = ();
    println!("Unit: {:?}", unit);

    // Single element tuple (ต้องมี comma)
    let single = (5,);
    println!("Single tuple: {:?}", single);

    // ไม่มี comma = ไม่ใช่ tuple (เป็นแค่ expression ที่มีวงเล็บ)
    let not_tuple = (5); // เป็น i32 ไม่ใช่ tuple
    println!("Not tuple: {}", not_tuple);

    // ==================== Mutable Tuples ====================
    let mut mutable_tup = (1, 2, 3);
    println!("Before: {:?}", mutable_tup);

    mutable_tup.0 = 10; // เปลี่ยนค่า element แรก
    mutable_tup.1 = 20;
    mutable_tup.2 = 30;
    println!("After: {:?}", mutable_tup);

    // ==================== Nested Tuples ====================
    let nested = ((1, 2), (3, 4), (5, 6));
    println!("Nested tuple: {:?}", nested);
    println!("nested.0 = {:?}", nested.0);
    println!("nested.0.1 = {}", nested.0.1);

    // ==================== Function Return Tuples ====================
    fn calculate(x: i32, y: i32) -> (i32, i32, i32, i32) {
        (x + y, x - y, x * y, x / y)
    }

    let (sum, diff, prod, quot) = calculate(20, 5);
    println!("20 + 5 = {}", sum);
    println!("20 - 5 = {}", diff);
    println!("20 * 5 = {}", prod);
    println!("20 / 5 = {}", quot);

    // ==================== Tuple as Function Parameter ====================
    fn print_coordinates((x, y): (i32, i32)) {
        println!("Coordinates: ({}, {})", x, y);
    }

    let point = (10, 20);
    print_coordinates(point);

    // ==================== Pattern Matching ====================
    let triple = (0, -2, 3);

    match triple {
        (0, y, z) => println!("First is 0, others are {} and {}", y, z),
        (x, 0, z) => println!("Second is 0, others are {} and {}", x, z),
        _ => println!("No zeros"),
    }

    // ==================== Tuple Size ====================
    let tup = (1, 2, 3, 4, 5);
    println!("Tuple size: {} bytes", std::mem::size_of_val(&tup));
}
```

---

### Step 17: Arrays (อาร์เรย์)

```rust
fn main() {
    // ==================== Basic Arrays ====================

    // Array มีขนาดคงที่และเก็บข้อมูล type เดียวกัน
    let numbers = [1, 2, 3, 4, 5];
    println!("Array: {:?}", numbers);

    // Array with type annotation
    let integers: [i32; 5] = [1, 2, 3, 4, 5];
    //             ^type ^size
    println!("Integers: {:?}", integers);

    // Initialize with same value
    let zeros = [0; 10]; // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    println!("Zeros: {:?}", zeros);

    // ==================== Accessing Elements ====================
    let arr = [10, 20, 30, 40, 50];

    let first = arr[0];
    let second = arr[1];
    println!("First: {}, Second: {}", first, second);

    // ⚠️ Invalid index causes panic!
    // let invalid = arr[10]; // Runtime panic!

    // Safe access using get()
    match arr.get(10) {
        Some(val) => println!("Value: {}", val),
        None => println!("Index out of bounds!"),
    }

    // ==================== Mutable Arrays ====================
    let mut mut_arr = [1, 2, 3, 4, 5];
    println!("Before: {:?}", mut_arr);

    mut_arr[0] = 10;
    mut_arr[2] = 30;
    println!("After: {:?}", mut_arr);

    // ==================== Array Properties ====================
    let arr = [1, 2, 3, 4, 5];

    // Length
    println!("Length: {}", arr.len());

    // Check if empty
    println!("Is empty: {}", arr.is_empty());

    // First and last
    println!("First: {:?}", arr.first());
    println!("Last: {:?}", arr.last());

    // ==================== Slices ====================
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Slice เป็นการ "view" ส่วนหนึ่งของ array
    let slice = &arr[2..5]; // elements at index 2, 3, 4
    println!("Slice [2..5]: {:?}", slice);

    let slice_from = &arr[5..]; // from index 5 to end
    println!("Slice [5..]: {:?}", slice_from);

    let slice_to = &arr[..3]; // from start to index 2
    println!("Slice [..3]: {:?}", slice_to);

    let slice_all = &arr[..]; // entire array
    println!("Slice [..]: {:?}", slice_all);

    // ==================== Iterating Arrays ====================
    let arr = [10, 20, 30, 40, 50];

    // Using for loop
    print!("For loop: ");
    for element in arr {
        print!("{} ", element);
    }
    println!();

    // With index
    print!("With index: ");
    for (i, element) in arr.iter().enumerate() {
        print!("[{}]={} ", i, element);
    }
    println!();

    // Using iterator methods
    let sum: i32 = arr.iter().sum();
    println!("Sum: {}", sum);

    let max = arr.iter().max();
    println!("Max: {:?}", max);

    // ==================== Multidimensional Arrays ====================
    let matrix: [[i32; 3]; 3] = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];

    println!("Matrix:");
    for row in matrix {
        println!("{:?}", row);
    }

    println!("Element [1][2]: {}", matrix[1][2]); // 6

    // ==================== Array Methods ====================
    let mut arr = [3, 1, 4, 1, 5, 9, 2, 6];

    // Sort
    arr.sort();
    println!("Sorted: {:?}", arr);

    // Reverse
    arr.reverse();
    println!("Reversed: {:?}", arr);

    // Contains
    println!("Contains 5: {}", arr.contains(&5));

    // Binary search (array ต้อง sorted)
    arr.sort();
    match arr.binary_search(&5) {
        Ok(index) => println!("Found 5 at index {}", index),
        Err(_) => println!("5 not found"),
    }
}
```

---

### Step 18: Functions (ฟังก์ชัน) พื้นฐาน

```rust
// ==================== Basic Functions ====================

// ฟังก์ชันที่ไม่มี parameter และไม่ return ค่า
fn say_hello() {
    println!("Hello, Rust!");
}

// ฟังก์ชันที่มี parameters
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// ฟังก์ชันที่มีหลาย parameters
fn add(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x + y);
}

// ==================== Return Values ====================

// ฟังก์ชันที่ return ค่า (ใช้ -> เพื่อระบุ return type)
fn add_return(x: i32, y: i32) -> i32 {
    x + y // ไม่มี semicolon = expression = return value
}

// Return แบบใช้ return keyword (early return)
fn divide(x: i32, y: i32) -> i32 {
    if y == 0 {
        return 0; // early return
    }
    x / y
}

// Return tuple
fn calculate(x: i32, y: i32) -> (i32, i32, i32, i32) {
    (x + y, x - y, x * y, x / y)
}

// ==================== Statements vs Expressions ====================

fn statement_vs_expression() -> i32 {
    let x = 5; // statement (ไม่ return ค่า)

    // Block เป็น expression
    let y = {
        let z = 10;
        z + x // ไม่มี semicolon = return value ของ block
    };

    y // return value ของฟังก์ชัน
}

// ==================== Unit Type ====================

// ฟังก์ชันที่ไม่ return ค่า จริงๆ คือ return () (unit type)
fn no_return() {
    println!("This returns unit type ()");
}

// เขียนแบบ explicit
fn no_return_explicit() -> () {
    println!("This explicitly returns ()");
}

// ==================== Function Parameters ====================

// Pass by value (copy)
fn modify_copy(mut x: i32) {
    x = 100; // เปลี่ยนเฉพาะ copy
    println!("Inside function: {}", x);
}

// Pass by reference (borrow)
fn modify_ref(x: &mut i32) {
    *x = 100; // เปลี่ยนค่าจริง
    println!("Inside function: {}", x);
}

// ==================== Main Function ====================

fn main() {
    println!("=== Basic Functions ===");
    say_hello();
    greet("Rust");
    add(5, 10);

    println!("\n=== Return Values ===");
    let sum = add_return(15, 25);
    println!("Sum: {}", sum);

    let result = divide(10, 2);
    println!("Division: {}", result);

    let (sum, diff, prod, quot) = calculate(20, 4);
    println!("Results: sum={}, diff={}, prod={}, quot={}",
             sum, diff, prod, quot);

    println!("\n=== Statements vs Expressions ===");
    let result = statement_vs_expression();
    println!("Result: {}", result);

    println!("\n=== Function Parameters ===");
    let x = 10;
    modify_copy(x);
    println!("After modify_copy: {}", x); // ยังเป็น 10

    let mut y = 10;
    modify_ref(&mut y);
    println!("After modify_ref: {}", y); // เปลี่ยนเป็น 100
}
```

---

### Step 19: Control Flow - if/else

```rust
fn main() {
    // ==================== Basic if ====================

    let number = 5;

    if number > 0 {
        println!("{} is positive", number);
    }

    // ==================== if-else ====================

    let age = 18;

    if age >= 18 {
        println!("You are an adult");
    } else {
        println!("You are a minor");
    }

    // ==================== if-else if-else ====================

    let score = 85;

    if score >= 90 {
        println!("Grade: A");
    } else if score >= 80 {
        println!("Grade: B");
    } else if score >= 70 {
        println!("Grade: C");
    } else if score >= 60 {
        println!("Grade: D");
    } else {
        println!("Grade: F");
    }

    // ==================== if as Expression ====================

    // if เป็น expression ดังนั้น return ค่าได้
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("Number: {}", number);

    // ⚠️ ทุก branch ต้อง return type เดียวกัน
    // let error = if condition { 5 } else { "six" }; // Error!

    // ==================== Multiple Conditions ====================

    let temperature = 25;
    let is_raining = false;

    // Logical AND (&&)
    if temperature > 20 && !is_raining {
        println!("Great weather for a walk!");
    }

    // Logical OR (||)
    if temperature < 10 || temperature > 30 {
        println!("Extreme temperature!");
    }

    // ==================== Nested if ====================

    let x = 10;
    let y = 20;

    if x > 0 {
        if y > 0 {
            println!("Both positive");
        } else {
            println!("x positive, y negative");
        }
    } else {
        if y > 0 {
            println!("x negative, y positive");
        } else {
            println!("Both negative");
        }
    }

    // ดีกว่า: ใช้ && แทน nested if
    if x > 0 && y > 0 {
        println!("Both positive");
    } else if x > 0 {
        println!("Only x positive");
    } else if y > 0 {
        println!("Only y positive");
    } else {
        println!("Both negative or zero");
    }

    // ==================== Complex Conditions ====================

    let age = 25;
    let has_license = true;
    let has_insurance = true;

    if age >= 18 && has_license && has_insurance {
        println!("Can drive!");
    } else {
        println!("Cannot drive:");
        if age < 18 {
            println!("  - Too young");
        }
        if !has_license {
            println!("  - No license");
        }
        if !has_insurance {
            println!("  - No insurance");
        }
    }

    // ==================== Pattern Matching with let ====================

    let maybe_number = Some(5);

    if let Some(n) = maybe_number {
        println!("Number is: {}", n);
    } else {
        println!("No number");
    }
}
```

---

### Step 20: Control Flow - loops

```rust
fn main() {
    // ==================== loop - infinite loop ====================

    // loop ทำงานตลอดจนกว่าจะมีการ break
    let mut counter = 0;

    loop {
        counter += 1;
        println!("Counter: {}", counter);

        if counter >= 5 {
            break; // ออกจาก loop
        }
    }

    // ==================== loop with return value ====================

    let mut count = 0;
    let result = loop {
        count += 1;

        if count == 10 {
            break count * 2; // return ค่าจาก loop
        }
    };

    println!("Result: {}", result); // 20

    // ==================== Nested loops with labels ====================

    let mut count_outer = 0;
    'outer: loop {
        println!("Outer loop: {}", count_outer);
        let mut count_inner = 0;

        loop {
            println!("  Inner loop: {}", count_inner);

            if count_inner >= 2 {
                break; // break inner loop เท่านั้น
            }

            if count_outer >= 2 {
                break 'outer; // break outer loop
            }

            count_inner += 1;
        }

        count_outer += 1;
    }

    // ==================== while loop ====================

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!");

    // ==================== for loop with range ====================

    // Exclusive range (ไม่รวม 5)
    print!("For 1..5: ");
    for i in 1..5 {
        print!("{} ", i); // 1 2 3 4
    }
    println!();

    // Inclusive range (รวม 5)
    print!("For 1..=5: ");
    for i in 1..=5 {
        print!("{} ", i); // 1 2 3 4 5
    }
    println!();

    // Reverse
    print!("Reverse: ");
    for i in (1..=5).rev() {
        print!("{} ", i); // 5 4 3 2 1
    }
    println!();

    // ==================== for loop with collections ====================

    let arr = [10, 20, 30, 40, 50];

    // Iterate over array
    print!("Array values: ");
    for element in arr {
        print!("{} ", element);
    }
    println!();

    // Iterate with index
    for (index, value) in arr.iter().enumerate() {
        println!("arr[{}] = {}", index, value);
    }

    // ==================== continue keyword ====================

    // skip บางรอบของ loop
    for i in 1..=10 {
        if i % 2 == 0 {
            continue; // skip เลขคู่
        }
        print!("{} ", i); // พิมพ์เฉพาะเลขคี่
    }
    println!();

    // ==================== Loop best practices ====================

    // 1. loop: เมื่อไม่รู้จำนวนรอบที่แน่นอน
    let mut input = 0;
    loop {
        input += 1;
        if input == 5 {
            break;
        }
    }

    // 2. while: เมื่อรู้เงื่อนไข
    let mut count = 5;
    while count > 0 {
        count -= 1;
    }

    // 3. for: เมื่อรู้จำนวนรอบหรือ iterate collection
    for i in 0..10 {
        println!("{}", i);
    }

    // ==================== Examples ====================

    // Factorial using loop
    fn factorial(n: u32) -> u32 {
        let mut result = 1;
        let mut i = 1;
        loop {
            if i > n {
                break;
            }
            result *= i;
            i += 1;
        }
        result
    }

    println!("5! = {}", factorial(5));

    // Fibonacci using while
    fn fibonacci(n: u32) -> u32 {
        if n <= 1 {
            return n;
        }
        let mut a = 0;
        let mut b = 1;
        let mut count = 2;

        while count <= n {
            let temp = a + b;
            a = b;
            b = temp;
            count += 1;
        }
        b
    }

    print!("Fibonacci sequence: ");
    for i in 0..10 {
        print!("{} ", fibonacci(i));
    }
    println!();
}
```

---

**[คู่มือจะดำเนินต่อไปอีก 980 ขั้นตอน...]**

*เนื่องจากคู่มือ 1000 ขั้นตอนจะมีขนาดใหญ่มาก ฉันจะสร้างโครงสร้างและตัวอย่างที่สำคัญในแต่ละระดับ และเตรียม export เป็น markdown ที่สมบูรณ์*

---


## 🎯 Chapter 2: Ownership พื้นฐาน (Steps 21-50)

### Step 21: ความหมายของ Ownership

**ทฤษฎี:**
Ownership เป็นแนวคิดหลักของ Rust ที่ทำให้มี memory safety โดยไม่ต้องใช้ garbage collector

**กฎ 3 ข้อของ Ownership:**
1. แต่ละค่าใน Rust มีเจ้าของ (owner) เพียงคนเดียว
2. มีเจ้าของได้แค่คนเดียวในเวลาเดียวกัน
3. เมื่อเจ้าของออกจาก scope ค่าจะถูก drop (คืนหน่วยความจำ)

**ตัวอย่างโค้ด:**
```rust
fn main() {
    // s ยังไม่ valid ในจุดนี้
    {
        let s = "hello"; // s valid ตั้งแต่จุดนี้
        
        // ทำงานกับ s
        println!("{}", s);
    } // scope สิ้นสุด, s ไม่ valid อีกต่อไป
    
    // println!("{}", s); // Error! s ไม่อยู่ใน scope
}
```

---

### Step 22: String และ String Slice

**ทฤษฎี:**
- `&str` (string slice) = ข้อมูลใน stack, immutable
- `String` = ข้อมูลใน heap, mutable, owned

**ตัวอย่างโค้ด:**
```rust
fn main() {
    // String literal (&str)
    let s1 = "Hello"; // stored in binary
    
    // String (heap-allocated)
    let mut s2 = String::from("Hello");
    s2.push_str(", World!"); // สามารถแก้ไขได้
    
    println!("s1: {}", s1);
    println!("s2: {}", s2);
    
    // การแปลง
    let s3: String = s1.to_string();
    let s4: &str = &s2;
    
    println!("s3: {}", s3);
    println!("s4: {}", s4);
}
```

---

### Step 23: Move Semantics

**ทฤษฎี:**
เมื่อ assign ค่า heap-allocated, ownership จะถูก move (ไม่ใช่ copy)

**ตัวอย่างโค้ด:**
```rust
fn main() {
    let s1 = String::from("Hello");
    let s2 = s1; // s1 ถูก move ไปที่ s2
    
    // println!("{}", s1); // Error! s1 ไม่ valid แล้ว
    println!("{}", s2); // OK
    
    // ใน stack types จะเป็น copy
    let x = 5;
    let y = x; // copy, ไม่ใช่ move
    
    println!("x: {}, y: {}", x, y); // ทั้งคู่ valid
}
```

---

### Step 24: Clone

**ทฤษฎี:**
ใช้ `.clone()` เพื่อ deep copy ข้อมูล heap

**ตัวอย่างโค้ด:**
```rust
fn main() {
    let s1 = String::from("Hello");
    let s2 = s1.clone(); // deep copy
    
    println!("s1: {}, s2: {}", s1, s2); // ทั้งคู่ valid
    
    // Clone มีค่าใช้จ่าย (expensive)
    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = v1.clone(); // copy ข้อมูลทั้งหมด
    
    println!("v1: {:?}, v2: {:?}", v1, v2);
}
```

---

### Step 25: Copy Trait

**ทฤษฎี:**
Types ที่เก็บใน stack implement Copy trait และจะถูก copy แทน move

**ตัวอย่างโค้ด:**
```rust
fn main() {
    // Types ที่ implement Copy:
    // - Integers (i32, u32, etc.)
    // - Boolean (bool)
    // - Floating point (f32, f64)
    // - Character (char)
    // - Tuples (ถ้า elements implement Copy)
    
    let x: i32 = 5;
    let y = x; // copy
    println!("x: {}, y: {}", x, y);
    
    let a = (1, 2.5, 'a');
    let b = a; // copy
    println!("a: {:?}, b: {:?}", a, b);
    
    // String ไม่ implement Copy
    // let s1 = String::from("hello");
    // let s2 = s1; // move, not copy
}
```

---

### Step 26: Functions และ Ownership

**ทฤษฎี:**
การส่งค่าเข้าฟังก์ชัน = move หรือ copy (ขึ้นกับ type)

**ตัวอย่างโค้ด:**
```rust
fn main() {
    let s = String::from("Hello");
    takes_ownership(s); // s ถูก move เข้าฟังก์ชัน
    // println!("{}", s); // Error! s ไม่ valid แล้ว
    
    let x = 5;
    makes_copy(x); // x ถูก copy
    println!("x: {}", x); // OK, x ยัง valid
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string ถูก drop ที่นี่

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer ถูก drop แต่ไม่มีผลอะไร
```

---

### Step 27: Return Values และ Ownership

**ทฤษฎี:**
Return value จะ transfer ownership ไปยัง caller

**ตัวอย่างโค้ด:**
```rust
fn main() {
    let s1 = gives_ownership(); // ได้ ownership
    println!("s1: {}", s1);
    
    let s2 = String::from("Hello");
    let s3 = takes_and_gives_back(s2); // s2 move เข้า, s3 ได้ ownership กลับ
    // println!("{}", s2); // Error!
    println!("s3: {}", s3);
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello");
    some_string // return และ move ownership
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // return และ move ownership
}
```

---

### Step 28: References และ Borrowing

**ทฤษฎี:**
References (&) ให้เราใช้ค่าโดยไม่ต้องรับ ownership

**ตัวอย่างโค้ด:**
```rust
fn main() {
    let s1 = String::from("Hello");
    let len = calculate_length(&s1); // ยืม (borrow) s1
    
    println!("Length of '{}' is {}", s1, len); // s1 ยัง valid
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s ออกจาก scope แต่ไม่ drop เพราะไม่มี ownership
```

---

### Step 29: Mutable References

**ทฤษฎี:**
ใช้ `&mut` เพื่อยืมแบบแก้ไขได้

**กฎสำคัญ:**
- มี mutable reference ได้แค่ 1 อันในเวลาเดียวกัน
- ไม่สามารถมี mutable และ immutable reference พร้อมกันได้

**ตัวอย่างโค้ด:**
```rust
fn main() {
    let mut s = String::from("Hello");
    
    change(&mut s); // ยืมแบบ mutable
    println!("{}", s);
    
    // กฎ: มี mutable ref ได้แค่ 1 อัน
    let r1 = &mut s;
    // let r2 = &mut s; // Error!
    println!("{}", r1);
    
    // OK: r1 ไม่ถูกใช้แล้ว
    let r2 = &mut s;
    println!("{}", r2);
}

fn change(some_string: &mut String) {
    some_string.push_str(", World!");
}
```

---

### Step 30: Dangling References

**ทฤษฎี:**
Rust ป้องกัน dangling references (pointer ชี้ไปที่หน่วยความจำที่ถูกคืนแล้ว)

**ตัวอย่างโค้ด:**
```rust
fn main() {
    // let reference_to_nothing = dangle(); // Error!
    let string = no_dangle();
    println!("{}", string);
}

// ❌ จะ compile ไม่ผ่าน
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // Error! s จะถูก drop พอออกจาก scope
// }

// ✅ ถูกต้อง
fn no_dangle() -> String {
    let s = String::from("hello");
    s // return ownership
}
```

---

### Step 31: Slices

**ทฤษฎี:**
Slices คือ reference ไปยังส่วนหนึ่งของ collection

**ตัวอย่างโค้ด:**
```rust
fn main() {
    let s = String::from("Hello World");
    
    let hello = &s[0..5];  // "Hello"
    let world = &s[6..11]; // "World"
    
    println!("{} {}", hello, world);
    
    // Slice syntax
    let slice1 = &s[0..5];  // index 0 to 4
    let slice2 = &s[..5];   // same as above
    let slice3 = &s[6..];   // index 6 to end
    let slice4 = &s[..];    // entire string
    
    println!("{}, {}, {}, {}", slice1, slice2, slice3, slice4);
    
    // Array slices
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4]; // [2, 3, 4]
    println!("{:?}", slice);
}
```

---

### Step 32: String Slices ในฟังก์ชัน

**ตัวอย่างโค้ด:**
```rust
fn main() {
    let my_string = String::from("hello world");
    
    // ทำงานกับ String
    let word = first_word(&my_string);
    println!("First word: {}", word);
    
    // ทำงานกับ string literal
    let my_string_literal = "hello world";
    let word = first_word(my_string_literal);
    println!("First word: {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}
```

---

### Step 33: Structs - โครงสร้างข้อมูล

**ทฤษฎี:**
Struct ใช้จัดกลุ่มข้อมูลที่เกี่ยวข้องกัน

**ตัวอย่างโค้ด:**
```rust
// กำหนด struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // สร้าง instance
    let user1 = User {
        email: String::from("user@example.com"),
        username: String::from("user123"),
        active: true,
        sign_in_count: 1,
    };
    
    println!("Username: {}", user1.username);
    println!("Email: {}", user1.email);
    
    // Mutable instance
    let mut user2 = User {
        email: String::from("another@example.com"),
        username: String::from("another567"),
        active: true,
        sign_in_count: 1,
    };
    
    user2.email = String::from("newemail@example.com");
    println!("New email: {}", user2.email);
}
```

---

### Step 34: Struct Update Syntax

**ตัวอย่างโค้ด:**
```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("user1@example.com"),
        username: String::from("user1"),
        active: true,
        sign_in_count: 1,
    };
    
    // สร้าง instance ใหม่โดยใช้ค่าจาก user1
    let user2 = User {
        email: String::from("user2@example.com"),
        ..user1 // ใช้ค่าที่เหลือจาก user1
    };
    
    println!("user2: {}", user2.email);
    // println!("{}", user1.username); // Error! username ถูก move
    println!("{}", user1.sign_in_count); // OK, u64 เป็น Copy
}
```

---

### Step 35: Tuple Structs

**ตัวอย่างโค้ด:**
```rust
// Tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    println!("Black: ({}, {}, {})", black.0, black.1, black.2);
    println!("Origin: ({}, {}, {})", origin.0, origin.1, origin.2);
    
    // Color และ Point เป็นคนละ type แม้โครงสร้างเหมือนกัน
    // let color: Color = origin; // Error!
}
```

---

### Step 36: Unit-Like Structs

**ตัวอย่างโค้ด:**
```rust
// Struct ที่ไม่มี field
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
    
    // ใช้สำหรับ implement traits โดยไม่ต้องเก็บข้อมูล
    println!("Created unit-like struct");
}
```

---

### Step 37: Method Syntax

**ตัวอย่างโค้ด:**
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method (รับ &self)
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // Associated function (ไม่รับ &self)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("Area: {}", rect1.area());
    
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    
    // เรียก associated function
    let sq = Rectangle::square(25);
    println!("Square area: {}", sq.area());
}
```

---

### Step 38: Enums

**ทฤษฎี:**
Enum ใช้กำหนด type ที่มีได้หลายค่าที่เป็นไปได้

**ตัวอย่างโค้ด:**
```rust
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    // Enum กับข้อมูลหลายรูปแบบ
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Hello"));
    let msg4 = Message::ChangeColor(255, 0, 0);
}
```

---

### Step 39: Option Enum

**ทฤษฎี:**
`Option<T>` ใช้แทน null เพื่อความปลอดภัย

**ตัวอย่างโค้ด:**
```rust
fn main() {
    let some_number: Option<i32> = Some(5);
    let some_string: Option<&str> = Some("a string");
    let absent_number: Option<i32> = None;
    
    // ต้อง handle None case
    match some_number {
        Some(num) => println!("Number: {}", num),
        None => println!("No number"),
    }
    
    // ใช้ในฟังก์ชัน
    let result = divide(10, 2);
    match result {
        Some(val) => println!("Result: {}", val),
        None => println!("Cannot divide by zero"),
    }
}

fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}
```

---

### Step 40: Match กับ Enums

**ตัวอย่างโค้ด:**
```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let coin = Coin::Penny;
    println!("Value: {} cents", value_in_cents(coin));
    
    // Match กับ Option
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    println!("six: {:?}", six);
    println!("none: {:?}", none);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
```

---

### Step 41: if let

**ทฤษฎี:**
`if let` เป็นวิธีที่สั้นกว่าในการ match pattern เดียว

**ตัวอย่างโค้ด:**
```rust
fn main() {
    let some_value = Some(3);
    
    // แบบใช้ match
    match some_value {
        Some(3) => println!("three"),
        _ => (),
    }
    
    // แบบใช้ if let (สั้นกว่า)
    if let Some(3) = some_value {
        println!("three");
    }
    
    // ตัวอย่างกับ enum
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(String),
    }
    
    let coin = Coin::Quarter(String::from("Alaska"));
    
    // นับเหรียญที่ไม่ใช่ quarter
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {}!", state);
    } else {
        count += 1;
    }
    
    println!("Count: {}", count);
}
```

---

### Step 42: Vectors

**ทฤษฎี:**
`Vec<T>` เก็บข้อมูลหลายค่าใน heap, ขนาดเปลี่ยนได้

**ตัวอย่างโค้ด:**
```rust
fn main() {
    // สร้าง vector
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3]; // ใช้ macro
    
    // เพิ่มข้อมูล
    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    
    println!("v3: {:?}", v3);
    
    // อ่านข้อมูล
    let third: &i32 = &v2[2]; // panic ถ้า index ไม่มี
    println!("Third element: {}", third);
    
    match v2.get(2) {
        Some(third) => println!("Third: {}", third),
        None => println!("No third element"),
    }
    
    // iterate
    for i in &v2 {
        println!("{}", i);
    }
    
    // iterate และแก้ไข
    let mut v4 = vec![100, 32, 57];
    for i in &mut v4 {
        *i += 50;
    }
    println!("v4: {:?}", v4);
}
```

---

### Step 43: String ขั้นสูง

**ตัวอย่างโค้ด:**
```rust
fn main() {
    // สร้าง String
    let mut s1 = String::new();
    let s2 = "initial contents".to_string();
    let s3 = String::from("initial contents");
    
    // เพิ่มข้อมูล
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');
    println!("{}", s); // foobar!
    
    // concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 ถูก move, ไม่สามารถใช้ต่อได้
    println!("{}", s3);
    
    // format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
    
    // Iteration
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
```

---

### Step 44: HashMap

**ตัวอย่างโค้ด:**
```rust
use std::collections::HashMap;

fn main() {
    // สร้าง HashMap
    let mut scores = HashMap::new();
    
    // เพิ่มข้อมูล
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    // อ่านข้อมูล
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    
    match score {
        Some(s) => println!("Score: {}", s),
        None => println!("Team not found"),
    }
    
    // iterate
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
    // Overwriting
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);
    
    // Insert if key doesn't exist
    scores.entry(String::from("Red")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50); // ไม่เกิดผล
    println!("{:?}", scores);
    
    // Update based on old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("{:?}", map);
}
```

---

### Step 45: Error Handling - panic!

**ตัวอย่างโค้ด:**
```rust
fn main() {
    // panic! สำหรับ unrecoverable errors
    // panic!("crash and burn");
    
    // panic จาก index out of bounds
    // let v = vec![1, 2, 3];
    // v[99]; // panic!
    
    // ใช้ RUST_BACKTRACE=1 เพื่อดู backtrace
    println!("Program continues...");
}
```

---

### Step 46: Result Type

**ตัวอย่างโค้ด:**
```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // Result<T, E>
    let f = File::open("hello.txt");
    
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening file: {:?}", other_error);
            }
        },
    };
    
    println!("File opened successfully");
}
```

---

### Step 47: unwrap และ expect

**ตัวอย่างโค้ด:**
```rust
use std::fs::File;

fn main() {
    // unwrap: panic ถ้า error
    // let f = File::open("hello.txt").unwrap();
    
    // expect: panic พร้อมข้อความ
    let f = File::open("hello.txt")
        .expect("Failed to open hello.txt");
    
    println!("File opened");
}
```

---

### Step 48: Propagating Errors

**ตัวอย่างโค้ด:**
```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("username.txt");
    
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut s = String::new();
    
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn main() {
    match read_username_from_file() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Error: {:?}", e),
    }
}
```

---

### Step 49: The ? Operator

**ตัวอย่างโค้ด:**
```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// แบบสั้นยิ่งขึ้น
fn read_username_short() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("username.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// แบบสั้นสุด
fn read_username_shorter() -> Result<String, io::Error> {
    std::fs::read_to_string("username.txt")
}

fn main() {
    match read_username_from_file() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Error: {:?}", e),
    }
}
```

---

### Step 50: Custom Error Types

**ตัวอย่างโค้ด:**
```rust
use std::fmt;

#[derive(Debug)]
enum CustomError {
    IoError(std::io::Error),
    ParseError(std::num::ParseIntError),
    CustomMessage(String),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::IoError(e) => write!(f, "IO Error: {}", e),
            CustomError::ParseError(e) => write!(f, "Parse Error: {}", e),
            CustomError::CustomMessage(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for CustomError {}

fn do_something() -> Result<(), CustomError> {
    // ตัวอย่าง
    Err(CustomError::CustomMessage(String::from("Something went wrong")))
}

fn main() {
    match do_something() {
        Ok(_) => println!("Success"),
        Err(e) => println!("{}", e),
    }
}
```

---

# <a id="section-2-intermediate"></a>📗 ส่วนที่ 2: ระดับกลาง (Steps 251-500)

## 🎯 Chapter 6: Generics (Steps 251-300)

### Step 251: Generic Functions

**ทฤษฎี:**
Generics ช่วยให้เขียนโค้ดที่ทำงานกับหลาย type

**ตัวอย่างโค้ด:**
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

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("Largest number: {}", result);
    
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("Largest char: {}", result);
}
```

---

### Step 252: Generic Structs

**ตัวอย่างโค้ด:**
```rust
struct Point<T> {
    x: T,
    y: T,
}

struct PointMixed<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Implementation สำหรับ type เฉพาะ
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let mixed = PointMixed { x: 5, y: 4.0 };
    
    println!("integer.x = {}", integer.x());
    println!("Distance: {}", float.distance_from_origin());
}
```

---

### Step 253: Generic Enums

**ตัวอย่างโค้ด:**
```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Custom generic enum
enum Operation<T> {
    Add(T, T),
    Subtract(T, T),
    Multiply(T, T),
}

fn main() {
    let add = Operation::Add(5, 3);
    let sub = Operation::Subtract(10.5, 3.2);
    
    match add {
        Operation::Add(a, b) => println!("{} + {} = {}", a, b, a + b),
        _ => {},
    }
}
```

---

### Step 254: Traits

**ทฤษฎี:**
Traits กำหนด functionality ที่ type ควรมี (คล้าย interface)

**ตัวอย่างโค้ด:**
```rust
pub trait Summary {
    fn summarize(&self) -> String;
    
    // Default implementation
    fn summarize_author(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Rust 1.50 Released"),
        location: String::from("Online"),
        author: String::from("Rust Team"),
        content: String::from("The Rust team is happy to announce..."),
    };
    
    println!("Article: {}", article.summarize());
    
    let tweet = Tweet {
        username: String::from("rustlang"),
        content: String::from("New version released!"),
        reply: false,
        retweet: false,
    };
    
    println!("Tweet: {}", tweet.summarize());
}
```

---

### Step 255: Trait Bounds

**ตัวอย่างโค้ด:**
```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

// Trait bound syntax
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple trait bounds
use std::fmt::Display;

pub fn notify_display<T: Summary + Display>(item: &T) {
    println!("{}", item);
}

// where clause (สำหรับ complex bounds)
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // implementation
    0
}

use std::fmt::Debug;

fn main() {
    println!("Trait bounds example");
}
```

---

### Step 256: Returning Traits

**ตัวอย่างโค้ด:**
```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        self.content.clone()
    }
}

pub struct Tweet {
    pub content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        self.content.clone()
    }
}

// Return trait
fn returns_summarizable() -> impl Summary {
    Tweet {
        content: String::from("Hello, world!"),
    }
}

fn main() {
    let tweet = returns_summarizable();
    println!("{}", tweet.summarize());
}
```

---

### Step 257: Lifetimes

**ทฤษฎี:**
Lifetimes บอก compiler ว่า references มีอายุนานแค่ไหน

**ตัวอย่างโค้ด:**
```rust
// ฟังก์ชันนี้ compile ไม่ผ่าน
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// ต้องระบุ lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string");
    let string2 = String::from("xyz");
    
    let result = longest(string1.as_str(), string2.as_str());
    println!("Longest: {}", result);
    
    // Lifetime ต้องถูกต้อง
    let string1 = String::from("long string");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("Longest: {}", result);
    }
    // println!("{}", result); // Error! string2 ไม่ valid แล้ว
}
```

---

### Step 258: Lifetime in Structs

**ตัวอย่างโค้ด:**
```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("No '.' found");
    
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("Excerpt: {}", excerpt.part);
}
```

---

### Step 259: Static Lifetime

**ตัวอย่างโค้ด:**
```rust
fn main() {
    // 'static lifetime = อยู่ได้ตลอดโปรแกรม
    let s: &'static str = "I have a static lifetime.";
    println!("{}", s);
    
    // String literals มี 'static lifetime
    let s = "Hello, world!";
}
```

---

### Step 260: Generic Types + Trait Bounds + Lifetimes

**ตัวอย่างโค้ด:**
```rust
use std::fmt::Display;

fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest_with_announcement(
        string1.as_str(),
        string2,
        "Today is someone's birthday!",
    );
    
    println!("Longest: {}", result);
}
```

