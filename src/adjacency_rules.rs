use crate::data::{direction, id::Id, tile::Tile};
use crate::model::Model;

//  Representation of the adjacency and direction between tile ids.
pub struct AdjacencyRules {
    pub rules: Vec<Vec<Vec<bool>>>,
}

impl AdjacencyRules {
    pub fn from_model(model: &Model) -> AdjacencyRules {
        let num_tiles = model.id_to_tile.keys().len();

        let mut rules = AdjacencyRules {
            rules: vec![vec![vec![false, false, false, false]; num_tiles]; num_tiles],
        };

        let tiles: Vec<&Tile> = model.tile_to_id.keys().collect();
        for a in &tiles {
            for b in &tiles {
                for direction in direction::ALL {
                    if a.overlaps(b, &direction, model.tile_dimensions) {
                        rules.allow(model.tile_to_id[&a], model.tile_to_id[&b], direction);
                    }
                }
            }
        }

        rules
    }

    pub fn allow(&mut self, a: Id, b: Id, direction: direction::Direction) {
        self.rules[a][b][direction] = true;
    }

    pub fn valid_neighbors(&self, a: Id, b: Id, direction: direction::Direction) -> bool {
        self.rules[a][b][direction]
    }
}
