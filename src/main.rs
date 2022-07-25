use std::collections::{HashMap, HashSet};

use adjacency_builder::AdjacencyRulesBuilder;
use color::Color;
use grid::Grid;
use id::Id;
use types::{ColorToId, IdToColor, PixelToId};

mod adjacency_builder;
mod color;
mod color_type;
mod direction;
mod grid;
mod helpers;
mod id;
mod image;
mod pixel;
mod types;

fn mk_color_map(grid: &Grid) -> IdToColor {
    let mut color_set: HashSet<Color> = HashSet::new();
    let mut color_map: IdToColor = HashMap::new();
    let mut color_idx = 0;
    grid.for_each(|(_, color)| {
        if !color_set.contains(color) {
            color_set.insert(color.clone());
            color_map.insert(Id(color_idx), color.clone());
            color_idx += 1;
        }
    });
    color_map
}

fn main() {
    let input = image::Image::from_png("simple_input.png");

    let id_to_color = mk_color_map(&input.grid);
    let color_to_id: ColorToId = id_to_color
        .iter()
        .map(|(idx, color)| (color.clone(), *idx))
        .collect();

    let pixel_to_id: PixelToId =
        input
            .grid
            .pixels()
            .iter()
            .fold(HashMap::new(), |mut acc, &pixel| {
                let idx = input
                    .grid
                    .get(pixel)
                    .map(|color| color_to_id.get(color))
                    .flatten()
                    .unwrap();

                acc.insert(pixel, *idx);
                acc
            });

    let adjacency_rules = AdjacencyRulesBuilder {
        image_height: input.height,
        image_width: input.width,
        pixel_to_id: pixel_to_id.clone(),
    }
    .build();

    image::output_image(input.width, input.height, &input.get_bytes());
}
