use std::iter::{Iterator};
use std::io;
use core::iter::Peekable;
use core::str::Chars;
use std::error::Error;


pub enum Whitespace {
    Space,
    Tab,
    Newline
}

pub enum Token {
    Whitespace(Whitespace)
}


pub struct Lexer<'a>{
    row: usize,
    col: usize,
    source: &'a mut Peekable<Chars<'a>>,
}

/// Lexer 
/// implement the `Iterator` trait to generate `Token`
impl<'a> Lexer<'a> {

    // create from peakable 
    pub fn new(source: &'a mut Peekable<Chars<'a>>) -> Self {
        Lexer {
            col: 0,
            row: 0,
            source: source,
        }
    }
    
    // call next and return token 
    // FIXME: should define some tokenize error replace the error 
    // And the result should be panic ?
    fn comsume_and_return(&mut self, t: Token) -> Option<Result<Token, Box<dyn Error>>> {
        self.source.next();
        Some(Ok(t))
    }

    fn parse_word() {
        // TODO: should identify the word to keyword or identifier
    }

    fn parse_str_const() {

    }

    fn parse_int_const() {

    }

    fn parse_float_const() {

    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, Box<dyn Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.source.peek() {
            // how to handle the logic in the char or string \n and \t when use the buffer logic 
            // because of the first character is identify by the ' ??
            Some(ref ch) => match ch {
                 ' ' => self.comsume_and_return(Token::Whitespace(Whitespace::Space)),
                 '\t' => self.comsume_and_return(Token::Whitespace(Whitespace::Tab)),
                 '\n' => self.comsume_and_return(Token::Whitespace(Whitespace::Newline)),
                 '\r' => {
                    self.source.next();
                    if let Some('\n') = self.source.peek() {
                        self.source.next();
                    }
                    self.comsume_and_return(Token::Whitespace(Whitespace::Newline))
                 },
                 _ => self.comsume_and_return(Token::Whitespace(Whitespace::Newline))
            },
            None => None
        }
    }

}


#[cfg(test)]
mod test {

}
