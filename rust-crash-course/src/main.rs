// Enable strict linting - all clippy warnings become compilation errors
#![deny(clippy::all)]
// T must implement Display
mod math;  // Tells Rust to look for math.rs

fn main() {
    println!("2 + 3 = {}", math::add(2, 3));
    println!("5 - 2 = {}", math::subtract(5, 2));
}