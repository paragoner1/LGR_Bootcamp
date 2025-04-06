enum Command {
    Undo,
    Redo,
    AddText(String),
    MoveCursor (i32, i32),
    Replace {
        from: String,
        to: String,
    }
}
impl Command {
    fn serialize(&self) -> String {
        match self {
            Command::Undo => String::from(
                "{ \"cmd\": \"undo\" }"
            ),
            Command::Redo => String::from(
                "{ \"cmd\": \"redo\" }"
            ),
            Command::AddText(s) => {
                format!(
                "{{ \
                    \"cmd\": \"add_text\", \
                    \"text\": \"{s}\"  
                }}"   
            )   
        },
            Command::MoveCursor(x, y) => {
                format!(
                "{{ \
                    \"cmd\": \"move_cursor\", \
                    \"x\": {x}, \
                    \"y\": {y} \
                }}"
            )
        }
            Command::Replace { from, to } => {
                format!(
                "{{ \
                    \"cmd\": \"replace\", \
                    \"from\": \"{from}\", \
                    \"to\": \"{to}\" \
                }}"
            )
        }   
    }
}
}

fn main() {
    let cmd1 = Command::Undo;
    let cmd2 = Command::Redo;
    let cmd3 = Command::AddText(String::from("test"));
    let cmd4 = Command::MoveCursor(22, 0);
    let cmd5 = Command::Replace {
        from: String::from("a"),
        to: String::from("b"),
    };

    println!("{}", cmd1.serialize());
    println!("{}", cmd2.serialize());
    println!("{}", cmd3.serialize());
    println!("{}", cmd4.serialize());
    println!("{}", cmd5.serialize());
}    

    
            


