use std::{collections::HashSet, hash::Hash};

#[derive(Debug)]
pub struct UniqueStack<T: Eq + Hash + Copy> {
    stack: Vec<T>,
    elems: HashSet<T>,
}

impl<T: Eq + Hash + Copy> UniqueStack<T> {
    pub fn new() -> UniqueStack<T> {
        UniqueStack {
            stack: vec![],
            elems: HashSet::new(),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop().map(|x| {
            self.elems.remove(&x);
            x
        })
    }

    pub fn push(&mut self, x: T) {
        if self.elems.insert(x) {
            self.stack.push(x);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.elems.is_empty()
    }
}

impl<T: Eq + Hash + Copy, const N: usize> From<[T; N]> for UniqueStack<T> {
    fn from(xs: [T; N]) -> UniqueStack<T> {
        let mut stack = UniqueStack::new();
        for x in xs {
            stack.push(x);
        }
        stack
    }
}
