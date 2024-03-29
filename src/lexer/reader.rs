//! # Line and File Readers
//! 
//! Manages reading different raw code sources so the Tokenizer can utilize them.
//! 
//! TODO:
//! - Replace `Result<(), String>` with custom error 
//! 

use regex::Regex;

// -=-=- SizeType for Pointer -=-=- //

/// SizeType ia a Type marker trait for an object that can be passed to the
/// `LineReader.next()` function.
/// 
/// ---
/// 
/// ## Example
/// 
/// ```
/// use interpreter::lexer::{SizeType, Reader, LineReader};
/// let mut reader = LineReader::new("abcdefg");
/// 
/// // make a new struct that impl SizeType
/// struct NewObject { size: usize }
/// impl SizeType for NewObject {
///     fn get_size(&self) -> usize { self.size }
/// }
/// 
/// // use new object as size for next
/// let obj = NewObject { size: 3 };
/// reader.next(obj);
/// 
/// let (val, ptr) = reader.read_next(3).unwrap();
/// assert_eq!("def", val);
/// ```
pub trait SizeType {
    /// get the reader size of a struct that implements the SizeType trait
    fn get_size(&self) -> usize;
}

impl SizeType for u32 {
    fn get_size(&self) -> usize {
        *self as usize
    }
}

/// A marker trait for SizeTypes with a len component.
trait SizeTypeLen {}
impl SizeTypeLen for str {}
impl<'a> SizeTypeLen for &'a str {}
impl SizeTypeLen for String {}
impl<'a> SizeTypeLen for &'a String {}

// all the string types
impl<T: ?Sized> SizeType for T
where
    T: AsRef<str>,
    T: SizeTypeLen
{
    fn get_size(&self) -> usize {
        self.as_ref().len()
    }
}


// -=-=- Read Pointer -=-=- //

/// A read-only pointer to a start and end position in a Reader's content
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ReadPointer {
    /// pointer stack
    stack: Vec<ReadPointer>,
    /// Format (start: line, col, end: line, col)
    pub line_pos: (u32,u32, u32,u32), 
    /// Format (start, end)
    pub read_pos: (u32, u32),
}

impl std::fmt::Display for ReadPointer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ptr(line:{} col:{} len:{})", self.line_pos.0, self.line_pos.1, self.len())
    }
}

impl ReadPointer {
    fn new() -> ReadPointer {
        ReadPointer::from_pos((0,0, 0,0), (0,0))
    }

    pub fn from_pos (line_pos: (u32,u32, u32,u32), read_pos: (u32,u32)) -> ReadPointer {
        ReadPointer {line_pos, read_pos, stack: vec![] }
    }

    /// make a new pointer that spans the position from one pointer to another.
    /// 
    /// ---
    /// 
    /// ## Example
    /// 
    /// ```
    /// use interpreter::lexer::ReadPointer;
    /// let ptr1 = ReadPointer::from_pos((0,3, 0,6), (3, 6));
    /// let ptr2 = ReadPointer::from_pos((0,6, 0,9), (6, 9));
    /// 
    /// let ptr3 = ReadPointer::from_to(&ptr1, &ptr2);
    /// assert_eq!(ptr3, ReadPointer::from_pos((0,3, 0,9), (3, 9)));
    /// ```
    pub fn from_to(from: &ReadPointer, to: &ReadPointer) -> ReadPointer {
        ReadPointer {
            line_pos: (from.line_pos.0, from.line_pos.1, to.line_pos.2, to.line_pos.3),
            read_pos: (from.read_pos.0, to.read_pos.1),
            stack: from.stack.clone(), // Required for parser backtracking
        }
    }

    /// Move a referenced pointer using the string provided
    /// 
    /// ---
    /// 
    /// ## Example
    /// 
    /// ``` ignore
    /// use interpreter::lexer::ReadPointer;
    /// let mut ptr = ReadPointer::from_pos((0,3, 0,6), (3, 6));
    /// 
    /// ReadPointer::move_pointer(&mut ptr, "abc\nabcd");
    /// assert_eq!(ptr, ReadPointer:from_pos((0,3, 1,4), (3, 14)))
    /// ```
    fn move_pointer(ptr: &mut ReadPointer, raw: &str) {
        let mut chars = raw.chars().peekable();
        while let Some(c) = chars.next() {
            ptr.increment();
            match c {
                '\n' => ptr.increment_line(),
                '\r' => {
                    if chars.peek() != Some(&'\n') { ptr.increment_line(); }
                }
                _ => {}
            }
        }
    }

