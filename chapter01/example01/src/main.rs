#![warn(clippy::all, clippy::pedantic)]
#![allow(mixed_script_confusables)]
#![allow(non_ascii_idents)]
#![allow(unused_assignments)]

fn main() {
    println!("みんなさん、こんにちは。初めまして。セバスチャンです。よろしくお願いします。");
    println!("こんにちは世界。");
    let π = std::f64::consts::PI;
    let θ = f64::sin(2.0 * π);
    let おやすみ = "Good Night";
    println!("{θ}");
    println!("{おやすみ}");
}
