mod my_structure;

use colored::Colorize;
use my_structure::direction::Direction;
use my_structure::room::{Room, RoomType};
use my_structure::rule_engine::RuleEngine;
use my_structure::{kitchen, main_entrance, rule_engine, wine_celler, all_purpose_room};
use std::collections::HashMap;
use std::io::stdin;
use std::rc::Rc;

use my_structure::main_entrance::MainEntrance;

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

    let mut current_room = make_game();

    current_room.knock_down_door();
    while is_running {
        println!(
            "{}{}{}",
            format!("\nWhat would you like to do? (Type").green(),
            format!(" actions").red(),
            format!(" to see what you can do)").green()
        );
        let mut user_input = String::new();
        stdin().read_line(&mut user_input).unwrap();
        let user_input = user_input.trim(); // remove the endline characters from the input
        // println!("this is what I saw: {}", user_input);

        /*         println!("{:?}", user_input.trim().eq("123"));
        println!("{:?}", "123".eq("123"));
        println!("{:?}", "xyz" == "xyz"); */
        if user_input.eq("exit") {
            println!("{}", format!("Thank you for playing! - GAME OVER").purple());
            break;
        }
        let outcome: Option<Rc<dyn RuleEngine>> = current_room.process(user_input);

        if outcome.is_some() {
            current_room = outcome.unwrap();
            current_room.knock_down_door();
        }

    }
}

fn make_game() -> Rc<dyn RuleEngine> {
    let mut my_entrance = all_purpose_room::AllPurposeRoom::new(RoomType::MainEntrance);
    let main_entrance_available_actions = rule_engine::Rule {
        action_name: "actions".to_owned(),
        // text_for_user: my_entrance.get_possible_action_names(),
        text_for_user: Some("actions, look_around, ring_reception_bell, go_left, go_forward, exit".to_owned()),
        outcome: None,
    };

    let look_around_action = rule_engine::Rule {
        action_name: "look_around".to_owned(),
        text_for_user: 
            Some("The place is decorated very plainly. I guess the owners are fans of IKEA minimalist designs".to_owned()),
        outcome: None,
    };

    let ring_bell_action = rule_engine::Rule {
        action_name: "ring_reception_bell".to_owned(),
        text_for_user: Some("The bell chimes with a small zingg".to_owned()),
        outcome: None,
    };
    // let mut the_kitchen = kitchen::Kitchen::default();
    
    let mut the_wine_place= all_purpose_room::AllPurposeRoom::new(RoomType::MainEntrance);
    let knock_on_wine_barrels = rule_engine::Rule {
        action_name: "knock_barrels".to_owned(),
        text_for_user: Some("You hear a hollow sound. They must be empty".to_owned()),
        outcome: None,
    };
    let wine_celler_actions = rule_engine::Rule {
        action_name: "actions".to_owned(),
        // text_for_user: my_entrance.get_possible_action_names(),
        text_for_user: Some("actions, knock_barrels, exit".to_owned()),
        outcome: None,
    };
    the_wine_place.add_possible_action(knock_on_wine_barrels);
    the_wine_place.add_possible_action(wine_celler_actions);

    let go_to_left_room = rule_engine::Rule {
        action_name: "go_left".to_owned(),
        text_for_user: None,
        outcome: Some(Rc::new(the_wine_place)),
    };
    my_entrance.add_possible_action(main_entrance_available_actions);
    my_entrance.add_possible_action(look_around_action);
    my_entrance.add_possible_action(ring_bell_action);
    my_entrance.add_possible_action(go_to_left_room);

    // let dungeon_kitchen = Box::new(kitchen::Kitchen {
    //     possible_actions: vec![
    //         "try cooking something".to_string(),
    //         "check the cupboards".to_string(),
    //     ],
    //     exits: HashMap::new(),
    // });
    // let temp_entrance = Box::new(main_entrance::MainEntrance {
    //     possible_actions: vec![
    //         "try cooking something".to_string(),
    //         "check the cupboards".to_string(),
    //     ],
    //     exits: HashMap::new(),
    // });
    //    let HashMap<Direction, Box<dyn Room>: my_map = HashMap::from([ (Direction::EAST, Box::new(dungeon_kitchen)), (Direction::EAST,
    //    Box::new(dungeon_kitchen)), ]);

    // let mut random_map: HashMap<Direction, Box<dyn Room>> = HashMap::new();
    // random_map.insert(Direction::EAST, dungeon_kitchen);
    // random_map.insert(Direction::WEST, temp_entrance);

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

    // let entrance = main_entrance::MainEntrance {
    //     possible_actions: vec!["ring bell".to_string(), "look around yourself".to_string()],
    //     exits: random_map,
    // };
    Rc::new(my_entrance)
}
