use std::collections::HashMap;

use super::{
    direction::Direction,
    room::{Room, RoomType},
};

pub struct Entrance<'a> {
    pub possible_actions: Vec<String>,
    pub exits: HashMap<Direction, &'a dyn Room>,
}

impl<'a> Entrance<'a> {
    pub fn ring_reception_bell(&self) {
        println!("The bell chimes with a small zingg");
    }

    pub fn look_around(&self) {
        println!("The place is decorated very plainly. I guess the owners are fans of IKEA minimalist designs");
    }
}

impl<'a> Room for Entrance<'a> {
    fn knock_down_door(&self) {
        println!("You have entered the {}", self.get_room_type());
    }

    fn display_possible_actions(&self) {
        println!("Here you can: {:?}", self.possible_actions);
    }

    fn get_room_type(&self) -> RoomType {
        RoomType::Entrance
    }
}
