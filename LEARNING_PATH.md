# 🦀 Rust Learning Path - เส้นทางการเรียน Rust
# แผนการเรียนรู้ 1000 ขั้นตอน จากพื้นฐานสู่มืออาชีพ

---

## 📚 ภาพรวมเนื้อหา

คู่มือนี้ครอบคลุมการเรียน Rust ทั้งหมด 1000 ขั้นตอน แบ่งเป็น 4 ระดับ:

1. **ระดับพื้นฐาน** (Steps 1-250) - 5 สัปดาห์
2. **ระดับกลาง** (Steps 251-500) - 6 สัปดาห์
3. **ระดับสูง** (Steps 501-750) - 8 สัปดาห์
4. **ระดับมืออาชีพ** (Steps 751-1000) - 10 สัปดาห์

**รวมเวลาโดยประมาณ: 29 สัปดาห์ (7 เดือน)**

---

## 🎯 ระดับพื้นฐาน (Steps 1-250)

### สัปดาห์ที่ 1-2: Introduction & Setup (Steps 1-50)
**เนื้อหา:**
- ✅ ทำความรู้จักกับ Rust
- ✅ ติดตั้ง Rust และเครื่องมือ
- ✅ เข้าใจ Cargo และ Package Management
- ✅ Hello World และ Basic Syntax
- ✅ Comments และ Documentation
- ✅ Variables, Mutability, Shadowing
- ✅ Constants และ Static

**ตัวอย่างโค้ด:**
- `examples/01-basics/01-hello-world/`
- `examples/01-basics/02-variables/`

**แบบฝึกหัด:**
1. สร้างโปรแกรม Hello World
2. ทดลองใช้ variables และ shadowing
3. สร้าง constants และ static variables

---

### สัปดาห์ที่ 2-3: Data Types (Steps 51-100)
**เนื้อหา:**
- ✅ Scalar Types: Integer, Float, Boolean, Character
- ✅ Compound Types: Tuples, Arrays
- ✅ String และ String Slices
- ✅ Type Conversion และ Casting
- ✅ Numeric Operations
- ✅ Bitwise Operations

**ตัวอย่างโค้ด:**
- `examples/01-basics/03-data-types/`

**แบบฝึกหัด:**
1. สร้างโปรแกรมคำนวณทางคณิตศาสตร์
2. ทดลอง type conversion
3. ทำงานกับ tuples และ arrays

---

### สัปดาห์ที่ 3-4: Functions & Control Flow (Steps 101-150)
**เนื้อหา:**
- ✅ Function Declaration และ Parameters
- ✅ Return Values
- ✅ Expressions vs Statements
- ✅ if/else Expressions
- ✅ match Expressions
- ✅ Loops: loop, while, for
- ✅ Pattern Matching
- ✅ if let และ while let

**ตัวอย่างโค้ด:**
- `examples/01-basics/04-functions/`
- `examples/01-basics/05-control-flow/`

**แบบฝึกหัด:**
1. สร้างฟังก์ชันคำนวณ factorial
2. สร้างโปรแกรม FizzBuzz
3. ทำโปรแกรมตรวจสอบจำนวนเฉพาะ

---

### สัปดาห์ที่ 4-5: Ownership & Borrowing (Steps 151-200)
**เนื้อหา:**
- ✅ Ownership Rules
- ✅ Move Semantics
- ✅ Clone และ Copy
- ✅ References และ Borrowing
- ✅ Mutable References
- ✅ Slices
- ✅ Dangling References

**ตัวอย่างโค้ด:**
- `examples/02-intermediate/ownership_demo/`

**แบบฝึกหัด:**
1. ทดลอง ownership และ move
2. ใช้งาน references และ borrowing
3. สร้างฟังก์ชันที่ใช้ slices

---

### สัปดาห์ที่ 5: Structs & Enums (Steps 201-250)
**เนื้อหา:**
- ✅ Struct Definition
- ✅ Method Syntax
- ✅ Associated Functions
- ✅ Tuple Structs
- ✅ Enum Definition
- ✅ Pattern Matching with Enums
- ✅ Option Enum
- ✅ Match Control Flow

**ตัวอย่างโค้ด:**
- `examples/02-intermediate/structs_enums/`

