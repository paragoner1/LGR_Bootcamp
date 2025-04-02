fn main() {
    let original_price = 51; // Set original_price to 51
    println!("Your sale price is {}", sale_price(original_price)); // Call sale_price with original_price (51) and print the returned sale price
}

fn sale_price(price: i32) -> i32 { // Define sale_price function that takes a price (i32) and returns the discounted price (i32)
    if is_even(price) { // Check if price is even by calling is_even
        price - 10 // If price is even, return price minus 10
    } else {
        price - 3 // If price is odd, return price minus 3
    }
}

fn is_even(num: i32) -> bool { // Define is_even function that takes a number (i32) and returns true if it's even, false if odd
    num % 2 == 0 // Return true if num is divisible by 2 (even), false otherwise
}

// Flow of the program:
// 1. main starts and sets original_price to 51.
// 2. main calls sale_price with original_price (51).
// 3. sale_price calls is_even with price (51) to check if it's even.
// 4. is_even returns false because 51 % 2 is 1 (odd).
// 5. sale_price uses the else branch since price is odd, returns 51 - 3 (48).
// 6. sale_price ends, returning 48 to main.
// 7. main prints "Your sale price is 48".
// 8. main ends.
