use std::{collections::HashMap, str::FromStr};

use adjacency_rules::AdjacencyRules;
use clap::{Parser, ValueEnum};
use data::{coord_2d::Vector2, id::Id, tile::Tile};
use model::Model;
use wave_function::WaveFunction;

mod adjacency_rules;
mod data;
mod helpers;
mod image;
mod model;
mod wave_function;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
/// Run wfc-rs
struct Args {
    /// input image location
    input: String,

    /// output dimensions in pixels
    #[clap(short, long, value_parser = parse_tuple_arg)]
    output_dimensions: (usize, usize),

    /// tile dimensions to parse from input image
    #[clap(short, long, value_parser = parse_tuple_arg)]
    tile_dimensions: (usize, usize),

    /// whether or not to take snapshot images
    #[clap(short, long)]
    snapshots: bool,

    /// whether or not create all variations (rotations and reflections) of tiles
    #[clap(short, long)]
    with_tile_variations: bool,

    /// parse input as a tiled map
    #[clap(long, arg_enum, default_value_t = ParseMethod::Overlap)]
    parse_method: ParseMethod,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum ParseMethod {
    Overlap,
    Tiled,
}

fn parse_tuple_arg(value: &str) -> Result<(usize, usize), String> {
    let parts = value
        .split(',')
        .map(|part| part.trim())
        .collect::<Vec<&str>>();

    if parts.len() == 2 {
        if let (Ok(w), Ok(h)) = (usize::from_str(parts[0]), usize::from_str(parts[1])) {
            Ok((w, h))
        } else {
            Err(format!(
                "Could not parse value into tuple of (usize, usize): {}",
                value
            ))
        }
    } else {
        Err(format!(
            "Could not parse value into tuple of (usize, usize): {}",
            value
        ))
    }
}

// TODO:
// - tiled mode
// - reflections and rotations of tiles
// - weights in directions
// - global weights of tiles
// - shannon entropy of cells
// - diagonal directions?

fn main() {
    let args: Args = Args::parse();

    let input = image::Image::from_png(&args.input);
    let model = Model::new(args.tile_dimensions, args.with_tile_variations, &input);
    let id_to_tile: HashMap<Id, Tile> = model.id_to_tile.clone();
    let adjacency_rules = AdjacencyRules::from_model(&model);

    let mut wave_function = WaveFunction::new(
        args.output_dimensions,
        adjacency_rules,
        model,
        args.snapshots,
        id_to_tile.clone(),
    );

    let state = wave_function.run();

    let (output_width, output_height) = args.output_dimensions;
    // let (tile_width, tile_height) = args.tile_dimensions;
    let mut data = vec![vec![0; output_width as usize * 3]; output_height as usize];

    for y in 0..output_height {
        for x in 0..output_width {
            let pixel = Vector2 {
                x: x as i32,
                y: y as i32,
            };

            let tile_id = state[&pixel];
            let tile = &id_to_tile[&tile_id];
            let color = &tile.pixels[0][0];

            data[y][x * 3] = color.0[0];
            data[y][x * 3 + 1] = color.0[1];
            data[y][x * 3 + 2] = color.0[2];
        }
    }

    image::output_image("final", data);
}
