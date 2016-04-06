use std::cmp;
use std::option::Option;
use pcre::{CompileOption, Match, Pcre};

pub struct LexRule {
    priority: i32,
    token: String,
    expr: Pcre
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
