use std::collections::HashMap;

use rand::{self, distributions::WeightedIndex, prelude::Distribution};

use crate::data::{cell_state::CellState, coord_2d::Coord2d, id::Id, tile::Tile};
use crate::{adjacency_rules::AdjacencyRules, helpers, image, model::Model};

type State = HashMap<Coord2d, CellState>;
type CollapsedState = HashMap<Coord2d, Id>;

pub struct WaveFunction {
    state: State,
    model: Model,
    adjacency_rules: AdjacencyRules,
    grid_width: usize,
    grid_height: usize,
    cells_to_collapse: usize,
    tile_size: usize,
    take_snapshots: bool,
    id_to_tile: HashMap<Id, Tile>,
}

impl WaveFunction {
    pub fn new(
        grid_width: usize,
        grid_height: usize,
        adjacency_rules: AdjacencyRules,
        model: Model,
        take_snapshots: bool,
        tile_size: usize,
        id_to_tile: HashMap<Id, Tile>,
    ) -> WaveFunction {
        let mut init = HashMap::new();
        let choices = model.id_to_tile.keys().map(|id| *id).collect::<Vec<Id>>();
        for y in 0..grid_height {
            for x in 0..grid_width {
                init.insert(
                    Coord2d {
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
            grid_width,
            grid_height,
            take_snapshots,
            tile_size,
            id_to_tile,
            state: init,
            cells_to_collapse: grid_width * grid_height,
        }
    }

    fn is_collapsed(&self) -> bool {
        self.cells_to_collapse == 0
    }

    pub fn run(&mut self) -> CollapsedState {
        self.iterate(0)
    }

    fn iterate(&mut self, depth: usize) -> CollapsedState {
        if self.take_snapshots {
            self.take_snapshot(depth);
        }

        let to_collapse = self.get_lowest_entropy_coord();
        self.collapse(to_collapse);
        self.propagate(to_collapse);

        while !self.is_collapsed() {
            self.iterate(depth + 1);
        }

        self.state
            .iter()
            .fold(HashMap::new(), |mut acc, (coord, cell_state)| {
                acc.insert(*coord, cell_state.state.unwrap());
                acc
            })
    }

    fn collapse(&mut self, to_collapse: Coord2d) {
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

    fn propagate(&mut self, collapsed: Coord2d) {
        let mut stack: Vec<Coord2d> = vec![collapsed];
        while !stack.is_empty() {
            if let Some(coord) = stack.pop() {
                if let Some(cell_state) = self.state.get(&coord) {
                    let neighbors =
                        helpers::get_neighbors(self.grid_width, self.grid_height, &coord);

                    let choices = cell_state.get_choices();

                    // For each neighbor, check if the cell we just collapsed affects
                    // the choices in that neighbor.
                    // Specifically, if the neighbor has any choices that still might
                    // work, then it's still fine.
                    // If so, remove that choice, then add the neighbor to the stack.
                    neighbors.iter().for_each(|(neighbor, direction)| {
                        let maybe_neighbor_state =
                            self.state.get_mut(neighbor).filter(|cs| !cs.is_collapsed());
                        if let Some(neighbor_cell_state) = maybe_neighbor_state {
                            let neighbor_choices = neighbor_cell_state.get_choices();

                            let mut add_neighbor = false;
                            for neighbor_choice in &neighbor_choices {
                                let is_valid = choices.iter().any(|choice| {
                                    self.adjacency_rules.valid_neighbors(
                                        choice,
                                        &neighbor_choice,
                                        direction,
                                    )
                                });

                                if !is_valid {
                                    neighbor_cell_state.remove_choice(neighbor_choice);
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

    fn get_lowest_entropy_coord(&self) -> Coord2d {
        let mut choices = self
            .state
            .iter()
            .filter(|(_, cell_state)| !cell_state.is_collapsed())
            .map(|(coord, cell_state)| (*coord, cell_state.get_choices().len()))
            .collect::<Vec<(Coord2d, usize)>>();

        choices.sort_by(|(_, choices_a), (_, choices_b)| choices_a.cmp(choices_b));

        let (coord, _) = choices.first().unwrap();
        *coord
    }

    fn take_snapshot(&self, depth: usize) {
        let mut data = vec![
            vec![128; self.grid_width * 3 * self.tile_size];
            self.grid_height * self.tile_size
        ];

        for g_y in 0..self.grid_height {
            for g_x in 0..self.grid_width {
                let pixel = Coord2d {
                    x: g_x as i32,
                    y: g_y as i32,
                };
                let state = &self.state[&pixel];
                let tile = state
                    .get_choices()
                    .iter()
                    .map(|id| &self.id_to_tile[id])
                    .map(|t| t.clone())
                    .reduce(|l, r| l.blend(r))
                    .unwrap();

                for n_y in 0..self.tile_size {
                    for n_x in 0..self.tile_size {
                        let color = &tile.pixels[n_y][n_x];
                        let f_x = (g_x * self.tile_size * 3) + (n_x * 3);
                        let f_y = (g_y * self.tile_size) + n_y;
                        data[f_y][f_x] = color.0[0];
                        data[f_y][f_x + 1] = color.0[1];
                        data[f_y][f_x + 2] = color.0[2];
                    }
                }
            }
        }

        image::output_image(&depth.to_string(), data);
    }
}
