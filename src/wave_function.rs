use std::collections::HashMap;

use rand::{self, distributions::WeightedIndex, prelude::Distribution};

use crate::data::{cell_state::CellState, coord_2d::Vector2, id::Id, tile::Tile};
use crate::unique_stack::UniqueStack;
use crate::{adjacency_rules::AdjacencyRules, helpers, image, model::Model};

type State = HashMap<Vector2, CellState>;
type CollapsedState = HashMap<Vector2, Id>;

pub struct WaveFunction {
    state: State,
    model: Model,
    adjacency_rules: AdjacencyRules,
    grid_dimensions: (usize, usize),
    cells_to_collapse: usize,
    take_snapshots: bool,
    id_to_tile: HashMap<Id, Tile>,
}

impl WaveFunction {
    pub fn new(
        output_dimensions: (usize, usize),
        adjacency_rules: AdjacencyRules,
        model: Model,
        take_snapshots: bool,
        id_to_tile: HashMap<Id, Tile>,
    ) -> WaveFunction {
        let (output_w, output_h) = output_dimensions;
        let mut init = HashMap::new();
        let choices = model.id_to_tile.keys().map(|id| *id).collect::<Vec<Id>>();
        for y in 0..output_h {
            for x in 0..output_w {
                init.insert(
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
            grid_dimensions: output_dimensions,
            take_snapshots,
            id_to_tile,
            state: init,
            cells_to_collapse: output_w * output_h,
        }
    }

    fn is_collapsed(&self) -> bool {
        self.cells_to_collapse == 0
    }

    fn print_progress(&self) {
        let (width, height) = self.grid_dimensions;
        let area = width * height;
        let ten_percent = area / 10;

        if self.cells_to_collapse % ten_percent == 0 {
            println!(
                "{}% done",
                (100 - (self.cells_to_collapse / ten_percent) * 10)
            )
        }
    }

    pub fn run(&mut self) -> CollapsedState {
        self.iterate()
    }

    fn iterate(&mut self) -> CollapsedState {
        let mut iterations = 0;
        while !self.is_collapsed() {
            self.print_progress();

            if self.take_snapshots {
                self.take_snapshot(iterations);
            }

            let to_collapse = self.get_lowest_entropy_coord();
            self.collapse(to_collapse);
            iterations += self.propagate(to_collapse);
        }

        println!("did {} iterations", iterations);

        self.state
            .iter()
            .fold(HashMap::new(), |mut acc, (coord, cell_state)| {
                acc.insert(*coord, cell_state.state.unwrap());
                acc
            })
    }

    fn collapse(&mut self, to_collapse: Vector2) {
        let choices = &self.state.get(&to_collapse).unwrap().get_choices();
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

    fn propagate(&mut self, collapsed: Vector2) -> usize {
        let mut iterations = 0;
        let mut stack = UniqueStack::from([collapsed]);

        while !stack.is_empty() {
            println!("{:?}", stack);
            iterations += 1;
            if let Some(coord) = stack.pop() {
                if let Some(cell_state) = self.state.get(&coord) {
                    let choices = cell_state.get_choices();

                    // For each neighbor, check if the cell we just collapsed affects
                    // the choices in that neighbor.
                    // Specifically, if the neighbor has any choices that still might
                    // work, then it's still fine.
                    // If so, remove that choice, then add the neighbor to the stack.
                    helpers::get_neighbors(self.grid_dimensions, &coord)
                        .iter()
                        .for_each(|(neighbor, direction)| {
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
                                            direction,
                                        )
                                    });

                                    if !is_valid {
                                        neighbor_state.remove_choice(neighbor_choice);
                                        add_neighbor = true;
                                    }
                                }
                                if add_neighbor {
                                    stack.push(*neighbor);
                                }
                            }
                        });
                }
            }
        }
        iterations
    }

    fn get_random_choice(&self, choices: &Vec<Id>) -> Id {
        let mut rng = rand::thread_rng();
        let weights = choices
            .iter()
            .flat_map(|id| self.model.frequency_hints.get(id))
            .collect::<Vec<&f64>>();
        let dist = WeightedIndex::new(weights).unwrap();
        choices[dist.sample(&mut rng)]
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

    fn take_snapshot(&self, depth: usize) {
        let (width, height) = self.grid_dimensions;
        let mut data = vec![vec![128; width * 3]; height];

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
                    .map(|id| &self.id_to_tile[id])
                    .map(|t| t.pixels[0][0].clone())
                    .reduce(|l, r| l.blend(&r))
                    .unwrap();

                data[y][x * 3] = color.0[0];
                data[y][x * 3 + 1] = color.0[1];
                data[y][x * 3 + 2] = color.0[2];
            }
        }

        image::output_image(&depth.to_string(), data);
    }
}
