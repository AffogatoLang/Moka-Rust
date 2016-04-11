use common::lexer::LexRule;

#[derive(Debug, Clone)]
pub struct Ruleset{
    rules: Vec<LexRule>
}

impl Ruleset {
    pub fn new() -> Ruleset {
        Ruleset {
            rules: Vec::new()
        }
    }

    pub fn add(&mut self, rule:LexRule) {
        self.rules.push(rule);
    }

    pub fn sort(&mut self) {
        self.rules.sort();
    }

    pub fn rules(self) -> Vec<LexRule> {
        self.rules
    }
}
