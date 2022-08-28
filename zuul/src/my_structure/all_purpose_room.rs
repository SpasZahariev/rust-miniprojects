use super::{
    room::{Room, RoomType},
    rule_engine::{Rule, RuleEngine},
};

pub struct AllPurposeRoom {
    pub room_type: RoomType,
    pub possible_actions: Vec<Rule>,
}

impl AllPurposeRoom {
    pub fn new(type_input: RoomType) -> Self {
        AllPurposeRoom {
            room_type: type_input,
            possible_actions: vec![],
        }
    }
}

impl Room for AllPurposeRoom {
    fn get_possible_actions(&self) -> &Vec<Rule> {
        &self.possible_actions
    }

    fn get_room_type(&self) -> RoomType {
        self.room_type
    }

    fn add_possible_action(&mut self, rule: Rule) {
        self.possible_actions.push(rule);
    }
}

impl RuleEngine for AllPurposeRoom {}
