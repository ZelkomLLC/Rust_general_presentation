fn main() {
    let numbers = [1, 2, 3, 4];

    match numbers.get(0) {
        Some(&number) => println!("Найдено: {}", number),
        None => println!("Элемент не найден"),
    }

    match numbers.get(100) {
        Some(&number) => println!("Найдено: {}", number),
        None => println!("Элемент не найден"),
    }
}