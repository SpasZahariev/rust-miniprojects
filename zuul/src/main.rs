mod my_structure;

use colored::Colorize;
use my_structure::kitchen;
use my_structure::room::Room;

fn main() {
    let dungeon_kitchen = kitchen::Kitchen {
        room_name: "the kitchen".to_string(),
        possible_actions: vec![
            "try cooking something".to_string(),
            "check the cupboards".to_string(),
        ],
    };

    println!("GAME HAS BEGUN");
    println!("{:?}", dungeon_kitchen.possible_actions);
    dungeon_kitchen.knock_down_door();
    dungeon_kitchen.cook();
    dungeon_kitchen.steal_knives();

    println!("Hello traveler, you've stumbled infront of an ordinary but very peculiar house...");
    let is_running = true;

    while is_running {
        println!("{}", format!("What would you like to do?").green());
    }
}
