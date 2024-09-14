#[derive(Clone)]
struct User {
    name: String,
    age: u32,
}

#[derive(Clone)]
struct Admin {
    name: String,
}

// Трейт для приветствия
trait Greet {
    fn greet(&self);

    fn say_num(&self, num: i32) {
        println!("Number is: {}", num);
    }
}

// Реализация метода для User
impl User {
    fn get_older(&mut self) {
        self.age += 1;
        println!("I'm now {} years old", self.age);
    }
}

// Реализация трейта Greet для User
impl Greet for User {
    fn greet(&self) {
        println!("Hello, my name is {}", self.name);
    }
}

// Реализация трейта Greet для Admin
impl Greet for Admin {
    fn greet(&self) {
        println!("Hello, Admin {}!", self.name);
    }
}

fn introduce(greeter: &impl Greet, num: i32) {
    greeter.greet();
    greeter.say_num(num);
}

fn introduce_gen<T: Greet + Clone>(greeter: &T, num: i32) {
    greeter.greet();
    greeter.say_num(num);
}

fn main() {
    let mut user = User {
        name: String::from("Alice"),
        age: 30,
    };

    let admin = Admin {
        name: String::from("Bob"),
    };

    introduce(&user, 200);
    introduce(&admin, 42);

    introduce_gen(&user, 100500);
    introduce_gen(&admin, 75);

    user.get_older();
}
