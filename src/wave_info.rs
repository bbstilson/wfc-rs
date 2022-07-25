use std::collections::{HashMap, HashSet};

use crate::{color::Color, grid::Grid, id::Id, pixel::Pixel};

#[derive(Clone, Debug)]
pub struct WaveInfo {
    pub id_to_color: HashMap<Id, Color>,
    pub color_to_id: HashMap<Color, Id>,
    pub pixel_to_id: HashMap<Pixel, Id>,
    pub id_frequency: HashMap<Id, f64>,
}

impl WaveInfo {
    pub fn init(grid: &Grid) -> WaveInfo {
        let id_to_color = mk_id_to_color(grid);
        let color_to_id: HashMap<Color, Id> = id_to_color
            .iter()
            .map(|(idx, color)| (color.clone(), *idx))
            .collect();

        let pixel_to_id = grid
            .pixels()
            .iter()
            .fold(HashMap::new(), |mut acc, &pixel| {
                let idx = grid
                    .get(&pixel)
                    .map(|color| color_to_id.get(color))
                    .flatten()
                    .unwrap();

                acc.insert(pixel, *idx);
                acc
            });

        let total_ids = *(&id_to_color.keys().len()) as f64;
        let id_frequency: HashMap<Id, f64> = color_to_id
            .iter()
            .fold(HashMap::new(), |mut freqs: HashMap<Id, i32>, (_, id)| {
                let next_frequency = freqs.get(id).map(|f| f + 1).unwrap_or(1);
                freqs.insert(*id, next_frequency);
                freqs
            })
            .iter()
            .map(|(id, freq)| (*id, *freq as f64 / total_ids))
            .collect();

        WaveInfo {
            id_to_color,
            color_to_id,
            pixel_to_id,
            id_frequency,
        }
    }
}

fn mk_id_to_color(grid: &Grid) -> HashMap<Id, Color> {
    let mut color_set: HashSet<Color> = HashSet::new();
    let mut id_to_color: HashMap<Id, Color> = HashMap::new();
    let mut color_idx = 0;
    grid.for_each(|(_, color)| {
        if !color_set.contains(color) {
            color_set.insert(color.clone());
            id_to_color.insert(Id(color_idx), color.clone());
            color_idx += 1;
        }
    });
    id_to_color
}
