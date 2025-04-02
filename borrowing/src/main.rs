fn main() {
    let mut s1 = String::from("Rust"); // heap allocated string
    let r1 = &s1; // r1 is a reference to s1
    print_string(r1); // r1 is borrowed by print_string
    let r2 = &mut s1; // r2 is a mutable reference to s1
    add_to_string(r2);
    println!("s1 is: {s1}"); // s1 is still valid here since it was passed a reference to print_string
    let s2 = generate_string(); // s2 takes ownership of the returned string
}

fn generate_string() -> String {
     String::from("Ferris")
}

fn add_to_string(mut p1: &mut String) {
    p1.push_str(" is awesome!"); // we can call on p1 directly, because rust has automatic dereferencing (*p1)
}   

fn print_string(p1: &String) { // print_string is now set to borrow a string
    println!("{p1}");
}
