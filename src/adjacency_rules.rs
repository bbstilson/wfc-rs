use crate::data::{direction, id::Id, tile::Tile};
use crate::model::Model;

//  Representation of the adjacency and direction between tile ids.
pub struct AdjacencyRules {
    pub rules: Vec<Vec<Vec<bool>>>,
}

impl AdjacencyRules {
    pub fn new(num_tiles: usize) -> AdjacencyRules {
        AdjacencyRules {
            rules: vec![vec![vec![false, false, false, false]; num_tiles]; num_tiles],
        }
    }

    pub fn from_overlap_model(model: &Model) -> AdjacencyRules {
        let num_tiles = model.id_to_tile.keys().len();
        let mut rules = AdjacencyRules::new(num_tiles);

        let tiles: Vec<&Tile> = model.tile_to_id.keys().collect();
        for a in &tiles {
            for b in &tiles {
                for direction in direction::ALL {
                    if a.overlaps(b, direction, model.mode) {
                        rules.allow(model.tile_to_id[&a], model.tile_to_id[&b], direction);
                    }
                }
            }
        }

        rules
    }

    // I would like to support two tiled models:
    // 1) from a pre-built world
    // 2) from a raw tile set
    //
    // At time of writing, only the former is supported.
    //
    // The tiled model differs from the overlap model greatly in the first model, and
    // only slightly in the second. For the first, we need to know the original
    // positions of the tiles so that we can compare their neighbors. This is the pick
    // up the "artistic intent" of the world design. In the second model, we still want
    // to do an overlap comparison, but only if the sides share the one-pixel-thick
    // edge.
    pub fn from_tile_model(model: &Model) -> AdjacencyRules {
        let num_tiles = model.id_to_tile.keys().len();
        let mut rules = AdjacencyRules::new(num_tiles);

        for (position, tile) in &model.position_to_tile {
            direction::ALL
                .iter()
                .filter_map(|d| {
                    model
                        .position_to_tile
                        .get(&position.in_direction(*d))
                        .map(|tile| (d, tile))
                })
                .for_each(|(direction, neighbor)| {
                    rules.allow(
                        model.tile_to_id[tile],
                        model.tile_to_id[neighbor],
                        *direction,
                    )
                });
        }

        rules
    }

    pub fn allow(&mut self, a: Id, b: Id, direction: direction::Direction) {
        self.rules[a][b][direction.idx()] = true;
    }

    pub fn valid_neighbors(&self, a: Id, b: Id, direction: direction::Direction) -> bool {
        self.rules[a][b][direction.idx()]
    }
}
