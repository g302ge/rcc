use std::iter::{Iterator};
use std::io;
use core::iter::Peekable;
use core::str::Chars;



pub enum Token {

}


pub struct Lexer<'a>{
    row: usize,
    col: usize,
    buffer: &'a mut Peekable<Chars<'a>>,
}

/// Lexer 
/// implement the `Iterator` trait to generate `Token`
impl<'a> Lexer<'a> {
    
    
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}


#[cfg(test)]
mod test {

}
