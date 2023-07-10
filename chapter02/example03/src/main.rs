// Very shortened version of previous example.
#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.trim().to_string(),
            greeting : greeting.to_string(),
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn get_name() -> String {
    println!("What is your name?");
    let mut name = String::new();

    // Here by using reference, we are letting read_line borrow name.
    // By using &mut, the borrowing function can mutate your variable.
    stdin()
        .read_line(&mut name)
        .expect("Failed to read line.");

    // Note: no semicolon after the returned value.
    // Shorthand for return.
    name.trim().to_string()
}

// Here, we invoke a lambda to find the name. Note that the name has already been trimmed
// by get_name.
fn find_visitor<'a>(name: &str, visitor_list: &'a [Visitor]) -> Option<&'a Visitor> {
    visitor_list
        .iter()
        .find(|visitor| name.to_lowercase() == visitor.name.to_lowercase())
}

fn main() {
    // An array of size 3 of Visitor.
    let visitor_list = [
        Visitor::new("Sebastian", "Hello, Magnificent Overlord!"),
        Visitor::new("Andrew", "Hello, boy of much smol."),
        Visitor::new("Duncan", "Meow meow meow meow meow meow."),
    ];

    find_visitor(get_name().as_str(), &visitor_list).map_or_else(
        || {
            println!("You are not on the list.");
        },
        |visitor| {
        visitor.greet_visitor();
    });
}
