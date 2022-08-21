use std::fmt::{self};

use super::{kitchen::Kitchen, main_entrance::MainEntrance, rule_engine::Rule};

// Debug macro makes my enum usable with debug print
#[derive(Debug)]
pub enum RoomType {
    Kitchen,
    SexDungeon,
    Basement,
    Cemetary,
    Workshop,
    MainEntrance,
}

//need to implement Display so that RoomTypes can be converted to strings and printed for Client facing messages

impl fmt::Display for RoomType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "{:?}", self)
        fmt::Debug::fmt(self, f)
    }
}

pub trait Room {
    fn knock_down_door(&self) {
        println!("You have entered the {}", self.get_room_type());
    }
    fn get_possible_actions(&self) -> Vec<Rule>;

    fn get_possible_action_names(&self) -> Option<String> {
        let action_names: Vec<&str> = self
            .get_possible_actions()
            .into_iter()
            .map(|rule| rule.action_name.as_str())
            .collect();

        let output = format!("{}, {:?}", "Here you can: ", action_names);
        if action_names.len() > 0 {
            return Some(output);
        }
        None
    }
    fn get_room_type(&self) -> RoomType;
    fn add_possible_action(&mut self, rule: Rule);
}
