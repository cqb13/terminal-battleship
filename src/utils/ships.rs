use crate::{Ship, ShipType, ShipOrientation};

pub fn get_ship(ship: ShipType) -> Ship {
    match ship {
        ShipType::CarrierHorizontal => Ship {
            ship_type: ShipType::CarrierHorizontal,
            orientation: ShipOrientation::Horizontal,
            length: 5,
        },
        ShipType::BattleshipHorizontal => Ship {
            ship_type: ShipType::BattleshipHorizontal,
            orientation: ShipOrientation::Horizontal,
            length: 4,
        },
        ShipType::CruiserHorizontal => Ship {
            ship_type: ShipType::CruiserHorizontal,
            orientation: ShipOrientation::Horizontal,
            length: 3,
        },
        ShipType::SubmarineHorizontal => Ship {
            ship_type: ShipType::SubmarineHorizontal,
            orientation: ShipOrientation::Horizontal,
            length: 3,
        },
        ShipType::DestroyerHorizontal => Ship {
            ship_type: ShipType::DestroyerHorizontal,
            orientation: ShipOrientation::Horizontal,
            length: 2,
        },
        ShipType::CarrierVertical => Ship {
            ship_type: ShipType::CarrierVertical,
            orientation: ShipOrientation::Vertical,
            length: 5,
        },
        ShipType::BattleshipVertical => Ship {
            ship_type: ShipType::BattleshipVertical,
            orientation: ShipOrientation::Vertical,
            length: 4,
        },
        ShipType::CruiserVertical => Ship {
            ship_type: ShipType::CruiserVertical,
            orientation: ShipOrientation::Vertical,
            length: 3,
        },
        ShipType::SubmarineVertical => Ship {
            ship_type: ShipType::SubmarineVertical,
            orientation: ShipOrientation::Vertical,
            length: 3,
        },
        ShipType::DestroyerVertical => Ship {
            ship_type: ShipType::DestroyerVertical,
            orientation: ShipOrientation::Vertical,
            length: 2,
        },
    }
}
