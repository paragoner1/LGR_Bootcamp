// Complete the structure definition.

struct Point(f64, f64); // added () to define tuple struct Point

impl Point {
    fn on_x_axis(&self) -> bool {
        self.1 == 0.0
    }
    fn on_y_axis(&self) -> bool {
        self.0 == 0.0
    }
}

fn main() {
    let point = Point(0.0, 0.0);
    if point.on_x_axis() && point.on_y_axis() {
        println!("Point is origin");
    }
}
