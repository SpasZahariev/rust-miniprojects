use std::collections::HashMap;

use super::{
    direction::Direction,
    room::{Room, RoomType},
};

pub struct Kitchen {
    pub possible_actions: Vec<String>,
    pub exits: HashMap<Direction, Box<dyn Room>>,
}

impl Kitchen {
    pub fn steal_knives(&self) {
        println!("You found some shiny knives in one of the lockers. Someone has sharpenned them recently");
    }

    pub fn cook(&self) {
        println!("You tried to cook something. It was very unssucessful!");
    }
}

impl Room for Kitchen {
    fn knock_down_door(&self) {
        println!(
            "Tasty tasty Overridden Food... You have entered the {}",
            self.get_room_type()
        );
    }

    fn display_possible_actions(&self) {
        println!("Here you can: {:?}", self.possible_actions);
    }

    fn get_room_type(&self) -> RoomType {
        RoomType::Kitchen
    }
}
