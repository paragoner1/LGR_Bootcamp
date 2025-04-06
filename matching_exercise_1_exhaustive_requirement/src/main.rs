// Make the following code compile.
// If you score 50 or less, you fail.

fn main() {
    // marks scored out of 100
    let marks = 75u8;
    match marks {
        91..=100 => println!("You performed excellent!"),
        71..=90 => println!("You performed good :)"),
        51..=70 => println!("Your performance was average..."),
        0..=50 => println!("You failed. Better luck next time."),// changed from 30 to 50
        101..=u8::MAX => println!("Invalid marks!!!"),
    }
}
