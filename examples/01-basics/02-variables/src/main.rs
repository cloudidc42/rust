// =====================================================
// 02 - Variables: ตัวแปรและการจัดการข้อมูล
// =====================================================

fn main() {
    println!("=== Variables and Mutability ===\n");

    // ==================== Immutable Variables ====================
    // ตัวแปรใน Rust เป็น immutable (ไม่สามารถเปลี่ยนค่าได้) โดยปกติ
    let x = 5;
    println!("Immutable x = {}", x);

    // ⚠️ ไม่สามารถเปลี่ยนค่าได้
    // x = 6; // ❌ Error: cannot assign twice to immutable variable

    // ==================== Mutable Variables ====================
    // ใช้ keyword mut เพื่อทำให้ตัวแปรเปลี่ยนค่าได้
    let mut y = 10;
    println!("\nMutable y (before) = {}", y);

    y = 15; // ✅ สามารถเปลี่ยนค่าได้
    println!("Mutable y (after) = {}", y);

    y += 5; // compound assignment
    println!("Mutable y (after +=) = {}", y);

    // ==================== Type Annotations ====================
    // ระบุชนิดข้อมูลอย่างชัดเจน
    let z: i32 = 20;
    let name: &str = "Rust";
    let is_awesome: bool = true;
    let pi: f64 = 3.14159;

    println!("\n=== Type Annotations ===");
    println!("Integer: {}", z);
    println!("String: {}", name);
    println!("Boolean: {}", is_awesome);
    println!("Float: {}", pi);

    // ==================== Multiple Declarations ====================
    // ประกาศหลายตัวแปรพร้อมกัน (destructuring)
    let (a, b, c) = (1, 2, 3);
    println!("\n=== Multiple Declarations ===");
    println!("a = {}, b = {}, c = {}", a, b, c);

    let (mut x, mut y, mut z) = (10, 20, 30);
    x += 1;
    y += 2;
    z += 3;
    println!("x = {}, y = {}, z = {}", x, y, z);

    // ==================== Shadowing ====================
    // การประกาศตัวแปรชื่อเดิมใหม่
    println!("\n=== Shadowing ===");

    let value = 5;
    println!("First value = {}", value);

    let value = value + 1; // shadow ด้วยค่าใหม่
    println!("Second value = {}", value);

    let value = value * 2; // shadow อีกครั้ง
    println!("Third value = {}", value);

    // Shadow ด้วยชนิดข้อมูลใหม่ (ต่างจาก mut)
    let spaces = "   "; // &str
    let spaces = spaces.len(); // usize
    println!("Spaces count = {}", spaces);

    // ⚠️ mut ไม่สามารถเปลี่ยนชนิดข้อมูลได้
    // let mut spaces_mut = "   ";
    // spaces_mut = spaces_mut.len(); // ❌ Error: expected `&str`, found `usize`

    // ==================== Shadowing in Scope ====================
    println!("\n=== Shadowing in Scope ===");

    let outer = 10;
    println!("Outer = {}", outer);

    {
        let outer = 20; // shadow ใน scope นี้
        println!("Inner outer = {}", outer);

        let inner = 30;
        println!("Inner = {}", inner);
    } // scope สิ้นสุด

    println!("Outer (after scope) = {}", outer); // กลับเป็น 10

    // ==================== Constants ====================
    println!("\n=== Constants ===");

    const MAX_POINTS: u32 = 100_000; // ต้องระบุ type
    const PI: f64 = 3.14159265359;
    const APP_NAME: &str = "RustApp";

    println!("MAX_POINTS = {}", MAX_POINTS);
    println!("PI = {}", PI);
    println!("APP_NAME = {}", APP_NAME);

    // Constants vs Variables:
    // 1. Constants ต้องระบุ type เสมอ
    // 2. Constants ไม่สามารถใช้ mut
    // 3. Constants สามารถประกาศใน global scope
    // 4. Constants ต้องเป็นค่าคงที่ (compile-time constant)

    // ==================== Static Variables ====================
    println!("\n=== Static Variables ===");

    static LANGUAGE: &str = "Rust";
    println!("LANGUAGE = {}", LANGUAGE);

    // ==================== Type Inference ====================
    println!("\n=== Type Inference ===");

    // Rust สามารถอนุมานชนิดข้อมูลได้
    let inferred_int = 42; // i32 (default)
    let inferred_float = 3.14; // f64 (default)
    let inferred_bool = true; // bool
    let inferred_str = "hello"; // &str

    println!("Inferred int: {}", inferred_int);
    println!("Inferred float: {}", inferred_float);
    println!("Inferred bool: {}", inferred_bool);
    println!("Inferred str: {}", inferred_str);

    // ==================== Unused Variables ====================
    // ใช้ _ prefix สำหรับตัวแปรที่ไม่ได้ใช้ (ป้องกัน warning)
    let _unused = 42;

    // ==================== Variable Naming ====================
    println!("\n=== Variable Naming ===");

    // snake_case เป็น convention
    let user_name = "Alice";
    let total_count = 100;
    let is_valid = true;

    println!("user_name = {}", user_name);
    println!("total_count = {}", total_count);
    println!("is_valid = {}", is_valid);

    // ==================== Examples ====================
    println!("\n=== Practical Examples ===");

    // Counter example
    let mut counter = 0;
    counter += 1;
    counter += 1;
    counter += 1;
    println!("Counter: {}", counter);

    // Temperature conversion
    let celsius = 25.0;
    let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
    println!("{}°C = {}°F", celsius, fahrenheit);

    // Calculate area
    let width = 10;
    let height = 20;
    let area = width * height;
    println!("Area: {} × {} = {}", width, height, area);

    // String manipulation with shadowing
    let text = "hello";
    let text = text.to_uppercase();
    let text = format!("{} Rust!", text);
    println!("Text: {}", text);

    // ==================== Summary ====================
    println!("\n=== สรุป ===");
    println!("✅ ตัวแปรเป็น immutable โดยปกติ");
    println!("✅ ใช้ mut เพื่อทำให้เปลี่ยนค่าได้");
    println!("✅ Shadowing ช่วยประกาศตัวแปรชื่อเดิมใหม่");
    println!("✅ Constants ใช้สำหรับค่าคงที่");
    println!("✅ Rust มี type inference ที่ดี");
}
