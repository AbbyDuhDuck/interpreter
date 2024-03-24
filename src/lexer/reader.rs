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
/// let val = reader.read_next(3).unwrap();
/// assert_eq!("def", val);
/// ```
pub trait SizeType {
    fn get_size(&self) -> usize;
}

impl SizeType for u32 {
    fn get_size(&self) -> usize {
        *self as usize
    }
}

// marker trait for SizeTypes with a len component.
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

/// A pointer to the positions in a Reader's content
#[derive(Clone, Copy, Debug)]
pub struct ReadPointer {
    /// Format (start: line, col, end: line, col)
    pub line_pos: (u32,u32, u32,u32), 
    /// Format (start, end)
    pub read_pos: (u32, u32),
}


impl ReadPointer {
    fn new() -> ReadPointer {
        ReadPointer {line_pos: (0,0, 0,0), read_pos: (0, 0)}
    }

    // -=-=- Seeking -=-=- //

    fn increment(&mut self) {
        // add one to col
        self.line_pos.3 += 1;
        self.read_pos.1 += 1;
    }

    fn increment_line(&mut self) {
        // add one to line
        self.line_pos.2 += 1;
        self.read_pos.0 += 1;
        // set col to start
        self.line_pos.3 = 0;
        self.read_pos.1 = 0;
    }

    fn pull(&mut self) {
        // (start, end)
        self.read_pos.0 = self.read_pos.1;
        // (start: line, col, end: line, col)
        self.line_pos.2 = self.line_pos.0;
        self.line_pos.3 = self.line_pos.1;
    }
}

// -=-=-=-=- Readers -=-=-=-=- //

/// Trait for managing the reading of a content source.
pub trait Reader {
    // Reading
    fn read_char(&self) -> Option<char>;
    fn read_current(&self) -> Option<&str>;
    fn read_next(&self, size: usize) -> Option<&str>;
    fn read_pointer(&self, ptr: &ReadPointer) -> Option<&str>;
    fn read_regex(&self, regex: Regex) -> Option<&str>;
    // Seeking
    fn next<T>(&mut self, size: T) -> Result<(), String> where T: SizeType;
    fn pull(&mut self);
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
            pointer: ReadPointer::new()
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
    /// let val: &str = reader.read_next(4).unwrap();
    /// assert_eq!("abcd", val);
    /// ```
    fn read_next(&self, size: usize) -> Option<&str> {
        // todo set up read bounds
        let i = self.pointer.read_pos.1 as usize;
        let j = i + size;
        Some(&self.content[i..j])
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
    /// let ptr = ReadPointer {line_pos: (0,3, 0,6), read_pos: (3, 6)};
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
    /// let val: &str = reader.read_regex(re).unwrap();
    /// assert_eq!("abcd", val);
    /// ```
    fn read_regex(&self, regex: Regex) -> Option<&str> {
        let i = self.pointer.read_pos.1 as usize;
        let m = regex.find(&self.content[i..])?;
        Some(m.as_str())
    }

    // -=-=-=- Seeking -=-=- //

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
    /// let val = reader.read_char().unwrap();
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
    /// let val = reader.read_next(3).unwrap().to_owned();
    /// let _ = reader.next(&val);
    /// assert_eq!("abc", val);
    /// 
    /// let val = reader.read_next(3).unwrap();
    /// assert_eq!("def", val);
    /// ```
    /// 
    fn next<T>(&mut self, size: T) -> Result<(), String> where T: SizeType {
        let count = size.get_size();
        let current = match self.read_next(count) {
            Some(val) => val,
            None => return Err(String::from("Couldn't read next,.."))
        }.to_owned();
        
        for c in current.chars() {
            self.pointer.increment();
            if c == 0xA as char {
                self.pointer.increment_line();
            }
        }
        
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
    /// reader.pull();
    /// reader.next(3);
    /// 
    /// let val: &str = reader.read_current().unwrap();
    /// assert_eq!("def", val);
    /// ``` 
    fn pull(&mut self) {
        self.pointer.pull();
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