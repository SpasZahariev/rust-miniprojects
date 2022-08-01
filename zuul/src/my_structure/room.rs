pub enum RoomType {
    Kitchen,
    SexDungeon,
    Basement,
    Cemetary,
    Workshop,
}

pub trait Room {
    fn knock_down_door(&self);
    fn display_possible_actions(&self);
    fn get_room_type(&self) -> RoomType;
}