    // -=-=- Seeking -=-=- //

    /// Increment the line column and read position of a pointer.
    fn increment(&mut self) {
        // add one to col
        self.line_pos.3 += 1;
        // add one to read pos
        self.read_pos.1 += 1;
    }

    /// Increment the line number and return the line column to 0.
    fn increment_line(&mut self) {
        // add one to line
        self.line_pos.2 += 1;
        // set col to start
        self.line_pos.3 = 0;
    }

    /// Move the start position to equal the end position.
    fn commit(&mut self) {
        // (start, end)
        self.read_pos.0 = self.read_pos.1;
        // (start: line, col, end: line, col)
        self.line_pos.0 = self.line_pos.2;
        self.line_pos.1 = self.line_pos.3;
    }

    fn back(&mut self) {
        // (start, end)
        self.read_pos.1 = self.read_pos.0;
        // (start: line, col, end: line, col)
        self.line_pos.2 = self.line_pos.0;
        self.line_pos.3 = self.line_pos.1;
    }
    
    fn push(&mut self) {
        self.stack.push(self.clone());
        // println!("PUSH [{}] {self}", self.stack.len())
    }
    
    fn pop(&mut self) {
        *self = self.stack.pop().unwrap_or_else(|| self.clone());
        // println!("POP  [{}] {self}", self.stack.len())
    }
    
    fn pull(&mut self) {
        let _ = self.stack.pop();
        // println!("PULL [{}] {self}", self.stack.len())
    }

    /// Get the length of a pointer
    pub fn len(&self) -> usize {
        ( self.read_pos.1 - self.read_pos.0 ) as usize
    }
}

// -=-=-=-=- Readers -=-=-=-=- //

/// Trait for managing the reading of a content source.
pub trait Reader {
    // -=- Reading -=- //
    
    /// Read the next character in the line
    fn read_char(&self) -> Option<char>;
    
    /// Read the current value pointed at internally
    fn read_current(&self) -> Option<&str>;
    
    /// Read the value pointed at by the ReadPointer
    fn read_pointer(&self, ptr: &ReadPointer) -> Option<&str>;
    
    /// Read the next value in the line with a length of `size`
    fn read_next(&self, size: usize) -> Option<(&str, ReadPointer)>;
    
    /// Read the next value in the line if it matches a regular expression
    fn read_regex(&self, regex: &Regex) -> Option<(&str, ReadPointer)>;
    
    // -=- Seeking -=- //
    
    /// Move the pointer ahead by the size of the supplied value.
    fn next<T>(&mut self, size: T) -> Result<(), String> where T: SizeType;
    
    fn push(&mut self);

    fn pop(&mut self);

    fn pull(&mut self);

    /// Pulls the pointers start position to the end position.
    fn commit(&mut self);

    /// resturn to the start pointer
    fn back(&mut self);
    
    // -=- Pointer -=- //
    
    /// Get the current pointer value
    fn get_pointer(&self) -> &ReadPointer;
    
    /// get a token's pointer using a starting pointer and raw value
    fn get_token_pointer(raw: &str, ptr: &ReadPointer) -> ReadPointer {
        let mut ptr = ptr.clone();
        ptr.commit();
        ReadPointer::move_pointer(&mut ptr, raw);
        ptr
    }
}

// -=-=- Line Reader -=-=- //

/// Takes a line of text for reading and implements the Reader functionality for it.
/// 
/// ---
/// 
/// ## Example
/// 
/// ```
/// use interpreter::lexer::LineReader;
/// let reader = LineReader::new("Line to Read.");
/// ```
pub struct LineReader {
    content: String,
    pointer: ReadPointer,
}

