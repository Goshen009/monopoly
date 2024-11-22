






use crate::location_cards::{OwnerEnum, PropertyCard};
use crate::RwLock;

pub static MAYFAIR: PropertyCard = PropertyCard {
    name: "Mayfair",
    reference_number: 50,
    price: 60,
    building_cost: super::BuildingCost::Cost50,
    rents_array: [2, 10, 30, 90, 160, 250],
    related_locations_array: [Some(&PARK_LANE), Some(&THIRD_ONE)],

    owner_of_location: RwLock::new(OwnerEnum::PlayerThree),
    current_rent_to_be_paid: RwLock::new(0),
    is_mortgaged: RwLock::new(false),
    number_of_owned_buildings: RwLock::new(1)
};

pub static PARK_LANE: PropertyCard = PropertyCard {
    name: "Park Lane",
    reference_number: 51,
    price: 60,
    building_cost: super::BuildingCost::Cost50,
    rents_array: [2, 10, 30, 90, 160, 250],
    related_locations_array: [Some(&MAYFAIR), Some(&THIRD_ONE)],

    owner_of_location: RwLock::new(OwnerEnum::None),
    current_rent_to_be_paid: RwLock::new(0),
    is_mortgaged: RwLock::new(false),
    number_of_owned_buildings: RwLock::new(1)
};

pub static THIRD_ONE: PropertyCard = PropertyCard {
    name: "Third One",
    reference_number: 52,
    price: 60,
    building_cost: super::BuildingCost::Cost50,
    rents_array: [2, 10, 30, 90, 160, 250],
    related_locations_array: [Some(&MAYFAIR), Some(&PARK_LANE)],

    owner_of_location: RwLock::new(OwnerEnum::None),
    current_rent_to_be_paid: RwLock::new(0),
    is_mortgaged: RwLock::new(false),
    number_of_owned_buildings: RwLock::new(0)
};