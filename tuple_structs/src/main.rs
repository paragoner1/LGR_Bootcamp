// tuple structs are similar to regular structs but they use () instead of {} to define the types of the fields,
// and the fields are accessed by index instead of by name, since they're not named.

fn main() { 
    // tuples
let rgb_color = (255, 106, 0);
let cmyk_color = (0, 58, 100, 0);

   // tuple structs
   struct RGB(i32, i32, i32);
   struct CMYK(i32, i32, i32, i32);

   let color1 = RGB(255, 106, 0);
   let color2 = CMYK(0, 58, 100, 0);

// unit-like structs
// are similar to tuple structs, but they don't have any fields at all, and they're rarely used.
// one common use for unit-like structs if you have a trait you want to implement on something, but you don't need any data.

struct MyStruct;
}
