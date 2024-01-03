use crate::{Ship, ShipOrientation, ShipType};

pub fn get_ship(ship: ShipType) -> Ship {
    match ship {
        ShipType::CarrierHorizontal => Ship {
            ship_type: ShipType::CarrierHorizontal,
            orientation: ShipOrientation::Horizontal,
        },
        ShipType::BattleshipHorizontal => Ship {
            ship_type: ShipType::BattleshipHorizontal,
            orientation: ShipOrientation::Horizontal,
        },
        ShipType::CruiserHorizontal => Ship {
            ship_type: ShipType::CruiserHorizontal,
            orientation: ShipOrientation::Horizontal,
        },
        ShipType::SubmarineHorizontal => Ship {
            ship_type: ShipType::SubmarineHorizontal,
            orientation: ShipOrientation::Horizontal,
        },
        ShipType::DestroyerHorizontal => Ship {
            ship_type: ShipType::DestroyerHorizontal,
            orientation: ShipOrientation::Horizontal,
        },
        ShipType::CarrierVertical => Ship {
            ship_type: ShipType::CarrierVertical,
            orientation: ShipOrientation::Vertical,
        },
        ShipType::BattleshipVertical => Ship {
            ship_type: ShipType::BattleshipVertical,
            orientation: ShipOrientation::Vertical,
        },
        ShipType::CruiserVertical => Ship {
            ship_type: ShipType::CruiserVertical,
            orientation: ShipOrientation::Vertical,
        },
        ShipType::SubmarineVertical => Ship {
            ship_type: ShipType::SubmarineVertical,
            orientation: ShipOrientation::Vertical,
        },
        ShipType::DestroyerVertical => Ship {
            ship_type: ShipType::DestroyerVertical,
            orientation: ShipOrientation::Vertical,
        },
    }
}
