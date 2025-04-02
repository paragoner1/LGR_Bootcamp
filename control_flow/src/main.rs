fn main() {
    // if/else
    let a = 5;

    if a > 5 {
        println!("bigger than 5");
    } else if a > 3 {
        println!("a is equal to 5");
    } else {
        println!("smaller or equal to 3");
    }

    let b = if a > 5 { 1 } else { -1 }; // means b is 1 if a > 5, and b is -1 if a <= 5
                                            // return type of each element must be the same
    // loop
    'outer: loop { // outer loop named with tick mark
        println!("outer loop");
        break; // break statement to stop outer loop. Since outer was named, we can break outer loop within inner loop with 'break 'outer;
        'inner: loop { // inner loop named with tick mark
            println!("inner loop");
            break; // break statement to stop inner loop
        }
} 
    // while loop (continue to loop until condition (a < 5) is false)
    let mut a = 0;

    while a < 5 {
        println!("while loop says a is {a}");
        a = a + 1;
    }// this says to print a while a is less than 5, and increment a by 1 each time

    // for loop (allows you to loop over a collection of items)
    let a = [1, 2, 3, 4, 5];
    
    for element in a {
        println!("for loop says {}", element);
    } 
}
