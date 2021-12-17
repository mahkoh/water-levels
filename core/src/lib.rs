use anyhow::Result;
use std::collections::BinaryHeap;
use std::ops::Range;

mod segment;
mod sink;
mod terrain;
#[cfg(test)]
mod tests;

pub fn elevation_levels(levels: &[f64], hours: f64) -> Result<Vec<f64>> {
    Terrain::new(levels)?.fill(hours)
}

#[derive(Debug, Clone)]
struct Segment {
    idx: usize,
    width_times_2: u32,
    elevation: f64,
    next: Option<usize>,
    prev: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct Terrain {
    segments: Vec<Segment>,
    sinks: Vec<Sink>,
    heap: BinaryHeap<SinkRef>,
}

#[derive(Clone, Default, Debug)]
struct Sink {
    pos: usize,
    ver: Option<u32>,
    prev: Option<usize>,
    next: Option<usize>,
    left_elevation: f64,
    right_elevation: f64,
    min_boundary: f64,
    width_times_2: u32,
    capacity: f64,
    fill: f64,
    fill_units: f64,
    overflow_unit: f64,
    capacity_side: CapacitySide,
    capacity_width_times_2: u32,
    segments: Range<usize>,
    capacity_segments: Range<usize>,
}

#[derive(Copy, Clone, Debug)]
struct SinkRef {
    pos: usize,
    ver: u32,
    overflow_unit: f64,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum CapacitySide {
    Left,
    Right,
    Both,
}
