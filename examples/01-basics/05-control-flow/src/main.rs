// =====================================================
// 05 - Control Flow: การควบคุมการทำงานของโปรแกรม
// =====================================================

fn main() {
    println!("=== Control Flow in Rust ===\n");

    // ==================== if Expressions ====================
    println!("=== if Expressions ===");

    let number = 7;
    if number < 5 {
        println!("{} is less than 5", number);
    } else if number == 5 {
        println!("{} is equal to 5", number);
    } else {
        println!("{} is greater than 5", number);
    }

    // if as expression
    let condition = true;
    let value = if condition { 10 } else { 20 };
    println!("Value: {}", value);

    // ==================== match Expressions ====================
    println!("\n=== match Expressions ===");

    let number = 3;
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 | 5 => println!("Four or Five"),
        6..=10 => println!("Six to Ten"),
        _ => println!("Something else"),
    }

    // match with return value
    let result = match number {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "many",
    };
    println!("Number in words: {}", result);

    // Destructuring with match
    let point = (0, 5);
    match point {
        (0, 0) => println!("Origin"),
        (0, y) => println!("On Y axis at {}", y),
        (x, 0) => println!("On X axis at {}", x),
        (x, y) => println!("Point at ({}, {})", x, y),
    }

    // ==================== loop - Infinite Loop ====================
    println!("\n=== loop (Infinite Loop) ===");

    let mut counter = 0;
    loop {
        counter += 1;
        print!("{} ", counter);

        if counter >= 5 {
            break;
        }
    }
    println!();

    // loop with return value
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2; // return value
        }
    };
    println!("Result from loop: {}", result);

    // ==================== Nested Loops with Labels ====================
    println!("\n=== Nested Loops with Labels ===");

    let mut count_outer = 0;
    'outer: loop {
        print!("Outer {}: ", count_outer);
        let mut count_inner = 0;

        loop {
            print!("{} ", count_inner);

            if count_inner >= 3 {
                break;
            }

            if count_outer >= 2 {
                println!();
                break 'outer;
            }

            count_inner += 1;
        }

        println!();
        count_outer += 1;
    }

    // ==================== while Loop ====================
    println!("\n=== while Loop ===");

    let mut number = 5;
    while number != 0 {
        print!("{}! ", number);
        number -= 1;
    }
    println!("LIFTOFF!");

    // ==================== for Loop ====================
    println!("\n=== for Loop ===");

    // Range (exclusive)
    print!("1..5: ");
    for i in 1..5 {
        print!("{} ", i);
    }
    println!();

    // Range (inclusive)
    print!("1..=5: ");
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();

    // Reverse
    print!("Reverse: ");
    for i in (1..=5).rev() {
        print!("{} ", i);
    }
    println!();

    // Iterate array
    let arr = [10, 20, 30, 40, 50];
    print!("Array: ");
    for element in arr {
        print!("{} ", element);
    }
    println!();

    // Iterate with index
    println!("With index:");
    for (i, element) in arr.iter().enumerate() {
        println!("  arr[{}] = {}", i, element);
    }

    // ==================== continue and break ====================
    println!("\n=== continue and break ===");

    // continue: skip current iteration
    print!("Odd numbers: ");
    for i in 1..=10 {
        if i % 2 == 0 {
            continue; // skip even numbers
        }
        print!("{} ", i);
    }
    println!();

    // break: exit loop
    print!("Until 5: ");
    for i in 1..=10 {
        if i > 5 {
            break;
        }
        print!("{} ", i);
    }
    println!();

    // ==================== if let ====================
    println!("\n=== if let ===");

    let some_value = Some(7);

    // Traditional match
    match some_value {
        Some(val) => println!("Value is: {}", val),
        None => println!("No value"),
    }

    // Using if let (shorter)
    if let Some(val) = some_value {
        println!("Value is: {}", val);
    } else {
        println!("No value");
    }

    // ==================== while let ====================
    println!("\n=== while let ===");

    let mut stack = vec![1, 2, 3, 4, 5];

    while let Some(top) = stack.pop() {
        print!("{} ", top);
    }
    println!();

    // ==================== Practical Examples ====================
    println!("\n=== Practical Examples ===");

    // Fizz Buzz
    println!("Fizz Buzz:");
    for i in 1..=15 {
        match (i % 3, i % 5) {
            (0, 0) => println!("{}: FizzBuzz", i),
            (0, _) => println!("{}: Fizz", i),
            (_, 0) => println!("{}: Buzz", i),
            _ => println!("{}", i),
        }
    }

    // Find first even number
    let numbers = [1, 3, 5, 7, 8, 9, 11];
    let mut first_even = None;

    for &num in &numbers {
        if num % 2 == 0 {
            first_even = Some(num);
            break;
        }
    }

    match first_even {
        Some(num) => println!("\nFirst even number: {}", num),
        None => println!("\nNo even number found"),
    }

    // Sum of array
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut sum = 0;

    for num in numbers {
        sum += num;
    }
    println!("Sum of 1..=10: {}", sum);

    // Prime number check
    fn is_prime(n: u32) -> bool {
        if n <= 1 {
            return false;
        }
        for i in 2..=((n as f64).sqrt() as u32) {
            if n % i == 0 {
                return false;
            }
        }
        true
    }

    println!("\nPrime numbers up to 20:");
    for i in 2..=20 {
        if is_prime(i) {
            print!("{} ", i);
        }
    }
    println!();

    // Factorial
    fn factorial(n: u32) -> u32 {
        let mut result = 1;
        for i in 1..=n {
            result *= i;
        }
        result
    }

    println!("\nFactorials:");
    for i in 1..=10 {
        println!("{}! = {}", i, factorial(i));
    }

    // Multiplication table
    println!("\nMultiplication Table (5):");
    for i in 1..=10 {
        println!("5 × {} = {}", i, 5 * i);
    }

    // Pattern matching with enums
    #[derive(Debug)]
    enum Direction {
        North,
        South,
        East,
        West,
    }

    let direction = Direction::North;
    match direction {
        Direction::North => println!("\nGoing North ↑"),
        Direction::South => println!("\nGoing South ↓"),
        Direction::East => println!("\nGoing East →"),
        Direction::West => println!("\nGoing West ←"),
    }

    // ==================== Summary ====================
    println!("\n=== สรุป ===");
    println!("✅ if/else สำหรับเงื่อนไข");
    println!("✅ match สำหรับ pattern matching");
    println!("✅ loop สำหรับวนซ้ำไม่จำกัด");
    println!("✅ while สำหรับวนซ้ำตามเงื่อนไข");
    println!("✅ for สำหรับวนซ้ำตามจำนวน");
    println!("✅ break/continue สำหรับควบคุม loop");
    println!("✅ if let/while let สำหรับ pattern matching แบบสั้น");
}
