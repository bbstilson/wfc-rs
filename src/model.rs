use std::collections::HashMap;

use crate::{
    data::{coord_2d::Coord2d, id::Id, tile::Tile},
    image::Image,
};

// Model hold all data relevant to constructing and resolving the wave.
#[derive(Clone, Debug)]
pub struct Model {
    pub id_to_tile: HashMap<Id, Tile>,
    pub tile_to_id: HashMap<Tile, Id>,
    pub frequency_hints: HashMap<Id, f64>,
}

impl Model {
    pub fn new(tile_size: usize, with_tile_variations: bool, image: &Image) -> Model {
        let tile_to_freq = mk_tiles(tile_size, image, with_tile_variations);
        let tile_to_id: HashMap<Tile, Id> = tile_to_freq
            .keys()
            .enumerate()
            .map(|(id, tile)| (tile.clone(), id))
            .collect();

        let id_to_tile: HashMap<Id, Tile> =
            tile_to_id.iter().map(|(k, v)| (*v, k.clone())).collect();

        let frequency_hints: HashMap<Id, f64> = mk_frequency_hints(&id_to_tile, &tile_to_id);

        Model {
            id_to_tile,
            tile_to_id,
            frequency_hints,
        }
    }
}

// Given an image and a tile size, construct and return the tiles
// paired with their frequency of occurance in the input image.
fn mk_tiles(tile_size: usize, image: &Image, with_tile_variations: bool) -> HashMap<Tile, i32> {
    let mut tile_to_freq: HashMap<Tile, i32> = HashMap::new();

    for y in 0..image.height as usize {
        for x in 0..image.width as usize {
            let mut pixels = vec![];
            for y_t in y..(y + tile_size) {
                let mut pixel_row = vec![];
                for x_t in x..(x + tile_size) {
                    let pixel = Coord2d {
                        x: (x_t as i32) % image.width,
                        y: (y_t as i32) % image.height,
                    };
                    let color = &image.pixels[&pixel];
                    pixel_row.push(color.clone());
                }
                pixels.push(pixel_row);
            }
            for tile in mk_versions(Tile { pixels }, with_tile_variations) {
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

fn mk_versions(tile: Tile, with_tile_variations: bool) -> Vec<Tile> {
    let mut tiles = vec![];
    tiles.push(tile);

    if with_tile_variations {
        unimplemented!()
        // TODO: conditionally make all the rotations
        // if symmetry.contains(S2_IDENTITY) {
        //     symm_bufs.push(buf.clone())
        // }
        // if symmetry.contains(S2_ROTATE_90) {
        //     symm_bufs.push(rotate90(&buf))
        // }
        // if symmetry.contains(S2_ROTATE_180) {
        //     symm_bufs.push(rotate180(&buf))
        // }
        // if symmetry.contains(S2_ROTATE_270) {
        //     symm_bufs.push(rotate270(&buf))
        // }
        // if symmetry.contains(S2_REFLECT_Y) {
        //     symm_bufs.push(flip_horizontal(&buf))
        // }
        // if symmetry.contains(S2_REFLECT_X) {
        //     symm_bufs.push(flip_vertical(&buf))
        // }
        // if symmetry.contains(S2_REFLECT_X_ROT90) {
        //     symm_bufs.push(flip_vertical(&rotate90(&buf)))
        // }
        // if symmetry.contains(S2_REFLECT_Y_ROT90) {
        //     symm_bufs.push(flip_horizontal(&rotate90(&buf)))
        // }
    }

    tiles
}
