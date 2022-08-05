use std;
use std::str::FromStr;

use anyhow::{anyhow, Ok, Result};
use clap::Parser;

use crate::data::parse_method::ParseMethod;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
/// Run wfc-rs
pub struct Args {
    /// input image location
    pub input: String,

    /// output dimensions in pixels
    #[clap(short, long, value_parser = parse_tuple_arg)]
    pub output_dimensions: (usize, usize),

    /// tile dimensions to parse from input image
    #[clap(short, long, value_parser = parse_tuple_arg)]
    pub tile_dimensions: (usize, usize),

    /// whether or not to make a gif (warning: very slow)
    #[clap(short, long)]
    pub make_gif: bool,

    /// whether or not create all variations (rotations and reflections) of tiles
    #[clap(short, long)]
    pub with_tile_variations: bool,

    /// parse input as a tiled map
    #[clap(long, arg_enum, default_value_t = ParseMethod::Overlap)]
    pub parse_method: ParseMethod,
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
