use common::lexer::{LexRule, Ruleset, ruleset_from_dir, Token};

pub struct Lexer {
    rules: Vec<LexRule>,
    is_verbose: bool
}

impl Lexer {
    pub fn new(rules: Vec<LexRule>, verbose: bool) -> Lexer {
        let mut lrules = rules.clone();
        lrules.sort();
        Lexer {
            rules: lrules,
            is_verbose: verbose
        }
    }

    pub fn new_silent(rules: Vec<LexRule>) -> Lexer {
        Lexer::new(rules, false)
    }

    pub fn new_verbose(rules: Vec<LexRule>) -> Lexer {
        Lexer::new(rules, true)
    }

    pub fn from_dir(dir_path: String, is_verbose: bool) -> Lexer {
        let mut rules = ruleset_from_dir(dir_path).unwrap().rules();
        rules.sort();
        if is_verbose {
            let ref r = rules;
            println!("Compiled list of {} lexical rules", r.len());
            for rule in r {
                println!("    :: {}", rule);
            }
        }

        Lexer::new((rules), is_verbose)
    }

    pub fn from_dir_silent(dir_path: String) -> Lexer {
        Lexer::from_dir(dir_path, false)
    }

    pub fn from_dir_verbose(dir_path: String) -> Lexer  {
        Lexer::from_dir(dir_path, true)
    }

    pub fn set_verbose(mut self, verbose:bool) {
        if verbose {
            println!("Enabling verbose mode in Lexer");
        }
        self.is_verbose = verbose;
    }

    pub fn tokenise(&self, src: String) -> Vec<Token> {
        vec![]
    }
}
