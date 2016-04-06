use common::lexer::LexRule;

#[derive(Debug)]
pub struct Ruleset {
    rules: Vec<LexRule>
}

impl Ruleset {
    pub fn new() -> Ruleset {
        Ruleset {
            rules: Vec::new()
        }
    }

    pub fn add(&mut self, rule:LexRule) -> () {
        self.rules.push(rule);
    }

    pub fn complete(&mut self) -> &[LexRule] {
        let mut set = self.rules.as_mut_slice();
        //TODO : Better sorting here
        set.sort();
        set
    }
}
