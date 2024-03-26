//! # Tokens and Tokenizer
//! 
//! Contains structures for managing and building tokens.
//! 

use std::collections::HashMap;

use regex::Regex;

use super::{ReadPointer, Reader};

/// A raw token object.
#[derive(Clone, Debug)]
pub struct Token {
    // should be an enum but I prefer a hierarchical naming.
    pub token_type: String,
    /// Raw value of the token
    pub value: String,
    /// The read position of where the token was found.
    pub position: ReadPointer
}

impl Token {
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
    pub fn new(token_type: &str, regex: &str) -> Result<TokenDef, String> {
        let regex = TokenDef::build_regex(regex)?;
        Ok(TokenDef { token_type: token_type.into(), regex })
    }

    /// builds a regex string 
    /// 
    /// # NOT FULLY IMPLEMENTED!
    fn build_regex(regex: &str) -> Result<Regex, String> {
        let regex = format!("\\A({regex})");
        match Regex::new(&regex){
            Ok(regex) => Ok(regex),
            Err(err) => Err(format!("{err:?}"))
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

    pub fn define(&mut self, def: TokenDef) {
        // unimplemented!()
        println!("{:#?}", def);
        self.definitions.insert(def.token_type.to_owned(), def);
    }

    // -=-=- Get Token -=-=- //

    pub fn get_next_token<T>(&self, token_type: &str, reader: &T) -> Option<Token>
    where T: Reader {
        let def = self.definitions.get(token_type)?;
        self.get_next(def, reader)
    }

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

    pub fn get_next<T>(&self, def: &TokenDef, reader: &T) -> Option<Token>
    where T: Reader {
        if let Some((value, position)) = reader.read_regex(&def.regex) {
            // this should not fail - if it does it means there is a pointer mismatch
            // let position = reader.get_pointer_next(value).ok()?;
            return Some(Token::new( &def.token_type, value, position));
        }
        None
    }
}
