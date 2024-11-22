




#![allow(dead_code)]
#![allow(unused_variables)]

use crate::{OwnerEnum, CardsEnum};
use crate::player::Player;

use std::sync::{RwLock, RwLockWriteGuard};
use inquire::Confirm;

enum BuildingCost {
    Cost50,
    Cost100,
    Cost150,
    Cost200
}

pub trait Mortgage: ActionWhenPlayerLandsOnCard {
    /* THIS IS OVER-WRITTEN FOR PROPERTY CARD !!! */
    fn check_if_this_location_can_be_mortgaged(&self) {
        let card_enum = self.get_card_enum();

        let name;
        let is_mortgaged;

        match card_enum {
            CardsEnum::TrainCardEnum(val) => {
                is_mortgaged = *val.is_mortgaged.read().unwrap();
                name = val.name;
            },
            CardsEnum::UtilityCardEnum(val) => {
                is_mortgaged = *val.is_mortgaged.read().unwrap();
                name = val.name;
            },
            _ => {
                return;
            }
        }

        if is_mortgaged == false {
            self.mortgage_or_unmortgage_this_location(true);
            println!("{} has been mortgaged", name);
        } else {
            println!("Cannot mortgage {} because it is already mortgaged", name);
        }
    }

    fn check_if_this_location_can_be_unmortgaged(&self) {
        let card_enum = self.get_card_enum();

        let name;
        let amount;
        let player_to_check;
        let is_mortgaged;

        match card_enum {
            CardsEnum::PropertyCardEnum(val) => {
                name = val.name;
                amount = val.get_unmortgage_value();
                player_to_check = *val.owner_of_location.read().unwrap();
                is_mortgaged = *val.is_mortgaged.read().unwrap();
            },
            CardsEnum::TrainCardEnum(val) => {
                name = val.name;
                amount = val.get_unmortgage_value();
                player_to_check = *val.owner_of_location.read().unwrap();
                is_mortgaged = *val.is_mortgaged.read().unwrap();
            },
            CardsEnum::UtilityCardEnum(val) => {
                name = val.name;
                amount = val.get_unmortgage_value();
                player_to_check = *val.owner_of_location.read().unwrap();
                is_mortgaged = *val.is_mortgaged.read().unwrap();
            },
            _ => {
                return;
            }
        }

        if is_mortgaged == true {
            if crate::player::check_if_this_player_can_afford(player_to_check, amount) {
                self.mortgage_or_unmortgage_this_location(false);
                println!("{} has been unmortgaged", name);
            } else {
                println!("You don't have enough funds to un-morgage {}", name);
            }
        } else {
            println!("{} is not a mortgaged property so it cannot be un-mortgaged", name);
        }
    }

    fn mortgage_or_unmortgage_this_location(&self, is_mortgaging: bool) {
        let card_enum = self.get_card_enum();

        let amount;
        let owner_of_this_location;

        match card_enum {
            CardsEnum::PropertyCardEnum(val) => {
                *val.is_mortgaged.write().unwrap() = is_mortgaging;
                owner_of_this_location = *val.owner_of_location.read().unwrap();

                if is_mortgaging {
                    amount = val.get_mortgage_value();
                } else {
                    amount = val.get_unmortgage_value();
                }
            },
            CardsEnum::TrainCardEnum(val) => {
                *val.is_mortgaged.write().unwrap() = is_mortgaging;
                owner_of_this_location = *val.owner_of_location.read().unwrap();

                if is_mortgaging {
                    amount = val.get_mortgage_value();
                } else {
                    amount = val.get_unmortgage_value();
                }
            },
            CardsEnum::UtilityCardEnum(val) => {
                *val.is_mortgaged.write().unwrap() = is_mortgaging;
                owner_of_this_location = *val.owner_of_location.read().unwrap();

                if is_mortgaging {
                    amount = val.get_mortgage_value();
                } else {
                    amount = val.get_unmortgage_value();
                }
            },
            _ => {
                return
            }
        }

        if is_mortgaging {
            crate::player::this_player_receives(owner_of_this_location, amount);
        } else {
            crate::player::this_player_pays(owner_of_this_location, amount);
        }

    }
}

