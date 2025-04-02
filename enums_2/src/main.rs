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
        String::from("JSON string")
      
    }
}
fn main() {
    let cmd = Command::Undo;
    let cmd = Command::Redo;
    let cmd = Command::AddText(String::from("test"));
    let cmd = Command::MoveCursor(22, 0);
    let cmd = Command::Replace {
        from: String::from("a"),
        to: String::from("b"),
    };

    let json_string = cmd.serialize();
}

