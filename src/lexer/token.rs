//! # Tokens and Tokenizer
//! 
//! Contains structures for managing and building tokens.
//! 

use std::collections::HashMap;

use regex::Regex;

use super::{ReadPointer, Reader, SizeType};

/// A raw token object.
#[derive(Clone, Debug)]
pub struct Token {
    // should be an enum but I prefer a hierarchical naming.
    /// String name of a token type
    pub token_type: String,
    /// Raw value of the token
    pub value: String,
    /// The read position of where the token was found.
    pub position: ReadPointer
}

impl SizeType for &Token {
    fn get_size(&self) -> usize {
        self.position.len()
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(
        //     f,
        //     "{}:{} ({}, {} - {})",
        //     self.token_type,
        //     self.value,
        //     self.position.line_pos.0,
        //     self.position.line_pos.1,
        //     self.position.len(),
        // )
        write!(
            f,
            "{}:{}",
            self.token_type,
            self.value,
        )
    }
}

impl Token {
    /// Make a new token
    pub fn new(token_type: &str, value: &str, position: ReadPointer) -> Token {
        Token { token_type: token_type.to_string(), value: value.to_string(), position }
    }
}

/// A regex definition for a token
#[derive(Debug)]
pub struct TokenDef {
    token_type: String,
    regex: Regex,
}

impl TokenDef {
    /// Make a new token definition for the type provided by `token_type`. Additionally
    /// the passed `regex` value is expected to compile without error or a Token Definition
    /// cannot be created.
    pub fn new(token_type: &str, regex: &str) -> Result<TokenDef, String> {
        let regex = TokenDef::build_regex(regex)?;
        Ok(TokenDef { token_type: token_type.into(), regex })
    }

    /// builds a regex string from the supplied value with the format `\A( {regex} )`. This
    /// ensures that the token definition requires that a token be next in the content when
    /// matching.
    fn build_regex(regex: &str) -> Result<Regex, String> {
        let regex = format!("\\A({regex})");
        match Regex::new(&regex){
            Ok(regex) => Ok(regex),
            Err(_) => Err(format!("Cannot Build Token Definition - Regex Error for: {regex:}"))
        }
    }
}

/// Works with the Parser to create a stream of Tokens from a Reader.
pub struct Lexer {
    definitions: HashMap<String, TokenDef>,
}

impl Lexer {
    /// Create a new tokenizer to parse the code source reader.
    pub fn new() -> Lexer {
        Lexer { definitions: HashMap::new() }
    }

    // -=-=- Define Token -=-=- //

    /// Add or replace a token definition in the current possible tokens that the 
    /// Lexer can parse.
    /// 
    /// ---
    /// 
    /// ## Example
    /// 
    /// ```
    /// use interpreter::lexer::{Lexer, TokenDef};
    /// let mut lexer = Lexer::new();
    /// 
    /// lexer.define("value:ident", "[a-zA-Z_]+")?;
    /// lexer.define("value:num", "[0-9]+")?;
    /// Ok::<(), String>(())
    /// ```
    pub fn define(&mut self, token_type: &str, regex: &str) -> Result<(), String> {
        self.define_token(TokenDef::new(token_type, regex)?);
        Ok(())
    }

    /// take ownership of a token definition an add it to the current possible
    /// tokens that the Lexer can parse.
    /// 
    /// ---
    /// 
    /// ## Example
    /// 
    /// ```
    /// use interpreter::lexer::{Lexer, TokenDef};
    /// let mut lexer = Lexer::new();
    /// 
    /// lexer.define_token(TokenDef::new("value:ident", "[a-zA-Z_]+")?);
    /// lexer.define_token(TokenDef::new("value:num", "[0-9]+")?);
    /// Ok::<(), String>(())
    /// ```
    pub fn define_token(&mut self, def: TokenDef) {
        // println!("{:#?}", def);
        self.definitions.insert(def.token_type.to_owned(), def);
    }

    // -=-=- Get Token -=-=- //

    /// Get the next token in the reader only if it is defined and the token type 
    /// matches the passed `token_type`.
    /// 
    /// ---
    /// 
    /// ## Example
    /// 
    /// ```
    /// use interpreter::lexer::{Lexer, TokenDef, LineReader};
    /// let reader = LineReader::new("12345abcdefg");
    /// let mut lexer = Lexer::new();
    /// lexer.define("num", "[0-9]+")?;
    /// let token = lexer.get_next_token("num", &reader);
    /// 
    /// let token = token.ok_or("Couldn't find token")?;
    /// assert_eq!(token.token_type, "num");
    /// assert_eq!(token.value, "12345");
    /// Ok::<(), String>(())
    /// ```
    pub fn get_next_token<T>(&self, token_type: &str, reader: &T) -> Option<Token>
    where T: Reader {
        let def = self.definitions.get(token_type)?;
        self.get_next(def, reader)
    }

    /// Get the next token in the reader that matches any of the defined token types.
    /// 
    /// ---
    /// 
    /// ## Example
    /// 
    /// ```
    /// use interpreter::lexer::{Lexer, TokenDef, LineReader};
    /// let reader = LineReader::new("12345abcdefg");
    /// let mut lexer = Lexer::new();
    /// lexer.define("num", "[0-9]+")?;
    /// let token = lexer.get_next_any(&reader);
    /// 
    /// let token = token.ok_or("Couldn't find token")?;
    /// assert_eq!(token.token_type, "num");
    /// assert_eq!(token.value, "12345");
    /// Ok::<(), String>(())
    /// ```
    pub fn get_next_any<T>(&self, reader: &T) -> Option<Token>
    where T: Reader {
        for key in self.definitions.keys() {
            let def = match self.definitions.get(key) {
                Some(tok) => tok,
                None => continue,
            };
            if let Some(t) = self.get_next(def, reader) {
                return Some(t);
            }
        }
        None
    }

    /// Get the next token in the reader that matches the provided token definition.
    /// 
    /// ---
    /// 
    /// ## Example
    /// 
    /// ```
    /// use interpreter::lexer::{Lexer, TokenDef, LineReader};
    /// let reader = LineReader::new("12345abcdefg");
    /// let mut lexer = Lexer::new();
    /// let token_def = TokenDef::new("num", "[0-9]+")?;
    /// let token = lexer.get_next(&token_def, &reader);
    /// 
    /// let token = token.ok_or("Couldn't find token")?;
    /// assert_eq!(token.token_type, "num");
    /// assert_eq!(token.value, "12345");
    /// Ok::<(), String>(())
    /// ```
    pub fn get_next<T>(&self, def: &TokenDef, reader: &T) -> Option<Token>
    where T: Reader {
        if let Some((value, position)) = reader.read_regex(&def.regex) {
            return Some(Token::new( &def.token_type, value, position));
        }
        None
    }
}
