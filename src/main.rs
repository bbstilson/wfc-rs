use std;
use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Ok, Result};
use clap::{Parser, ValueEnum};

use adjacency_rules::AdjacencyRules;
use data::{coord_2d::Vector2, id::Id, tile::Tile};
use image::Image;
use model::Model;
use wave_function::WaveFunction;

mod adjacency_rules;
mod data;
mod gif_builder;
mod helpers;
mod image;
mod model;
mod unique_stack;
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

    /// whether or not to make a gif (warning: very slow)
    #[clap(short, long)]
    make_gif: bool,

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

fn parse_tuple_arg(value: &str) -> Result<(usize, usize)> {
    let parts = value
        .split(',')
        .map(|part| part.trim())
        .collect::<Vec<&str>>();

    if parts.len() == 2 {
        if let (std::result::Result::Ok(w), std::result::Result::Ok(h)) =
            (usize::from_str(parts[0]), usize::from_str(parts[1]))
        {
            Ok((w, h))
        } else {
            Err(anyhow!(
                "Could not parse value into tuple of (usize, usize): {}",
                value
            ))
        }
    } else {
        Err(anyhow!(
            "Could not parse value into tuple of (usize, usize): {}",
            value
        ))
    }
}

fn main() -> Result<()> {
    let args: Args = Args::parse();

    let input = image::Image::from_png(&args.input);
    let model = Model::new(args.tile_dimensions, args.with_tile_variations, &input);
    let id_to_tile: HashMap<Id, Tile> = model.id_to_tile.clone();
    let adjacency_rules = AdjacencyRules::from_model(&model);

    println!("Unique tiles found: {}", id_to_tile.keys().len());
    println!(
        "Grid area to solve: {}",
        args.output_dimensions.0 * args.output_dimensions.1
    );

    let mut wave_function = WaveFunction::new(
        args.output_dimensions,
        adjacency_rules,
        model,
        args.make_gif,
        id_to_tile.clone(),
    );

    let state = wave_function.run()?;

    let (width, height) = args.output_dimensions;
    let mut final_image = Image::new(width as u32, height as u32);

    for y in 0..height {
        for x in 0..width {
            let pixel = Vector2 {
                x: x as i32,
                y: y as i32,
            };

            let tile_id = state[&pixel];
            let tile = &id_to_tile[&tile_id];
            let color = tile.pixels[0][0];
            final_image.set_color(pixel, color);
        }
    }

    final_image.save("output")?;

    println!("Done!");

    Ok(())
}
