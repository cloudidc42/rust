// =====================================================
// Ownership Demo - ตัวอย่างการใช้งาน Ownership ครบถ้วน
// =====================================================

fn main() {
    println!("=== Rust Ownership Complete Demo ===\n");

    // ==================== Ownership Basics ====================
    println!("=== 1. Ownership Basics ===");
    {
        let s1 = String::from("Hello");
        let s2 = s1; // s1 ถูก move ไปที่ s2

        // println!("{}", s1); // ❌ Error! s1 ไม่ valid แล้ว
        println!("s2: {}", s2);
    }

    // ==================== Clone ====================
    println!("\n=== 2. Clone (Deep Copy) ===");
    {
        let s1 = String::from("Hello");
        let s2 = s1.clone(); // Deep copy

        println!("s1: {}, s2: {}", s1, s2); // ✅ ทั้งคู่ valid
    }

    // ==================== Copy Trait ====================
    println!("\n=== 3. Copy Trait (Stack Data) ===");
    {
        let x = 5;
        let y = x; // Copy, ไม่ใช่ move

        println!("x: {}, y: {}", x, y); // ทั้งคู่ valid

        // Types ที่ implement Copy
        let a: i32 = 10;
        let b = a;

        let c: bool = true;
        let d = c;

        let e: char = 'A';
        let f = e;

        let g = (1, 2.5);
        let h = g;

        println!("All stack types copied successfully");
    }

    // ==================== References (Borrowing) ====================
    println!("\n=== 4. Immutable References ===");
    {
        let s = String::from("Hello");

        let len = calculate_length(&s); // ยืม s
        println!("Length of '{}' is {}", s, len); // s ยัง valid

        // หลาย immutable references ได้
        let r1 = &s;
        let r2 = &s;
        let r3 = &s;

        println!("r1: {}, r2: {}, r3: {}", r1, r2, r3);
    }

    // ==================== Mutable References ====================
    println!("\n=== 5. Mutable References ===");
    {
        let mut s = String::from("Hello");

        change(&mut s);
        println!("After change: {}", s);

        // มี mutable ref ได้แค่ 1 อันในเวลาเดียวกัน
        let r1 = &mut s;
        r1.push_str("!");
        println!("r1: {}", r1);

        // ไม่สามารถมี mutable และ immutable ref พร้อมกัน
        // let r2 = &s; // ❌ Error!
        // println!("{}, {}", r1, r2);
    }

    // ==================== Slices ====================
    println!("\n=== 6. String Slices ===");
    {
        let s = String::from("Hello World");

        let hello = &s[0..5];
        let world = &s[6..11];

        println!("First word: {}", hello);
        println!("Second word: {}", world);

        let word = first_word(&s);
        println!("First word (function): {}", word);
    }

    // ==================== Array Slices ====================
    println!("\n=== 7. Array Slices ===");
    {
        let arr = [1, 2, 3, 4, 5];

        let slice = &arr[1..4];
        println!("Array slice: {:?}", slice);

        process_slice(slice);
    }

    // ==================== Ownership in Functions ====================
    println!("\n=== 8. Ownership in Functions ===");
    {
        let s = String::from("Hello");
        takes_ownership(s); // s ถูก move
        // println!("{}", s); // ❌ Error!

        let x = 5;
        makes_copy(x); // x ถูก copy
        println!("x is still valid: {}", x); // ✅ OK
    }

    // ==================== Return Values ====================
    println!("\n=== 9. Ownership and Return Values ===");
    {
        let s1 = gives_ownership();
        println!("s1: {}", s1);

        let s2 = String::from("Hello");
        let s3 = takes_and_gives_back(s2);
        println!("s3: {}", s3);
        // println!("{}", s2); // ❌ Error! s2 ถูก move
    }

    // ==================== References in Structs ====================
    println!("\n=== 10. Practical Example: Text Analyzer ===");
    {
        let text = String::from("The quick brown fox jumps over the lazy dog");
        let analyzer = TextAnalyzer::new(&text);

        analyzer.analyze();
    }

    println!("\n=== สรุป ===");
    println!("✅ Ownership: แต่ละค่ามีเจ้าของเพียงคนเดียว");
    println!("✅ Move: ค่า heap ถูก move, ไม่ใช่ copy");
    println!("✅ Clone: Deep copy ด้วย .clone()");
    println!("✅ Copy: Stack types copy โดยอัตโนมัติ");
    println!("✅ References: ยืมค่าโดยไม่รับ ownership");
    println!("✅ Mutable ref: มีได้แค่ 1 อันในเวลาเดียวกัน");
}

// ==================== Helper Functions ====================

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", World");
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

fn process_slice(slice: &[i32]) {
    println!("Processing slice: {:?}", slice);
    for &item in slice {
        print!("{} ", item);
    }
    println!();
}

fn takes_ownership(some_string: String) {
    println!("Took ownership: {}", some_string);
} // some_string ถูก drop

fn makes_copy(some_integer: i32) {
    println!("Made copy: {}", some_integer);
}

fn gives_ownership() -> String {
    String::from("Ownership given")
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// ==================== Practical Example ====================

struct TextAnalyzer<'a> {
    text: &'a str,
}

impl<'a> TextAnalyzer<'a> {
    fn new(text: &'a str) -> Self {
        TextAnalyzer { text }
    }

    fn analyze(&self) {
        println!("\n--- Text Analysis ---");
        println!("Original: {}", self.text);
        println!("Length: {} characters", self.text.len());
        println!("Words: {}", self.word_count());
        println!("First word: {}", self.first_word());
        println!("Last word: {}", self.last_word());
        println!("Uppercase: {}", self.text.to_uppercase());
    }

    fn word_count(&self) -> usize {
        self.text.split_whitespace().count()
    }

    fn first_word(&self) -> &str {
        self.text.split_whitespace().next().unwrap_or("")
    }

    fn last_word(&self) -> &str {
        self.text.split_whitespace().last().unwrap_or("")
    }
}
