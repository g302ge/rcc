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

pub enum Keyword {
    Const,
    Static,
    Int,
    Float,
    Unsigned,
    Long,
    LongLong,
    For,
    If,
    Else,
    While,
    Do,
    Loop,
    Switch,
    Case,
}

// TODO: we need Integer enum to identify the integer parse form stream ?
// TODO: we need Float enum to identify the flot number ?

/// Token is the Lexer generated type to identify the Lex elements 
/// Token.0 is row and Token.1 is col always 
pub enum Token {
    Whitespace(usize, usize, Whitespace),
    Character(usize, usize, char),
    StringIteral(usize, usize, String), // FIXME: should check the name ?
    // TODO: Integer and Float
    Plus(usize, usize), // Token +
    PlusPlus(usize, usize), // Token ++
    Minus(usize, usize), // Token -
    MinusMinus(usize, usize), // Token --
    Multiply(usize, usize), // Token * TODO: not only multiply but pointer
    Divide(usize, usize),  // Token / TODO: not only divide but slash for comment
    Modular(usize, usize), // Token %
    LeftShift(usize, usize), // Token <<
    RightShift(usize, usize), // Token >>
    Gt(usize, usize), // Token >
    Ge(usize, usize), // Token >=
    Lt(usize, usize), // Token <
    Le(usize, usize), // Token <=
    Equal(usize,usize), // Token = 
    EuqalEqual(usize, usize), // Token == 
    Not(usize, usize), // Token !
    And(usize, usize), // Token &
    AndAnd(usize, usize), // Token &&
    Or(usize, usize), // Token |
    OrOr(usize, usize), // Token ||

    Keyword(usize, usize, Keyword), // Token for keywords see Keyword enum
    Identifier(usize, usize, String), // Token for ID
    Label(usize, usize, String), // Token label format xxx:     
}

/// Error of Lexer parsing
#[derive(Debug, PartialEq)]
pub struct LexerError {
    message: String,
    row: usize,
    col: usize,
}

/// Lexer to parse the file to Token stream 
pub struct Lexer<'a>{
    row: usize,
    col: usize,
    source: &'a mut Peekable<Chars<'a>>,
}

/// Lexer 
/// implement the `Iterator` trait to generate `Token`
/// if next function get `None` identify the file is finished
impl<'a> Lexer<'a> {
    // create from peakable 
    pub fn new(source: &'a mut Peekable<Chars<'a>>) -> Self {
        Lexer {
            col: 0,
            row: 1,
            source: source,
        }
    }
    

    // consume current Token from parse logic 
    // and modify the row and col counter
    fn consume(&mut self, token: Token, row: usize, col: usize) -> Option<Result<Token, LexerError>> {
        self.source.next();
        if row > 0 {
            self.row += row;
        }
        if col == 0 {
            self.col = 0;
        } else {
            self.col += col;
        }
        Some(Ok(token))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.source.peek() {
            // how to handle the logic in the char or string \n and \t when use the buffer logic 
            // because of the first character is identify by the ' ??
            Some(ref ch) => match ch {
                 ' ' => self.consume(
                        Token::Whitespace(self.row, self.col, Whitespace::Space),
                        0,
                        1
                     ),
                 '\t' => self.consume(
                        Token::Whitespace(self.row, self.col, Whitespace::Tab),
                        0,
                        4
                     ),
                 '\n' => self.consume(
                        Token::Whitespace(self.row, self.col, Whitespace::Newline),
                        1,
                        0
                     ),
                 '\r' => {
                    self.source.next();
                    if let Some('\n') = self.source.peek() {
                        self.source.next();
                    }
                    self.consume(
                            Token::Whitespace(self.row, self.col, Whitespace::Newline),
                            1,
                            0
                        )
                 },
                 '/' => {
                    // have to handle not only // but also /*xxx*/ format comment 
                    None
                 },
                 '+' => None,
                 '-' => None,
                 '*' => None, // multiply and pointer
                 '%' => None,
                 '&' => None, // & &&
                 '|' => None, // | ||
                 '^' => None,
                 '<' => None, // < <= << 
                 '>' => None, // > >= >>
                 '=' => None, // = == 
                 '!' => None, // ! not
                 ';' => None,
                 ',' => None,
                 '.' => None,
                 '(' => None,
                 ')' => None,
                 '[' => None,
                 ']' => None,
                 '{' => None,
                 '}' => None,
                 'a'..='z' | 'A'..='Z' | '_' => None, // identifier also keyword 
                 '\'' => None, // char const,
                 '"' => None, // string const,
                 '1'..='9' => None, // number int float double 
                 '0' => None, // 0x int number
                 _ => Some(Err(
                        LexerError{
                            message: format!("unexcept character {}", ch),
                            row: self.row,
                            col: self.col
                        }
                     ))
            },
            // at this point identify the lexer arrived end of file
            None => None
        }
    }

}


#[cfg(test)]
mod test {
    use crate::lex;

    #[test]
    fn single_character_test() {
    
    }

}
