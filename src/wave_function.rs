use std::collections::HashMap;

use rand::{self, distributions::WeightedIndex, prelude::Distribution};

use crate::{
    adjacency_rules::AdjacencyRules, direction::Direction, helpers, id::Id, pixel::Pixel,
    wave_info::WaveInfo,
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

    pub fn collapse(&mut self) -> CollapsedState {
        println!();
        println!();
        println!("~~~ COLLPASE ~~~");

        let to_collapse = self.get_lowest_entropy_pixel();
        let choices = &self.state.get(&to_collapse).unwrap().choices;

        self.state.iter().for_each(|(pixel, state)| {
            println!("{:?} -> {:?}", pixel, state);
        });

        // choose based on global likelihood
        let mut rng = rand::thread_rng();
        let weights = choices
            .iter()
            .flat_map(|id| self.info.id_frequency.get(id))
            .collect::<Vec<&f64>>();
        println!("choices -> {:?}", choices);
        println!("weights -> {:?}", weights);
        let dist = WeightedIndex::new(weights).unwrap();
        let choice = choices[dist.sample(&mut rng)];

        // collapse state
        self.cells_to_collapse -= 1;
        self.state.insert(
            to_collapse,
            CellState {
                choices: vec![],
                state: Some(choice),
            },
        );

        // make changes to the rest of the wave
        self.propagate_from(&to_collapse);

        if self.is_collapsed() {
            self.state
                .iter()
                .fold(HashMap::new(), |mut acc, (pixel, cell_state)| {
                    acc.insert(*pixel, cell_state.state.unwrap());
                    acc
                })
        } else {
            self.collapse()
        }
    }

    // propagate the wave
    // 1. get neighbors of collapsed cell
    // 2. for each neighbor, if any need updates, return it
    // 3. for each neighbor that needs updates, update it.
    // 4. for each neighbor updated, go to step 1
    fn propagate_from(&mut self, collapsed: &Pixel) {
        println!("collapsed -> {:?}", collapsed);
        // 1. get neighbors of collapsed cell
        let neighbors = helpers::get_neighbors(self.width, self.height, collapsed);
        println!("neighbors -> {:?}", neighbors);

        // this is technically unsafe, but the state should be set before we enter
        // this function
        // if the cell is totally collapsed, then we have just one id
        // otherwise, we have all the choices that the cell _could_ be
        let collapsed_cell_state = self.state.get(collapsed).unwrap();
        println!("collapsed_cell_state -> {:?}", collapsed_cell_state);
        let collapsed_ids = if let Some(id) = collapsed_cell_state.state {
            vec![id]
        } else {
            collapsed_cell_state.choices.clone()
        };
        println!("collapsed_ids -> {:?}", collapsed_ids);

        // A neighbor should be updated if it has a choice can can't be in the opposite
        // direction of the collapsed cell. For example, if a SEA tile happens at (0,0),
        // then (1, 0) (to the right) can't be a LAND tile because LAND can't directly
        // touch SEA.
        // For each choice that a neighbor has, check if the collpased cell in the
        // flipped direction, is allowed.
        let neighbors_to_update = neighbors
            .iter()
            .filter_map(|(neighbor, direction)| {
                let maybe_state_update = self
                    .state
                    .get(neighbor)
                    .filter(|cell_state| !cell_state.is_collapsed())
                    .map(|cell_state| {
                        let valid_choices =
                            self.get_valid_choices(&collapsed_ids, cell_state, direction);

                        if valid_choices.len() != cell_state.choices.len() {
                            Some((neighbor, valid_choices))
                        } else {
                            None
                        }
                    })
                    .flatten();

                println!("maybe_state_update -> {:?}", maybe_state_update);

                maybe_state_update.iter().for_each(|(pixel, choices)| {
                    self.state.insert(
                        **pixel,
                        CellState {
                            choices: choices.clone(),
                            state: None,
                        },
                    );
                });

                maybe_state_update.map(|(pixel, _)| pixel)
            })
            .collect::<Vec<&Pixel>>();

        // propagate from each neighbor that was updated
        for neighbor in neighbors_to_update {
            self.propagate_from(neighbor);
        }
    }

    fn get_valid_choices(
        &self,
        collapsed_ids: &Vec<Id>,
        cell_state: &CellState,
        direction: &Direction,
    ) -> Vec<Id> {
        cell_state
            .choices
            .iter()
            .filter(|choice| {
                let collapsed_direction = direction.flip();
                self.adjacency_rules
                    .get(choice)
                    .map(|rules| {
                        collapsed_ids
                            .iter()
                            .flat_map(|collapsed_id| rules.get(collapsed_id))
                            .collect::<Vec<&HashMap<Direction, i32>>>()
                    })
                    .unwrap_or(vec![])
                    .iter()
                    .map(|direction_frequencies| {
                        direction_frequencies.contains_key(&collapsed_direction)
                    })
                    .any(|b| b)
            })
            .map(|id| *id)
            .collect()
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
}
