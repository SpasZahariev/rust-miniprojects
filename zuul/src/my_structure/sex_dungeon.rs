use std::collections::HashMap;

use super::{
    direction::Direction,
    room::{Room, RoomType},
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

    fn display_possible_actions(&self) {
        println!("Here you can: {:?}", self.possible_actions);
    }

    fn get_room_type(&self) -> RoomType {
        RoomType::SexDungeon
    }
}
