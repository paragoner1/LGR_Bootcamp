// Make the following code compile by modifying only one statement.

fn main() {
    // Action 1: Create a string "I will master rust 🦀 🦀" using get_new_string
    let mut str1 = get_new_string();
    // Action 2: Print the string using str1
    println!("Printing through str1: {}", str1);
    // Action 3: Move str1 to str2, str1 is now invalid
    let mut str2 = str1;
    // Action 4: Print the string using str2
    println!("Printing through str2: {}", str2);
    // Action 5: Move str2 back to str1, str2 is now invalid
    str1 = str2;
    // Action 6: Print the string using str1 again
    println!("Again printing through str1: {}", str1);
    // Action 7: Clone str1 to str2, both are now valid
    str2 = str1.clone();// here's the fix I added to make the code compile
    // Action 8: Print the string using str2
    println!("Again printing through str2: {}", str2);
    // Action 9: Print the string using both str1 and str2
    println!("Printing thourgh both: {}, {}", str1, str2);
}

// Action 10 (when called): Create and return a new string "I will master rust 🦀 🦀"
fn get_new_string() -> String {
    let new_string = String::from("I will master rust 🦀 🦀");
    new_string  // Ownership is transferred to the caller
}

// Flow of the program:
// 1. Create str1 as "I will master rust 🦀 🦀".
// 2. Print str1 ("I will master rust 🦀 🦀").
// 3. Move str1 to str2, str1 is invalid.
// 4. Print str2 ("I will master rust 🦀 🦀").
// 5. Move str2 to str1, str2 is invalid.
// 6. Print str1 ("I will master rust 🦀 🦀").
// 7. Clone str1 to str2, both are valid.
// 8. Print str2 ("I will master rust 🦀 🦀").
// 9. Print str1 and str2 ("I will master rust 🦀 🦀, I will master rust 🦀 🦀").