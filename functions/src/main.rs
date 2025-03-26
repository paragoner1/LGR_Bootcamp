fn main() {
    let z = my_function(22); // Call my_function with the argument 22, which is passed to the parameter x; store the returned i32 in z
    println!("my_function returned: {}", z);
}

fn my_function(x: i32) -> i32 { // To add more parameters (e.g., x: i32, y: i32), include them here separated by commas, with their types
    println!("my_function called with: {}", x);
    let y = 10;
    y
}
// The function signature specifies a return type of i32, so we return y, which is an i32 (set to 10)

// Flow of the program:
// 1. main is called as the program’s entry point.
// 2. main calls my_function with the argument 22, which is passed to the parameter x, 
    // and assigns the return value to the local variable z.
// 3. my_function executes, printing "my_function called with: 22" using the parameter x.
// 4. my_function creates a local variable y set to 10.
// 5. my_function returns y (10) to main.
// 6. my_function ends, and control returns to main.
// 7. main prints "my_function returned: 10" using the value of z.
// 8. main ends, and the program terminates.

// Bonus: why did my_function execute before main?
// because my_function is called with an argument (22), so the function must execute completely 
// before its return value can be assigned to z.

// my_function’s println! executes first because my_function is called before main can proceed 
// with its own println!.

