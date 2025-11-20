// Enable strict linting - all clippy warnings become compilation errors
#![deny(clippy::all)]

// Derive Debug trait to allow printing the struct with {:?} formatter
#[derive(Debug)]
// Define a tuple struct representing a 3D point with x, y, z coordinates
struct Point(i32, i32, i32);

// Implementation block - adds methods to the Point struct
impl Point {
    // Constructor function to create a new Point instance
    // This is an associated function (no self parameter)
    fn new(x: i32, y: i32, z: i32) -> Point {
        Point(x, y, z)
    }

    // Returns a new Point with all coordinates doubled
    // Takes immutable reference (&self) - doesn't modify the original
    fn twice(&self) -> Point {
        // Access tuple fields with .0, .1, .2
        Point(self.0 * 2, self.1 * 2, self.2 * 2)
    }

    // Doubles all coordinates in-place
    // Takes mutable reference (&mut self) - modifies the original Point
    fn make_twice(&mut self) {
        self.0 *= 2;
        self.1 *= 2;
        self.2 *= 2;
    }

    // Returns a formatted string representation of the point
    // Takes immutable reference (&self)
    fn describe(&self) -> String {
        format!("({},{},{})", self.0, self.1, self.2)
    }

    // Associated function that returns a Point at origin (0,0,0)
    // No self parameter - called with Point::zero()
    fn zero() -> Point {
        Point(0, 0, 0)
    }
}

fn main() {
    // Using new() - constructor to create a Point
    let mut p = Point::new(1, 2, 3);
    println!("Original point: {:?}", p);

    // Using describe() - get string representation
    let description = p.describe();
    println!("Description: {}", description);

    // Using twice() - creates a new Point with doubled values
    let twice = p.twice();
    println!("Twice (new Point): {:?}", twice);
    println!("Original unchanged: {:?}", p);

    // Using make_twice() - modifies the Point in-place
    p.make_twice();
    println!("After make_twice(): {:?}", p);

    // Using zero() - associated function to create Point at origin
    let origin = Point::zero();
    println!("Origin point: {:?}", origin);
}