pub trait ActionWhenPlayerLandsOnCard {
    fn get_card_enum(&self) -> &CardsEnum;

    fn update_rent(&self, current_player: &mut RwLockWriteGuard<'_, Player>);

    fn player_landed_on(&self, current_player: &mut RwLockWriteGuard<'_, Player>, card: &CardsEnum) {
        let owner_of_this_location;
        let current_rent_to_be_paid;
        let reference_number;
        let is_mortgaged;
        let price;
        let name;

        match card {
            CardsEnum::PropertyCardEnum(val) => {
                owner_of_this_location = *val.owner_of_location.read().unwrap();
                current_rent_to_be_paid = *val.current_rent_to_be_paid.read().unwrap();
                reference_number = val.reference_number;
                is_mortgaged = *val.is_mortgaged.read().unwrap();
                price = val.price;
                name = val.name;
            },
            CardsEnum::TrainCardEnum(val) => {
                owner_of_this_location = *val.owner_of_location.read().unwrap();
                current_rent_to_be_paid = *val.current_rent_to_be_paid.read().unwrap();
                reference_number = val.reference_number;
                is_mortgaged = *val.is_mortgaged.read().unwrap();
                price = val.price;
                name = val.name;
            },
            CardsEnum::UtilityCardEnum(val) => {
                owner_of_this_location = *val.owner_of_location.read().unwrap();
                current_rent_to_be_paid = *val.current_rent_multiplier.read().unwrap();
                reference_number = val.reference_number;
                is_mortgaged = *val.is_mortgaged.read().unwrap();
                price = val.price;
                name = val.name;
            },
            _ => {
                println!("This shouldn't be possible. Check player_landed_on()");
                return
            }
        }
        
        if owner_of_this_location == OwnerEnum::None {
            let tuple = (price, name, reference_number);
            self.this_location_is_not_owned(current_player, tuple);

        } else if owner_of_this_location != current_player.get_player_enum() {
            if is_mortgaged == false {
                let tuple = (owner_of_this_location, current_rent_to_be_paid);
                self.player_that_landed_on_does_not_own_this_location(current_player, tuple);
            } else {
                println!("{:?} can't receive rent on {} because it is mortgaged", owner_of_this_location, name);
            }
        }
    }

    /* THIS IS OVERWRITTEN FOR UTILITY CARD !!! */
    fn player_that_landed_on_does_not_own_this_location(&self, current_player: &mut RwLockWriteGuard<'_, Player>, tuple: (OwnerEnum, i32)) {
        let (owner_of_this_location, rent_to_be_paid) = tuple;

        current_player.pay(rent_to_be_paid);
        crate::player::this_player_receives(owner_of_this_location, rent_to_be_paid);
    }

    fn this_location_is_not_owned(&self, current_player: &mut RwLockWriteGuard<'_, Player>, tuple: (i32, &str, u8)) {
        let (price, name, reference_number) = tuple;
        if current_player.can_player_afford(price) {
            let message = &format!("No one owns {}. It costs {}. Do you wish to buy?", name, price);

            let response = Confirm::new(&message)
                .with_default(false)
                .prompt();

            match response {
                Ok(true) => {
                    let x_tuple = (price, reference_number, name);
                    self.current_player_bought_this_location(current_player, x_tuple);
                },
                Ok(false) => {
                    println!("Well then! You do not wish to buy");
                },
                Err(err) => println!("Um... An error: {}", err),
            }
        } else {
            println!("You cannot afford {}. Your turn has been ended", name);
        }
    }

    fn current_player_bought_this_location(&self, current_player: &mut RwLockWriteGuard<'_, Player>, tuple: (i32, u8, &str)) {
        let (price, reference_number, name) = tuple;
        current_player.pay(price);
        current_player.bought_location(reference_number);

        self.update_rent(current_player);
        println!("Rent for {} has been updated", name)
    }
}


pub struct GoCard {
    pay: i32
}

impl GoCard {
    pub fn player_landed_on(&self, current_player: &mut RwLockWriteGuard<'_, Player>) {
        current_player.receive(self.pay);
        println!("Current player, {:?} reached go and collected {}", current_player.get_player_enum(), self.pay);
    }
}

pub struct TrainCard {
    name: &'static str,
    reference_number: u8,
    price: i32,
    rents_array: [i32; 4],
    related_locations_array: [&'static TrainCard; 3],

    owner_of_location: RwLock<OwnerEnum>,
    current_rent_to_be_paid: RwLock<i32>,
    is_mortgaged : RwLock<bool>,
}

impl Mortgage for TrainCard { }

impl ActionWhenPlayerLandsOnCard for TrainCard {
    fn get_card_enum(&self) -> &CardsEnum {
        crate::POSITIONS.get(&self.reference_number).unwrap()
    }

