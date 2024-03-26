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

    fn from_to(from: ReadPointer, to: ReadPointer) -> ReadPointer {
        ReadPointer {
            line_pos: (from.line_pos.0, from.line_pos.1, to.line_pos.2, to.line_pos.3),
            read_pos: (from.read_pos.0, to.read_pos.1)
        }
    }

    fn move_pointer(ptr: &mut ReadPointer, raw: &str) {
        for c in raw.chars() {
            ptr.increment();
            if c == 0xA as char {
                ptr.increment_line();
            }
        }
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
    fn read_pointer(&self, ptr: &ReadPointer) -> Option<&str>;
    fn read_next(&self, size: usize) -> Option<(&str, ReadPointer)>;
    fn read_regex(&self, regex: &Regex) -> Option<(&str, ReadPointer)>;
    // Pointer
    fn get_pointer(&self) -> ReadPointer;
    // fn get_pointer_next<T>(&self, size: T) -> Result<ReadPointer, String> where T: SizeType;
    // Seeking
    fn next<T>(&mut self, size: T) -> Result<(), String> where T: SizeType;
    fn pull(&mut self);

    // token pointer
    fn get_token_pointer(raw: &str, ptr: &ReadPointer) -> ReadPointer {
        let mut ptr = ptr.clone();
        ptr.pull();
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

    fn get_pointer(&self) -> ReadPointer {
        self.pointer
    }

    // fn get_pointer_next<T>(&self, size: T) -> Result<ReadPointer, String> where T: SizeType {
    //     let count = size.get_size();
    //     let current = match self.read_next(count) {
    //         Some(val) => val,
    //         None => return Err(String::from("Couldn't read next,.."))
    //     };

    //     let mut ptr = self.pointer; // im pretty sure this does a copy
    //     ReadPointer::move_pointer(&mut ptr, current)?;
    //     Ok(ptr)
    // }

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
        let ptr = match self.read_next(count) {
            Some((_val, ptr)) => ptr,
            None => return Err(String::from("Couldn't read next,.."))
        };
        
        // ReadPointer::move_pointer(&mut self.pointer, current);
        self.pointer = ReadPointer::from_to(self.pointer, ptr);

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

impl Reader for _FileReader {
    fn read_char(&self) -> Option<char> {
        todo!()
    }

    fn read_current(&self) -> Option<&str> {
        todo!()
    }

    fn read_next(&self, _size: usize) -> Option<(&str, ReadPointer)> {
        todo!()
    }

    fn read_pointer(&self, _ptr: &ReadPointer) -> Option<&str> {
        todo!()
    }

    fn read_regex(&self, _regex: &Regex) -> Option<(&str, ReadPointer)> {
        todo!()
    }

    fn get_pointer(&self) -> ReadPointer {
        todo!()
    }

    // fn get_pointer_next<T>(&self, _size: T) -> Result<ReadPointer, String> where T: SizeType {
    //     todo!()
    // }

    fn next<T>(&mut self, _size: T) -> Result<(), String> where T: SizeType {
        todo!()
    }

    fn pull(&mut self) {
        todo!()
    }
}