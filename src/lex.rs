use std::iter::{Iterator};
use std::io;



pub enum Token {

}


pub struct Lexer {

    row: usize,
    col: usize
}

/// Lexer 
/// implement the `Iterator` trait to generate `Token`
impl Lexer {
    
    pub fn new(file: &str) -> Self {
        unimplemented!()
    }

    pub fn from_slice(slice: &[char]) -> Self {
        unimplemented!()
    }

    // fill the inner buffer read from current file with only one line 
    fn fill_buffer(&mut self) -> Result<(), io::Error> {
        unimplemented!()
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}


#[cfg(test)]
mod test {

}
