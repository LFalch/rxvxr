use std::cmp::{Ord, PartialOrd, Ordering};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Plank {
    pub x: u8,
    pub y: u8,
    pub pointing_down: bool,
    pub length: u8,
}

impl Ord for Plank {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.y.cmp(&other.y)
            .then_with(|| self.x.cmp(&other.x))
            .then_with(|| self.pointing_down.cmp(&other.pointing_down))
            .then_with(|| self.length.cmp(&other.length))
    }
}

impl PartialOrd for Plank {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}