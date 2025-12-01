// Enable strict linting - all clippy warnings become compilation errors
#![deny(clippy::all)]
struct User {
    name: String,
    age: u32,
}

fn find_user<'a>(users: &'a Vec<User>, name: &str) -> Option<&'a User> {
    for user in users {
        if user.name == name {
            return Some(user);
        }
    }
    None
}

fn main() {
    let users = vec![
        User { name: String::from("Alice"), age: 30 },
        User { name: String::from("Bob"), age: 25 },
    ];
    
    // Find existing user
    match find_user(&users, "Alice") {
        Some(user) => println!("Found: {} (age {})", user.name, user.age),
        None => println!("User not found"),
    }
    
    // Find non-existing user
    match find_user(&users, "Charlie") {
        Some(user) => println!("Found: {}", user.name),
        None => println!("User not found"),
    }
}