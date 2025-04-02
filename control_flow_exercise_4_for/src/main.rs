// Here's a bucket list of cities I'd like to visit one day, and I'd like to
// share it with the world. Fix the loop so it announces the cities I'd like to visit.

fn main() {
    // Define an array of city names as string slices
    let cities = ["Perth", "Qingdao", "Rome"];
     // Loop through each city in the array using an iterator; 'cities' implements IntoIterator,
    // so Rust converts it to an iterator, and 'city' is bound to each element in turn
    for city in cities { 
        // Print a formatted string with the city name
        println!("I'd like to visit {} someday!", city);
    }
}