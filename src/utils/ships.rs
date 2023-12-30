use crate::{Ship, ShipType};

pub fn get_ship(ship: ShipType) -> Ship {
    match ship {
        ShipType::CarrierHorizontal => Ship::new(ShipType::CarrierHorizontal, 5),
        ShipType::BattleshipHorizontal => Ship::new(ShipType::BattleshipHorizontal, 4),
        ShipType::CruiserHorizontal => Ship::new(ShipType::CruiserHorizontal, 3),
        ShipType::SubmarineHorizontal => Ship::new(ShipType::SubmarineHorizontal, 3),
        ShipType::DestroyerHorizontal => Ship::new(ShipType::DestroyerHorizontal, 2),
        ShipType::CarrierVertical => Ship::new(ShipType::CarrierVertical, 5),
        ShipType::BattleshipVertical => Ship::new(ShipType::BattleshipVertical, 4),
        ShipType::CruiserVertical => Ship::new(ShipType::CruiserVertical, 3),
        ShipType::SubmarineVertical => Ship::new(ShipType::SubmarineVertical, 3),
        ShipType::DestroyerVertical => Ship::new(ShipType::DestroyerVertical, 2),
    }
}
