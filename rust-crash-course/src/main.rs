// Enable strict linting - all clippy warnings become compilation errors
#![deny(clippy::all)]
fn main() {
    let mut full_name = String::from("Alice");
    
    // Append to string (needs mut)
    full_name.push_str(" ");
    full_name.push_str("Johnson");
    
    println!("Full name: {}", full_name);
    
    // If you want to keep using it, borrow it:
    display_name(&full_name);
    display_name(&full_name);  // Can borrow multiple times!
}

fn display_name(name: &String) {
    println!("👤 {}", name);
}