    fn update_rent(&self, current_player: &mut RwLockWriteGuard<'_, Player>) {
        *self.owner_of_location.write().unwrap() = current_player.get_player_enum();

        let mut rent_pointer = 0;
        let current_player_enum = current_player.get_player_enum();

        for rl in self.related_locations_array {
            if *rl.owner_of_location.read().unwrap() == current_player_enum {
                rent_pointer+= 1;
            }
        }

        *self.current_rent_to_be_paid.write().unwrap() = self.rents_array[rent_pointer];
    }
}

impl TrainCard {
    fn get_mortgage_value(&self) -> i32 {
        self.price / 2
    }

    fn get_unmortgage_value(&self) -> i32 {
        let interest = self.get_mortgage_value() * 10 / 100;
        self.get_mortgage_value() + interest
    }
}

pub struct UtilityCard {
    name: &'static str,
    reference_number: u8,
    price: i32,
    rent_multiplier_array: [i32; 2],
    related_locations_array: [&'static UtilityCard; 1],

    owner_of_location: RwLock<OwnerEnum>,
    current_rent_multiplier: RwLock<i32>,
    is_mortgaged : RwLock<bool>,
}

impl Mortgage for UtilityCard { }

impl ActionWhenPlayerLandsOnCard for UtilityCard {
    fn get_card_enum(&self) -> &CardsEnum {
        crate::POSITIONS.get(&self.reference_number).unwrap()
    }

    fn update_rent(&self, current_player: &mut RwLockWriteGuard<'_, Player>) {
        *self.owner_of_location.write().unwrap() = current_player.get_player_enum();
        
        let mut rent_pointer = 0;
        let current_player_enum = current_player.get_player_enum();

        for rl in self.related_locations_array {
            if *rl.owner_of_location.read().unwrap() == current_player_enum {
                rent_pointer+= 1;
            }
        }

        *self.current_rent_multiplier.write().unwrap() = self.rent_multiplier_array[rent_pointer];
    }

    fn player_that_landed_on_does_not_own_this_location(&self, current_player: &mut RwLockWriteGuard<'_, Player>, tuple: (OwnerEnum, i32)) {
        let (owner_of_this_location, rent_multiplier) = tuple;
        let rent_to_be_paid = rent_multiplier * crate::get_dice_number();

        current_player.pay(rent_to_be_paid);
        crate::player::this_player_receives(owner_of_this_location, rent_to_be_paid);
    }

}

impl UtilityCard {
    fn get_mortgage_value(&self) -> i32 {
        self.price / 2
    }

    fn get_unmortgage_value(&self) -> i32 {
        let interest = self.get_mortgage_value() * 10 / 100;
        self.get_mortgage_value() + interest
    }
}

pub struct PropertyCard {
    name: &'static str,
    reference_number: u8,
    price: i32,
    building_cost: BuildingCost,
    rents_array: [i32; 6],
    related_locations_array: [Option<&'static PropertyCard>; 2],
    
    owner_of_location: RwLock<OwnerEnum>,
    current_rent_to_be_paid: RwLock<i32>,
    is_mortgaged : RwLock<bool>,
    number_of_owned_buildings: RwLock<usize>
}

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



//                  REMOVE THIS AFTER TESTING!
pub mod testss;

pub mod locations_info {
    use super::{OwnerEnum, BuildingCost};
    use super::{GoCard, PropertyCard, TrainCard, UtilityCard};
    use super::RwLock;

    pub static GO: GoCard = GoCard {
        pay: 200
    };

