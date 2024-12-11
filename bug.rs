fn main() {    let mut x = 5;    let y = &mut x; // y is a mutable reference to x    let z = &y;     // z is an immutable reference to y (which is mutable)    *y = 10; // Modify x through y
    println!("x = {}", x); //Prints 10
    println!("y = {}", *y); //Prints 10
    println!("z = {}", *z); //Prints 10

    let a = 5;    let b = &a;  // b is an immutable reference to a
    //let c = &mut a;  // This line will cause a compile time error because we can't create a mutable reference to an immutable value.
    // *b = 10; //This line will also cause a compile-time error because we cannot modify a value through an immutable reference.
}
