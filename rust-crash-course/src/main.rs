// Enable strict linting - all clippy warnings become compilation errors
#![deny(clippy::all)]
// T must implement Display
fn print_it<T: std::fmt::Display>(value: T) {
    println!("Value: {}", value);
}

// T must implement Clone
fn duplicate<T: Clone>(value: T) -> (T, T) {
    (value.clone(), value)
}

fn main() {
    print_it(42);
    print_it("hello");
    
    let s = String::from("hello");
    let (s1, s2) = duplicate(&s);
    println!("{}, {}, {}", s1, s2,s);
}