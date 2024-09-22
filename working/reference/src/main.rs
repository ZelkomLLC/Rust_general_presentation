#[allow(unused_variables, unused_assignments)]
fn main() {
    let r;
    {
        let x = 5;
        r = &x; // временная ссылка
    } // x уничтожается, r будет недействительна

    // println!("{}", r); // Ошибка из-за времени жизни ссылки
}