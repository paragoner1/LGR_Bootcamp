fn main() {}
   // Complete this function to return the bigger number!
    // Do not use:
    // - another function call
    // - additional variables
pub fn bigger(a: i32, b: i32) -> i32 {
    if a > b { // if a is greater than b, return a
        a
    } else { // if a is not greater than b, return b
        b
    }
}
// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}
    


