use crate::{combat::Health, console::Console};
use bevy::prelude::Color;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum TileType {
    None,
    Grass,
    Dirt,
    FenceHorizontal,
    FenceVertical,
    Bush,
    Flower,
    Road,
    HayCart,
    ReferTo(usize),
    Barn,
    LeftButte,
    CaveMouth,
    Cauldron,
    CobbleTL,
    CobbleT,
    CobbleTR,
    CobbleL,
    Cobble,
    CobbleR,
    CobbleBL,
    CobbleB,
    CobbleBR,
    WaterL,
    WaterTL,
    WaterT,
    WaterTR,
    WaterR,
    WaterBL,
    WaterB,
    WaterBR,
    //WaterLR,
    WaterTB,
    WaterV2,
    WaterV3,
    WaterV4,
    WaterV5,
    WaterV6,
    Anvil,
    CaveFloor,
    Water,
    Grain,
    Fire,
    GoldEgg,
}

impl TileType {
    pub fn index(&self) -> usize {
        match self {
            TileType::None => 0, // Special case
            TileType::Grass => 0,
            TileType::Dirt => 1,
            TileType::FenceHorizontal => 2,
            TileType::FenceVertical => 3,
            TileType::Bush => 4,
            TileType::Flower => 5,
            TileType::Road => 6,
            TileType::Cauldron => 7,
            TileType::Anvil => 8,
            TileType::CaveFloor => 9,
            TileType::Water => 10,
            TileType::Grain => 11,
            TileType::Fire => 12,
            TileType::GoldEgg => 13,
            TileType::CobbleTL => 19,
            TileType::CobbleT => 20,
            TileType::CobbleTR => 21,
            TileType::CobbleL => 35,
            TileType::Cobble => 36,
            TileType::CobbleR => 37,
            TileType::CobbleBL => 51,
            TileType::CobbleB => 52,
            TileType::CobbleBR => 53,
            TileType::WaterL => 22,
            TileType::WaterTL => 23,
            TileType::WaterT => 24,
            TileType::WaterTR => 25,
            TileType::WaterR => 26,
            TileType::WaterBL => 27,
            TileType::WaterB => 28,
            TileType::WaterBR => 29,
            //TileType::WaterLR => 30,
            TileType::WaterTB => 31,
            TileType::WaterV2 => 38,
            TileType::WaterV3 => 39,
            TileType::WaterV4 => 40,
            TileType::WaterV5 => 41,
            TileType::WaterV6 => 42,
            _ => 0,
        }
    }

    pub fn should_render(&self) -> bool {
        match self {
            TileType::None => false,
            TileType::ReferTo(..) => false,
            _ => true,
        }
    }

    pub fn extra_big(&self) -> Option<(i32, i32, Vec<usize>)> {
        match self {
            TileType::HayCart => Some((3, 2, vec![16, 17, 18, 32, 33, 34])),
            TileType::Barn => Some((2, 3, vec![48, 49, 64, 65, 80, 81])),
            TileType::LeftButte => Some((
                2,
                7,
                vec![
                    112, 113, 128, 129, 144, 145, 160, 161, 176, 177, 192, 193, 208, 209,
                ],
            )),
            TileType::CaveMouth => Some((
                6,
                1,
                vec![
                    //146, 147, 148, 149, 150, 151, 162, 163, 164, 165, 166, 167,
                    178, 179, 180, 181, 182, 183,
                ],
            )),
            _ => None,
        }
    }

    pub fn can_player_enter(&self) -> bool {
        match self {
            TileType::FenceHorizontal
            | TileType::FenceVertical
            | TileType::Bush
            | TileType::HayCart
            | TileType::Barn
            | TileType::Anvil
            | TileType::CaveMouth
            | TileType::Water
            | TileType::WaterV2
            | TileType::WaterV3
            | TileType::WaterV4
            | TileType::WaterV5
            | TileType::WaterV6
            | TileType::Fire => false,
            _ => true,
        }
    }

    pub fn interact(&self, console: &Console, health: &mut Health) {
        match self {
            TileType::FenceHorizontal | TileType::FenceVertical => {
                console.write(
                    "There's a fence here. Maybe you can Jump over it?",
                    Color::WHITE,
                );
            }
            TileType::Bush => {
                console.write(
                    "This bush is prickly, but you might be able to jump it.",
                    Color::WHITE,
                );
            }
            TileType::Cauldron => {
                console.write(
                    "I guess that explains why you're a 6 foot tall chicken!",
                    Color::YELLOW,
                );
                console.write("Farmer Tom's Magic Miracle Grow", Color::YELLOW);
            }
            TileType::Grain => {
                console.write("Yummy, grain!", Color::GREEN);
                health.current = health.max;
            }
            _ => {}
        }
    }
}
