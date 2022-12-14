// use std::fmt::{self};

use std::rc::Rc;

use super::room::Room;
// use std::option::Option;

pub struct Rule {
    pub action_name: String,
    // pub text_for_user: Box<dyn Fn() -> Option<String>>, // I want to compute this function every time I call get_outcome
    pub text_for_user: Option<String>,
    pub outcome: Option<Rc<dyn RuleEngine>>,
}

impl Rule {
    fn evaluate(&self, action_expression: &str) -> bool {
        self.action_name.eq(action_expression)
    }
    // Prints the text from the user action and Returns a Room or None if the action does not lead to another room
    fn get_outcome(&self) -> Option<Rc<dyn RuleEngine>> {
        match &self.text_for_user {
            Some(text) => println!("{}", text),
            None => (),
        }

        match &self.outcome {
            Some(my_rc) => Some(my_rc.clone()),
            None => None,
        }
        // self.outcome.map(|my_rc| Rc::clone(my_rc))
    }
}

// Does the user input parsing. If it doesn't match anything returns a &None
pub trait RuleEngine: Room {
    fn process(&self, user_input: &str) -> Option<Rc<dyn RuleEngine>> {
        let first_rule = self
            .get_possible_actions()
            .into_iter()
            .find(|rule| rule.evaluate(user_input));

        let outcome = match first_rule {
            Some(first_rule) => self.execute_rule(&first_rule),
            None => {
                println!("{}", "This input was invalid!!!");
                None
            }
        };

        outcome
    }
    // fn get_rules(&self) -> &Vec<Rule>;

    // fn execute_rule(&self, first_rule: &Rule) -> &Option<Box<dyn Room>> {
    //     first_rule.get_outcome()
    // }
    fn execute_rule<'a>(&'a self, first_rule: &'a Rule) -> Option<Rc<dyn RuleEngine>> {
        first_rule.get_outcome()
    }

    // fn get_outcome_for_invalid_input() -> Option<Box<dyn Room>> {
    //     println!("{}", format!("{}", "this input was invalid").pink());
    //     None
    // }
}
