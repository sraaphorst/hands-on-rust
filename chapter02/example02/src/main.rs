#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String,
}

// STRING NOTES:
// 1. A String is a growable, mutable, heap-allocated structure.
// 2. &str is an string slice: immutable view into an already existing String.
// Function implementations for a structure.
impl Visitor {
    // Constructor.
    // This is an associated function since param list does not contain self.
    // Thus, it must be called Visitor::new().
    // Self is shorthand for Visitor here.
    fn new(name: &str, greeting: &str) -> Self {
        // No semicolon implies a return.
        Self {
            // Note that:
            // 1. trim() provides a view into existing &str or String;
            // 2. to_lowercase() must create a new String.
            // Here we want the name to represent the official name of the Visitor, so we
            // leave the case as created.
            name: name.trim().to_string(),
            greeting : greeting.trim().to_string(),
        }
    }

    // methods with &self have access to the instance contents.
    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn get_name() -> String {
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

    println!("What is your name?");
    let name = get_name();

    // Look up the visitor.
    let optional_visitor = find_visitor(name.as_str(), &visitor_list);
    optional_visitor.map_or_else(
        || {
            println!("You are not on the list.");
        },
        |visitor| {
        visitor.greet_visitor();
    });
}