**แบบฝึกหัด:**
1. สร้าง struct สำหรับ User
2. สร้าง enum สำหรับ Shape
3. ใช้ Option และ match

---

## 🎯 ระดับกลาง (Steps 251-500)

### สัปดาห์ที่ 6-7: Collections (Steps 251-300)
**เนื้อหา:**
- ✅ Vectors
- ✅ String Operations
- ✅ HashMap
- ✅ HashSet
- ✅ Iterators
- ✅ Closure Basics

**ตัวอย่างโค้ด:**
- `examples/02-intermediate/collections_demo/`

**แบบฝึกหัด:**
1. สร้างโปรแกรมจัดการ Todo List
2. ทำ word counter ด้วย HashMap
3. ใช้ iterators และ closures

---

### สัปดาห์ที่ 8-9: Error Handling (Steps 301-350)
**เนื้อหา:**
- ✅ panic! Macro
- ✅ Result Type
- ✅ unwrap และ expect
- ✅ Propagating Errors
- ✅ ? Operator
- ✅ Custom Error Types
- ✅ Error Handling Best Practices

**ตัวอย่างโค้ด:**
- `examples/02-intermediate/error_handling/`

**แบบฝึกหัด:**
1. สร้างโปรแกรมอ่านไฟล์พร้อม error handling
2. สร้าง custom error type
3. ใช้ ? operator

---

### สัปดาห์ที่ 9-10: Modules & Packages (Steps 351-400)
**เนื้อหา:**
- ✅ Module System
- ✅ Packages และ Crates
- ✅ use Keyword
- ✅ pub Visibility
- ✅ File Hierarchy
- ✅ External Crates
- ✅ Cargo.toml Configuration

**แบบฝึกหัด:**
1. สร้างโปรเจคแบบ multi-module
2. ใช้ external crates
3. จัดโครงสร้างโค้ดให้ดี

---

### สัปดาห์ที่ 10-11: Testing (Steps 401-450)
**เนื้อหา:**
- ✅ Unit Tests
- ✅ Integration Tests
- ✅ Test Organization
- ✅ assert! Macros
- ✅ Custom Test Messages
- ✅ Test Fixtures
- ✅ Benchmarking

**แบบฝึกหัด:**
1. เขียน unit tests
2. เขียน integration tests
3. ทำ test-driven development

---

### สัปดาห์ที่ 11-12: Generics & Traits (Steps 451-500)
**เนื้อหา:**
- ✅ Generic Functions
- ✅ Generic Structs
- ✅ Generic Enums
- ✅ Trait Definition
- ✅ Trait Implementation
- ✅ Trait Bounds
- ✅ Returning Traits
- ✅ Operator Overloading

**ตัวอย่างโค้ด:**
- `examples/02-intermediate/generics_traits/`

**แบบฝึกหัด:**
1. สร้าง generic collection
2. Implement custom traits
3. ใช้ trait bounds

---

## 🎯 ระดับสูง (Steps 501-750)

### สัปดาห์ที่ 13-14: Lifetimes (Steps 501-550)
**เนื้อหา:**
- ✅ Lifetime Syntax
- ✅ Lifetime Annotations
- ✅ Lifetime in Functions
- ✅ Lifetime in Structs
- ✅ Lifetime Elision
- ✅ Static Lifetime
- ✅ Advanced Lifetime Patterns

**แบบฝึกหัด:**
1. ทำงานกับ lifetime annotations
2. สร้าง struct ที่มี references
3. เข้าใจ lifetime elision rules

---

### สัปดาห์ที่ 15-16: Smart Pointers (Steps 551-600)
**เนื้อหา:**
- ✅ Box<T>
- ✅ Rc<T> - Reference Counting
- ✅ Arc<T> - Atomic Reference Counting
- ✅ RefCell<T>
- ✅ Cell<T>
- ✅ Deref Trait
- ✅ Drop Trait
- ✅ Interior Mutability Pattern

**แบบฝึกหัด:**
1. ใช้ Box สำหรับ recursive types
2. ทดลอง Rc และ Arc
3. ใช้ RefCell สำหรับ interior mutability

---

### สัปดาห์ที่ 17-18: Concurrency (Steps 601-650)
**เนื้อหา:**
- ✅ Threads
- ✅ Message Passing (Channels)
- ✅ Shared State Concurrency
- ✅ Mutex<T>
- ✅ Arc<Mutex<T>> Pattern
- ✅ Send และ Sync Traits
- ✅ Thread Safety

