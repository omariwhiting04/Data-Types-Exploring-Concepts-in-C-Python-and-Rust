fn main() {
    enum Number { I(i32), F(f32) }
    let n1 = Number::I(5);
    let n2 = Number::F(3.14);
    match n1 { Number::I(i)=>println!("{}",i), Number::F(f)=>println!("{}",f) }
    match n2 { Number::I(i)=>println!("{}",i), Number::F(f)=>println!("{}",f) }
}
