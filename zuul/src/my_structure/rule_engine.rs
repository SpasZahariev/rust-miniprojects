// use std::fmt::{self};

use super::room::Room;
// use std::option::Option;
pub struct Rule {
    pub action_name: String,
    pub text_for_user: Option<String>,
    pub outcome: Option<Box<dyn Room>>,
}

impl Rule {
    fn evaluate(&self, action_expression: &str) -> bool {
        self.action_name.eq(action_expression)
    }
    // Prints the text from the user action and Returns a Room or None if the action does not lead to another room
    fn get_outcome(&self) -> &Option<Box<dyn Room>> {
        match self.text_for_user {
            Some(text) => println!("{}", text),
            None => (),
        }
        &self.outcome
    }
}

// Does the user input parsing. If it doesn't match anything returns a &None
pub trait RuleEngine: Room {
    fn process(&self, user_input: &str) -> &Option<Box<dyn Room>> {
        let first_rule = self
            .get_possible_actions()
            .into_iter()
            .find(|rule| rule.evaluate(user_input));

        let outcome = match first_rule {
            Some(first_rule) => self.execute_rule(&first_rule),
            None => &None,
        };

        outcome
    }
    // fn get_rules(&self) -> &Vec<Rule>;

    fn execute_rule(&self, first_rule: &Rule) -> &Option<Box<dyn Room>> {
        first_rule.get_outcome()
    }
}
