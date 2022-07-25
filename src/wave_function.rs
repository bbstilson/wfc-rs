use std::collections::HashMap;

use rand::{self, distributions::WeightedIndex, prelude::Distribution};

use crate::{
    adjacency_rules::AdjacencyRules, color::Color, direction::Direction, helpers, id::Id, image,
    pixel::Pixel, wave_info::WaveInfo,
};

#[derive(Debug)]
struct CellState {
    choices: Vec<Id>,
    state: Option<Id>,
}

impl CellState {
    pub fn is_collapsed(&self) -> bool {
        self.state.is_some()
    }

    pub fn get_choices(&self) -> Vec<Id> {
        self.state
            .map(|id| vec![id])
            .unwrap_or(self.choices.clone())
    }

    pub fn remove_choice(&mut self, choice: &Id) {
        if let Some(idx) = self.choices.iter().position(|id| id == choice) {
            self.choices.remove(idx);
        }
    }
}

type State = HashMap<Pixel, CellState>;
type CollapsedState = HashMap<Pixel, Id>;
pub struct WaveFunction<'a> {
    state: State,
    info: &'a WaveInfo,
    adjacency_rules: &'a AdjacencyRules,
    width: i32,
    height: i32,
    cells_to_collapse: i32,
}

impl<'b> WaveFunction<'b> {
    pub fn init<'a>(
        width: i32,
        height: i32,
        adjacency_rules: &'a AdjacencyRules,
        info: &'a WaveInfo,
    ) -> WaveFunction<'a> {
        let mut init = HashMap::new();
        let choices = info.id_to_color.keys().map(|id| *id).collect::<Vec<Id>>();
        for y in 0..height {
            for x in 0..width {
                init.insert(
                    Pixel { x, y },
                    CellState {
                        choices: choices.clone(),
                        state: None,
                    },
                );
            }
        }
        WaveFunction {
            state: init,
            info,
            adjacency_rules,
            width,
            height,
            cells_to_collapse: width * height,
        }
    }

    fn is_collapsed(&self) -> bool {
        self.cells_to_collapse == 0
    }

    pub fn run(&mut self) -> CollapsedState {
        self.iterate(0)
    }

    fn iterate(&mut self, depth: i32) -> CollapsedState {
        self.snapshot(depth);

        let to_collapse = self.get_lowest_entropy_pixel();
        self.collapse(to_collapse);
        self.propagate_from(to_collapse);

        if self.is_collapsed() {
            self.state
                .iter()
                .fold(HashMap::new(), |mut acc, (pixel, cell_state)| {
                    acc.insert(*pixel, cell_state.state.unwrap());
                    acc
                })
        } else {
            self.iterate(depth + 1)
        }
    }

    fn collapse(&mut self, to_collapse: Pixel) {
        let choices = &self.state.get(&to_collapse).unwrap().choices;
        let choice = self.get_random_choice(choices);

        self.cells_to_collapse -= 1;
        self.state.insert(
            to_collapse,
            CellState {
                choices: vec![],
                state: Some(choice),
            },
        );
    }

    fn propagate_from(&mut self, collapsed: Pixel) {
        let mut stack: Vec<Pixel> = vec![collapsed];
        while !stack.is_empty() {
            if let Some(cell) = stack.pop() {
                if let Some(cell_state) = self.state.get(&cell) {
                    let cur_possible_tiles = cell_state.get_choices();

                    let neighbors = helpers::get_neighbors(self.width, self.height, &cell);

                    for (neighbor, direction) in neighbors {
                        self.state
                            .get(&neighbor)
                            .map(|cs| cs.get_choices())
                            .unwrap_or(vec![])
                            .iter()
                            .for_each(|neighbor_id| {
                                let other_tile_possible = cur_possible_tiles
                                    .iter()
                                    .any(|id| self.valid_pair(id, &neighbor_id, &direction));

                                if !other_tile_possible {
                                    if let Some(cell_state) = self.state.get_mut(&neighbor) {
                                        cell_state.remove_choice(neighbor_id);
                                    };
                                    stack.push(neighbor);
                                }
                            });
                    }
                }
            }
        }
    }

    fn valid_pair(&self, id: &Id, neighbor: &Id, direction: &Direction) -> bool {
        self.adjacency_rules
            .get(id)
            .and_then(|rules| rules.get(neighbor))
            .and_then(|dir_to_freq| dir_to_freq.get(direction))
            .is_some()
    }

    fn get_random_choice(&self, choices: &Vec<Id>) -> Id {
        let mut rng = rand::thread_rng();
        let weights = choices
            .iter()
            .flat_map(|id| self.info.id_frequency.get(id))
            .collect::<Vec<&f64>>();
        let dist = WeightedIndex::new(weights).unwrap();
        choices[dist.sample(&mut rng)]
    }

    fn get_lowest_entropy_pixel(&self) -> Pixel {
        let mut choices = self
            .state
            .iter()
            .filter(|(_, cell_state)| !cell_state.is_collapsed())
            .map(|(pixel, cell_state)| (*pixel, cell_state.choices.len()))
            .collect::<Vec<(Pixel, usize)>>();

        choices.sort_by(|(_, choices_a), (_, choices_b)| choices_a.cmp(choices_b));

        let (pixel, _) = choices.first().unwrap();
        *pixel
    }

    fn snapshot(&self, depth: i32) {
        let mut output_bytes = Vec::new();

        for h in 0..self.height {
            for w in 0..self.width {
                let pixel = Pixel { x: w, y: h };
                let cell_state = self.state.get(&pixel).unwrap();
                let color = cell_state
                    .state
                    .map(|id| self.info.id_to_color.get(&id))
                    .flatten()
                    .map(|c| Color(c.0.clone()))
                    .unwrap_or_else(|| {
                        let bytes = cell_state
                            .choices
                            .iter()
                            .map(|id| self.info.id_to_color.get(id))
                            .flatten()
                            .map(|c| Color(c.0.clone()))
                            .reduce(|acc, color| acc.blend(color))
                            .unwrap()
                            .0
                            .clone();

                        Color(bytes)
                    });

                output_bytes.append(&mut color.0.clone())
            }
        }

        image::output_image(
            self.width,
            self.height,
            &depth.to_string(),
            &output_bytes.as_slice(),
        );
    }
}
