use std::collections::HashMap;

use crate::{color::Color, pixel::Pixel};

pub struct Grid {
    grid: HashMap<Pixel, Color>,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            grid: HashMap::new(),
        }
    }

    pub fn insert(&mut self, k: Pixel, v: Color) {
        self.grid.insert(k, v);
    }

    pub fn get(&self, k: Pixel) -> Option<&Color> {
        self.grid.get(&k)
    }

    pub fn for_each<F>(&self, f: F)
    where
        F: FnMut((&Pixel, &Color)),
    {
        self.grid.iter().for_each(f)
    }

    pub fn pixels(&self) -> Vec<Pixel> {
        self.grid.keys().map(|f| *f).collect()
    }
}
