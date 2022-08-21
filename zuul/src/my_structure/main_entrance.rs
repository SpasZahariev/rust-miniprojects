use std::collections::HashMap;

use super::{
    direction::Direction,
    room::{Room, RoomType},
    rule_engine::{Rule, RuleEngine},
};

pub struct MainEntrance {
    pub possible_actions: Vec<String>,
    pub exits: HashMap<Direction, Box<dyn Room>>,
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
        println!("Here you can: {:?}", self.possible_actions);
    }

    fn get_room_type(&self) -> RoomType {
        RoomType::MainEntrance
    }
}

impl RuleEngine for MainEntrance {
    fn get_rules(&self) -> &Vec<Rule> {
        todo!()
    }
}
