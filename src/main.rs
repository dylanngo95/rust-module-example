extern crate phrases;

fn main() {
    println!("Hello, world!");
    println!("Hello English: {}", phrases::english::greetings::hello());
    println!("Goodbye English: {}", phrases::english::farewells::goodbye());
    println!("Hello Japanese: {}", phrases::japanese::farewells::goodbye());
    println!("Goodbye Japanese: {}", phrases::japanese::farewells::goodbye());
}
