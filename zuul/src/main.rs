mod my_structure;

use colored::Colorize;
use my_structure::direction::Direction;
use my_structure::room::Room;
use my_structure::{entrance, kitchen};
use std::collections::HashMap;
use std::io::stdin;

use my_structure::entrance::Entrance;

fn main() {
    println!("{}", format!("\n\n\nGAME HAS BEGUN").purple());
    // println!("{:?}", dungeon_kitchen.possible_actions);
    // dungeon_kitchen.knock_down_door();
    // dungeon_kitchen.cook();
    // dungeon_kitchen.steal_knives();

    println!(
        "{}",
        format!("Hello traveler, you've stumbled before an ordinary but very peculiar house...\n")
            .purple()
    );
    let is_running = true;

    while is_running {
        println!(
            "{}{}{}",
            format!("What would you like to do? (Type").green(),
            format!(" actions").red(),
            format!(" to see what you can do)").green()
        );
        let mut user_input = String::new();
        stdin().read_line(&mut user_input).unwrap();
        println!("this is what I saw: {}", user_input);
    }
}

fn make_game() -> Box<dyn Room> {
    let dungeon_kitchen = Box::new(kitchen::Kitchen {
        possible_actions: vec![
            "try cooking something".to_string(),
            "check the cupboards".to_string(),
        ],
        exits: HashMap::new(),
    });
    let temp_entrance = Box::new(entrance::Entrance {
        possible_actions: vec![
            "try cooking something".to_string(),
            "check the cupboards".to_string(),
        ],
        exits: HashMap::new(),
    });
    //    let HashMap<Direction, Box<dyn Room>: my_map = HashMap::from([ (Direction::EAST, Box::new(dungeon_kitchen)), (Direction::EAST,
    //    Box::new(dungeon_kitchen)), ]);

    let mut random_map: HashMap<Direction, Box<dyn Room>> = HashMap::new();
    random_map.insert(Direction::EAST, dungeon_kitchen);
    random_map.insert(Direction::WEST, temp_entrance);

    /*     let random_map2 = HashMap::<Direction, Box<dyn Room>>::from([
        (Direction::EAST, dungeon_kitchen),
        (Direction::WEST, temp_entrance),
    ]);

    let random_map3: HashMap<Direction, Box<dyn Room>> = [
        (Direction::WEST, temp_entrance),
        (Direction::EAST, dungeon_kitchen),
    ]
    .into::<Direction, Box<dyn Room>>(); */

    // random_map.insert(Direction::EAST, dungeon_kitchen);
    // random_map.insert(Direction::WEST, temp_entrance);

    let entrance = entrance::Entrance {
        possible_actions: vec!["ring bell".to_string(), "look around yourself".to_string()],
        exits: random_map,
    };
    Box::new(entrance)
}
