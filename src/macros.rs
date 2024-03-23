//! # Interpreter's Macros Module
//! 
//! Contains sub-modules for different macros needed in the interpreter
//! not to be used outside the interpreter crate... probably.

pub mod io {

    /// flush the std output
    /// 
    /// ---
    /// 
    /// ## Example
    /// 
    /// ``` ignore
    /// use macros::io::*;
    /// print!("some input ")
    /// flush!();
    /// ```
    macro_rules! flush {
        () => {
            use std::io::Write;
            let _ = std::io::stdout().flush();
        };
    }
    
    /// read a line of user input
    /// 
    /// ---
    /// 
    /// ## Example
    /// 
    /// ``` ignore
    /// use macros::io::*;
    /// let input: String = read_line!();
    /// ```
    macro_rules! read_line {
        () => {{
            let mut input = String::new();
            let _ = std::io::stdin().read_line(&mut input);
            input
        }};
    }
    
    /// prompt the user for input with the prompt `p` being displayed on the same line 
    /// before the stdin read_line value is passed back.
    /// 
    /// ---
    /// 
    /// ## Example
    /// 
    /// ``` ignore
    /// use macros::io::*;
    /// let input: String = prompt!("Enter something: ");
    /// ```
    macro_rules! prompt {
        ($p:expr) => {{
            print!($p);
            flush!();
            read_line!()
        }};
    }
    
    pub(crate) use flush;
    pub(crate) use read_line;
    pub(crate) use prompt;
}
