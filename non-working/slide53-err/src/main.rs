enum Message {
    Move { x: i32, y: i32 },
    Write(String),
    Quit,
}

fn process_message(msg: Message) {
    match msg {
        Message::Move { x, y } => println!("Движение в точку ({}, {})", x, y),
        Message::Write(text) => println!("Сообщение: {}", text),
        // Message::Quit => println!("Завершение программы"),
    }
}

fn main() {
    let msg = Message::Move { x: 10, y: 20 };
    process_message(msg);

    let msg = Message::Write("Я добрался до точки!".to_string());
    process_message(msg);

    let msg = Message::Quit;
    process_message(msg);
}