#[derive(Clone)]
struct User {
    name: String,
    age: u32,
}

impl User {
    // Метод для приветствия
    fn greet(&self) {
        println!("Hello, my name is {}", self.name);
    }

    // Метод для увеличения возраста
    fn get_older(&mut self) {
        self.age += 1;
        println!("I'm now {} years old", self.age);
    }
}

fn main() {
    let mut user = User {
        name: String::from("Alice"),
        age: 30,
    };

    user.greet();
    user.get_older();
}