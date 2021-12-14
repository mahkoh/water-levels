use crate::{CapacitySide, Sink, SinkRef};
use std::cmp::Ordering;

impl Sink {
    pub fn width(&self) -> f64 {
        self.width_times_2 as f64 / 2.0
    }

    pub fn capacity_width(&self) -> f64 {
        self.capacity_width_times_2 as f64 / 2.0
    }
}

impl Default for CapacitySide {
    fn default() -> Self {
        CapacitySide::Both
    }
}

impl Eq for SinkRef {}

impl PartialEq<Self> for SinkRef {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.ver == other.ver
    }
}

impl PartialOrd<Self> for SinkRef {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SinkRef {
    fn cmp(&self, other: &Self) -> Ordering {
        self.overflow_unit
            .partial_cmp(&other.overflow_unit)
            .unwrap()
            .then_with(|| self.pos.cmp(&other.pos))
            .then_with(|| self.ver.cmp(&other.ver))
            .reverse()
    }
}