    pub static KING_CROSS_STATION: TrainCard = TrainCard {
        name: "King's Cross Station",
        reference_number: 1,
        price: 200,
        rents_array: [25, 50, 100, 200],
        related_locations_array : [&MARYLEBRONE_STATION, &FRENCHUS_ST_STATION, &LIVERPOOL_STREET_STATION],

        owner_of_location: RwLock::new(OwnerEnum::None),
        current_rent_to_be_paid: RwLock::new(0),
        is_mortgaged: RwLock::new(false),
    };
    pub static MARYLEBRONE_STATION : TrainCard = TrainCard {
        name: "Marylebrone Station",
        reference_number: 0,
        price: 200,
        rents_array: [25, 50, 100, 200],
        related_locations_array : [&KING_CROSS_STATION, &FRENCHUS_ST_STATION, &LIVERPOOL_STREET_STATION],

        owner_of_location: RwLock::new(OwnerEnum::None),
        current_rent_to_be_paid: RwLock::new(0),
        is_mortgaged: RwLock::new(false),
    };
    pub static FRENCHUS_ST_STATION: TrainCard = TrainCard {
        name: "Frenchus St Station",
        reference_number: 0,
        price: 200,
        rents_array: [25, 50, 100, 200],
        related_locations_array : [&KING_CROSS_STATION, &MARYLEBRONE_STATION, &LIVERPOOL_STREET_STATION],

        owner_of_location: RwLock::new(OwnerEnum::None),
        current_rent_to_be_paid: RwLock::new(0),
        is_mortgaged: RwLock::new(false),
    };
    pub static LIVERPOOL_STREET_STATION: TrainCard = TrainCard {
        name: "Liverpool Street Station",
        reference_number: 0,
        price: 200,
        rents_array: [25, 50, 100, 200],
        related_locations_array : [&KING_CROSS_STATION, &MARYLEBRONE_STATION, &FRENCHUS_ST_STATION],

        owner_of_location: RwLock::new(OwnerEnum::None),
        current_rent_to_be_paid: RwLock::new(0),
        is_mortgaged: RwLock::new(false),
    };


    pub static ELECTRIC_COMPANY: UtilityCard = UtilityCard {
        name: "Electric Company",
        reference_number: 9,
        price: 150,
        rent_multiplier_array: [4, 10],
        related_locations_array: [&WATER_WORKS],

        owner_of_location: RwLock::new(OwnerEnum::None),
        current_rent_multiplier: RwLock::new(0),
        is_mortgaged: RwLock::new(false),
    };
    pub static WATER_WORKS: UtilityCard = UtilityCard {
        name: "Water Works",
        reference_number: 10,
        price: 150,
        rent_multiplier_array: [4, 10],
        related_locations_array: [&ELECTRIC_COMPANY],

        owner_of_location: RwLock::new(OwnerEnum::None),
        current_rent_multiplier: RwLock::new(0),
        is_mortgaged: RwLock::new(false),
    };


    pub static OLD_KENT_ROAD: PropertyCard = PropertyCard {
        name: "Old Kent Road",
        reference_number: 5,
        price: 60,
        building_cost: BuildingCost::Cost50,
        rents_array: [2, 10, 30, 90, 160, 250],
        related_locations_array: [Some(&WHITECHAPEL_ROAD), None],

        owner_of_location: RwLock::new(OwnerEnum::None),
        current_rent_to_be_paid: RwLock::new(0),
        is_mortgaged: RwLock::new(false),
        number_of_owned_buildings: RwLock::new(0)
    };

    pub static WHITECHAPEL_ROAD: PropertyCard = PropertyCard {
        name: "Whitechapel Road",
        reference_number: 6,
        price: 60,
        building_cost: BuildingCost::Cost50,
        rents_array: [4, 20, 60, 180, 320, 450],
        related_locations_array: [Some(&OLD_KENT_ROAD), None],

        owner_of_location: RwLock::new(OwnerEnum::None),
        current_rent_to_be_paid: RwLock::new(0),
        is_mortgaged: RwLock::new(false),
        number_of_owned_buildings: RwLock::new(0)
    };
}
