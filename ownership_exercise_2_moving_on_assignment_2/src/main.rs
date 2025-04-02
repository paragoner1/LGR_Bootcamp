// Fix the code so that it compiles. Modify only one statement.

fn main() {
    // Create a mutable string "Example"
    let mut my_str = String::from("Example");
    // Initialize temp as an empty string to store clones
    let mut temp = String::new();
    // Loop while my_str has characters
    while my_str.len() > 0 {
        // Clone my_str into temp
        temp = my_str.clone();
        // Print the length of the cloned string
        println!("Length of temporary string is: {}", temp.len());
        // Remove the last character from my_str
        my_str.pop();
    }
}