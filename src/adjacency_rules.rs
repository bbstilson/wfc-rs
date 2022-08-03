use crate::data::{direction::Direction, id::Id, tile::Tile};
use crate::model::Model;

//  Representation of the adjacency and direction between tile ids.
pub struct AdjacencyRules {
    rules: Vec<Vec<Vec<bool>>>,
}

impl AdjacencyRules {
    pub fn from_model(model: &Model) -> AdjacencyRules {
        let num_tile_ids = model.id_to_tile.keys().len();
        let mut rules = AdjacencyRules {
            rules: vec![vec![vec![false, false, false, false]; num_tile_ids]; num_tile_ids],
        };

        let tiles: Vec<&Tile> = model.tile_to_id.keys().collect();

        tiles.iter().for_each(|a| {
            tiles.iter().for_each(|b| {
                for direction in Direction::all() {
                    if a.overlaps(b, &direction, model.tile_dimensions) {
                        rules.allow(model.tile_to_id[&a], model.tile_to_id[&b], direction);
                    }
                }
            });
        });

        rules
    }

    pub fn allow(&mut self, a: Id, b: Id, direction: Direction) {
        self.rules[a][b][direction.idx()] = true;
    }

    pub fn valid_neighbors(&self, me: Id, neighbor: Id, direction: &Direction) -> bool {
        self.rules[me][neighbor][direction.idx()]
    }
}

// #[cfg(test)]
// mod tests {
// use std::collections::{HashMap, HashSet};

// use crate::data::direction::Direction;

// use super::AdjacencyRules;

// #[test]
// fn test_valid_neighbors() {
//     let adjacency_rules = AdjacencyRules {
//         rules: HashMap::from([(0, HashMap::from([(1, HashSet::from([Direction::Up]))]))]),
//     };

//     assert!(adjacency_rules.valid_neighbors(&0, &1, &Direction::Up));
//     assert!(!adjacency_rules.valid_neighbors(&0, &0, &Direction::Up));
//     assert!(!adjacency_rules.valid_neighbors(&0, &1, &Direction::Down));
// }

// #[test]
// fn test_allow() {
//     let mut rules = AdjacencyRules::new();
//     let a = 0;
//     let b = 1;

//     rules.allow(a, b, Direction::Up);
//     rules.allow(a, b, Direction::Up);
//     rules.allow(a, b, Direction::Down);
//     rules.allow(a, b, Direction::Down);

//     assert!(rules.rules[&a][&b].contains(&Direction::Up));
//     assert!(rules.rules[&a][&b].contains(&Direction::Down));
// }
// }
