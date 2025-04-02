// Complete the code by addressing the TODO.

#[derive(Debug)] // this line makes the enum variants printable!
 // TODO: define a few types of messages as used below
enum Message {
    Quit,
    Echo,
    Move { x: i32, y: i32 },
    ChangeColor { r: i32, g: i32, b: i32 },
   
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move { x: 1, y: 2 });
    println!("{:?}", Message::ChangeColor { r: 3, g: 4, b: 5 });
}
