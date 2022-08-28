mod my_structure;

use colored::Colorize;
use my_structure::direction::Direction;
use my_structure::room::{self, Room, RoomType};
use my_structure::rule_engine::RuleEngine;
use my_structure::{
    all_purpose_room::AllPurposeRoom, kitchen, main_entrance, rule_engine, wine_celler,
};
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
    // let mut the_kitchen = kitchen::Kitchen::default();

    let the_wine_place = setup_wine_place();

    let the_sex = setup_sex_dungeon();
    let the_kitchen = setup_kitchen(the_sex);

    let my_entrance = setup_entrance(the_wine_place, the_kitchen);

    Rc::new(my_entrance)
}

fn setup_wine_place() -> AllPurposeRoom {
    let mut the_wine_place = AllPurposeRoom::new(RoomType::MainEntrance);
    let room_specific_actions = "knock_barrels".to_owned();
    let wine_celler_actions = rule_engine::Rule {
        action_name: "actions".to_owned(),
        // text_for_user: my_entrance.get_possible_action_names(),
        text_for_user: Some(format!(
            "{}{}{}",
            "actions, ", room_specific_actions, ", exit"
        )),
        outcome: None,
    };
    let knock_on_wine_barrels = rule_engine::Rule {
        action_name: "knock_barrels".to_owned(),
        text_for_user: Some("You hear a hollow sound. They must be empty".to_owned()),
        outcome: None,
    };
    the_wine_place.add_possible_action(knock_on_wine_barrels);
    the_wine_place.add_possible_action(wine_celler_actions);
    the_wine_place
}

fn setup_sex_dungeon() -> AllPurposeRoom {
    let mut the_sex_dungeon = AllPurposeRoom::new(RoomType::SexDungeon);
    let room_specific_actions = "lights, do_the_nasty".to_owned();
    let sex_dungeon_actions = rule_engine::Rule {
        action_name: "actions".to_owned(),
        // text_for_user: my_entrance.get_possible_action_names(),
        text_for_user: Some(format!(
            "{}{}{}",
            "actions, ", room_specific_actions, ", exit"
        )),
        outcome: None,
    };
    let turn_on_lights = rule_engine::Rule {
        action_name: "lights".to_owned(),
        text_for_user: Some(
            "Yup, your eyes are not deceiving you... it is a sex dungeon".to_owned(),
        ),
        outcome: None,
    };
    let nasty_action = rule_engine::Rule {
        action_name: "do_the_nasty".to_owned(),
        text_for_user: Some("Wow... You must be pretty desperate to try that here!".to_owned()),
        outcome: None,
    };

    let easter_egg_action= rule_engine::Rule {
        action_name: "dusi".to_owned(),
        text_for_user: Some("Oh no, the Spas has found you and slaps your cute butt ;) You suddenly regret searching for this easter egg! ".to_owned()),
        outcome: None,
    };
    the_sex_dungeon.add_possible_action(sex_dungeon_actions);
    the_sex_dungeon.add_possible_action(turn_on_lights);
    the_sex_dungeon.add_possible_action(nasty_action);
    the_sex_dungeon.add_possible_action(easter_egg_action);
    the_sex_dungeon
}

fn setup_kitchen(the_sex: AllPurposeRoom) -> AllPurposeRoom {
    let mut the_kitchen = AllPurposeRoom::new(RoomType::Kitchen);
    let room_specific_actions = "steal_knives, try_cooking, go_forward".to_owned();
    let kitchen_actions = rule_engine::Rule {
        action_name: "actions".to_owned(),
        // text_for_user: my_entrance.get_possible_action_names(),
        text_for_user: Some(format!(
            "{}{}{}",
            "actions, ", room_specific_actions, ", exit"
        )),
        outcome: None,
    };
    let steal_knives_action= rule_engine::Rule {
        action_name: "steal_knives".to_owned(),
        text_for_user: Some("You found some shiny knives in one of the lockers. Someone has sharpenned them recently".to_owned()),
        outcome: None,
    };
    let try_cooking_action = rule_engine::Rule {
        action_name: "try_cooking".to_owned(),
        text_for_user: Some("You tried to cook something. It was very unssucessful!".to_owned()),
        outcome: None,
    };

    let go_forward_action = rule_engine::Rule {
        action_name: "go_forward".to_owned(),
        text_for_user: Some(
            "You find some dark stairs and decide it cant hurt to see what is down there"
                .to_owned(),
        ),
        outcome: Some(Rc::new(the_sex)),
    };
    the_kitchen.add_possible_action(kitchen_actions);
    the_kitchen.add_possible_action(steal_knives_action);
    the_kitchen.add_possible_action(try_cooking_action);
    the_kitchen.add_possible_action(go_forward_action);
    the_kitchen
}

fn setup_entrance(the_wine_place: AllPurposeRoom, the_kitchen: AllPurposeRoom) -> AllPurposeRoom {
    let mut my_entrance = AllPurposeRoom::new(RoomType::MainEntrance);
    let room_specific_actions = "look_around, ring_reception_bell, go_left, go_right".to_owned();
    let main_entrance_available_actions = rule_engine::Rule {
        action_name: "actions".to_owned(),
        // text_for_user: my_entrance.get_possible_action_names(),
        text_for_user: Some(format!(
            "{}{}{}",
            "actions, ", room_specific_actions, ", exit"
        )),
        outcome: None,
    };
    let look_around_action = rule_engine::Rule {
        action_name: "look_around".to_owned(),
        text_for_user: Some("The place is decorated very plainly. I guess the owners are fans of IKEA minimalist designs".to_owned()),
        outcome: None,
    };
    let ring_bell_action = rule_engine::Rule {
        action_name: "ring_reception_bell".to_owned(),
        text_for_user: Some("The bell chimes with a small zingg".to_owned()),
        outcome: None,
    };
    let go_to_left_room = rule_engine::Rule {
        action_name: "go_left".to_owned(),
        text_for_user: None,
        outcome: Some(Rc::new(the_wine_place)),
    };

    let go_to_right_room = rule_engine::Rule {
        action_name: "go_right".to_owned(),
        text_for_user: None,
        outcome: Some(Rc::new(the_kitchen)),
    };
    my_entrance.add_possible_action(main_entrance_available_actions);
    my_entrance.add_possible_action(look_around_action);
    my_entrance.add_possible_action(ring_bell_action);
    my_entrance.add_possible_action(go_to_left_room);
    my_entrance.add_possible_action(go_to_right_room);
    my_entrance
}
