// Fix the code so that it compiles.

fn last_element(nums: &[i32]) -> Option<i32> {
    if nums.len() > 0 {
        Some(nums[nums.len() - 1]) // -1 because the index starts at 0, and we want the last element
    } else {
        None
    }
}

fn main() {
    let my_nums = [1, 1, 2, 3, 5, 8, 13];
    match last_element(&my_nums) {
        Some(ele) => println!("Last element: {ele}"), // added ele inside of Sum() to fix the code
        None => println!("Empty array"),
    }
}
