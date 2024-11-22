



use std::io;



fn main() {
    switch_turn(1);
}

struct Player {
    name : String,
    position : u8,
    money : i32,
    cards_owned : Vec<u8>
}

impl Default for Player {
    fn default() -> Self {
        Player {
            name : String::new(),
            position : 0,
            money : 15000,
            cards_owned : vec![]
        }
    }
}

impl Player {
    fn move_position(&mut self, move_amount: u8) -> u8 {
        self.position = self.position + move_amount;
        if self.position >= 3 {
            self.position = 0;
        }
        self.position
    }

    fn receive(&mut self, amount: i32) {
        self.money = self.money + amount;
        println!("I gained {}", self.money);
    }
}

use once_cell::sync::Lazy;
use std::sync::{RwLock, RwLockWriteGuard};

static PLAYER_ONE : Lazy<RwLock<Player>> = Lazy::new(|| RwLock::new(Player::default()));
static PLAYER_TWO : Lazy<RwLock<Player>> = Lazy::new(|| RwLock::new(Player::default()));
static PLAYER_THREE : Lazy<RwLock<Player>> = Lazy::new(|| RwLock::new(Player::default()));
static PLAYER_FOUR : Lazy<RwLock<Player>> = Lazy::new(|| RwLock::new(Player::default()));

fn switch_turn(mut current_player: u8) {
    let mut input = String::new();
    println!("Player {} switch turn? ", current_player);
    io::stdin().read_line(&mut input).expect("couldn't read line");

    current_player = current_player + 1;
    if current_player >= 3 {
        current_player = 1;
    }

    {
        let mut current_player_struct = match current_player {
            1 => PLAYER_ONE.write().unwrap(),
            2 => PLAYER_TWO.write().unwrap(),
            
            _ => PLAYER_ONE.write().unwrap()
        };

        let position = current_player_struct.move_position(1);
        println!("positon is {}", position);

         if let Some(value) = POSITIONS.get(&position) {
            match value {
                Cards::GoCardEnum(val) => val.player_landed_on(&mut current_player_struct),
                Cards::LandCardEnum(val) => val.player_landed_on(&mut current_player_struct),
                Cards::TrainCardEnum(val) => val.player_landed_on(&mut current_player_struct),
            }
         }
    }

    switch_turn(current_player);
}

struct LandCardInfo {
    cost: u8,
}
impl LandCardInfo {
    fn increase_cost(&mut self) {
        self.cost = self.cost + 5;
        println!("I cost {}", self.cost);
    }
}

struct TrainCardInfo {

    cost: u8,
}

static OLD_KENT_ROAD_CARD_INFO : Lazy<RwLock<LandCardInfo>> = Lazy::new(|| RwLock::new(LandCardInfo{
    cost : 90,
}));

// static  KING_CROSS_STATION_CARD_INFO : Lazy<RwLock<TrainCardInfo>> = Lazy::new(|| RwLock::new(TrainCardInfo {
//     cost: 100,
// }));

use std::collections::HashMap;

static POSITIONS : Lazy<HashMap<u8, Cards>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(0, Cards::GoCardEnum(&GO));
    map.insert(1, Cards::TrainCardEnum(&KING_CROSS_STATION));
    map.insert(2, Cards::LandCardEnum(&OLD_KENT_ROAD));
    map
});
impl Actions for GoCard {
    fn player_landed_on(&self, current_player: &mut RwLockWriteGuard<'_, Player>) {
        current_player.receive(200);
    }
}
impl Actions for TrainCard {
    fn player_landed_on(&self, current_player: &mut RwLockWriteGuard<'_, Player>) {
        
    }
}
impl Actions for LandCard {
    fn player_landed_on(&self, current_player: &mut RwLockWriteGuard<'_, Player>) {
        println!("my pricce is {}", self.price);
        let mut myInfo = self.my_info.write().unwrap();
        myInfo.increase_cost();
    }
}

