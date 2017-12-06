use common::lexer::{LexRule, ruleset_from_dir_v, Token};
use std::vec::Vec;
use common::util;

pub struct Lexer {
    rules: Vec<LexRule>,
    is_verbose: bool,
    ignore_whitespace: bool
}

impl Lexer {
    pub fn new(rules: Vec<LexRule>, verbose: bool) -> Lexer {
        let mut lrules = rules.clone();
        lrules.sort();
        Lexer {
            rules: lrules,
            is_verbose: verbose,
            ignore_whitespace: false
        }
    }

    pub fn new_silent(rules: Vec<LexRule>) -> Lexer {
        Lexer::new(rules, false)
    }

    pub fn new_verbose(rules: Vec<LexRule>) -> Lexer {
        Lexer::new(rules, true)
    }

    pub fn from_dir(dir_path: String, is_verbose: bool) -> Lexer {
        let mut rules = ruleset_from_dir_v(dir_path, is_verbose).unwrap().rules();
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

    pub fn set_verbose(&mut self, verbose:bool) {
        if verbose {
            println!("Enabling verbose mode in Lexer");
        }
        self.is_verbose = verbose;
    }

    pub fn set_ignore_whitespace(&mut self, ignore_whitespace: bool) {
        self.ignore_whitespace = ignore_whitespace;
    }

    pub fn tokenise(&mut self, file: (&String, &String)) -> Result<Vec<Token>, String> {
        if self.is_verbose {
            println!("Tokenising Input");
            println!("    :: Ignore Whitespace [{:?}]", self.ignore_whitespace);
        }
        let filename = file.0;
        let src = file.1;

        let mut tokens : Vec<Token> = Vec::new();

        let mut stub : &str;

        let mut index : usize = 0;
        let mut prev_index = index;
        let src_len = src.len();

        let mut eof_or_stop = false;
        let mut loop_check = false;
        let mut found_token;

        while !eof_or_stop {
            if loop_check {
                if index == prev_index {
                    panic!("Loop has iterated twice without changing index value, possible infinite loop");
                } else {
                    loop_check = false;
                }
            }
            if index == prev_index {
                loop_check = true;
            }
            prev_index = index;
            stub = src.split_at(index).1;

            if self.ignore_whitespace && stub.chars().next().unwrap().is_whitespace() {
                index = index + 1;
                found_token = true; // Emulate found token for parsing whitespace
            } else {
                found_token = false;
                let stub_string = &stub.to_string();
                for rule in &self.rules {
                    let mut l_rule = rule.clone();
                    let m = l_rule.match_at(&stub_string, 0);
                    if m.is_some() {
                        let m_val = m.unwrap();

                        let t_content = if m_val.string_count() > 1 {
                            m_val.group(1)
                        } else {
                            m_val.group(0)
                        };

                        let token = Token::new(
                            rule.clone_ident(),
                            t_content.into(),
                            filename.clone(),
                            util::get_line_col(&src, index)
                        );

                        if self.is_verbose {
                            println!("Created new token {}", token);
                        }

                        tokens.push(token);

                        found_token = true;
                        index = index + m_val.group_end(0);
                        break;
                    }
                }
            }
            if !found_token {
                // TODO: Better error messages
                if !self.ignore_whitespace && stub.chars().next().unwrap().is_whitespace() {
                    return Err(util::generate_syntax_error_with_slug_from_index(src, index, "Encountered unhandled whitespace"));
                } else {
                    return Err(util::generate_syntax_error_with_slug_from_index(src, index, "No rule for encountered sequence"));
                }
            }
            if index == src_len {
                eof_or_stop = true;
            }
        }
        Ok(tokens)
    }
}
