// Enable strict linting - all clippy warnings become compilation errors
#![deny(clippy::all)]


fn main() {
let values =["foo", "bar", "baz"];

for value in values.iter(){
    println!("{}", value);
}
}