trait Actions{
    fn player_landed_on(&self, current_player: &mut RwLockWriteGuard<'_, Player>);
}
struct GoCard {
    name : &'static str,
    ref_number: u8
}

struct TrainCard {
    name: &'static str,
    price : u32,
    ref_number: u8
}

struct LandCard {
    name : &'static str,
    price : u32,
    ref_number : u8,
    my_info : &'static Lazy<RwLock<LandCardInfo>>
}

static GO : GoCard = GoCard {
    name: "GO",
    ref_number: 0,
};
static KING_CROSS_STATION : TrainCard = TrainCard {
    name: "King's Cross", 
    price: 500,
    ref_number: 1
};
static OLD_KENT_ROAD : LandCard = LandCard {
    name: "Old Kent Road",
    price: 60,
    ref_number: 2,
    my_info : &OLD_KENT_ROAD_CARD_INFO
};

enum Cards {
    GoCardEnum(&'static GoCard),
    LandCardEnum(&'static LandCard),
    TrainCardEnum(&'static TrainCard)
}




use serde::{Serialize, Deserialize};
use std::fmt::Debug;

use lazy_static::lazy_static;
use std::sync::RwLock;


pub mod inventory_item {
    use super::*;
    use std::collections::HashMap;

    lazy_static!{
        static ref INVENTORY : RwLock<InventoryItem> = RwLock::new(InventoryItem::default());
    }

    static INVENTORY_FILEPATH: &str = "inventory.txt";
    // global_instance.items.entry("Fish".to_string()).and_modify(|i| *i += 90);

    #[derive(Serialize, Deserialize, Debug)]
    pub struct InventoryItem {
        items: HashMap<String, u16>
    }
    
    impl Default for InventoryItem {
        fn default() -> Self {
            let default_map =  HashMap::from([
                ("Iron".to_string(), 0),
                ("Wood".to_string(), 900),
                ("Fish".to_string(), 0),
            ]);

            InventoryItem {
                items: default_map,
            }
        }
    }

    pub fn save() {
        let global_instance = INVENTORY.read().unwrap();
        save_to_file(&*global_instance, INVENTORY_FILEPATH);
    }

    pub fn load() {
        let result = read_from_file::<InventoryItem>(INVENTORY_FILEPATH);
        let mut global_instance = INVENTORY.write().unwrap();
        
        *global_instance = match result {
            Ok(value) => value,
            Err(_e) => {
                println!("An error when reading from file! Turning to default");
                InventoryItem::default()
            },
        };
    }
}

pub mod player_stats {
    use super::*;
    // static PLAYERSTATS_FILEPATH: &str ="playerstats.txt";

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PlayerStats {
        name: String,
    }
    
    impl Default for PlayerStats {
        fn default() -> Self {
            PlayerStats {
                name : String::new(),
            }
        }
    }
}


// #[cfg(FALSE)]
// pub mod test_item {
//     use super::*;
//     static TEST_FILEPATH: &str ="test.txt";

//     #[derive(Serialize, Deserialize, Debug)]
//     pub struct TestItem {
//         name: String
//     }

//     impl Default for TestItem {
//         fn default() -> Self {
//             TestItem {
//                 name: "Favie".to_string()
//             }
//         }
//     }

//     impl TestItem {
        
//     }

//     fn test_codes() {
//         let new_item = TestItem::default();

//         // for serialization - private function
//         let json = to_json(&new_item);
//         println!("Serialized struct is: {}", json);

//         // for deserialzation - private function
//         let mut second_item : TestItem = to_struct(json);
//         println!("Deserialized struct is: {:#?}", second_item);

//         // for saving - public function
//         save_to_file(&second_item, TEST_FILEPATH);

//         // for reading - public function
//         let result = read_from_file::<TestItem>(TEST_FILEPATH);
//         let second_item = match result {
//             Ok(value) => value,
//             Err(_e) => {
//                 println!("An error when reading from file! Turning to default");
//                 TestItem::default()
//             },
//         };
//         println!("Deserialized struct is: {:#?}", second_item);

//     }


















impl PropertyCard {
    // pub fn player_landed_on(&self, current_player: &mut RwLockWriteGuard<'_, Player>) {
    //     let owner_of_this_location = *self.owner_of_location.read().unwrap();
        
    //     if owner_of_this_location == OwnerEnum::None {
    //         self.noone_owns_this_location(current_player);
    //     } else if owner_of_this_location == current_player.get_player_enum() {
    //         self.player_that_landed_on_owns_this_location(current_player);
    //     } else {
    //         self.player_that_landed_on_does_not_own_this_location(current_player, owner_of_this_location);
    //     }
    // }

    fn player_bought_this_location(&self, current_player: &mut RwLockWriteGuard<'_, Player>) {
        current_player.pay(self.price);
        current_player.bought_location(self.reference_number);

        {
            *self.owner_of_location.write().unwrap() = current_player.get_player_enum();
        }

        {
            let number_of_owned_buildings = *self.number_of_owned_buildings.write().unwrap();

            if number_of_owned_buildings == 0 {
                let mut player_owns_colour_set = true;
                let current_player_enum = current_player.get_player_enum();

                for rl in self.related_locations_array {
                    if let Some(val) = rl {
                        if *val.owner_of_location.read().unwrap() != current_player_enum {
                            player_owns_colour_set = false;
                        }
                    }
                }

                if player_owns_colour_set {
                    *self.current_rent_to_be_paid.write().unwrap() = self.rents_array[number_of_owned_buildings] * 2;
                } else {
                    *self.current_rent_to_be_paid.write().unwrap() = self.rents_array[number_of_owned_buildings];
                }
            } else {
                *self.current_rent_to_be_paid.write().unwrap() = self.rents_array[number_of_owned_buildings];
            }
        }
    }

    // fn noone_owns_this_location(&self, current_player: &mut RwLockWriteGuard<'_, Player>) {
    //     if current_player.can_player_afford(self.price) {
    //         let message = &format!("No one owns {}. It costs {}. Do you wish to buy?", self.name, self.price);

    //         let response = Confirm::new(&message)
    //             .with_default(false)
    //             .prompt();

    //         match response {
    //             Ok(true) => self.player_bought_this_location(current_player),
    //             Ok(false) => {},
    //             Err(err) => println!("Um... An error: {}", err),
    //         }
    //     } else {
    //         println!("You cannot afford {}. Your turn has been ended", self.name);
    //     }
    // }

    // fn player_that_landed_on_owns_this_location(&self, current_player: &mut RwLockWriteGuard<'_, Player>) {
        
    // }

    // fn player_that_landed_on_does_not_own_this_location(&self, current_player: &mut RwLockWriteGuard<'_, Player>, owner_of_this_locaton: OwnerEnum) {
    //     {
    //         let rent_to_be_paid = *self.current_rent_to_be_paid.read().unwrap();

    //         current_player.pay(rent_to_be_paid);
    //         crate::player::this_player_receives(owner_of_this_locaton, rent_to_be_paid);
    //     }

    //     let message = &format!("You paid rent on {}", self.name);
    // }
}

pub struct TrainCard {
    name: &'static str,
    reference_number: u8,
    price: i32,
    mortgage_value: i32,
    rents_array: [i32; 4],
    related_locations_array: [&'static TrainCard; 3],

    owner_of_location: RwLock<OwnerEnum>,
    current_rent_to_be_paid: RwLock<i32>,
}

impl PropertyTrainUtility for TrainCard {

}




fn mortgage_or_unmortgage_this_location <T: PropertyTrainUtility> (this_self: &mut T, is_mortgaging: bool) {
    if let Some(type_is_utility_card) = this_self.as_any().downcast_mut::<PropertyCard>() {

    }
}








    fn mortgage_or_unmortgage_this_location(&mut self, is_mortgaging: bool) {
        let mut amount = self.mortgage_value;

        if is_mortgaging {
            crate::player::this_player_receives(*self.owner_of_location.read().unwrap(), amount);
        } else {
            let interest = amount * 10 / 100;
            amount = amount + interest;
            crate::player::this_player_pays(*self.owner_of_location.read().unwrap(), amount);
        }

        *self.is_mortgaged.write().unwrap() = is_mortgaging;

        if let Some(type_is_utility_card) = self.as_any().downcast_ref::<PropertyCard>() {

        }

        if let Some(this_type) = self.as_any().downcast_ref::<UtilityCard>() {
            println!("Utility card: {}", this_type.name);
        } else {
            println!("Nope! not a utility");
        }
    }



    fn mortgage_or_unmortgage_this_location(&self, is_mortgaging: bool) {
        let mut amount = self.mortgage_value;

        if is_mortgaging {
            crate::player::this_player_receives(*self.owner_of_location.read().unwrap(), amount);
        } else {
            amount = self.get_unmortgage_value();
            crate::player::this_player_pays(*self.owner_of_location.read().unwrap(), amount);
        }

        *self.is_mortgaged.write().unwrap() = is_mortgaging;
    }
































    use super::*;

    impl Mortgage for PropertyCard {
        fn check_if_this_location_can_be_mortgaged(&self) {
            if *self.is_mortgaged.read().unwrap() == true {
                println!("{} cannot be mortgaged because it is already mortgaged", self.name);
                return;
            }
    
            if *self.number_of_owned_buildings.read().unwrap() == 0 {
                let mut can_mortgage = true;
    
                for rl in self.related_locations_array {
                    if let Some(val) = rl {
                        if *val.number_of_owned_buildings.read().unwrap() > 0 {
                            can_mortgage = false;
                        }
                    }
                }
    
                if can_mortgage == true {
                    self.mortgage_or_unmortgage_this_location(true);
                    println!("{} has been mortgaged", self.name);
                } else {
                    println!("{} cannot be mortgaged because some of the properties in this colour set still have buildings on them", self.name);
                }
            } else {
                println!("{} cannot be mortgaged because it still has buildings on it", self.name);
            }
        }
    }
    
    impl ActionWhenPlayerLandsOnCard for PropertyCard {
        fn get_card_enum(&self) -> &CardsEnum {
            crate::POSITIONS.get(&self.reference_number).unwrap()
        }
    
        fn update_rent(&self, current_player: &mut RwLockWriteGuard<'_, Player>) {
            *self.owner_of_location.write().unwrap() = current_player.get_player_enum();
    
            let number_of_owned_buildings = *self.number_of_owned_buildings.write().unwrap();
           
            if number_of_owned_buildings == 0 {
                let mut multiplier = 2;
                let current_player_enum = current_player.get_player_enum();
    
                for rl in self.related_locations_array {
                    if let Some(val) = rl {
                        if *val.owner_of_location.read().unwrap() != current_player_enum {
                            multiplier = 1;
                        }
                    }
                }
    
                *self.current_rent_to_be_paid.write().unwrap() = self.rents_array[number_of_owned_buildings] * multiplier;
            } else {
                *self.current_rent_to_be_paid.write().unwrap() = self.rents_array[number_of_owned_buildings];
            }
        }
    }
    
    impl PropertyCard {
        fn get_mortgage_value(&self) -> i32 {
            self.price / 2
        }
    
        fn get_unmortgage_value(&self) -> i32 {
            let interest = self.get_mortgage_value() * 10 / 100;
            self.get_mortgage_value() + interest
        }
    }
    
    impl PropertyCard {
        fn get_building_cost(&self) -> i32 {
            match self.building_cost {
                BuildingCost::Cost50 => 50,
                BuildingCost::Cost100 => 100,
                BuildingCost::Cost150 => 150,
                BuildingCost::Cost200 => 200,
            }
        }
    
        fn get_sell_building_cost(&self) -> i32 {
            self.get_building_cost() / 2
        }
    
        fn check_if_player_can_buy_building_on_this_location(&self) {
            let mut can_buy = true;
    
            if *self.is_mortgaged.read().unwrap() == true {
                println!("Cannot build on {} because it is morgaged", self.name);
                return;
            }
    
            for rl in self.related_locations_array {
                if let Some(val) = rl {
                    if *val.is_mortgaged.read().unwrap() == true {
                        println!("Cannot build on {} because some of the properties in this colour set are still still mortgaged", self.name);
                        return;
                    }
                }
            }
    
            let player_to_check = *self.owner_of_location.read().unwrap();
            let amount = self.get_building_cost();
    
            if crate::player::check_if_this_player_can_afford(player_to_check, amount) == false {
                println!("{:?} cannot afford to build on {}", player_to_check, self.name);
                return;
            }
    
            let number_of_owned_buildings = *self.number_of_owned_buildings.read().unwrap();
            
            if number_of_owned_buildings >= 5 {
                println!("Cannot build on {} because it has reached maximum of 5 buildings", self.name);
                return;
            }
    
            for rl in self.related_locations_array {
                if let Some(val) = rl {
                    if *val.number_of_owned_buildings.read().unwrap() == number_of_owned_buildings || 
                            *val.number_of_owned_buildings.read().unwrap() == (number_of_owned_buildings + 1)
                    {
                        // just because i don't know how to get the else
                    } else {
                        println!("Build on {} before you can build on this location: {}", val.name, self.name);
                        can_buy = false;
                    }
                }
            }
    
            if can_buy {
                let tuple = (player_to_check, amount);
                self.buy_or_sell_building_on_this_location(true, tuple);
    
                println!("Bought new building on {}. It now has {} buildings", self.name, *self.number_of_owned_buildings.read().unwrap());
            }
        }
    
        fn check_if_player_can_sell_building_on_this_location(&self) {
            let mut can_sell = true;
    
            if *self.is_mortgaged.read().unwrap() == true {
                println!("Cannot sell on {} because it is morgaged", self.name);
                return;
            }
    
            let number_of_owned_buildings = *self.number_of_owned_buildings.read().unwrap();
            if number_of_owned_buildings <= 0 {
                println!("Cannot sell on {} because it has no buildings to sell", self.name);
                return;
            }
    
            for rl in self.related_locations_array {
                if let Some(val) = rl {
                    if *val.number_of_owned_buildings.read().unwrap() == number_of_owned_buildings || 
                            *val.number_of_owned_buildings.read().unwrap() == (number_of_owned_buildings - 1)
                    {
                        // just because i don't know how to get the else
                    } else {
                        println!("Sell on {} before you can sell on this location: {}", val.name, self.name);
                        can_sell = false;
                    }
                }
            }
    
            if can_sell {
                let tuple = (*self.owner_of_location.read().unwrap(), self.get_sell_building_cost());
                self.buy_or_sell_building_on_this_location(false, tuple);
    
                println!("Sold building on {}. It now has {} buildings", self.name, *self.number_of_owned_buildings.read().unwrap());
            }
        }
    
        fn buy_or_sell_building_on_this_location(&self, is_buying: bool, tuple: (OwnerEnum, i32)) {
            let (owner_of_location, amount) = tuple;
    
            if is_buying {
                crate::player::this_player_pays(owner_of_location, amount);
                *self.number_of_owned_buildings.write().unwrap() += 1;
            } else {
                crate::player::this_player_receives(owner_of_location, amount);
                *self.number_of_owned_buildings.write().unwrap() -= 1;
            }
        }
    }