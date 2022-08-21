use std::collections::HashMap;

use super::{
    direction::Direction,
    room::{Room, RoomType},
    rule_engine::{Rule, RuleEngine},
};

pub struct MainEntrance {
    // pub possible_actions: Vec<String>,
    // pub exits: HashMap<Direction, Box<dyn Room>>,
    pub possible_actions: Vec<Rule>,
}

impl MainEntrance {
    pub fn ring_reception_bell(&self) {
        println!("The bell chimes with a small zingg");
    }

    pub fn look_around(&self) {
        println!("The place is decorated very plainly. I guess the owners are fans of IKEA minimalist designs");
    }
}

impl Room for MainEntrance {
    fn display_possible_actions(&self) {
        let action_names: Vec<&str> = self
            .possible_actions
            .into_iter()
            .map(|rule| rule.action_name.as_str())
            .collect();

        println!("Here you can: {:?}", action_names);
    }

    fn get_room_type(&self) -> RoomType {
        RoomType::MainEntrance
    }
}

impl RuleEngine for MainEntrance {
    fn get_rules(&self) -> &Vec<Rule> {
        &self.possible_actions
    }
}
