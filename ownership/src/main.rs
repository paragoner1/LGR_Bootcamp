fn main() {
    let s1 = String::from("Rust"); // heap allocated string
    let s2 = s1.clone(); // the value of s1 is copied to s2 and both are valid
     
    println!("s1 is: {s1}");

    let x = 10;
    let y = x; // x is copied to y
    println!("x is: {x}"); // x still prints because it is copied (since primitive) to y by default.

    // in rust, primitive types that are stored on the stack
    //(e.g. integers, floating point #'s, booleans, or characters) 
    // are copied by default when assigned to another variable.

}
