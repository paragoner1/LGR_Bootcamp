//start by creating a constant called max players that will hold 
//max # players in a game we want to play, i.e. poker.
//constants can be declared in any scope, including the global scope
//and are valid for the entire time a program runs, within the scope 
//they are declared in a constant can't become mutable, and value 
//must be a constant value, computed at compile time.
//always screaming snake case.

const MAX_PLAYERS: u8 = 10;

//statics are similar to constants, but they can be marked as
//mutable, however, accessing or modifying a mutable static variable
//is unsafe, so have to modify those only in an unsafe block
//statics have a 'static lifetime, which means they can be valid for
//the entire duration of the program.  statics can be mutable, but
//they are unsafe to access or modify. 

static mut CASINO_NAME: &str = "Rusty Casino";

fn main() {

    let a: u8 = 10;//the value of the constant will be inline (MAX_PLAYERS will be replaced by 10)
    let b: u8 = 10;

    //commenting out due to error let c: &str = CASINO_NAME;
    //uncomment these to see error let d: &str = CASINO_NAME;
}
//constants are inlined, so the value of the constant is replaced
//everywhere it is used, so the value of a and b will be 10, not MAX_PLAYERS
//statics are not inlined, so the value of c and d will be "Rusty Casino"
//not CASINO_NAME