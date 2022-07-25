use std::collections::HashMap;

use crate::{direction::Direction, helpers, id::Id, pixel::Pixel};

/*

    {
    L -> {
        S -> {
            Direction::UP -> 3
            Direction::DOWN -> 2
            LEFT -> 1
            RIGHT -> 0
        },
        L -> {
            Direction::UP -> 3
            Direction::DOWN -> 3
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

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{direction::Direction, id::Id, pixel::Pixel};

    use super::init;

    #[test]
    fn test_init() {
        let expected = HashMap::from([
            (
                Id(0), // sand is next to...
                HashMap::from([
                    (
                        Id(0), // sand
                        HashMap::from([
                            (Direction::Right, 1),
                            (Direction::DownRight, 1),
                            (Direction::Left, 1),
                            (Direction::UpLeft, 1),
                        ]),
                    ),
                    (
                        Id(1), // sea
                        HashMap::from([
                            (Direction::Up, 3),
                            (Direction::Right, 1),
                            (Direction::UpRight, 2),
                            (Direction::UpLeft, 1),
                        ]),
                    ),
                    (
                        Id(2), // land
                        HashMap::from([
                            (Direction::Down, 2),
                            (Direction::Left, 1),
                            (Direction::DownRight, 1),
                            (Direction::DownLeft, 1),
                        ]),
                    ),
                ]),
            ),
            (
                Id(1), // sea is next to:
                HashMap::from([
                    // land
                    (Id(2), HashMap::from([(Direction::DownLeft, 1)])),
                    // sea
                    (
                        Id(1),
                        HashMap::from([
                            (Direction::Left, 2),
                            (Direction::Down, 1),
                            (Direction::Up, 1),
                            (Direction::DownRight, 1),
                            (Direction::Right, 2),
                            (Direction::UpLeft, 1),
                        ]),
                    ),
                    // sand
                    (
                        Id(0),
                        HashMap::from([
                            (Direction::Left, 1),
                            (Direction::Down, 3),
                            (Direction::DownRight, 1),
                            (Direction::DownLeft, 2),
                        ]),
                    ),
                ]),
            ),
            (
                Id(2), // land is next to:
                HashMap::from([
                    (
                        Id(0), // sand
                        HashMap::from([
                            (Direction::Right, 1),
                            (Direction::Up, 2),
                            (Direction::UpLeft, 1),
                            (Direction::UpRight, 1),
                        ]),
                    ),
                    (
                        Id(2), // land
                        HashMap::from([(Direction::Left, 1), (Direction::Right, 1)]),
                    ),
                    // sea
                    (Id(1), HashMap::from([(Direction::UpRight, 1)])),
                ]),
            ),
        ]);

        let pixel_to_id = HashMap::from([
            (Pixel { x: 0, y: 0 }, Id(1)),
            (Pixel { x: 1, y: 0 }, Id(1)),
            (Pixel { x: 2, y: 0 }, Id(1)),
            (Pixel { x: 0, y: 1 }, Id(0)),
            (Pixel { x: 1, y: 1 }, Id(0)),
            (Pixel { x: 2, y: 1 }, Id(1)),
            (Pixel { x: 0, y: 2 }, Id(2)),
            (Pixel { x: 1, y: 2 }, Id(2)),
            (Pixel { x: 2, y: 2 }, Id(0)),
        ]);
        let adjacency_rules = init(3, 3, &pixel_to_id);
        assert_eq!(adjacency_rules, expected)
    }
}
