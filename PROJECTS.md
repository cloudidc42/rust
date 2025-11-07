# 🚀 Rust Projects Guide
# คู่มือโปรเจคตัวอย่าง Rust

---

## 📚 โปรเจคตัวอย่างทั้งหมด

โครงการนี้มีตัวอย่างโปรเจคตั้งแต่พื้นฐานจนถึงระดับมืออาชีพ ทุกโปรเจคใช้งานได้จริง 100%

---

## 📁 โครงสร้างโปรเจค

```
rust/
├── examples/           # ตัวอย่างโค้ดแบ่งตามระดับ
│   ├── 01-basics/     # ระดับพื้นฐาน
│   ├── 02-intermediate/  # ระดับกลาง
│   ├── 03-advanced/      # ระดับสูง
│   └── 04-professional/  # ระดับมืออาชีพ
└── projects/          # โปรเจคที่สมบูรณ์
    ├── cli_tool/      # Command Line Tool
    ├── web_server/    # Web Server
    ├── rest_api/      # REST API
    └── more...
```

---

## 🎯 ระดับพื้นฐาน (Basic Level)

### 1. Hello World
**Path:** `examples/01-basics/01-hello-world/`

**Features:**
- พิมพ์ข้อความ
- การใช้ format strings
- Debug printing
- Comments

**วิธีรัน:**
```bash
cd examples/01-basics/01-hello-world
cargo run
```

**เรียนรู้:**
- ✅ Syntax พื้นฐาน
- ✅ println! macro
- ✅ Format specifiers

---

### 2. Variables
**Path:** `examples/01-basics/02-variables/`

**Features:**
- Immutable และ mutable variables
- Shadowing
- Constants
- Type annotations

**วิธีรัน:**
```bash
cd examples/01-basics/02-variables
cargo run
```

**เรียนรู้:**
- ✅ Variable declaration
- ✅ Mutability
- ✅ Type inference

---

### 3. Data Types
**Path:** `examples/01-basics/03-data-types/`

**Features:**
- Integer types
- Floating-point
- Boolean และ Character
- Tuples และ Arrays
- Type conversion

**วิธีรัน:**
```bash
cd examples/01-basics/03-data-types
cargo run
```

**เรียนรู้:**
- ✅ Scalar types
- ✅ Compound types
- ✅ Type casting

---

### 4. Functions
**Path:** `examples/01-basics/04-functions/`

**Features:**
- Function declaration
- Parameters และ return values
- Recursion
- Higher-order functions
- Closures

**วิธีรัน:**
```bash
cd examples/01-basics/04-functions
cargo run
```

**เรียนรู้:**
- ✅ Function syntax
- ✅ Return values
- ✅ Closures

---

### 5. Control Flow
**Path:** `examples/01-basics/05-control-flow/`

**Features:**
- if/else expressions
- match expressions
- Loops (loop, while, for)
- Pattern matching
- FizzBuzz example

**วิธีรัน:**
```bash
cd examples/01-basics/05-control-flow
cargo run
```

**เรียนรู้:**
- ✅ Conditional logic
- ✅ Pattern matching
- ✅ Loop types

---

## 🎯 ระดับกลาง (Intermediate Level)

### 6. Ownership Demo
**Path:** `examples/02-intermediate/ownership_demo/`

**Features:**
- Ownership basics
- Move vs Copy
- References และ Borrowing
- Mutable references
- Slices
- Text analyzer example

**วิธีรัน:**
```bash
cd examples/02-intermediate/ownership_demo
cargo run
```

**เรียนรู้:**
- ✅ Ownership rules
- ✅ Borrowing
- ✅ Lifetimes basics

---

### 7. Structs & Enums
**Path:** `examples/02-intermediate/structs_enums/`

**Features:**
- Struct definition
- Methods
- Enum variants
- Pattern matching
- Option และ Result

**วิธีรัน:**
```bash
cd examples/02-intermediate/structs_enums
cargo run
```

**เรียนรู้:**
- ✅ Custom types
- ✅ Methods
- ✅ Enums

---

### 8. Collections Demo
**Path:** `examples/02-intermediate/collections_demo/`

**Features:**
- Vectors
- Strings
- HashMap
- Iterators
- Practical examples

**วิธีรัน:**
```bash
cd examples/02-intermediate/collections_demo
cargo run
```

**เรียนรู้:**
- ✅ Vec<T>
- ✅ HashMap<K,V>
- ✅ Iterators

---

### 9. Error Handling
**Path:** `examples/02-intermediate/error_handling/`

**Features:**
- Result type
- Option type
- ? operator
- Custom errors
- Best practices

**วิธีรัน:**
```bash
cd examples/02-intermediate/error_handling
cargo run
```

**เรียนรู้:**
- ✅ Error handling
- ✅ Result และ Option
- ✅ Custom error types

---

### 10. Generics & Traits
**Path:** `examples/02-intermediate/generics_traits/`

**Features:**
- Generic functions
- Generic structs
- Trait definition
- Trait bounds
- Practical examples

**วิธีรัน:**
```bash
cd examples/02-intermediate/generics_traits
cargo run
```

**เรียนรู้:**
- ✅ Generics
- ✅ Traits
- ✅ Trait bounds

---

## 🚀 โปรเจคจริง (Complete Projects)

### 1. CLI Todo Tool
**Path:** `projects/cli_tool/`

**Description:**
เครื่องมือจัดการ Todo List แบบ Command Line Interface

