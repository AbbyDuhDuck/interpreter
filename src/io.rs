//! IO for a simple interpreter
//! 

/// flush the std output
// #[macro_export]
macro_rules! flush {
    () => { 
        use std::io::Write;
        let _ = std::io::stdout().flush();
    };
}

/// read a line of user input
// #[macro_export]
macro_rules! read_line {
    () => {{
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);
        input
    }};
}

/// prompt the user for input.
/// 
/// ---
/// 
/// ## Example 
/// 
/// ``` ignore
/// use interpreter::io;
/// let input: String = io::prompt!(">>> ");
/// ```
// #[macro_export]
macro_rules! prompt {
    ($p:expr) => {{
        print!($p);
        use std::io::Write;
        let _ = std::io::stdout().flush();
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);
        input
    }};
}

pub(crate) use flush;
pub(crate) use read_line;
pub(crate) use prompt;

