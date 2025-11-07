// =====================================================
// CLI Tool Example - โปรเจคตัวอย่าง Command Line Tool
// =====================================================
// เครื่องมือ CLI สำหรับจัดการ Todo List
//
// Features:
// - Add todo item
// - List all todos
// - Mark todo as done
// - Remove todo
// - Save/Load from file
// =====================================================

use std::fs;
use std::io::{self, Write};

/// โครงสร้างข้อมูลสำหรับ Todo item
#[derive(Debug, Clone)]
struct Todo {
    id: usize,
    title: String,
    completed: bool,
}

impl Todo {
    /// สร้าง Todo ใหม่
    fn new(id: usize, title: String) -> Self {
        Todo {
            id,
            title,
            completed: false,
        }
    }

    /// แสดงข้อมูล Todo
    fn display(&self) {
        let status = if self.completed { "✓" } else { " " };
        println!("[{}] {}. {}", status, self.id, self.title);
    }
}

/// โครงสร้างสำหรับจัดการ Todo List
struct TodoList {
    todos: Vec<Todo>,
    next_id: usize,
}

impl TodoList {
    /// สร้าง TodoList ใหม่
    fn new() -> Self {
        TodoList {
            todos: Vec::new(),
            next_id: 1,
        }
    }

    /// เพิ่ม Todo ใหม่
    fn add(&mut self, title: String) {
        let todo = Todo::new(self.next_id, title);
        self.todos.push(todo);
        self.next_id += 1;
        println!("✅ เพิ่ม todo สำเร็จ!");
    }

    /// แสดงรายการ Todo ทั้งหมด
    fn list(&self) {
        if self.todos.is_empty() {
            println!("📝 ไม่มี todo ในรายการ");
            return;
        }

        println!("\n📝 Todo List:");
        println!("─────────────────────────────");
        for todo in &self.todos {
            todo.display();
        }
        println!("─────────────────────────────");
        let completed = self.todos.iter().filter(|t| t.completed).count();
        let total = self.todos.len();
        println!("สำเร็จแล้ว: {}/{}", completed, total);
    }

    /// ทำเครื่องหมาย Todo ว่าเสร็จแล้ว
    fn complete(&mut self, id: usize) {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.completed = true;
            println!("✅ ทำเครื่องหมาย todo #{} เป็นเสร็จแล้ว", id);
        } else {
            println!("❌ ไม่พบ todo #{}", id);
        }
    }

    /// ลบ Todo
    fn remove(&mut self, id: usize) {
        let len_before = self.todos.len();
        self.todos.retain(|t| t.id != id);

        if self.todos.len() < len_before {
            println!("🗑️  ลบ todo #{} สำเร็จ", id);
        } else {
            println!("❌ ไม่พบ todo #{}", id);
        }
    }

    /// บันทึกลงไฟล์
    fn save(&self, filename: &str) -> io::Result<()> {
        let mut content = String::new();
        for todo in &self.todos {
            let line = format!("{}|{}|{}\n",
                             todo.id,
                             if todo.completed { "1" } else { "0" },
                             todo.title);
            content.push_str(&line);
        }

        fs::write(filename, content)?;
        println!("💾 บันทึกลงไฟล์ {} สำเร็จ", filename);
        Ok(())
    }

    /// โหลดจากไฟล์
    fn load(&mut self, filename: &str) -> io::Result<()> {
        let content = fs::read_to_string(filename)?;

        self.todos.clear();
        self.next_id = 1;

        for line in content.lines() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 3 {
                if let Ok(id) = parts[0].parse::<usize>() {
                    let completed = parts[1] == "1";
                    let title = parts[2].to_string();

                    self.todos.push(Todo {
                        id,
                        title,
                        completed,
                    });

                    if id >= self.next_id {
                        self.next_id = id + 1;
                    }
                }
            }
        }

        println!("📂 โหลดจากไฟล์ {} สำเร็จ ({} รายการ)",
                 filename, self.todos.len());
        Ok(())
    }
}

/// แสดงเมนูหลัก
fn show_menu() {
    println!("\n╔══════════════════════════════╗");
    println!("║   🦀 Rust Todo CLI Tool     ║");
    println!("╚══════════════════════════════╝");
    println!("1. เพิ่ม todo");
    println!("2. แสดงรายการ");
    println!("3. ทำเครื่องหมายเสร็จแล้ว");
    println!("4. ลบ todo");
    println!("5. บันทึกลงไฟล์");
    println!("6. โหลดจากไฟล์");
    println!("0. ออก");
    print!("\nเลือกคำสั่ง: ");
    io::stdout().flush().unwrap();
}

/// อ่านข้อมูลจากผู้ใช้
fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

/// ฟังก์ชันหลัก
fn main() {
    let mut todo_list = TodoList::new();
    let default_file = "todos.txt";

    // พยายามโหลดข้อมูลเดิม
    let _ = todo_list.load(default_file);

    loop {
        show_menu();
        let choice = read_input();

        match choice.as_str() {
            "1" => {
                // เพิ่ม todo
                print!("📝 ชื่อ todo: ");
                io::stdout().flush().unwrap();
                let title = read_input();

                if !title.is_empty() {
                    todo_list.add(title);
                } else {
                    println!("❌ กรุณาระบุชื่อ todo");
                }
            }
            "2" => {
                // แสดงรายการ
                todo_list.list();
            }
            "3" => {
                // ทำเครื่องหมายเสร็จแล้ว
                print!("🎯 ระบุ ID ที่ต้องการทำเครื่องหมาย: ");
                io::stdout().flush().unwrap();
                let id_str = read_input();

                if let Ok(id) = id_str.parse::<usize>() {
                    todo_list.complete(id);
                } else {
                    println!("❌ ID ไม่ถูกต้อง");
                }
            }
            "4" => {
                // ลบ todo
                print!("🗑️  ระบุ ID ที่ต้องการลบ: ");
                io::stdout().flush().unwrap();
                let id_str = read_input();

                if let Ok(id) = id_str.parse::<usize>() {
                    todo_list.remove(id);
                } else {
                    println!("❌ ID ไม่ถูกต้อง");
                }
            }
            "5" => {
                // บันทึกลงไฟล์
                if let Err(e) = todo_list.save(default_file) {
                    println!("❌ เกิดข้อผิดพลาดในการบันทึก: {}", e);
                }
            }
            "6" => {
                // โหลดจากไฟล์
                if let Err(e) = todo_list.load(default_file) {
                    println!("❌ เกิดข้อผิดพลาดในการโหลด: {}", e);
                }
            }
            "0" => {
                // ออกจากโปรแกรม
                println!("👋 ขอบคุณที่ใช้งาน Rust Todo CLI!");

                // บันทึกอัตโนมัติก่อนออก
                let _ = todo_list.save(default_file);
                break;
            }
            _ => {
                println!("❌ คำสั่งไม่ถูกต้อง กรุณาเลือกใหม่");
            }
        }

        // หยุดรอผู้ใช้กด Enter
        println!("\nกด Enter เพื่อดำเนินการต่อ...");
        read_input();
    }
}
