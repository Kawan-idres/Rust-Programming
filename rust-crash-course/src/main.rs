// Enable strict linting - all clippy warnings become compilation errors
#![deny(clippy::all)]
fn main() {
    let mut todos: Vec<String> = Vec::new();
    
    // Add todos
    todos.push(String::from("Learn Rust"));
    todos.push(String::from("Build a project"));
    todos.push(String::from("Practice daily"));
    
    // Display todos
    println!("📝 Todo List:");
    for (i, todo) in todos.iter().enumerate() {
        println!("  {}. {}", i + 1, todo);
    }
    
    // Complete a todo (remove it)
    let completed = todos.remove(0);
    println!("\n✅ Completed: {}", completed);
    
    // Show remaining
    println!("\n📝 Remaining:");
    for (i, todo) in todos.iter().enumerate() {
        println!("  {}. {}", i + 1, todo);
    }
}