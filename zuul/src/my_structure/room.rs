use std::fmt::{self};

use super::{kitchen::Kitchen, main_entrance::MainEntrance};

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
    fn display_possible_actions(&self);
    fn get_room_type(&self) -> RoomType;
}
