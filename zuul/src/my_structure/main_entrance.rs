use std::collections::HashMap;

use super::{
    direction::Direction,
    room::{Room, RoomType},
    rule_engine::{Rule, RuleEngine},
};

#[derive(Default)]
pub struct MainEntrance {
    // pub possible_actions: Vec<String>,
    // pub exits: HashMap<Direction, Box<dyn Room>>,
    pub possible_actions: Vec<Rule>,
}

impl MainEntrance {
    pub fn ring_reception_bell(&self) {
        println!("");
    }

    pub fn look_around(&self) {
        println!("");
    }
}

impl Room for MainEntrance {
    fn get_possible_actions(&self) -> Vec<Rule> {
        self.possible_actions
    }

    fn get_room_type(&self) -> RoomType {
        RoomType::MainEntrance
    }

    fn add_possible_action(&mut self, rule: Rule) {
        self.possible_actions.push(rule);
    }
}

impl RuleEngine for MainEntrance {
    // fn get_rules(&self) -> &Vec<Rule> {
    //     &self.possible_actions
    // }
}
