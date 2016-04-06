use std::cmp;
use std::ops::Add;
use std::option::Option;
use pcre::{Match, Pcre};

#[derive(Debug)]
pub struct LexRule {
    priority: i32,
    token: String,
    pub src: String,
    expr: Pcre
}

impl LexRule {
    pub fn new(priority: i32, token: String, expr: String) -> Result<LexRule, String> {
        let re = match Pcre::compile(&expr) {
            Ok(regex) => regex,
            Err(_) => return Err(String::from("Unable to compile regex ").add(&expr))
        };

        Ok(LexRule {
            priority: priority,
            token: token.clone(),
            src: expr.clone(),
            expr: re
        })
    }

    pub fn match_at<'a>(&'a mut self, src:&'a String, pos:usize) -> Option<Match> {
        self.expr.exec_from(&(**src), pos)
    }
}

impl cmp::Eq for LexRule {}

impl cmp::PartialEq for LexRule {
    fn eq (&self, other:&Self) -> bool{
        (self.priority, &self.token) == (other.priority, &other.token)
    }
}

impl cmp::Ord for LexRule {
    fn cmp (&self, other: &Self) -> cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl cmp::PartialOrd for LexRule {
    fn partial_cmp (&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.priority.cmp(&other.priority))
    }
}
