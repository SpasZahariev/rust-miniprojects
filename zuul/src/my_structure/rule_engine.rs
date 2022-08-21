// use std::fmt::{self};

use super::room::Room;
// use std::option::Option;
pub struct Rule {
    pub action_name: String,
    pub outcome: Option<Box<dyn Room>>,
}

impl Rule {
    fn evaluate(&self, action_expression: &str) -> bool {
        self.action_name.eq(action_expression)
    }
    fn get_outcome(&self) -> &Option<Box<dyn Room>> {
        &self.outcome
    }
}
pub trait RuleEngine {
    fn get_rules(&self) -> &Vec<Rule>;
    fn process(&self, user_input: &str) -> &Option<Box<dyn Room>> {
        let first_rule = self
            .get_rules()
            .into_iter()
            .find(|rule| rule.evaluate(user_input));

        let outcome = match first_rule {
            Some(first_rule) => first_rule.get_outcome(),
            None => &None,
        };

        outcome
    }
}