**แบบฝึกหัด:**
1. สร้าง multi-threaded application
2. ใช้ channels สำหรับ communication
3. ใช้ Mutex สำหรับ shared state

---

### สัปดาห์ที่ 19-20: Async Programming (Steps 651-700)
**เนื้อหา:**
- ✅ async/await Syntax
- ✅ Future Trait
- ✅ Tokio Runtime
- ✅ async fn
- ✅ .await Keyword
- ✅ async Blocks
- ✅ Stream และ Sink
- ✅ Error Handling in async

**แบบฝึกหัด:**
1. สร้าง async function
2. ใช้ Tokio runtime
3. ทำ concurrent async tasks

---

### สัปดาห์ที่ 20-21: Advanced Traits (Steps 701-750)
**เนื้อหา:**
- ✅ Associated Types
- ✅ Default Type Parameters
- ✅ Fully Qualified Syntax
- ✅ Supertraits
- ✅ Newtype Pattern
- ✅ Type Aliases
- ✅ Never Type
- ✅ Sized Trait

**แบบฝึกหัด:**
1. สร้าง complex trait hierarchies
2. ใช้ associated types
3. Implement advanced patterns

---

## 🎯 ระดับมืออาชีพ (Steps 751-1000)

### สัปดาห์ที่ 22-23: Macros (Steps 751-800)
**เนื้อหา:**
- ✅ Declarative Macros (macro_rules!)
- ✅ Procedural Macros
- ✅ Derive Macros
- ✅ Attribute Macros
- ✅ Function-like Macros
- ✅ Macro Patterns
- ✅ Hygiene

**แบบฝึกหัด:**
1. สร้าง declarative macro
2. เขียน derive macro
3. Implement attribute macro

---

### สัปดาห์ที่ 24-25: Unsafe Rust (Steps 801-850)
**เนื้อหา:**
- ✅ unsafe Keyword
- ✅ Raw Pointers
- ✅ unsafe Functions
- ✅ unsafe Traits
- ✅ FFI (Foreign Function Interface)
- ✅ Calling C from Rust
- ✅ Safety Guarantees

**แบบฝึกหัด:**
1. ใช้ unsafe code อย่างปลอดภัย
2. Wrap C library
3. ทำ FFI bindings

---

### สัปดาห์ที่ 26-27: Design Patterns (Steps 851-900)
**เนื้อหา:**
- ✅ Builder Pattern
- ✅ Strategy Pattern
- ✅ State Pattern
- ✅ Visitor Pattern
- ✅ Command Pattern
- ✅ Observer Pattern
- ✅ Dependency Injection

**แบบฝึกหัด:**
1. Implement design patterns
2. Refactor code using patterns
3. Create reusable components

---

### สัปดาห์ที่ 28-29: Performance & Production (Steps 901-1000)
**เนื้อหา:**
- ✅ Profiling และ Benchmarking
- ✅ Memory Optimization
- ✅ CPU Optimization
- ✅ Compile-time Optimization
- ✅ Zero-cost Abstractions
- ✅ Production Best Practices
- ✅ Deployment Strategies
- ✅ Monitoring และ Logging
- ✅ Security Best Practices
- ✅ Cross-platform Development

**โปรเจคจริง:**
- CLI Applications
- Web Servers (Actix/Rocket)
- REST APIs
- Database Applications
- Game Development
- System Programming

---

## 📊 แผนการเรียนแนะนำ

### แบบเข้มข้น (Intensive - 3-4 เดือน)
- **เวลา:** 6-8 ชั่วโมง/วัน, 6 วัน/สัปดาห์
- **เหมาะสำหรับ:** ผู้ที่มีเวลาเต็มที่และมีพื้นฐานโปรแกรม

### แบบปกติ (Regular - 6-7 เดือน)
- **เวลา:** 3-4 ชั่วโมง/วัน, 5 วัน/สัปดาห์
- **เหมาะสำหรับ:** ผู้ที่เรียนควบคู่กับทำงาน

### แบบผ่อนคลาย (Relaxed - 10-12 เดือน)
- **เวลา:** 1-2 ชั่วโมง/วัน, 4-5 วัน/สัปดาห์
- **เหมาะสำหรับ:** ผู้เริ่มต้นหรือเรียนเป็นงานอดิเรก

