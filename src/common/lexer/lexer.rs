use common::lexer::LexRule;

pub struct Lexer<'a> {
    rules: &'a [LexRule],
    is_verbose: bool
}

impl<'a> Lexer<'a> {
    pub fn new(rules: &'a[LexRule]) -> Lexer {
        Lexer {
            rules: rules,
            is_verbose: false
        }
    }
    pub fn set_verbose(mut self, verbose:bool) {
        if verbose {
            println!("Enabling verbose mode in Lexer");
        }
        self.is_verbose = verbose;
    }
}
