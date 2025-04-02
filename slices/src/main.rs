fn main() {

// because of "Deref Coersion", as a general rule, you should use &str, 
// so callers of the function can pass in either a string literal or a string object
    
    let tweet = String::from(
        "This is my tweet and it's very very long"
    );
    let trimmed_tweet = &trim_tweet(&tweet); 

    let tweet2= "This is my tweet and it's very very long";
    let trimmed_tweet2 = &trim_tweet(tweet2);

    println!("{trimmed_tweet}");
    println!("{trimmed_tweet2}");

    let a = [1, 2, 3, 4, 5, 6];
    let a_slice = &a[0..3];
    println!("{:?}", a_slice);
}



fn trim_tweet(tweet: &str) -> &str {
    &tweet[0..20]
} 