impl LineReader {
    /// Make a new line reader.
    /// 
    /// ---
    /// 
    /// ## Example
    /// 
    /// ```
    /// use interpreter::lexer::LineReader;
    /// let reader = LineReader::new("Line to Read.");
    /// ```
    pub fn new(line: &str) -> LineReader {
        LineReader{
            content: line.to_string(),
            pointer: ReadPointer::new(),
        }
    }

}

impl Reader for LineReader {

    // -=-=- Reading -=-=- //

    /// Read the next character in the line
    /// 
    /// ---
    /// 
    /// ## Example
    /// 
    /// ```
    /// use interpreter::lexer::{Reader, LineReader};
    /// let mut reader = LineReader::new("abcdefg");
    /// let _ = reader.next(3);
    /// 
    /// let ch: char = reader.read_char().unwrap();
    /// assert_eq!('d', ch);
    /// ```
    fn read_char(&self) -> Option<char> {
        let i = self.pointer.read_pos.1 as usize;
        self.content.chars().nth(i)
    }
    
    /// Read the current value pointed at internally
    /// 
    /// ---
    /// 
    /// ## Example
    /// 
    /// ```
    /// use interpreter::lexer::{Reader, LineReader};
    /// let mut reader = LineReader::new("abcdefg");
    /// let _ = reader.next(3);
    /// 
    /// let val: &str = reader.read_current().unwrap();
    /// assert_eq!("abc", val);
    /// ```
    fn read_current(&self) -> Option<&str> {
        self.read_pointer(&self.pointer)
    }

    /// Read the next value in the line with a length of `size`
    /// 
    /// ---
    /// 
    /// ## Example
    /// 
    /// ```
    /// use interpreter::lexer::{Reader, LineReader};
    /// let mut reader = LineReader::new("abcdefg");
    /// 
    /// let (val, ptr) = reader.read_next(4).unwrap();
    /// assert_eq!("abcd", val);
    /// ```
    fn read_next(&self, size: usize) -> Option<(&str, ReadPointer)> {
        // todo set up read bounds
        let i = self.pointer.read_pos.1 as usize;
        let j = i + size;
        let raw = &self.content[i..j];
        Some((raw, <Self as Reader>::get_token_pointer(raw, &self.pointer)))
    }

    /// Read the value pointed at by the ReadPointer
    /// 
    /// ---
    /// 
    /// ## Example
    /// 
    /// ```
    /// use interpreter::lexer::{ReadPointer, Reader, LineReader};
    /// let mut reader = LineReader::new("abcdefg");
    /// let ptr = ReadPointer::from_pos((0,3, 0,6), (3, 6));
    /// 
    /// let val: &str = reader.read_pointer(&ptr).unwrap();
    /// assert_eq!("def", val);
    /// ```
    fn read_pointer(&self, ptr: &ReadPointer) -> Option<&str> {
        // todo set up read bounds
        let i = ptr.read_pos.0 as usize;
        let j = ptr.read_pos.1 as usize;
        Some(&self.content[i..j])
    }

    /// Read the next value in the line if it matches a regular expression
    /// 
    /// ---
    /// 
    /// ## Example
    /// 
    /// ```
    /// use interpreter::lexer::{Reader, LineReader};
    /// use regex::Regex;
    /// let mut reader = LineReader::new("abcdefg");
    /// let re = Regex::new("^[a-d]+").unwrap();
    /// 
    /// let (val, ptr) = reader.read_regex(&re).unwrap();
    /// assert_eq!("abcd", val);
    /// ```
    fn read_regex(&self, regex: &Regex) -> Option<(&str, ReadPointer)> {
        let i = self.pointer.read_pos.1 as usize;
        let m = regex.find(&self.content[i..])?;
        let raw = m.as_str();
        Some((raw, <Self as Reader>::get_token_pointer(raw, &self.pointer)))
    }
    
    // -=-=- Pointer -=-=- //

    /// Get the current pointer value
    fn get_pointer(&self) -> &ReadPointer {
        &self.pointer
    }

    // -=-=- Seeking -=-=- //

