use std::collections::HashMap;

use crate::{color::Color, direction::Direction, id::Id, pixel::Pixel};

/*

    {
    L -> {
        S -> {
            UP -> 3
            DOWN -> 2
            LEFT -> 1
            RIGHT -> 0
        },
        L -> {
            UP -> 3
            DOWN -> 3
            LEFT -> 3
            RIGHT -> 3
        }
    }
}*/
/// A type that represents how frequently, and in which direction, colors appear
/// next to other colors.
pub type AdjacencyRules = HashMap<Id, HashMap<Id, HashMap<Direction, i32>>>;

pub type PixelToId = HashMap<Pixel, Id>;
pub type IdToColor = HashMap<Id, Color>;
pub type ColorToId = HashMap<Color, Id>;
