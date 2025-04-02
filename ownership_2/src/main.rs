fn main() {
    // --- String Operations (Ownership Focus) --- Heap allocated
    // Action 1: Create a String "Rust" on the heap, s1 owns it
    let s1 = String::from("Rust");
    // Action 2: Clone s1 to create s2, s2 owns the clone, s1 retains ownership
    let s2 = s1.clone();
    // Action 3: Clone s1 again, pass ownership of the clone to print_string
    print_string(s1.clone());
    // Action 4: Call generate_string, take ownership of the returned String as s3
    let s3 = generate_string();//Ferris
    // Action 5: Pass ownership of s2 to add_to_string, take ownership of the returned String as s4
    let s4 = add_to_string(s2); //Rust is awesome!
    // Action 6: Print s1, which still owns "Rust"
    println!("s1 is: {s1}"); // Rust
    // Action 7: Print s3, which owns "Ferris"
    println!("s3 is: {s3}"); //Ferris
    // Action 8: Print s4, which owns "Rust is awesome!"
    println!("s4 is: {s4}"); // Rust is awesome!

    // --- Integer Operations (Copy Focus) --- Stack allocated
    // Action 9: Create an integer x with value 10
    let x = 10;
    // Action 10: Copy x to y (i32 implements Copy, no ownership transfer)
    let y = x;
    // Action 11: Pass a copy of x to print_integer (no ownership transfer)
    print_integer(x);
    // Action 12: Print x, which is still valid since it was copied
    println!("x is: {x}");// 10
} // s1, s3, s4 are dropped here

// --- Helper Functions --- 
// Action 13 (when called): Print the given integer (receives a copy of i32)
fn print_integer(i: i32) {
    println!("i is: {i}");// i is 10
}

// Action 14 (when called): Take ownership of p1, append " is awesome!", return ownership
fn add_to_string(mut p1: String) -> String {
    p1.push_str(" is awesome!");
    p1
}

// Action 15 (when called): Create a String "Ferris", transfer ownership to caller
fn generate_string() -> String {
    String::from("Ferris")
}

// Action 16 (when called): Take ownership of p1, print it, p1 is dropped
fn print_string(p1: String) {
    println!("{p1}");
}

// Flow of the program (Ownership Focus):
// 1. Create s1, s1 owns "Rust".
// 2. Clone s1, s2 owns the clone, s1 retains ownership.
// 3. Clone s1, pass ownership of the clone to print_string, print "Rust".
// 4. Call generate_string, take ownership of "Ferris" as s3.
// 5. Pass ownership of s2 to add_to_string, take ownership of "Rust is awesome!" as s4.
// 6. Print s1 ("Rust"), s1 still owns it.
// 7. Print s3 ("Ferris"), s3 owns it.
// 8. Print s4 ("Rust is awesome!"), s4 owns it.
// 9. Create x (10).
// 10. Copy x to y, no ownership transfer (i32 implements Copy).
// 11. Pass a copy of x to print_integer, print "10".
// 12. Print x ("10"), x is still valid.