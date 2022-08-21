use std::collections::HashMap;

use super::{
    direction::Direction,
    room::{Room, RoomType},
    rule_engine::Rule,
};

pub struct SexDungeon {
    possible_actions: Vec<String>,
    exits: HashMap<Direction, Box<dyn Room>>,
}

impl SexDungeon {
    pub fn do_the_nasty() {
        println!("Wow... You must be pretty desperate to try that here!");
    }
}

impl Room for SexDungeon {
    fn knock_down_door(&self) {
        todo!()
    }

    fn get_room_type(&self) -> RoomType {
        RoomType::SexDungeon
    }

    fn get_possible_actions(&self) -> &Vec<Rule> {
        todo!()
    }

    fn add_possible_action(&mut self, rule: Rule) {
        todo!()
    }
}
