use std::collections::HashMap;

use crate::{
    data::{coord_2d::Vector2, id::Id, tile::Tile},
    image::Image,
};

// Model hold all data relevant to constructing and resolving the wave.
#[derive(Clone, Debug)]
pub struct Model {
    pub tile_dimensions: (usize, usize),
    pub id_to_tile: HashMap<Id, Tile>,
    pub tile_to_id: HashMap<Tile, Id>,
    pub frequency_hints: HashMap<Id, f64>,
}

impl Model {
    pub fn new(
        tile_dimensions: (usize, usize),
        with_tile_variations: bool,
        image: &Image,
    ) -> Model {
        let tile_to_freq = mk_tiles(tile_dimensions, image, with_tile_variations);
        let tile_to_id: HashMap<Tile, Id> = tile_to_freq
            .keys()
            .enumerate()
            .map(|(id, tile)| (tile.clone(), id))
            .collect();

        let id_to_tile: HashMap<Id, Tile> =
            tile_to_id.iter().map(|(k, v)| (*v, k.clone())).collect();

        let frequency_hints: HashMap<Id, f64> = mk_frequency_hints(&id_to_tile, &tile_to_id);

        Model {
            tile_dimensions,
            id_to_tile,
            tile_to_id,
            frequency_hints,
        }
    }
}

// Given an image and a tile size, construct and return the tiles
// paired with their frequency of occurance in the input image.
fn mk_tiles(
    tile_dimensions: (usize, usize),
    image: &Image,
    with_tile_variations: bool,
) -> HashMap<Tile, i32> {
    let (tile_w, tile_h) = tile_dimensions;
    let mut tile_to_freq: HashMap<Tile, i32> = HashMap::new();

    for y in 0..image.height as usize {
        for x in 0..image.width as usize {
            let mut pixels = vec![];
            for y_t in y..(y + tile_h) {
                let mut pixel_row = vec![];
                for x_t in x..(x + tile_w) {
                    let pixel = Vector2 {
                        x: (x_t as i32) % image.width as i32,
                        y: (y_t as i32) % image.height as i32,
                    };
                    let color = image.at(pixel);
                    pixel_row.push(color.clone());
                }
                pixels.push(pixel_row);
            }

            let tiles = if with_tile_variations {
                Tile::permute(pixels)
            } else {
                vec![Tile { pixels }]
            };

            for tile in tiles {
                let freq = tile_to_freq.get(&tile).map(|f| f + 1).unwrap_or(1);
                tile_to_freq.insert(tile, freq);
            }
        }
    }
    tile_to_freq
}

fn mk_frequency_hints(
    id_to_tile: &HashMap<Id, Tile>,
    tile_to_id: &HashMap<Tile, Id>,
) -> HashMap<Id, f64> {
    let total_ids = *(&id_to_tile.keys().len()) as f64;

    tile_to_id
        .iter()
        .fold(HashMap::new(), |mut freqs: HashMap<Id, i32>, (_, id)| {
            let next_frequency = freqs.get(id).map(|f| f + 1).unwrap_or(1);
            freqs.insert(*id, next_frequency);
            freqs
        })
        .iter()
        .map(|(id, freq)| (*id, *freq as f64 / total_ids))
        .collect()
}
