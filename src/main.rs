
pub mod io;
// use io::prompt;

/// The main entry point to running our program
fn main() {
    loop {
        // prompt the user for input
        let input = io::prompt!("@> ");

        if input.trim() == "exit" {
            break;
        }
    
        // exec the input
        interpreter::exec(input.trim());
    }
}