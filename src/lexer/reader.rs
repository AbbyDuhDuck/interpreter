//! # Line and File Readers
//! 
//! Manages reading different raw code sources so the Tokenizer can utilize them.
//! 
//! TODO:
//! [ ] - Replace `Result<(), String>` with custom error 
//! 

use regex::Regex;

// -=-=- SizeType for Pointer -=-=- //

pub trait SizeType {
    fn get_size(&self) -> usize;
}

impl SizeType for u32 {
    fn get_size(&self) -> usize {
        *self as usize
    }
}

impl SizeType for &str {
    fn get_size(&self) -> usize {
        self.len()
    }
}

// -=-=- Read Pointer -=-=- //

struct ReadPointer {
    // (start: line, col, end: line, col)
    line_pos: (u32,u32, u32,u32), 
    // (start, end)
    read_pos: (u32, u32),
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

trait Reader {
    // Reading
    fn read_char(&self) -> Option<char>;
    fn read_current(&self) -> Option<&str>;
    fn read_next(&self, size: usize) -> Option<&str>;
    fn read_regex(&self, regex: Regex) -> Option<&str>;
    // Seeking
    fn next<T>(&mut self, size: T) -> Result<(), String> where T: SizeType;
    fn pull(&mut self);
}

// -=-=- Line Reader -=-=- //

pub struct LineReader {
    content: String,
    pointer: ReadPointer,
}

impl LineReader {
    pub fn new(line: &str) -> LineReader {
        LineReader{
            content: line.to_string(),
            pointer: ReadPointer::new()
        }
    }
}

impl Reader for LineReader {

    // -=-=- Reading -=-=- //

    fn read_char(&self) -> Option<char> {
        let i = self.pointer.read_pos.1 as usize;
        self.content.chars().nth(i)
    }
    
    fn read_current(&self) -> Option<&str> {
        // todo set up read bounds
        let i = self.pointer.read_pos.0 as usize;
        let j = self.pointer.read_pos.1 as usize;
        Some(&self.content[i..j])
    }

    fn read_next(&self, size: usize) -> Option<&str> {
        // todo set up read bounds
        let i = self.pointer.read_pos.1 as usize;
        let j = i + size;
        Some(&self.content[i..j])
    }

    fn read_regex(&self, regex: Regex) -> Option<&str> {
        let i = self.pointer.read_pos.1 as usize;
        let m = regex.find(&self.content[i..])?;
        Some(m.as_str())
    }

    // -=-=-=- Seeking -=-=- //

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
    
    fn pull(&mut self) {
        self.pointer.pull();
    }
    
    

}


struct _FileReader {}