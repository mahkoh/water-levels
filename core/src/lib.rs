use anyhow::Result;
use std::collections::BinaryHeap;
use std::ops::Range;

mod segment;
mod sink;
mod terrain;
#[cfg(test)]
mod tests;

/// Calculates elevation levels.
///
/// Assuming that `levels` describe the elevation levels at time 0, this function
/// calculates the elevation levels after `hours` hours of rain.
///
/// Each level and the number of hours must be in the range `[0, u32::MAX]` and
/// `levels` must contain at least one but not more than `u32::MAX/2` elements.
/// This is so that we do not have to deal with floating point overflow and so that the
/// imprecisions of our floating point calculations stay in a reasonable range.
pub fn elevation_levels(levels: &[f64], hours: f64) -> Result<Vec<f64>> {
    Terrain::new(levels)?.rain(hours)
}

// Some notes regarding the implementation:
// 
// In the paper, sinks and segments are part of doubly-linked lists and priority queues.
// This would be quite problematic in rust since we would have to rely heavily on interior
// mutability.
//
// Instead, we simulate the doubly-linked lists as follows: After initially constructing
// the sinks, we store the sinks and the segments in two vectors. Each sink and segment
// contains a field that contains its index in the vector. Furthermore, they contain the
// indices of the elements that come immediately before/after them (if any).
//
// Therefore, we can remove elements from the linked lists by manipulating the fields
// containing the indices of the neighboring elements.
//
// For the priority queue, we use a `BinaryHeap`. For each `Sink` in the doubly-linked
// list, we store a `SinkRef` in the heap. This object references the `Sink` by storing
// its index. It also contains the overflow time, `\ot(S)`, so that it can be correctly
// sorted within the heap.
//
// At several points in the algorithm, we modify the overflow times in sinks that are
// being merged. The sort-order in the heap must be updated accordingly. However, given a
// `Sink`, we have no way to access the `SinkRef` that refers to it. Therefore we add
// version fields `ver` to `Sink` and `SinkRef`. Whenever we modify sink, we increment its
// version field and push a new `SinkRef` with the updated version number and overflow
// time onto the heap. When we pop a `SinkRef` with an outdated version number from the
// heap, we simply ignore it. Since in every iteration we pop one element from the heap
// and push at most two, and since we push at most 2n times (as per the complexity proof
// in the paper), it is easy to see that the heap never contains more than 4n element.
// Therefore our complexity analysis holds.
//
// In the paper, we proved everything based on segments with arbitrary widths. However,
// since as input we only accept segments with width 1, every segment that occurs within
// the algorithm actually has a width that is a multiple of 0.5. To avoid problems with
// floating point imprecision, we store all widths as `u32`s which contain double the
// actual width. When we perform computations, we divide this number by 2.
//
// In general, this implementation is significantly more involved than the description
// in the paper because we have to carefully maintain the linked lists.

#[derive(Debug, Clone)]
struct Segment {
    idx: usize,
    width_times_2: u32,
    elevation: f64,
    next: Option<usize>,
    prev: Option<usize>,
}

#[derive(Clone, Default, Debug)]
struct Sink {
    idx: usize,
    /// `None` iff this sink is no longer in the linked list.
    ver: Option<u32>,
    prev: Option<usize>,
    next: Option<usize>,
    /// The elevations of the left/right boundary. `\infty` if this is the
    /// leftmost/rightmost sink.
    left_elevation: f64,
    right_elevation: f64,
    /// The minimum of `left_elevation` and `right_elevation`.
    min_boundary: f64,
    width_times_2: u32,
    /// The capacity until the sink overflows.
    capacity: f64,
    /// Stores how much water is in the sink at time `fill_time`.
    fill: f64,
    fill_time: f64,
    /// Stores the time at which the sink will overflow assuming that no other sink
    /// overflows first.
    overflow_time: f64,
    /// On which side the sink will overflow if `overflow_time < \infty`.
    capacity_side: CapacitySide,
    /// The width of the part of the sink that is <= the water level at which the sink
    /// overflows.
    capacity_width_times_2: u32,
    /// `start` points to the first segment in the sink. `end - 1` points to the last
    /// segment in the sink.
    segments: Range<usize>,
    /// `start` points to the first capacity segment. `end` points to the first segment
    /// in the linked list that's not a capacity segment of this sink. If no such segment
    /// exists, it has the same value as `segments.end`.
    capacity_segments: Range<usize>,
}

#[derive(Copy, Clone, Debug)]
struct SinkRef {
    pos: usize,
    ver: u32,
    overflow_time: f64,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum CapacitySide {
    Left,
    Right,
    Both,
}

/// Working storage for the algorithm.
#[derive(Clone, Debug)]
struct Terrain {
    segments: Vec<Segment>,
    sinks: Vec<Sink>,
    heap: BinaryHeap<SinkRef>,
}
