// Fix the code so that it compiles.

fn main() {
    let mut str1 = String::from("modifiable");
    let str2 = String::from("fixed string");
    let mut str_ptr: &String;
    str_ptr = &str1;// added reference to str1 so it can be modified since str1 is mutable because
    str_ptr = &str2;// added reference to str2 so it can be modified since str2 is immutable
    println!("ptr currently points to {str_ptr}");
    str1.push_str(" string");
    str_ptr = &str1; // added reference to str1 so it can be modified since str1 is mutable
    println!("ptr currently points to {str_ptr}");
}
