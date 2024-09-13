fn main() {
    let s1 = String::from("Привет");
    let len = calculate_length(s1); // передача без заимствования

    println!("Длина строки '{}': {}", s1, len); // s1 недоступен, т.к. использован
}

fn calculate_length(s: &String) -> usize {
    s.len()
}