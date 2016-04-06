use std::cmp;
use std::ops::Add;
use std::option::Option;
use pcre::{Match, Pcre};

#[derive(Debug)]
pub struct LexRule {
    priority: u32,
    ident: String,
    pub src: String,
    expr: Pcre
}

impl LexRule {
    pub fn new(priority: u32, ident: String, expr: String) -> Result<LexRule, String> {
        let re = match Pcre::compile(&expr) {
            Ok(regex) => regex,
            Err(_) => return Err(String::from("Unable to compile regex ").add(&expr))
        };

        Ok(LexRule {
            priority: priority,
            ident: ident.clone(),
            src: expr.clone(),
            expr: re
        })
    }

    pub fn from_string(src:String) -> Result<LexRule, String> {
        //TODO Remove unwraps, handle those errors properly
        let mut buf_priority = String::new();
        let mut buf_ident = String::new();
        let mut buf_regex = String::new();

        let mut index = src.chars()
        .position(|c| c == ':')
        .unwrap();

        let (b_pr_str, mut rest_str) = src.split_at(index);
        rest_str = rest_str.split_at(1).1.trim(); // Split to remove colon (pos 0), trim
        buf_priority = buf_priority.add(b_pr_str.trim());

        index = rest_str.chars()
        .position(|c| c == ':')
        .unwrap();

        let (b_idt_str, mut b_reg_str) = rest_str.split_at(index);
        b_reg_str = b_reg_str.split_at(1).1.trim(); // Split to remove colon (pos 0), trim
        buf_ident = buf_ident.add(b_idt_str.trim());
        buf_regex = buf_regex.add(b_reg_str.trim());

        let priority:u32 = buf_priority.parse().unwrap();
        LexRule::new(priority, buf_ident.clone(), buf_regex.clone())
    }

    pub fn match_at<'a>(&'a mut self, src:&'a String, pos:usize) -> Option<Match> {
        self.expr.exec_from(&(**src), pos)
    }
}

impl cmp::Eq for LexRule {}

impl cmp::PartialEq for LexRule {
    fn eq (&self, other:&Self) -> bool{
        (self.priority, &self.ident) == (other.priority, &other.ident)
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
