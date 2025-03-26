// Title: Variables

fn main() {
    //creation
    let a = 5;

    //mutability
    let mut b: i32 = 5;
    b = 10;

    // shadowing
    let c = 10;
    let c = 20;

    println!("c is: {c}");

    // scope
    let d = 30; // d here lives in the scope of the main function (outer scope)

    {
        let d = 40; // d here lives in the scope of the inner block
        println!("inner d is: {d}"); //inner scope has access to the variables defined in the outer scope
                                     // however since we've shadowed d, the inner d is the one that is printed}
    }
    println!("d is: {d}"); //outer scope printed since outside inner scope{}
}
