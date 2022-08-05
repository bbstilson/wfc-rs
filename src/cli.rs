use std::str::FromStr;
use std::{self, path::PathBuf};

use anyhow::{anyhow, Ok, Result};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
/// Run wfc-rs
pub struct Args {
    /// Input location. If running in 'tiled set' mode, this is assumed to be a directory.
    pub input: PathBuf,

    /// Output dimensions. If running in 'tiled' mode, then this is the number of tiles.
    /// If in 'overlap' mode, then it is in pixels.
    #[clap(short, long, value_parser = parse_tuple_arg)]
    pub output_dimensions: (u32, u32),

    #[clap(subcommand)]
    pub mode: Mode,

    /// whether or not to make a gif (warning: very slow)
    #[clap(long)]
    pub make_gif: bool,

    /// whether or not create all variations (rotations and reflections) of tiles
    #[clap(short, long)]
    pub with_tile_variations: bool,
}

#[derive(Subcommand)]
pub enum Mode {
    Overlap {
        /// tile dimensions to parse from input image
        #[clap(short, long, value_parser = parse_tuple_arg)]
        tile_dimensions: (u32, u32),
    },
    Tile {
        /// tile dimensions to parse from input image
        #[clap(short, value_parser = parse_tuple_arg)]
        tile_dimensions: (u32, u32),
    },
}

fn parse_tuple_arg(value: &str) -> Result<(u32, u32)> {
    let parts = value
        .split(',')
        .map(|part| part.trim())
        .collect::<Vec<&str>>();

    if parts.len() == 2 {
        if let (std::result::Result::Ok(w), std::result::Result::Ok(h)) =
            (u32::from_str(parts[0]), u32::from_str(parts[1]))
        {
            Ok((w, h))
        } else {
            Err(anyhow!(
                "Could not parse value into tuple of (u32, u32): {}",
                value
            ))
        }
    } else {
        Err(anyhow!(
            "Could not parse value into tuple of (u32, u32): {}",
            value
        ))
    }
}
