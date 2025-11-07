// =====================================================
// 01 - Hello World: โปรแกรม Rust แรกของคุณ
// =====================================================

/// จุดเริ่มต้นของโปรแกรม
///
/// ฟังก์ชัน main() เป็นจุดเริ่มต้นของทุกโปรแกรม Rust
/// โปรแกรมจะเริ่มทำงานจากฟังก์ชันนี้เสมอ
fn main() {
    // ==================== การพิมพ์ข้อความพื้นฐาน ====================

    // println! เป็น macro (สังเกตเครื่องหมาย !)
    // ใช้สำหรับพิมพ์ข้อความและขึ้นบรรทัดใหม่
    println!("Hello, World!");
    println!("สวัสดี Rust! 🦀");

    // ==================== การพิมพ์ด้วย Format ====================

    // ใช้ {} เป็น placeholder สำหรับค่าที่ต้องการแสดง
    let language = "Rust";
    let year = 2024;

    println!("ฉันกำลังเรียน {}", language);
    println!("ปี {}", year);
    println!("{} เปิดตัวในปี 2010", language);

    // ==================== การใช้ Multiple Placeholders ====================

    let name = "มานะ";
    let age = 25;
    let city = "กรุงเทพ";

    println!("ชื่อ: {}, อายุ: {}, เมือง: {}", name, age, city);

    // ==================== Named Arguments ====================

    println!("{name} อายุ {age} ปี อาศัยอยู่ที่{city}",
             name=name, age=age, city=city);

    // หรือใช้แบบสั้น (Rust 2021+)
    println!("{name} อายุ {age} ปี");

    // ==================== Format Specifiers ====================

    let number = 42;

    println!("Decimal: {}", number);           // 42
    println!("Binary: {:b}", number);          // 101010
    println!("Octal: {:o}", number);           // 52
    println!("Hexadecimal: {:x}", number);     // 2a
    println!("Hexadecimal (upper): {:X}", number); // 2A

    // ==================== Padding และ Alignment ====================

    println!("Right aligned: {:>10}", number);  // "        42"
    println!("Left aligned: {:<10}", number);   // "42        "
    println!("Center aligned: {:^10}", number); // "    42    "
    println!("Zero padded: {:05}", number);     // "00042"

    // ==================== Floating Point ====================

    let pi = 3.14159265359;

    println!("Pi: {}", pi);              // แสดงทุกหลัก
    println!("Pi (2 decimals): {:.2}", pi); // 3.14
    println!("Pi (4 decimals): {:.4}", pi); // 3.1416

    // ==================== Debug Printing ====================

    // {:?} สำหรับ debug printing
    let tuple = (1, 2, 3);
    let array = [10, 20, 30];

    println!("Tuple: {:?}", tuple);
    println!("Array: {:?}", array);

    // {:#?} สำหรับ pretty-print
    let nested = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    println!("Nested (compact): {:?}", nested);
    println!("Nested (pretty):");
    println!("{:#?}", nested);

    // ==================== Multiple Lines ====================

    println!("บรรทัดที่ 1");
    println!("บรรทัดที่ 2");
    println!("บรรทัดที่ 3");

    // ใช้ \n สำหรับขึ้นบรรทัดใหม่
    println!("บรรทัดที่ 1\nบรรทัดที่ 2\nบรรทัดที่ 3");

    // ==================== Raw String Literals ====================

    // ใช้ r"..." สำหรับ raw string (ไม่ต้อง escape)
    println!(r"Path: C:\Users\Documents\file.txt");

    // ==================== Multi-line String ====================

    println!("
    ╔══════════════════════════════╗
    ║   🦀 ยินดีต้อนรับสู่ Rust   ║
    ║  โปรแกรมแรกของคุณสำเร็จ!   ║
    ╚══════════════════════════════╝
    ");

    // ==================== print! vs println! ====================

    // print! ไม่ขึ้นบรรทัดใหม่
    print!("Hello, ");
    print!("World");
    println!("!"); // ขึ้นบรรทัดใหม่

    // ==================== สรุป ====================

    println!("\n=== สรุป ===");
    println!("✅ ใช้ println!() สำหรับพิมพ์ข้อความ");
    println!("✅ ใช้ {{}} สำหรับแทรกค่าตัวแปร");
    println!("✅ ใช้ {{:?}} สำหรับ debug printing");
    println!("✅ ใช้ {{:.2}} สำหรับกำหนดทศนิยม");
    println!("✅ รองรับ Unicode และ Emoji 🎉");
}
