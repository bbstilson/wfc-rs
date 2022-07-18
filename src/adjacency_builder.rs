use std::collections::HashMap;

use crate::{
    direction::Direction,
    id::Id,
    pixel::Pixel,
    types::{AdjacencyRules, PixelToId},
};

pub struct AdjacencyRulesBuilder {
    pub image_width: i32,
    pub image_height: i32,
    pub pixel_to_id: PixelToId,
}

impl AdjacencyRulesBuilder {
    pub fn build(&self) -> AdjacencyRules {
        self.pixel_to_id.iter().fold(
            HashMap::new(),
            |mut adjacency_rules: AdjacencyRules, (pixel, id)| {
                let neighbors = self.get_neighbors(pixel);

                for (neighbor_id, direction) in neighbors {
                    match adjacency_rules.get_mut(id) {
                        Some(rules) => {
                            match rules.get_mut(&neighbor_id) {
                                // this neighbor pixel was already found next to this pixel id
                                Some(direction_frequency) => {
                                    match direction_frequency.get(&direction) {
                                        Some(frequency) => {
                                            direction_frequency.insert(direction, frequency + 1);
                                        }

                                        None => {
                                            direction_frequency.insert(direction, 1);
                                        }
                                    };
                                }
                                // this neighbor pixel has never been adjacent to this
                                // pixel id, but there might be other rules.
                                None => {
                                    rules.insert(neighbor_id, HashMap::from([(direction, 1)]));
                                }
                            }
                        }
                        // we've never seen this id before.
                        None => {
                            adjacency_rules.insert(
                                *id,
                                HashMap::from([(neighbor_id, HashMap::from([(direction, 1)]))]),
                            );
                        }
                    };
                }

                adjacency_rules
            },
        )
    }

    fn get_neighbors(&self, pixel: &Pixel) -> Vec<(Id, Direction)> {
        let x = pixel.x;
        let y = pixel.y;
        vec![
            if y == 0 {
                None
            } else {
                Some((Pixel { x: x, y: y - 1 }, Direction::UP))
            },
            if y == (self.image_height - 1) {
                None
            } else {
                Some((Pixel { x: x, y: y + 1 }, Direction::DOWN))
            },
            if x == 0 {
                None
            } else {
                Some((Pixel { x: x - 1, y }, Direction::LEFT))
            },
            if x == (self.image_width - 1) {
                None
            } else {
                Some((Pixel { x: x + 1, y }, Direction::RIGHT))
            },
        ]
        .iter()
        .flatten()
        .map(|(neighbor, direction)| (self.pixel_to_id[neighbor], *direction))
        .collect::<Vec<(Id, Direction)>>()
    }
}