    /// Move the pointer ahead by the size of the supplied value.
    /// 
    /// ---
    /// 
    /// ## Example
    /// 
    /// ```
    /// use interpreter::lexer::{Reader, LineReader};
    /// let mut reader = LineReader::new("abcdefg");
    /// 
    /// let _ = reader.next(3);
    /// let val: char = reader.read_char().unwrap();
    /// assert_eq!('d', val);
    /// ```
    /// 
    /// ---
    /// 
    /// the intended use of `next` is to use the value that the reader has read (or an object
    /// defined by it - such as a Token) for the `size` parameter. 
    /// 
    /// ```
    /// use interpreter::lexer::{Reader, LineReader};
    /// let mut reader = LineReader::new("abcdefg");
    /// 
    /// let (val, ptr) = reader.read_next(3).unwrap();
    /// let val = val.to_owned(); // fix mutability error
    /// let _ = reader.next(&val);
    /// assert_eq!("abc", val);
    /// 
    /// let (val, ptr) = reader.read_next(3).unwrap();
    /// assert_eq!("def", val);
    /// ```
    /// 
    fn next<T>(&mut self, size: T) -> Result<(), String> where T: SizeType {
        let count = size.get_size();
        let (val, ptr) = match self.read_next(count) {
            Some((val, ptr)) => (val, ptr),
            None => return Err(String::from("Couldn't read next,.."))
        };
        let raw = val.to_owned();
        
        // -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=- //
        // Stack for self.pointer must be preserved or there will be a bug that 
        // prevents compiling.
        // -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=- //
        // Error: the stack is not preserved here so it locks the current state 
        //   for pop this creates an error for the Parser where it ignores things 
        //   it's already found.
        self.pointer = ReadPointer::from_to(&self.pointer, &ptr);

        // better option
        // ReadPointer::move_pointer(&mut self.pointer, &raw);
        Ok(())
    }
    
    /// Pulls the pointers start position to the end position.
    /// 
    /// ---
    /// 
    /// ## Example
    /// 
    /// ```
    /// use interpreter::lexer::{Reader, LineReader};
    /// let mut reader = LineReader::new("abcdefg");
    /// let _ = reader.next(3);
    /// 
    /// reader.commit();
    /// reader.next(3);
    /// 
    /// let val: &str = reader.read_current().unwrap();
    /// assert_eq!("def", val);
    /// ``` 
    fn commit(&mut self) {
        self.pointer.commit();
        // println!("Commit {:?}", self.pointer);
    }

    
    fn back(&mut self) {
        self.pointer.back();
        // println!("Back {:?}", self.pointer);
    }

    fn push(&mut self) {
        self.pointer.push();
        // println!("Push [{}] {:?}", self.stack.len(), self.pointer);
    }
    
    fn pop(&mut self) {
        self.pointer.pop();
        // println!("Pop [{}] {:?}", self.stack.len(), self.pointer);
    }

    
    fn pull(&mut self) {
        self.pointer.pull();
        // println!("Pop [{}] {:?}", self.stack.len(), self.pointer);
    }
    
    

}

/// Takes a file path and reads the file contents for implementing the Reader functionality.
/// 
/// ---
/// 
/// ## Example
/// 
/// ``` ignore
/// use interpreter::lexer::FileReader;
/// let reader = FileReader::new("./path/to/file.ext");
/// ```
struct _FileReader {}

impl Reader for _FileReader {
    // -=-=- Reading -=-=- //
    
    /// Read the next character in the line
    fn read_char(&self) -> Option<char> {
        todo!()
    }
    
    /// Read the current value pointed at internally
    fn read_current(&self) -> Option<&str> {
        todo!()
    }
    
    /// Read the next value in the line with a length of `size`
    fn read_next(&self, _size: usize) -> Option<(&str, ReadPointer)> {
        todo!()
    }
    
    /// Read the value pointed at by the ReadPointer
    fn read_pointer(&self, _ptr: &ReadPointer) -> Option<&str> {
        todo!()
    }
    
    /// Read the next value in the line if it matches a regular expression
    fn read_regex(&self, _regex: &Regex) -> Option<(&str, ReadPointer)> {
        todo!()
    }
    
    // -=-=- Seeking -=-=- //
    
    /// Move the pointer ahead by the size of the supplied value.
    fn next<T>(&mut self, _size: T) -> Result<(), String> where T: SizeType {
        todo!()
    }
    
    /// Pulls the pointers start position to the end position.
    fn commit(&mut self) {
        todo!()
    }

    
    fn back(&mut self) {
        todo!()
    }
    
