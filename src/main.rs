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

    let (model, adjacency_rules) = match args.mode {
        cli::Mode::Overlap { tile_dimensions } => {
            let model = Model::overlap(args.input, tile_dimensions, args.with_tile_variations)?;
            let adjacency_rules = AdjacencyRules::from_overlap_model(&model);
            (model, adjacency_rules)
        }
        cli::Mode::Tile { tile_dimensions } => {
            let model = Model::tiled(args.input, tile_dimensions, args.with_tile_variations)?;
            let adjacency_rules = AdjacencyRules::from_tile_model(&model);
            (model, adjacency_rules)
        }
    };

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
