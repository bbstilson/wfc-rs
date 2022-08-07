use std::{collections::HashMap, path::PathBuf};

use anyhow::Result;

use crate::{
    data::{id::Id, mode::Mode, tile::Tile, vector2::Vector2},
    image::Image,
};

// Model holds all data relevant to constructing and resolving the wave.
#[derive(Clone)]
pub struct Model {
    pub mode: Mode,
    pub tile_dimensions: (u32, u32),
    pub tile_to_positions: HashMap<Tile, Vec<Vector2>>,
    pub position_to_tile: HashMap<Vector2, Tile>,
    pub id_to_tile: HashMap<Id, Tile>,
    pub tile_to_id: HashMap<Tile, Id>,
    pub frequency_hints: HashMap<Id, f64>,
}

impl Model {
    // Given an image and a tile size, construct and return the tiles
    // paired with their frequency of occurance in the input image.
    pub fn overlap(
        image_path: PathBuf,
        tile_dimensions: (u32, u32),
        with_tile_variations: bool,
    ) -> Result<Model> {
        let image = Image::from_png(image_path)?;
        let (tile_w, tile_h) = tile_dimensions;
        let mut tile_to_freq: HashMap<Tile, i32> = HashMap::new();

        for y in 0..image.height {
            for x in 0..image.width {
                let mut pixels = vec![];
                for y_t in y..(y + tile_h) {
                    for x_t in x..(x + tile_w) {
                        let pixel = Vector2 {
                            x: (x_t as i32) % image.width as i32,
                            y: (y_t as i32) % image.height as i32,
                        };
                        let color = image.at(pixel);
                        pixels.push(color.clone());
                    }
                }

                let tile = Tile {
                    width: tile_w,
                    height: tile_h,
                    pixels,
                };

                let tiles = if with_tile_variations {
                    tile.permute() // TODO
                } else {
                    vec![tile]
                };

                for tile in tiles {
                    let freq = tile_to_freq.get(&tile).map(|f| f + 1).unwrap_or(1);
                    tile_to_freq.insert(tile, freq);
                }
            }
        }
        Ok(Model::from_frequency_and_position_maps(
            Mode::Overlap,
            tile_to_freq,
            // tiles generated in an overlap mode don't have positions
            HashMap::new(),
            HashMap::new(),
        ))
    }

    pub fn tiled(
        image_path: PathBuf,
        tile_dimensions: (u32, u32),
        with_tile_variations: bool,
    ) -> Result<Model> {
        let image = Image::from_png(image_path)?;
        let (tile_width, tile_height) = tile_dimensions;

        assert!(
            image.width % tile_width == 0 && image.height % tile_height == 0,
            "Tiled Sheet must be evenly divisible by the tile dimensions."
        );

        let mut tile_to_positions: HashMap<Tile, Vec<Vector2>> = HashMap::new();
        let mut position_to_tile: HashMap<Vector2, Tile> = HashMap::new();
        let mut tile_to_freq: HashMap<Tile, i32> = HashMap::new();

        for y in (0..image.height).step_by(tile_height as usize) {
            for x in (0..image.width).step_by(tile_width as usize) {
                // normalize the position
                let position = Vector2 {
                    x: (x / tile_width) as i32,
                    y: (y / tile_height) as i32,
                };
                let mut pixels = vec![];

                for t_y in y..(y + tile_height) {
                    for t_x in x..(x + tile_width) {
                        let pixel = Vector2 {
                            x: (t_x as i32) % image.width as i32,
                            y: (t_y as i32) % image.height as i32,
                        };
                        pixels.push(image.at(pixel));
                    }
                }

                let tile = Tile {
                    width: tile_width,
                    height: tile_height,
                    pixels,
                };

                let tiles = if with_tile_variations {
                    tile.permute() // TODO
                } else {
                    vec![tile.clone()]
                };

                for tile in tiles {
                    let freq = tile_to_freq.get(&tile).map(|f| f + 1).unwrap_or(1);
                    tile_to_freq.insert(tile, freq);
                }

                position_to_tile.insert(position, tile.clone());
                match tile_to_positions.get_mut(&tile) {
                    Some(positions) => positions.push(position),
                    None => {
                        tile_to_positions.insert(tile, vec![position]);
                    }
                }
            }
        }

        Ok(Model::from_frequency_and_position_maps(
            Mode::Tile,
            tile_to_freq,
            tile_to_positions,
            position_to_tile,
        ))
    }

    fn from_frequency_and_position_maps(
        mode: Mode,
        tile_to_freq: HashMap<Tile, i32>,
        tile_to_positions: HashMap<Tile, Vec<Vector2>>,
        position_to_tile: HashMap<Vector2, Tile>,
    ) -> Model {
        let tile_to_id: HashMap<Tile, Id> = tile_to_freq
            .keys()
            .enumerate()
            .map(|(id, tile)| (tile.clone(), id))
            .collect();

        let id_to_tile: HashMap<Id, Tile> =
            tile_to_id.iter().map(|(k, v)| (*v, k.clone())).collect();

        let frequency_hints: HashMap<Id, f64> = mk_frequency_hints(&tile_to_id);

        let first_tile = tile_to_id.iter().next().unwrap().0;
        let tile_dimensions = (first_tile.width, first_tile.height);

        Model {
            mode,
            id_to_tile,
            tile_to_id,
            frequency_hints,
            tile_dimensions,
            tile_to_positions,
            position_to_tile,
        }
    }
}

fn mk_frequency_hints(tile_to_id: &HashMap<Tile, Id>) -> HashMap<Id, f64> {
    let total_ids = *(&tile_to_id.keys().len()) as f64;

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
