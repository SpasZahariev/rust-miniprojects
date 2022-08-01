use super::room::{Room, RoomType};

pub struct Kitchen {
    pub room_name: String,
    pub possible_actions: Vec<String>,
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
        println!("You have entered the {}", self.room_name);
    }

    fn display_possible_actions(&self) {
        println!("Here you can: {:?}", self.possible_actions);
    }

    fn get_room_type(&self) -> RoomType {
        RoomType::Kitchen
    }
}