---

## 🎓 วิธีการเรียนที่มีประสิทธิภาพ

### 1. **อ่านทฤษฎี** (20%)
- อ่านคู่มือในแต่ละ Step
- ทำความเข้าใจแนวคิด
- จดบันทึกส่วนสำคัญ

### 2. **ดูโค้ดตัวอย่าง** (20%)
- ศึกษาโค้ดและ comments ละเอียด
- เข้าใจทำไมเขียนแบบนี้
- ดู pattern ที่ใช้

### 3. **พิมพ์โค้ดเอง** (30%)
- **อย่า copy-paste!**
- พิมพ์เองทุกบรรทัด
- ช่วยจำและเข้าใจลึกขึ้น

### 4. **ทดลองแก้ไข** (20%)
- เปลี่ยนค่าต่างๆ ดูผลลัพธ์
- ลองทำให้ compile error
- แก้ไข error ด้วยตัวเอง

### 5. **ทำโปรเจคเอง** (10%)
- สร้างโปรเจคเล็กๆ เอง
- Apply ความรู้ที่ได้เรียน
- แชร์โค้ดกับผู้อื่น

---

## ✅ Checklist การเรียนรู้

### ระดับพื้นฐาน
- [ ] รู้จักและใช้งาน Cargo
- [ ] เข้าใจ Ownership และ Borrowing
- [ ] ใช้ Structs และ Enums ได้
- [ ] เข้าใจ Pattern Matching
- [ ] ทำ Error Handling พื้นฐาน
- [ ] สร้างโปรเจคเล็กๆ ได้

### ระดับกลาง
- [ ] ใช้ Collections ได้คล่อง
- [ ] จัดการ Errors อย่างมืออาชีพ
- [ ] เข้าใจ Generics และ Traits
- [ ] ใช้ Lifetimes ได้
- [ ] จัดโครงสร้างโค้ดด้วย Modules
- [ ] เขียน Tests

### ระดับสูง
- [ ] ใช้ Smart Pointers
- [ ] เขียน Concurrent Code
- [ ] ใช้ async/await
- [ ] Implement Advanced Traits
- [ ] เข้าใจ Advanced Patterns

### ระดับมืออาชีพ
- [ ] เขียน Macros
- [ ] ใช้ Unsafe Rust อย่างปลอดภัย
- [ ] Apply Design Patterns
- [ ] Optimize Performance
- [ ] สร้าง Production-ready Applications

---

## 📚 ทรัพยากรเพิ่มเติม

### Official Resources
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)
- [Rust Playground](https://play.rust-lang.org/)

### Community
- [Rust Discord](https://discord.gg/rust-lang)
- [r/rust](https://reddit.com/r/rust)
- [Rust Users Forum](https://users.rust-lang.org/)
- [This Week in Rust](https://this-week-in-rust.org/)

### Advanced
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Async Book](https://rust-lang.github.io/async-book/)

---

## 🎯 เป้าหมายหลังจบคอร์ส

หลังจากเรียนครบ 1000 ขั้นตอน คุณจะสามารถ:

✅ เขียน Rust อย่างมั่นใจ
✅ สร้าง CLI Tools
✅ พัฒนา Web Services
✅ ทำ Systems Programming
✅ เข้าใจ Memory Management
✅ เขียนโค้ดที่ปลอดภัยและรวดเร็ว
✅ Contribute ใน Open Source Projects
✅ สอนผู้อื่นได้

---

## 💡 Tips สำหรับความสำเร็จ

1. **ฝึกบ่อยๆ** - ยิ่งเขียนมากยิ่งเก่ง
2. **อย่าเร่งรีบ** - ใช้เวลาให้เข้าใจลึก
3. **อ่าน Compiler Errors** - Rust compiler ช่วยมาก
4. **ถามคำถาม** - Community ช่วยเหลือดี
5. **สร้างโปรเจคจริง** - ลงมือทำของจริง
6. **Review โค้ดเดิม** - กลับมาดูโค้ดเก่าๆ
7. **เข้า Community** - เรียนรู้จากผู้อื่น

---

**🦀 Happy Learning Rust!**

*ขอให้คุณประสบความสำเร็จในการเรียนรู้ Rust!*
