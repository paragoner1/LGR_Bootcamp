fn main() {
    let username = get_username(1);
    if let Some(name) = username { // name is a new variable that is created, to replace the value of username
        println!("{name}");
        
    }
// general rule of thumb: if you want to represent something that can either be a valid value or nothing, use Option<T> enum
    // if you want to represent an operation that can either succeed or fail, use Result<T, E> enum
  
    fn get_username(user_id: u32) -> Option<String> { // get username from database
        let query  = 
            format!(" GET username FROM users WHERE id = {user_id}"); // query the database
       let db_result = query_db(query); // query the database
        // if user_id is 1, return the result, else return None
           db_result.ok() // this turns the result enum into an option enum
  

        }
    }

    fn query_db(query: String) -> Result<String, String> {
        if query.is_empty() {
            Err(String::from("Query is empty!"))
        } else {
            Ok(String::from("Ferris"))
        }
    }


