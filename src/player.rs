





use crate::OwnerEnum;

pub struct Player {
    name : &'static str,
    player_enum : OwnerEnum,
    position : u8,
    money : i32,
    cards_owned : Vec<u8>
}

pub fn this_player_pays(player_to_be_paid: OwnerEnum, amount: i32) {
    match player_to_be_paid {
        OwnerEnum::PlayerOne => crate::PLAYER_ONE.write().unwrap().pay(amount),
        OwnerEnum::PlayerTwo => crate::PLAYER_TWO.write().unwrap().pay(amount),
        OwnerEnum::PlayerThree => crate::PLAYER_THREE.write().unwrap().pay(amount),
        OwnerEnum::PlayerFour => crate::PLAYER_FOUR.write().unwrap().pay(amount),
        OwnerEnum::None => println!("No player can pay because OwnerEnum is None"),
    }
}

pub fn this_player_receives(player_to_receive: OwnerEnum, amount: i32) {
    match player_to_receive {
        OwnerEnum::PlayerOne => crate::PLAYER_ONE.write().unwrap().receive(amount),
        OwnerEnum::PlayerTwo => crate::PLAYER_TWO.write().unwrap().receive(amount),
        OwnerEnum::PlayerThree => crate::PLAYER_THREE.write().unwrap().receive(amount),
        OwnerEnum::PlayerFour => crate::PLAYER_FOUR.write().unwrap().receive(amount),
        OwnerEnum::None => println!("No player can receive because OwnerEnum is None"),
    }
}

pub fn check_if_this_player_can_afford(player_to_check: OwnerEnum, amount: i32) -> bool {
    match player_to_check {
        OwnerEnum::PlayerOne => crate::PLAYER_ONE.write().unwrap().can_player_afford(amount),
        OwnerEnum::PlayerTwo => crate::PLAYER_TWO.write().unwrap().can_player_afford(amount),
        OwnerEnum::PlayerThree => crate::PLAYER_THREE.write().unwrap().can_player_afford(amount),
        OwnerEnum::PlayerFour => crate::PLAYER_FOUR.write().unwrap().can_player_afford(amount),
        OwnerEnum::None => {
            println!("No player to check on because OwnerEnum is None");
            false
        },
    }
}

impl Player {
    pub fn default(player: OwnerEnum) -> Self {
        Player {
            name : "",
            player_enum : player,
            position : 0,
            money : 1500,
            cards_owned : vec![]
        }
    }
}

impl Player {
    pub fn set_name(&mut self, player_name: &'static str) {
        self.name = player_name;
    }

    pub fn move_position(&mut self, move_amount: u8) -> u8 {
        self.position = self.position + move_amount;
        if self.position >= 3 {
            self.position = 0;
        }
        self.position
    }

    pub fn receive(&mut self, amount: i32) {
        self.money = self.money + amount;
        println!("{:?} received {} and now has {} left", self.player_enum, amount, self.money);
    }

    pub fn pay(&mut self, amount: i32) {
        self.money = self.money - amount;
        println!("{:?} paid {} and now has {} left", self.player_enum, amount, self.money);
    }

    pub fn bought_location(&mut self, location: u8) {
        self.cards_owned.push(location);

        println!("{:?} bought {} and now owns these locations: {:?}", self.player_enum, location, self.cards_owned);
    }

    pub fn sold_location(&mut self, location: u8) {
        self.cards_owned.retain(|&x| x != location);
        println!("{:?} sold {} and has these locations remaining {:?}", self.player_enum, location, self.cards_owned);
    }


    pub fn get_player_enum(&self) -> OwnerEnum {
        self.player_enum
    }

    pub fn can_player_afford(&self, price: i32) -> bool {
        if self.money >= price {
            return true;
        }

        false
    }
}