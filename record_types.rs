struct Student { name: String, id: i32 }
fn main() {
    let s = Student { name: String::from("Alice"), id: 1 };
    println!("{} {}", s.name, s.id);
}
