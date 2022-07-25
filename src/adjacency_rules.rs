use std::collections::HashMap;

use crate::{direction::Direction, helpers, id::Id, pixel::Pixel};

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

pub fn init(
    image_width: i32,
    image_height: i32,
    pixel_to_id: &HashMap<Pixel, Id>,
) -> AdjacencyRules {
    pixel_to_id.iter().fold(
        HashMap::new(),
        |mut adjacency_rules: AdjacencyRules, (pixel, id)| {
            let neighbors = helpers::get_neighbors(image_width, image_height, pixel);

            for (neighbor, direction) in neighbors {
                let neighbor_id = pixel_to_id[&neighbor];
                match adjacency_rules.get_mut(id) {
                    Some(rules) => {
                        match rules.get_mut(&neighbor_id) {
                            // this neighbor pixel was already found next to this pixel id
                            Some(direction_frequency) => {
                                let next_frequency = direction_frequency
                                    .get(&direction)
                                    .map(|frequency| frequency + 1)
                                    .unwrap_or(1);

                                direction_frequency.insert(direction, next_frequency);
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
