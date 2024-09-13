fn main() {
    let s1 = String::from("Привет");
    let len = calculate_length(&s1); // попытка передачи по ссылке (заимствование)

    println!("Длина строки '{}': {}", s1, len); // s1 недоступен
}

fn calculate_length(s: String) -> usize {
    s.len()
}