use anyhow::Result;
use clap::Parser;

use adjacency_rules::AdjacencyRules;
use model::Model;
use wave_function::WaveFunction;

mod adjacency_rules;
mod cli;
mod data;
mod gif_builder;
mod helpers;
mod image;
mod model;
mod unique_stack;
mod wave_function;

fn main() -> Result<()> {
    let args: cli::Args = cli::Args::parse();

    let input = image::Image::from_png(&args.input);
    let model = Model::new(args.tile_dimensions, args.with_tile_variations, &input);
    let adjacency_rules = AdjacencyRules::from_model(&model);

    println!("Unique tiles found: {}", model.id_to_tile.keys().len());
    println!(
        "Grid area to solve: {}",
        args.output_dimensions.0 * args.output_dimensions.1
    );

    let mut wave_function = WaveFunction::new(
        args.output_dimensions,
        adjacency_rules,
        model,
        args.make_gif,
    );

    wave_function.run()
}
