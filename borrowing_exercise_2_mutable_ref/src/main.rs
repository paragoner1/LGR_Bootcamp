// Fix the code so that it compiles.

fn main() {
    let mut s = String::from("Hello, ");
    let s_ref = &mut s;// added mutable reference to s so it can be modified since s is mutable
    change_string(s_ref);
    println!("{s_ref}");
}

fn change_string(s: &mut  String) {// added mutable reference to s so it can be modified since s is mutable
    s.push_str(" world!");
}