enum Day { Monday, Tuesday, Wednesday }
fn main() {
    let d = Day::Tuesday;
    match d { 
        Day::Monday => println!("Monday"), 
        Day::Tuesday => println!("Tuesday"), 
        Day::Wednesday => println!("Wednesday") 
    }
}