**Features:**
- ✅ Add/List/Complete/Delete todos
- ✅ Save/Load from file
- ✅ Interactive menu
- ✅ Error handling
- ✅ File I/O

**วิธีรัน:**
```bash
cd projects/cli_tool
cargo run
```

**เทคโนโลยี:**
- Structs และ Enums
- File I/O
- Error handling
- User input
- Pattern matching

**Use Cases:**
- Task management
- Learning Rust basics
- CLI development

---

### 2. Web Server (Coming Soon)
**Path:** `projects/web_server/`

**Description:**
Simple HTTP web server

**Features:**
- HTTP request handling
- Static file serving
- Routing
- Middleware
- Template rendering

**เทคโนโลยี:**
- TCP/IP
- HTTP protocol
- Concurrency
- Async/await

---

### 3. REST API (Planned)
**Description:**
RESTful API with database

**Features:**
- CRUD operations
- Database integration
- Authentication
- API documentation
- Testing

**เทคโนโลยี:**
- Actix/Rocket
- SQLx/Diesel
- JWT
- Serde (JSON)

---

### 4. Database Application (Planned)
**Description:**
Application with database operations

**Features:**
- SQL database
- Migrations
- Query builder
- Connection pooling
- Transactions

**เทคโนโลยี:**
- PostgreSQL/MySQL
- Diesel ORM
- Migration tools

---

### 5. Game Engine (Planned)
**Description:**
Simple 2D game engine

**Features:**
- Entity-Component System
- Rendering
- Input handling
- Physics (basic)
- Asset management

**เทคโนโลยี:**
- Graphics (SDL2/ggez)
- Game loop
- ECS pattern

---

## 📋 การใช้งานโปรเจค

### สำหรับผู้เริ่มต้น

1. **เริ่มจาก Basics**
   ```bash
   cd examples/01-basics/01-hello-world
   cargo run
   ```

2. **อ่านโค้ดละเอียด**
   - ดู comments
   - เข้าใจทุกบรรทัด
   - ลองแก้ไข

3. **ทดลองเอง**
   - สร้าง variant ของโปรเจค
   - เพิ่ม features
   - แก้ bugs

### สำหรับระดับกลาง

1. **ศึกษา Ownership**
   ```bash
   cd examples/02-intermediate/ownership_demo
   cargo run
   ```

2. **ทำโปรเจคจริง**
   ```bash
   cd projects/cli_tool
   cargo run
   ```

3. **ปรับปรุง**
   - เพิ่ม features
   - Refactor code
   - เขียน tests

---

## 🎯 แนวทางการเรียนรู้

### Step 1: อ่านและเข้าใจ
- อ่านโค้ดทั้งหมด
- ทำความเข้าใจ logic
- ดู comments อธิบาย

### Step 2: รันและทดสอบ
```bash
cargo run
cargo test
cargo build --release
```

### Step 3: แก้ไขและทดลอง
- เปลี่ยนค่าต่างๆ
- เพิ่ม features
- ลอง break และ fix

### Step 4: สร้างของตัวเอง
- Clone โปรเจค
- Customize
- สร้าง features ใหม่

---

## 💡 Tips สำหรับการเรียนรู้

### 1. เริ่มจากง่ายไปยาก
- อย่าข้ามขั้นตอน
- ทำความเข้าใจพื้นฐานให้ดี
- ค่อยๆ เพิ่มความซับซ้อน

### 2. อ่าน Compiler Errors
- Error messages ของ Rust ดีมาก
- มีคำแนะนำให้
- เรียนรู้จาก errors

### 3. ใช้ Rust Tools
```bash
cargo fmt      # Format code
cargo clippy   # Lint code
cargo doc      # Generate docs
cargo check    # Quick check
```

### 4. ทดลองใน Playground
- [play.rust-lang.org](https://play.rust-lang.org/)
- ทดสอบไอเดียได้เร็ว
- แชร์โค้ดได้ง่าย

---

## 🔧 การ Customize โปรเจค

### CLI Todo Tool - Ideas
- [ ] Add priorities (High/Medium/Low)
- [ ] Add due dates
- [ ] Add categories
- [ ] Add search functionality
- [ ] Add export to CSV
- [ ] Add colored output
- [ ] Add undo/redo

### Web Server - Ideas
- [ ] Add routing
- [ ] Add templates
- [ ] Add database
- [ ] Add authentication
- [ ] Add logging
- [ ] Add middleware

---

## 📚 Resources

### Official Docs
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)

### Crates
- [crates.io](https://crates.io/) - Package registry
- [docs.rs](https://docs.rs/) - Documentation

### Tools
- [rustup](https://rustup.rs/) - Toolchain manager
- [cargo-edit](https://github.com/killercup/cargo-edit) - Manage dependencies

---

## ✅ Checklist

### พื้นฐาน
- [ ] Hello World
- [ ] Variables
- [ ] Data Types
- [ ] Functions
- [ ] Control Flow

### กลาง
- [ ] Ownership
- [ ] Structs & Enums
- [ ] Collections
- [ ] Error Handling
- [ ] Generics & Traits

### โปรเจคจริง
- [ ] CLI Tool (Todo)
- [ ] Web Server
- [ ] REST API
- [ ] Database App

---

**🦀 Happy Coding!**

*สร้างโปรเจคใหม่ๆ และสนุกกับการเรียนรู้ Rust!*
