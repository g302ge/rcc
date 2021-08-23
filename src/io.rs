

pub<'a> trait LetterStream<'a> {
    
    fn new(destination: &str) -> Self;

    fn next(&mut self) -> Result<char, std::io::Error>;
    
    fn scan(&'a self) -> Result<&'a str, std::io::Error>;
}



pub struct FileStream {

}

impl<'a> LetterStream<'a> for FileStream<'a>{

}


// TODO: the source code could be fetch from remote over TCP
