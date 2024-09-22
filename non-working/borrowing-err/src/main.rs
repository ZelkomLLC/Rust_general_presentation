fn main() {
    let mut s1 = String::from("Hello");

    // Неизменяемое заимствование
    let s2 = &s1; // s2 заимствует s1, но не владеет ею
    println!("s1: {}, s2: {}", s1, s2); // Мы можем использовать и s1, и s2

    change_string(s1); // Мы пытеамсы изменить s1 через заимствование

    println!("s1 after changing: {}", s1);
}

fn change_string(s: &mut String) {
    s.push_str(" world!"); // Добавляем к строке новое значение
}
