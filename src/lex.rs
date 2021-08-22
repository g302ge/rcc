
use std::error::Error;
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
// Token 
pub enum Token {
    White
}

// Stream using the iterator to split the Token from letters Stream
pub struct Stream {
    file: File
}

impl Stream {
    

    // open file and create the Stream instance 
    pub fn new(file: &str) -> Result<Stream, Box<dyn Error>> {
        let path =  Path::new(file);
        let mut file = match File::open(&path) {
            Ok(f) => f,
            Err(e) => return Err(Box::new(e)),
        };


        Ok(Stream{
            file,
        })
    }


    // return next Token 
    // the outer should use the std::cell::RefCell<Stream> to use this function 
    pub fn next(&mut self) -> Result<Token, Box<dyn Error>> {


        Ok(Token::White)
    }
    
    // consume current token remove it from token cache
    pub fn consume(&mut self) {

    }

    // back one token 
    pub fn back(&mut self) {

    }


    // TODO: implement the lexer main logic 
    fn lex(&mut self) {
    


    }

    // TODO: how to rollback move the file pointer and re-read or cache all text 
}
