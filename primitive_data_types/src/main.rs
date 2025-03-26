fn main() {
    // boolean
    let b1: bool = true;

    // unsigned integers
    let i1: u8 = 1;
    let i2: u16 = 1;
    let i3: u32 = 1;
    let i4: u64 = 1;
    let i6: u128 = 1;

    // signed integers
    let i7: i8 = 1;
    let i8: i16 = 1;
    let i9: i32 = 1;
    let i10: i64 = 1;
    let i11: i128 = 1;

    // floating point
    let f1: f32 = 1.0;
    let f2: f64 = 1.0;

    // platform specific integers
    let p1: usize = 1;
    let p2: isize = 1;

    // characters, &str, and String
    let c1: char = 'c';
    let s1: &str = "hello";
    let s2: String = String::from("hello");

    // arrays (data type is [T; N]) like seen below
    //arrays hold multiple values of the same type
    let a1: [i32; 5] = [1, 2, 3, 4, 5];
    let i1 = a1[4]; // You can index values in an array using the array name, followed by []

    // tuples
    // tuples hold multiple values of different types
    let t1: (i32, i32, i32) = (1, 2, 3);
    let t1: (i32, f64, &str) = (5, 5.0, "5");

    let s1: &str = t1.2; // You can access values in a tuple using the tuple name, followed by . and the index of the value you want to access
    let (i1, f1, s1) = t1; // You can also destructure a tuple to get the individual values

    let unit: () = (); // The unit type is a tuple with no elements
                    // functions that don't return a value implicitly return the unit type
    
    // Type aliasing
    type age = u8; // You can create a type alias using the type keyword

    let a1: age = 38; 
}