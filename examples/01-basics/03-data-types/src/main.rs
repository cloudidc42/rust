// =====================================================
// 03 - Data Types: ชนิดข้อมูลใน Rust
// =====================================================

fn main() {
    println!("=== Rust Data Types ===\n");

    // ==================== Integer Types ====================
    println!("=== Integer Types ===");

    // Signed integers (มีเครื่องหมาย)
    let a: i8 = -128; // -128 to 127
    let b: i16 = 32_767;
    let c: i32 = 2_147_483_647; // default integer type
    let d: i64 = 9_223_372_036_854_775_807;
    let e: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;
    let f: isize = 100; // ขนาดตาม architecture

    println!("Signed integers:");
    println!("  i8:    {}", a);
    println!("  i16:   {}", b);
    println!("  i32:   {}", c);
    println!("  i64:   {}", d);
    println!("  i128:  {}", e);
    println!("  isize: {}", f);

    // Unsigned integers (ไม่มีเครื่องหมาย)
    let g: u8 = 255;
    let h: u16 = 65_535;
    let i: u32 = 4_294_967_295;
    let j: u64 = 18_446_744_073_709_551_615;
    let k: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;
    let l: usize = 100;

    println!("\nUnsigned integers:");
    println!("  u8:    {}", g);
    println!("  u16:   {}", h);
    println!("  u32:   {}", i);
    println!("  u64:   {}", j);
    println!("  u128:  {}", k);
    println!("  usize: {}", l);

    // Integer literals
    println!("\n=== Integer Literals ===");
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A'; // u8 only

    println!("Decimal:     {}", decimal);
    println!("Hexadecimal: {}", hex);
    println!("Octal:       {}", octal);
    println!("Binary:      {}", binary);
    println!("Byte:        {}", byte);

    // ==================== Floating-Point Types ====================
    println!("\n=== Floating-Point Types ===");

    let x: f32 = 3.14159; // 32-bit
    let y: f64 = 2.71828; // 64-bit (default)

    println!("f32: {}", x);
    println!("f64: {}", y);

    // Special values
    let infinity = f64::INFINITY;
    let neg_infinity = f64::NEG_INFINITY;
    let nan = f64::NAN;

    println!("\nSpecial values:");
    println!("  INFINITY:     {}", infinity);
    println!("  NEG_INFINITY: {}", neg_infinity);
    println!("  NAN:          {}", nan);

    // ==================== Boolean Type ====================
    println!("\n=== Boolean Type ===");

    let t = true;
    let f: bool = false;

    println!("true:  {}", t);
    println!("false: {}", f);

    // Boolean operations
    println!("\nBoolean operations:");
    println!("  true && false = {}", true && false);
    println!("  true || false = {}", true || false);
    println!("  !true         = {}", !true);

    // ==================== Character Type ====================
    println!("\n=== Character Type ===");

    let c = 'z';
    let heart = '❤';
    let emoji = '😀';
    let thai = 'ก';

    println!("Character: {}", c);
    println!("Heart:     {}", heart);
    println!("Emoji:     {}", emoji);
    println!("Thai:      {}", thai);
    println!("Size:      {} bytes", std::mem::size_of::<char>());

    // ==================== Tuple Type ====================
    println!("\n=== Tuple Type ===");

    let tup: (i32, f64, u8, &str) = (500, 6.4, 1, "hello");
    println!("Tuple: {:?}", tup);

    // Destructuring
    let (a, b, c, d) = tup;
    println!("a={}, b={}, c={}, d={}", a, b, c, d);

    // Accessing by index
    println!("tup.0 = {}", tup.0);
    println!("tup.1 = {}", tup.1);
    println!("tup.2 = {}", tup.2);
    println!("tup.3 = {}", tup.3);

    // ==================== Array Type ====================
    println!("\n=== Array Type ===");

    let arr = [1, 2, 3, 4, 5];
    println!("Array: {:?}", arr);

    // Array with type annotation
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Numbers: {:?}", numbers);

    // Initialize with same value
    let zeros = [0; 10];
    println!("Zeros: {:?}", zeros);

    // Accessing elements
    println!("First:  {}", arr[0]);
    println!("Second: {}", arr[1]);
    println!("Length: {}", arr.len());

    // ==================== String Types ====================
    println!("\n=== String Types ===");

    // String slice (&str)
    let str_literal: &str = "Hello, Rust!";
    println!("String slice: {}", str_literal);

    // String (heap-allocated)
    let mut string = String::from("Hello");
    string.push_str(", World!");
    println!("String: {}", string);

    // ==================== Type Conversion ====================
    println!("\n=== Type Conversion ===");

    // Integer to float
    let int_num: i32 = 10;
    let float_num: f64 = int_num as f64;
    println!("i32 {} as f64 = {}", int_num, float_num);

    // Float to integer (truncates)
    let pi: f64 = 3.14159;
    let int_pi: i32 = pi as i32;
    println!("f64 {} as i32 = {}", pi, int_pi);

    // Boolean to integer
    let bool_val = true;
    let int_val = bool_val as i32;
    println!("bool {} as i32 = {}", bool_val, int_val);

    // Character to integer
    let char_a = 'A';
    let ascii = char_a as u8;
    println!("char '{}' as u8 = {}", char_a, ascii);

    // ==================== Numeric Operations ====================
    println!("\n=== Numeric Operations ===");

    let sum = 5 + 10;
    let difference = 95 - 4;
    let product = 4 * 30;
    let quotient = 56 / 32;
    let remainder = 43 % 5;

    println!("5 + 10  = {}", sum);
    println!("95 - 4  = {}", difference);
    println!("4 * 30  = {}", product);
    println!("56 / 32 = {}", quotient);
    println!("43 % 5  = {}", remainder);

    // ==================== Bitwise Operations ====================
    println!("\n=== Bitwise Operations ===");

    let a = 0b1100; // 12
    let b = 0b1010; // 10

    println!("a     = {:04b} ({})", a, a);
    println!("b     = {:04b} ({})", b, b);
    println!("a & b = {:04b} ({})", a & b, a & b);
    println!("a | b = {:04b} ({})", a | b, a | b);
    println!("a ^ b = {:04b} ({})", a ^ b, a ^ b);
    println!("!a    = {:08b}", !a);
    println!("a << 1 = {:05b} ({})", a << 1, a << 1);
    println!("a >> 1 = {:03b} ({})", a >> 1, a >> 1);

    // ==================== Type Sizes ====================
    println!("\n=== Type Sizes (bytes) ===");
    println!("i8:    {}", std::mem::size_of::<i8>());
    println!("i16:   {}", std::mem::size_of::<i16>());
    println!("i32:   {}", std::mem::size_of::<i32>());
    println!("i64:   {}", std::mem::size_of::<i64>());
    println!("i128:  {}", std::mem::size_of::<i128>());
    println!("f32:   {}", std::mem::size_of::<f32>());
    println!("f64:   {}", std::mem::size_of::<f64>());
    println!("bool:  {}", std::mem::size_of::<bool>());
    println!("char:  {}", std::mem::size_of::<char>());

    // ==================== Summary ====================
    println!("\n=== สรุป ===");
    println!("✅ Integer: i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize");
    println!("✅ Float: f32, f64");
    println!("✅ Boolean: bool");
    println!("✅ Character: char (4 bytes, Unicode)");
    println!("✅ Tuple: (T1, T2, ..., Tn)");
    println!("✅ Array: [T; N]");
}
