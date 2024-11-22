





use std::io;
use std::collections::HashMap;

use once_cell::sync::Lazy;
use std::sync::RwLock;

use player::Player;
use location_cards::{GoCard, PropertyCard, ActionWhenPlayerLandsOnCard, TrainCard, UtilityCard};


pub mod location_cards;
pub mod player;

/* THERE ARE ONLY TWO PLACES I'M UNWRAPPING THE PLAYERS 
    1. IN THIS MAIN MODULE
    2. IN THE PLAYER MODULE
    !! BE CAREFUL WITH IT */

static PLAYER_ONE: Lazy<RwLock<Player>> = Lazy::new(
    || RwLock::new(Player::default(OwnerEnum::PlayerOne)));

static PLAYER_TWO: Lazy<RwLock<Player>> = Lazy::new(
    || RwLock::new(Player::default(OwnerEnum::PlayerTwo)));

static PLAYER_THREE : Lazy<RwLock<Player>> = Lazy::new(
    || RwLock::new(Player::default(OwnerEnum::PlayerThree)));

static PLAYER_FOUR : Lazy<RwLock<Player>> = Lazy::new(
    || RwLock::new(Player::default(OwnerEnum::PlayerFour)));



pub enum CardsEnum {
    GoCardEnum(&'static GoCard),
    PropertyCardEnum(&'static PropertyCard),
    TrainCardEnum(&'static TrainCard),
    UtilityCardEnum(&'static UtilityCard)
}

#[derive(Debug, Clone, Copy, PartialEq)]
/* NEVER PUT DATA INTO THIS STRUCT! 
 IT IMPLEMENTS THE COPY TRAIT! */
pub enum OwnerEnum {
    None,
    PlayerOne,
    PlayerTwo,
    PlayerThree,
    PlayerFour
}

static POSITIONS: Lazy<HashMap<u8, CardsEnum>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(0, CardsEnum::GoCardEnum(&location_cards::locations_info::GO));
    map.insert(1, CardsEnum::TrainCardEnum(&location_cards::locations_info::KING_CROSS_STATION));
    map.insert(5, CardsEnum::PropertyCardEnum(&location_cards::locations_info::OLD_KENT_ROAD));
    map.insert(6, CardsEnum::PropertyCardEnum(&location_cards::locations_info::WHITECHAPEL_ROAD));
    map.insert(9, CardsEnum::UtilityCardEnum(&location_cards::locations_info::ELECTRIC_COMPANY));
    map.insert(10, CardsEnum::UtilityCardEnum(&location_cards::locations_info::WATER_WORKS));

    map.insert(50, CardsEnum::PropertyCardEnum(&MAYFAIR));
    map.insert(51, CardsEnum::PropertyCardEnum(&PARK_LANE));
    map.insert(52, CardsEnum::PropertyCardEnum(&THIRD_ONE));
    map
});

use location_cards::testss::*;

fn main() {
    switch_turn(1);
}

fn get_dice_number() -> i32 {
    5
}

fn switch_turn(mut current_player: u8) {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("couldn't read line");

    current_player = current_player + 1;
    if current_player >= 3 {
        current_player = 1;
    }

    println!("Now player {} ", current_player);

    {
        let mut current_player_struct = match current_player {
            1 => PLAYER_ONE.write().unwrap(),
            2 => PLAYER_TWO.write().unwrap(),
            3 => PLAYER_THREE.write().unwrap(),
            4 => PLAYER_FOUR.write().unwrap(),
            _ => PLAYER_ONE.write().unwrap(),
        };

        // let position = current_player_struct.move_position(1);
        let position: u8 = 1;

        if let Some(value) = POSITIONS.get(&position) {
            match value {
                CardsEnum::GoCardEnum(val) => val.player_landed_on(&mut current_player_struct),
                CardsEnum::PropertyCardEnum(val) => val.player_landed_on(&mut current_player_struct, value),
                CardsEnum::TrainCardEnum(val) => val.player_landed_on(&mut current_player_struct, value),
                CardsEnum::UtilityCardEnum(val) => val.player_landed_on(&mut current_player_struct, value),
            }
        }
    }

    switch_turn(current_player);
}
