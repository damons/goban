use crate::pieces::util::coord::Coord;
use std::collections::HashSet;
use crate::pieces::stones::Color;

type Point = Coord;
type Set = HashSet<Point>;


#[derive(Clone, Debug, CopyGetters, PartialEq, Getters, Eq)]
pub struct GoString {
    #[get_copy = "pub"]
    color: Color,
    #[get = "pub"]
    stones: Set,
    #[get = "pub"]
    liberties: Set,
}

impl GoString {
    pub fn new(color: Color, stones: Set, liberties: Set) -> GoString {
        GoString {
            color,
            stones,
            liberties,
        }
    }

    #[inline]
    pub fn is_dead(&self) -> bool {
        self.liberties.len() == 0
    }

    #[inline]
    pub fn number_of_liberties(&self) -> usize {
        self.liberties.len()
    }

    #[inline]
    pub fn contains_stone(&self, point: Point) -> bool {
        self.stones.contains(&point)
    }

    #[inline]
    pub fn remove_liberty(&mut self, point: Point) {
        debug_assert!(self.liberties.contains(&point));
        self.liberties.remove(&point);
    }

    #[inline]
    pub fn add_liberty(&mut self, point: Point) {
        debug_assert!(!self.liberties.contains(&point));
        self.liberties.insert(point);
    }

    ///
    /// Takes ownership of self and the other string the merge into one string
    ///
    #[inline]
    pub fn merge_with(mut self, GoString { color, mut stones, mut liberties }: GoString) -> Self {
        debug_assert!(color == self.color);
        self.stones.extend(stones.drain());
        self.liberties.extend(liberties.drain());
        self.liberties = self.liberties.difference(&self.stones)
            .map(|x| *x)
            .collect();

        self
    }
}

