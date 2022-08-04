use crate::data::id::Id;

pub struct CellState {
    pub choices: Vec<Id>,
    pub state: Option<Id>,
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
