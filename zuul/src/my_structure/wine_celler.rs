use super::{
    room::{Room, RoomType},
    rule_engine::{Rule, RuleEngine},
};

#[derive(Default)]
pub struct WineCeller {
    pub possible_actions: Vec<Rule>,
}

impl Room for WineCeller {
    fn get_possible_actions(&self) -> &Vec<Rule> {
        &self.possible_actions
    }

    fn get_room_type(&self) -> RoomType {
        RoomType::WineCeller
    }

    fn add_possible_action(&mut self, rule: Rule) {
        self.possible_actions.push(rule);
    }
}

impl RuleEngine for WineCeller {}
