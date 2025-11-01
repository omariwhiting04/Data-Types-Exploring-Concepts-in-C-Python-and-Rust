fn grade_category(g: i32) -> &'static str {
    if g < 50 {
        "Fail"
    } else if g < 70 {
        "Pass"
    } else if g < 80 {
        "Credit"
    } else if g < 90 {
        "Distinction"
    } else {
        "High Distinction"
    }
}

fn main() {
    let grades = [45, 67, 72, 85, 93, 58, 79, 88, 100];
    let (mut fail, mut pass, mut credit, mut dist, mut high) = (0, 0, 0, 0, 0);

    for g in grades {
        let cat = grade_category(g);
        println!("{} -> {}", g, cat);
        match cat {
            "Fail" => fail += 1,
            "Pass" => pass += 1,
            "Credit" => credit += 1,
            "Distinction" => dist += 1,
            _ => high += 1,
        }
    }
    println!("Summary: F={} P={} C={} D={} HD={}", fail, pass, credit, dist, high);
}
