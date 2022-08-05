use anyhow::Result;
use rand::{self, distributions::WeightedIndex, prelude::Distribution};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::data::color::Color;
use crate::data::{cell_state::CellState, coord_2d::Vector2, id::Id};
use crate::gif_builder::GifBuilder;
use crate::unique_stack::UniqueStack;
use crate::{adjacency_rules::AdjacencyRules, helpers, image::Image, model::Model};

// Rendering a snapshot everytime a cell collapses is visually uninteresting (no one
// likes to watch a 40 second gif). To save space and time, we only cache a state if
// `SNAPSHOT_COUNTER % GIF_SIZE_FACTOR == 0`. See: `WaveFunction::should_take_snapshot`.
static SNAPSHOT_COUNTER: AtomicUsize = AtomicUsize::new(0);
const GIF_SIZE_FACTOR: usize = 10;

pub struct WaveFunction {
    model: Model,
    adjacency_rules: AdjacencyRules,
    // wave data
    state: HashMap<Vector2, CellState>,
    dimensions: (usize, usize),
    cells_to_collapse: usize,
    // gif related fields
    make_gif: bool,
    snapshots: Vec<Image>,
}

impl WaveFunction {
    pub fn new(
        dimensions: (usize, usize),
        adjacency_rules: AdjacencyRules,
        model: Model,
        make_gif: bool,
    ) -> WaveFunction {
        let (width, height) = dimensions;
        let mut state = HashMap::new();
        let choices = model.id_to_tile.keys().map(|id| *id).collect::<Vec<Id>>();
        for y in 0..height {
            for x in 0..width {
                state.insert(
                    Vector2 {
                        x: x as i32,
                        y: y as i32,
                    },
                    CellState {
                        choices: choices.clone(),
                        state: None,
                    },
                );
            }
        }
        WaveFunction {
            model,
            adjacency_rules,
            dimensions,
            state,
            make_gif,
            snapshots: vec![],
            cells_to_collapse: width * height,
        }
    }

    fn is_collapsed(&self) -> bool {
        self.cells_to_collapse == 0
    }

    fn print_progress(&self) {
        let (width, height) = self.dimensions;
        let area = width * height;
        let ten_percent = area / 10;

        if self.cells_to_collapse % ten_percent == 0 {
            println!(
                "Progress: {}%",
                (100 - (self.cells_to_collapse / ten_percent) * 10)
            )
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self.iterate()
    }

    fn iterate(&mut self) -> Result<()> {
        let mut iterations = 0;

        while !self.is_collapsed() {
            let to_collapse = self.get_lowest_entropy_coord();
            self.collapse(to_collapse)?;
            iterations += self.propagate(to_collapse)?;
        }

        println!("Iterations completed: {}", iterations);

        if self.make_gif {
            // take final snapshot of state, then make the gif
            self.take_snapshot();
            GifBuilder::make_gif(&self.snapshots)
        } else {
            self.state_to_image().save("output")
        }
    }

    fn collapse(&mut self, to_collapse: Vector2) -> Result<()> {
        let choices = &self.state.get(&to_collapse).unwrap().get_choices();
        let choice = self.get_random_choice(choices)?;

        self.print_progress();

        if self.should_take_snapshot() {
            self.take_snapshot();
        }

        self.cells_to_collapse -= 1;
        self.state.insert(
            to_collapse,
            CellState {
                choices: vec![],
                state: Some(choice),
            },
        );

        Ok(())
    }

    fn should_take_snapshot(&self) -> bool {
        self.make_gif && SNAPSHOT_COUNTER.fetch_add(1, Ordering::SeqCst) % GIF_SIZE_FACTOR == 0
    }

    fn propagate(&mut self, collapsed: Vector2) -> Result<usize> {
        let mut iterations = 0;
        let mut stack = UniqueStack::from([collapsed]);

        while !stack.is_empty() {
            iterations += 1;
            if let Some(coord) = stack.pop() {
                if let Some(cell_state) = self.state.get(&coord) {
                    let choices = cell_state.get_choices();

                    // For each neighbor, check if the cell we just collapsed affects
                    // the choices in that neighbor.
                    // Specifically, if the neighbor has any choices that still might
                    // work, then it's still fine.
                    // Otherwise, remove that choice, then add the neighbor to the stack.
                    let neighbors = helpers::get_neighbors(self.dimensions, &coord);
                    for (neighbor, direction) in &neighbors {
                        let maybe_neighbor_state =
                            self.state.get_mut(neighbor).filter(|cs| !cs.is_collapsed());
                        if let Some(neighbor_state) = maybe_neighbor_state {
                            let neighbor_choices = neighbor_state.get_choices();

                            let mut add_neighbor = false;
                            for neighbor_choice in &neighbor_choices {
                                let is_valid = choices.iter().any(|choice| {
                                    self.adjacency_rules.valid_neighbors(
                                        *choice,
                                        *neighbor_choice,
                                        *direction,
                                    )
                                });

                                if !is_valid {
                                    neighbor_state.remove_choice(neighbor_choice);
                                    add_neighbor = true;
                                }
                            }
                            if add_neighbor {
                                if neighbor_state.get_choices().len() == 1 {
                                    self.collapse(neighbor.clone())?;
                                }
                                stack.push(*neighbor);
                            }
                        }
                    }
                }
            }
        }
        Ok(iterations)
    }

    fn get_random_choice(&self, choices: &Vec<Id>) -> Result<Id> {
        let mut rng = rand::thread_rng();
        let weights = choices
            .iter()
            .flat_map(|id| self.model.frequency_hints.get(id))
            .collect::<Vec<&f64>>();
        let dist = WeightedIndex::new(weights)?;
        Ok(choices[dist.sample(&mut rng)])
    }

    fn get_lowest_entropy_coord(&self) -> Vector2 {
        let mut choices = self
            .state
            .iter()
            .filter(|(_, cell_state)| !cell_state.is_collapsed())
            .map(|(coord, cell_state)| (*coord, cell_state.get_choices().len()))
            .collect::<Vec<(Vector2, usize)>>();

        choices.sort_by(|(_, choices_a), (_, choices_b)| choices_a.cmp(choices_b));

        let (coord, _) = choices.first().unwrap();
        *coord
    }

    fn state_to_image(&self) -> Image {
        let (width, height) = self.dimensions;
        let mut img = Image::new(width as u32, height as u32);

        for y in 0..height {
            for x in 0..width {
                let pixel = Vector2 {
                    x: x as i32,
                    y: y as i32,
                };
                let state = &self.state[&pixel];
                let color = state
                    .get_choices()
                    .iter()
                    .map(|id| &self.model.id_to_tile[id])
                    .map(|t| t.pixels[0][0].clone()) // take the top left pixel from the tile
                    .reduce(|l, r| l.blend(&r)) // blend all the pixels together
                    .unwrap();

                img.set_color(pixel, color);
            }
        }

        img
    }

    fn take_snapshot(&mut self) {
        self.snapshots.push(self.state_to_image());
    }
}
