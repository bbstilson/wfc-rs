use std::collections::HashMap;

use rand::{self, distributions::WeightedIndex, prelude::Distribution};

use crate::data::{cell_state::CellState, coord_2d::Coord2d, id::Id};
use crate::{adjacency_rules::AdjacencyRules, helpers, model::Model};

type State = HashMap<Coord2d, CellState>;
type CollapsedState = HashMap<Coord2d, Id>;

pub struct WaveFunction {
    state: State,
    model: Model,
    adjacency_rules: AdjacencyRules,
    grid_width: i32,
    grid_height: i32,
    cells_to_collapse: i32,
}

impl WaveFunction {
    pub fn new(
        grid_width: i32,
        grid_height: i32,
        adjacency_rules: AdjacencyRules,
        model: Model,
    ) -> WaveFunction {
        let mut init = HashMap::new();
        let choices = model.id_to_tile.keys().map(|id| *id).collect::<Vec<Id>>();
        for y in 0..grid_height {
            for x in 0..grid_width {
                init.insert(
                    Coord2d { x, y },
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
            state: init,
            cells_to_collapse: grid_width * grid_height,
        }
    }

    fn is_collapsed(&self) -> bool {
        self.cells_to_collapse == 0
    }

    pub fn run(&mut self) -> CollapsedState {
        self.iterate()
    }

    fn iterate(&mut self) -> CollapsedState {
        let to_collapse = self.get_lowest_entropy_coord();
        self.collapse(to_collapse);
        self.propagate(to_collapse);

        while !self.is_collapsed() {
            // println!("~~~~~~~~~~~");
            self.iterate();
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
                            // println!("~~~~ {:?}", coord);
                            // println!("~~~~ {:?}", neighbor);
                            // println!("~~~~ {:?}", choices);
                            // println!("~~~~ {:?}", neighbor_choices);
                            for neighbor_choice in &neighbor_choices {
                                let is_valid = choices.iter().any(|choice| {
                                    self.adjacency_rules.valid_neighbors(
                                        choice,
                                        &neighbor_choice,
                                        direction,
                                    )
                                });

                                if !is_valid {
                                    // println!(
                                    //     "{:?}\tcannot be a\t{:?}\tneighbor of\t{:?}",
                                    //     neighbor_choice, direction, choices
                                    // );
                                    neighbor_cell_state.remove_choice(neighbor_choice);
                                    add_neighbor = true;
                                }
                            }
                            // println!("~~ {:?}", neighbor_cell_state);
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
}
