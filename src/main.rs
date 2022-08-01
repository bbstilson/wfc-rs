use std::collections::HashMap;

use adjacency_rules::AdjacencyRules;
use argh::{self, FromArgs};
use data::{coord_2d::Coord2d, id::Id, tile::Tile};
use model::Model;
use wave_function::WaveFunction;

mod adjacency_rules;
mod data;
mod helpers;
mod image;
mod model;
mod wave_function;

#[derive(FromArgs, Debug)]
/// Run wfc-rs
struct WFCArgs {
    /// input image
    #[argh(option)]
    input: String,

    /// output width in tiles
    #[argh(option, default = "30")]
    output_width: usize,

    /// output height in tiles
    #[argh(option, default = "15")]
    output_height: usize,

    /// size of tile to parse from input image
    #[argh(option, default = "3")]
    tile_size: usize,

    /// whether or not to take snapshot images
    #[argh(switch)]
    take_snapshots: bool,

    /// whether or not create all variations (rotations and reflections) of tiles
    #[argh(switch)]
    with_tile_variations: bool,
}

// TODO:
// - overlap or tiled
// - reflections and rotations of tiles

fn main() {
    let args: WFCArgs = argh::from_env();

    let input = image::Image::from_png(&args.input);
    let model = Model::new(args.tile_size, args.with_tile_variations, &input);
    let id_to_tile: HashMap<Id, Tile> = model.id_to_tile.clone();
    let adjacency_rules = AdjacencyRules::from_model(&model);

    let mut wave_function = WaveFunction::new(
        args.output_width,
        args.output_height,
        adjacency_rules,
        model,
        args.take_snapshots,
        args.tile_size,
        id_to_tile.clone(),
    );

    let state = wave_function.run();

    let mut data = vec![
        vec![128; args.output_width as usize * 3 * args.tile_size];
        args.output_height as usize * args.tile_size
    ];

    for g_y in 0..args.output_height as usize {
        for g_x in 0..args.output_width as usize {
            let pixel = Coord2d {
                x: g_x as i32,
                y: g_y as i32,
            };
            let id = state[&pixel];
            let tile = &id_to_tile[&id];

            for n_y in 0..args.tile_size {
                for n_x in 0..args.tile_size {
                    let color = &tile.pixels[n_y][n_x];
                    let f_x = (g_x * args.tile_size * 3) + (n_x * 3);
                    let f_y = (g_y * args.tile_size) + n_y;
                    data[f_y][f_x] = color.0[0];
                    data[f_y][f_x + 1] = color.0[1];
                    data[f_y][f_x + 2] = color.0[2];
                }
            }
        }
    }

    image::output_image("final", data);
}
