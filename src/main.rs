extern crate phrases;

use phrases::english::{ greetings, farewells};
use phrases::japanese;

fn main() {
    println!("Hello, world!");
    println!("Hello English: {}", greetings::hello());
    println!("Goodbye English: {}", farewells::goodbye());
    println!(
        "Hello Japanese: {}",
        japanese::goodbye()
    );
    println!(
        "Goodbye Japanese: {}",
        japanese::goodbye()
    );
}
