// Fix the code so that it compiles.

fn main() {
    // Create a string "I love rust bootcamp 💕"
    let my_string = String::from("I love rust bootcamp 💕");
    // Count occurrences of 'o' in a clone of my_string
    let occurence_count = count_occurences(my_string.clone(), 'o');// added clone here to solve.
    // Print the string and the count of 'o's                     If we passed my_string directly, it would be moved 
                                                                //into the function, and my_string would no longer 
                                                                //be usable in main for the println! statement.
    println!("The number of times 'o' apprears in \"{my_string}\" = {occurence_count}");
}

// Count how many times a letter appears in the text
fn count_occurences(text: String, letter: char) -> u32 {
    // Initialize a counter
    let mut res = 0;
    // Loop through each character in the text
    for ch in text.chars() {
        // Increment counter if the character matches the letter
        if ch == letter {
            res += 1;
        }
    }
    // Return the final count
    res
}