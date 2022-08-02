use std::collections::{HashMap, HashSet};

use crate::data::{direction::Direction, id::Id, tile::Tile};
use crate::model::Model;

type Rules = HashMap<Id, HashMap<Id, HashSet<Direction>>>;

//  Representation of the adjacency and direction between tile ids.
pub struct AdjacencyRules {
    rules: Rules,
}

impl AdjacencyRules {
    pub fn new() -> AdjacencyRules {
        AdjacencyRules {
            rules: HashMap::new(),
        }
    }

    pub fn from_model(model: &Model) -> AdjacencyRules {
        let mut adjacency_rules = AdjacencyRules::new();
        let tiles: Vec<&Tile> = model.tile_to_id.keys().collect();

        tiles.iter().for_each(|a| {
            tiles.iter().for_each(|b| {
                for direction in Direction::all() {
                    if a.overlaps(b, &direction, model.tile_dimensions) {
                        adjacency_rules.allow(
                            model.tile_to_id[&a],
                            model.tile_to_id[&b],
                            direction,
                        );
                    }
                }
            });
        });

        adjacency_rules
    }

    pub fn allow(&mut self, a: Id, b: Id, direction: Direction) {
        if let Some(rules) = self.rules.get_mut(&a) {
            if let Some(neighbors) = rules.get_mut(&b) {
                neighbors.insert(direction);
            } else {
                rules.insert(b, HashSet::from([direction]));
            }
        } else {
            self.rules
                .insert(a, HashMap::from([(b, HashSet::from([direction]))]));
        }
    }

    pub fn valid_neighbors(&self, me: &Id, neighbor: &Id, direction: &Direction) -> bool {
        let maybe_dir = self.rules.get(me).and_then(|rules| rules.get(neighbor));
        if let Some(directions) = maybe_dir {
            directions.contains(direction)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use crate::data::direction::Direction;

    use super::AdjacencyRules;

    #[test]
    fn test_valid_neighbors() {
        let adjacency_rules = AdjacencyRules {
            rules: HashMap::from([(0, HashMap::from([(1, HashSet::from([Direction::Up]))]))]),
        };

        assert!(adjacency_rules.valid_neighbors(&0, &1, &Direction::Up));
        assert!(!adjacency_rules.valid_neighbors(&0, &0, &Direction::Up));
        assert!(!adjacency_rules.valid_neighbors(&0, &1, &Direction::Down));
    }

    #[test]
    fn test_allow() {
        let mut rules = AdjacencyRules::new();
        let a = 0;
        let b = 1;

        rules.allow(a, b, Direction::Up);
        rules.allow(a, b, Direction::Up);
        rules.allow(a, b, Direction::Down);
        rules.allow(a, b, Direction::Down);

        assert!(rules.rules[&a][&b].contains(&Direction::Up));
        assert!(rules.rules[&a][&b].contains(&Direction::Down));
    }
}
