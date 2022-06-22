use crate::console::Console;
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
            TileType::CobbleTL => 19,
            TileType::CobbleT => 20,
            TileType::CobbleTR => 21,
            TileType::CobbleL => 35,
            TileType::Cobble => 36,
            TileType::CobbleR => 37,
            TileType::CobbleBL => 51,
            TileType::CobbleB => 52,
            TileType::CobbleBR => 53,
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
            _ => None,
        }
    }

    pub fn can_player_enter(&self) -> bool {
        match self {
            TileType::FenceHorizontal
            | TileType::FenceVertical
            | TileType::Bush
            | TileType::HayCart
            | TileType::Barn => false,
            TileType::LeftButte => false,
            _ => true,
        }
    }

    pub fn interact(&self, console: &Console) {
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
            _ => {}
        }
    }
}
