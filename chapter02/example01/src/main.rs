// A treehouse example.

#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;
use once_cell::sync::Lazy;
use std::collections::HashSet;


// Static members: only initialize once.
// static TREEHOUSE_MEMBERS: Lazy<HashSet<String>> = Lazy::new(|| {
//     let mut set = HashSet::new();
//     set.insert("sebastian".to_string());
//     set.insert("セバスチャン".to_string());
//     set.insert("andrew".to_string());
//     set.insert("アンドリュー".to_string());
//     set
// });

static TREEHOUSE_MEMBERS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut set = HashSet::new();
    set.insert("sebastian");
    set.insert("セバスチャン");
    set.insert("andrew");
    set.insert("アンドリュー");
    set
});


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

fn check_name(name: &str) -> bool {
    //TREEHOUSE_MEMBERS.contains(&name.to_lowercase()) || TREEHOUSE_MEMBERS.contains(name)
    TREEHOUSE_MEMBERS.contains(name.to_lowercase().as_str()) || TREEHOUSE_MEMBERS.contains(name)
}

fn main() {
    println!("お名前です？");
    let name = get_name();

    if check_name(&name) {
        println!("入ってください、{name}。");
    } else {
        println!("もう去ってください、{name}。");
    }

    // Debug: prints the string in name in quotes with all characters, e.g. \n.
    // println!("こんにちは、{:?}。", name);
}
