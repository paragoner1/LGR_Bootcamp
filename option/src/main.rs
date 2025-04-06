fn main() {
    let username = get_username(1);
    if let Some(name) = username { // name is a new variable that is created, to replace the value of username
        println!("{name}");
        
    }
// general rule of thumb: if you want to represent something that can either be a valid value or nothing, use Option<T> enum
    // if you want to represent an operation that can either succeed or fail, use Result<T, E> enum
  
    fn get_username(user_id: u32) -> Option<String> { // get username from database
       let db_result = String::from("Ferris");
       if user_id == 1 {
           Some(db_result)
       } else {
        None

        }
    }
}



