fn main() {
    let mut x = 5;
    { // Scope for mutable reference
        let y = &mut x; // y is a mutable reference to x
        *y = 10; // Modify x through y
    }
    println!("x = {}", x); //Prints 10

    let mut a = 5; // a must be mutable to allow mutable reference
    { //Scope for mutable reference
        let b = &mut a; // b is a mutable reference to a
        *b = 10; //Modify a through b
    }
    println!("a = {}", a); //Prints 10
}
