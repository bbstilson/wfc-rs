use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
};

pub struct UniqueStack<T: Eq + Hash + Copy> {
    stack: VecDeque<T>,
    elems: HashSet<T>,
}

impl<T: Eq + Hash + Copy> UniqueStack<T> {
    pub fn new() -> UniqueStack<T> {
        UniqueStack {
            stack: VecDeque::new(),
            elems: HashSet::new(),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop_front().map(|x| {
            self.elems.remove(&x);
            x
        })
    }

    pub fn push(&mut self, x: T) {
        if self.elems.insert(x) {
            self.stack.push_back(x);
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

#[cfg(test)]
mod tests {
    use super::UniqueStack;

    #[test]
    fn test_unique_stack_from() {
        let mut us = UniqueStack::from([1, 2, 3, 4]);
        assert_eq!(us.pop(), Some(4));
        assert_eq!(us.pop(), Some(3));
        assert_eq!(us.pop(), Some(2));
        assert_eq!(us.pop(), Some(1));
    }

    #[test]
    fn test_unique_stack_push_pop() {
        let mut us = UniqueStack::new();
        us.push(1);
        us.push(1);
        us.push(1);
        us.push(1);
        assert_eq!(us.pop(), Some(1));
        assert_eq!(us.pop(), None);
    }
}
