// =====================================================
// 04 - Functions: ฟังก์ชันใน Rust
// =====================================================

// ==================== Function Without Parameters ====================
fn say_hello() {
    println!("Hello from function!");
}

// ==================== Function With Parameters ====================
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn add_numbers(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x + y);
}

// ==================== Function With Return Value ====================
fn add(x: i32, y: i32) -> i32 {
    x + y // expression ไม่มี semicolon = return value
}

fn subtract(x: i32, y: i32) -> i32 {
    return x - y; // แบบใช้ return keyword
}

fn multiply(x: i32, y: i32) -> i32 {
    let result = x * y;
    result // return expression
}

// ==================== Multiple Return Values (Tuple) ====================
fn calculate(x: i32, y: i32) -> (i32, i32, i32, i32) {
    (x + y, x - y, x * y, x / y)
}

fn swap(x: i32, y: i32) -> (i32, i32) {
    (y, x)
}

// ==================== Early Return ====================
fn divide(x: i32, y: i32) -> i32 {
    if y == 0 {
        println!("Error: Division by zero!");
        return 0; // early return
    }
    x / y
}

// ==================== Expressions vs Statements ====================
fn expression_example() -> i32 {
    let x = 5;

    // Block expression
    let y = {
        let z = 10;
        z + x // expression = return value
    };

    y * 2 // return
}

// ==================== Function with Mutable Parameters ====================
fn increment(x: &mut i32) {
    *x += 1;
}

fn double_value(x: &mut i32) {
    *x *= 2;
}

// ==================== Function with Slice Parameters ====================
fn print_array(arr: &[i32]) {
    print!("[");
    for (i, &val) in arr.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{}", val);
    }
    println!("]");
}

fn sum_array(arr: &[i32]) -> i32 {
    let mut total = 0;
    for &val in arr {
        total += val;
    }
    total
}

// ==================== Recursive Functions ====================
fn factorial(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// ==================== Function with Option Return ====================
fn safe_divide(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        None
    } else {
        Some(x / y)
    }
}

// ==================== Function with Result Return ====================
fn checked_divide(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(x / y)
    }
}

// ==================== Higher-Order Functions ====================
fn apply_operation<F>(x: i32, y: i32, operation: F) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    operation(x, y)
}

// ==================== Methods (Associated Functions) ====================
struct Calculator {
    result: i32,
}

impl Calculator {
    // Associated function (like static method)
    fn new() -> Calculator {
        Calculator { result: 0 }
    }

    // Method (takes &self)
    fn add(&mut self, value: i32) {
        self.result += value;
    }

    fn get_result(&self) -> i32 {
        self.result
    }
}

// ==================== Main Function ====================
fn main() {
    println!("=== Rust Functions ===\n");

    // Basic functions
    println!("=== Basic Functions ===");
    say_hello();
    greet("Rust");
    add_numbers(10, 20);

    // Functions with return values
    println!("\n=== Functions with Return Values ===");
    let sum = add(15, 25);
    let diff = subtract(50, 20);
    let product = multiply(7, 8);
    println!("Sum: {}", sum);
    println!("Difference: {}", diff);
    println!("Product: {}", product);

    // Multiple return values
    println!("\n=== Multiple Return Values ===");
    let (sum, diff, prod, quot) = calculate(20, 4);
    println!("20 + 4 = {}", sum);
    println!("20 - 4 = {}", diff);
    println!("20 * 4 = {}", prod);
    println!("20 / 4 = {}", quot);

    let (a, b) = swap(10, 20);
    println!("After swap: a={}, b={}", a, b);

    // Early return
    println!("\n=== Early Return ===");
    println!("10 / 2 = {}", divide(10, 2));
    println!("10 / 0 = {}", divide(10, 0));

    // Expression example
    println!("\n=== Expressions ===");
    println!("Expression result: {}", expression_example());

    // Mutable parameters
    println!("\n=== Mutable Parameters ===");
    let mut num = 10;
    println!("Before: {}", num);
    increment(&mut num);
    println!("After increment: {}", num);
    double_value(&mut num);
    println!("After double: {}", num);

    // Array parameters
    println!("\n=== Array Parameters ===");
    let numbers = [1, 2, 3, 4, 5];
    print!("Array: ");
    print_array(&numbers);
    println!("Sum: {}", sum_array(&numbers));

    // Recursive functions
    println!("\n=== Recursive Functions ===");
    println!("5! = {}", factorial(5));
    print!("Fibonacci: ");
    for i in 0..10 {
        print!("{} ", fibonacci(i));
    }
    println!();

    // Option and Result
    println!("\n=== Option and Result ===");
    match safe_divide(10, 2) {
        Some(result) => println!("10 / 2 = {}", result),
        None => println!("Error!"),
    }

    match safe_divide(10, 0) {
        Some(result) => println!("10 / 0 = {}", result),
        None => println!("Cannot divide by zero!"),
    }

    match checked_divide(20, 4) {
        Ok(result) => println!("20 / 4 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match checked_divide(20, 0) {
        Ok(result) => println!("20 / 0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // Higher-order functions
    println!("\n=== Higher-Order Functions ===");
    let result = apply_operation(10, 5, |x, y| x + y);
    println!("10 + 5 = {}", result);

    let result = apply_operation(10, 5, |x, y| x * y);
    println!("10 * 5 = {}", result);

    // Methods
    println!("\n=== Methods ===");
    let mut calc = Calculator::new();
    calc.add(10);
    calc.add(20);
    calc.add(5);
    println!("Calculator result: {}", calc.get_result());

    // Summary
    println!("\n=== สรุป ===");
    println!("✅ ฟังก์ชันประกาศด้วย fn");
    println!("✅ Parameters ต้องระบุ type");
    println!("✅ Return type ใช้ ->");
    println!("✅ Expression ไม่มี semicolon = return value");
    println!("✅ รองรับ recursion และ higher-order functions");
}
