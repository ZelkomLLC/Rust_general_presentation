fn main() {
    let s1 = String::from("Привет");
    let len = calculate_length(&s1); // передача по ссылке (заимствование)

    println!("Длина строки '{}': {}", s1, len); // s1 доступен, т.к. заимствован
}

fn calculate_length(s: &String) -> usize {
    s.len() // доступ к данным через ссылку
}