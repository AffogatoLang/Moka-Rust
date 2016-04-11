use std::fmt;
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct Token {
    ident: String,
    content: String,
    fileloc: (String, usize, usize)
}

impl Token {
    pub fn new(ident: String, content: String, file: String, location: (usize, usize)) -> Token {
        Token {
            ident: ident,
            content: content,
            fileloc: (
                file,
                location.0,
                location.1
            )
        }
    }

    pub fn line_col(&self) -> (usize, usize) {
        (self.fileloc.1, self.fileloc.2)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0} :: {1} @ {2} [{3}, {4}]",
        self.ident, self.content, self.fileloc.0, self.fileloc.1, self.fileloc.2)
    }
}