    fn push(&mut self) {
        todo!()
    }
    
    fn pop(&mut self) {
        todo!()
    }

    fn pull(&mut self) {
        todo!()
    }
    
    // -=-=- Pointer -=-=- //
    
    /// Get the current pointer value
    fn get_pointer(&self) -> &ReadPointer {
        todo!()
    }
}


// -=-=-=-=- Unit Tests -=-=-=-=- //

#[cfg(test)]
mod doctest {
    use super::*;
    // use interpreter::lexer::ReadPointer;

    /// Copy of the ignored Doctest for `ReadPointer::move_pointer()`
    #[test]
    fn move_pointer() {
        let mut ptr = ReadPointer::from_pos((0, 3, 0, 6), (3, 6) );
        
        // Call the function to be tested
        ReadPointer::move_pointer(&mut ptr, "abc\nabcd");

        // Assert that the pointer has moved correctly
        assert_eq!(ptr, ReadPointer::from_pos((0, 3, 1, 4), (3, 14) ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_pointer_with_all_line_endings() {
        // Unix line ending
        let mut ptr_unix = ReadPointer::from_pos((0, 3, 0, 6), (3, 6) );
        ReadPointer::move_pointer(&mut ptr_unix, "abc\nabcd");

        // Windows line ending
        let mut ptr_windows = ReadPointer::from_pos((0, 3, 0, 6), (3, 6) );
        ReadPointer::move_pointer(&mut ptr_windows, "abc\r\nabcd");

        // Old Mac line ending
        let mut ptr_old_mac = ReadPointer::from_pos((0, 3, 0, 6), (3, 6) );
        ReadPointer::move_pointer(&mut ptr_old_mac, "abc\rabcd");

        // Assert Unix line ending
        assert_eq!(ptr_unix, ReadPointer::from_pos((0, 3, 1, 4), (3, 14) ),
            "Unix Line Ending");

        // Assert Windows line ending
        assert_eq!(ptr_windows, ReadPointer::from_pos((0, 3, 1, 4), (3, 15) ),
            "Windows Line Ending");

        // Assert Old Mac line ending
        assert_eq!(ptr_old_mac, ReadPointer::from_pos((0, 3, 1, 4), (3, 14) ),
            "Old Mac Line Ending");
    }

    #[test]
    fn pointer_increment() {
        let mut ptr = ReadPointer::from_pos((0, 3, 0, 6), (3, 6) );
        ptr.increment();
        assert_eq!(ptr, ReadPointer::from_pos((0, 3, 0, 7), (3, 7) ));
    }

    #[test]
    fn pointer_increment_line() {
        let mut ptr = ReadPointer::from_pos((0, 3, 0, 6), (3, 6) );
        ptr.increment_line();
        assert_eq!(ptr, ReadPointer::from_pos((0, 3, 1, 0), (3, 6) ));
    }

    #[test]
    fn pointer_commit() {
        let mut ptr = ReadPointer::from_pos((0, 3, 1, 6), (3, 9) );
        ptr.commit();
        assert_eq!(ptr, ReadPointer::from_pos((1, 6, 1, 6), (9, 9) ));
    }

    #[test]
fn pointer_push_pop() {
    // Create a ReadPointer instance
    let mut ptr = ReadPointer::from_pos((0, 3, 1, 6), (3, 9));
    // Push the current state
    let state_0 = ptr.clone();
    ptr.push();
    // Modify the pointer's state
    ptr.increment();
    ptr.increment_line();
    // Ensure the pointer's state has changed
    assert_ne!(ptr, state_0);
    // Push the current state
    let state_1 = ptr.clone();
    ptr.push();
    // Modify the pointer's state
    ptr.increment_line();
    ptr.commit();
    ptr.increment();
    // Ensure the pointer's state has changed
    assert_ne!(ptr, state_0);
    assert_ne!(ptr, state_1);
    // Pop the state back
    ptr.pop();
    // Ensure the state is restored to the original
    assert_eq!(ptr, state_1);
    // Pop the state back
    ptr.pop();
    // Ensure the state is restored to the original
    assert_eq!(ptr, state_0);
}
}