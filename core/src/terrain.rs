use crate::{CapacitySide, Segment, Sink, SinkRef, Terrain};
use anyhow::{bail, Result};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn valid_f64(f: f64) -> bool {
    f.is_finite() && !f.is_sign_negative() && f <= u32::MAX as f64
}

impl Terrain {
    /// Computes then input to the algorithm.
    pub fn new(levels: &[f64]) -> Result<Self> {
        if levels.len() as u64 > u32::MAX as u64 / 2 {
            bail!("There can be at most u32::MAX/2 segments")
        }
        if levels.is_empty() {
            bail!("There must be at least one segment")
        }
        let mut segments: Vec<Segment> = vec![];
        let mut sinks: Vec<Sink> = vec![Sink {
            left_elevation: f64::INFINITY,
            right_elevation: f64::INFINITY,
            ..Default::default()
        }];
        // `true` iff the previous elevation level was larger than the elevation level before it.
        let mut last_is_max = false;
        // Merge the segments and create the sinks.
        for &level in levels {
            if !valid_f64(level) {
                bail!("Each `level` must be in the range [0, u32::MAX]");
            }
            if let Some(prev) = segments.last_mut() {
                if level == prev.elevation {
                    // Merge the segments.
                    prev.width_times_2 += 2;
                    continue;
                }
                let next = prev.idx + 1;
                prev.next = Some(next);
                if last_is_max && level < prev.elevation {
                    // The previous segment is a local maximum. Split it.
                    prev.width_times_2 /= 2;
                    let prev_sink = {
                        // Finish up the previous sink.
                        let prev_sink = sinks.last_mut().unwrap();
                        prev_sink.segments = prev_sink.segments.start..next;
                        prev_sink.right_elevation = prev.elevation;
                        prev_sink.next = Some(prev_sink.idx + 1);
                        prev_sink.idx
                    };
                    // Create the next sink.
                    sinks.push(Sink {
                        idx: sinks.len(),
                        prev: Some(prev_sink),
                        left_elevation: prev.elevation,
                        // This is currently the last sink so set the right elevation to infinity.
                        right_elevation: f64::INFINITY,
                        segments: next..next + 1,
                        ..Default::default()
                    });
                    // Push the right half of the local maximum.
                    let seg = Segment {
                        idx: next,
                        width_times_2: prev.width_times_2,
                        elevation: prev.elevation,
                        next: Some(next + 1),
                        prev: Some(prev.idx),
                    };
                    segments.push(seg);
                    last_is_max = false;
                } else if level > prev.elevation {
                    last_is_max = true;
                }
            }
            segments.push(Segment {
                idx: segments.len(),
                width_times_2: 2,
                elevation: level,
                next: None,
                prev: segments.last().map(|s| s.idx),
            });
        }
        {
            // Finish the last sink.
            let last_sink = sinks.last_mut().unwrap();
            last_sink.segments = last_sink.segments.start..segments.len();
        }
        let mut heap = BinaryHeap::new();
        // Compute the various properties of the sink.
        for sink in &mut sinks {
            // Compute the side on which the sink will overflow and the capacity segments.
            sink.capacity_segments = sink.segments.clone();
            match sink
                .left_elevation
                .partial_cmp(&sink.right_elevation)
                .unwrap()
            {
                Ordering::Less => {
                    sink.capacity_side = CapacitySide::Left;
                    for i in sink.segments.clone() {
                        if segments[i].elevation > sink.left_elevation {
                            sink.capacity_segments = sink.segments.start..i;
                            break;
                        }
                    }
                }
                Ordering::Equal => {
                    sink.capacity_side = CapacitySide::Both;
                }
                Ordering::Greater => {
                    sink.capacity_side = CapacitySide::Right;
                    for i in sink.segments.clone() {
                        if segments[i].elevation <= sink.right_elevation {
                            sink.capacity_segments = i..sink.segments.end;
                            break;
                        }
                    }
                }
            }
            sink.min_boundary = sink.left_elevation.min(sink.right_elevation);
            // Compute the width, capacity width, and capacity of the sinks.
            for i in sink.segments.clone() {
                sink.width_times_2 += segments[i].width_times_2;
                if sink.capacity_segments.contains(&i) {
                    sink.capacity_width_times_2 += segments[i].width_times_2;
                    sink.capacity +=
                        (sink.min_boundary - segments[i].elevation) * (segments[i].width());
                }
            }
            // Compute the overflow time.
            sink.overflow_time = sink.capacity / sink.width();
            sink.ver = Some(0);
            heap.push(SinkRef {
                pos: sink.idx,
                ver: 0,
                overflow_time: sink.overflow_time,
            });
        }
        Ok(Terrain {
            segments,
            sinks,
            heap,
        })
    }

    pub fn rain(mut self, hours: f64) -> Result<Vec<f64>> {
        if !valid_f64(hours) {
            bail!("`hours` must be in the range [0, u32::MAX]");
        }
        self.merge_segments(hours);
        let first_segment = self.fill_sinks(hours);
        Ok(self.extract_elevations(first_segment))
    }

    /// Implements the first loop from the algorithm.
    fn merge_segments(&mut self, target: f64) {
        // NOTE: The segments are accessed in a pretty random order so its easiest to not
        // hold on to references to them. Instead we re-index whenever we need to access a
        // segment. This macro makes this a bit easier.
        //
        // NOTE: This macro is written in this convoluted way because otherwise
        // IntellijRust will not be able to infer the type of its return value.
        macro_rules! sgm {
            ($pos:expr) => {
                <[Segment]>::get_mut(&mut self.segments, $pos).unwrap()
            };
        }
        loop {
            let sink_ref = self.heap.pop().unwrap();
            let (capacity_side, prev, next) = {
                let sink = &mut self.sinks[sink_ref.pos];
                if sink.ver != Some(sink_ref.ver) {
                    // Ignore outdated refs.
                    continue;
                }
                if sink.overflow_time >= target {
                    // None of the remaining sinks overflow until `target`.
                    break;
                }
                (sink.capacity_side, sink.prev, sink.next)
            };
            let mut head = &mut self.sinks[..];
            macro_rules! split {
                ($pos:expr) => {
                    if let Some(pos) = $pos {
                        let (h, t): (_, &mut [Sink]) = head.split_at_mut(pos);
                        #[allow(unused_assignments)]
                        {
                            head = h;
                        }
                        Some(&mut t[0])
                    } else {
                        None
                    }
                };
            }
            // Extract mutable references to the sink that we just popped `mid`, the sink
            // to its left `left` (if any), and the sink to its right `right` (if any).
            let right = split!(next);
            let mid = split!(Some(sink_ref.pos)).unwrap();
            let left = split!(prev);
            // Mark `mid` as being removed from the linked list.
            mid.ver = None;
            if capacity_side == CapacitySide::Both {
                let right = right.unwrap();
                let left = left.unwrap();
                left.ver = Some(left.ver.unwrap() + 1);
                right.ver = Some(right.ver.unwrap() + 1);
                // Calculate the current fill value.
                left.fill += (mid.overflow_time - left.fill_time) * left.width();
                right.fill += (mid.overflow_time - right.fill_time) * right.width();
                left.fill_time = mid.overflow_time;
                right.fill_time = mid.overflow_time;
                // Update the widths of the rightmost segment in `left` and the leftmost
                // segment in `right`.
                let old_left_plane = sgm![left.segments.end - 1].width_times_2;
                let old_right_plane = sgm![right.segments.start].width_times_2;
                let plane_width = (mid.width_times_2 + old_left_plane + old_right_plane) / 2;
                sgm![left.segments.end - 1].width_times_2 = plane_width;
                sgm![right.segments.start].width_times_2 = plane_width;
                left.width_times_2 += plane_width - old_left_plane;
                right.width_times_2 += plane_width - old_right_plane;
                if left.capacity_side != CapacitySide::Left {
                    // If `left` overflows on the right, the rightmost segment in `left`
                    // is part of the capacity width of `left`.
                    left.capacity_width_times_2 += plane_width - old_left_plane;
                }
                if right.capacity_side != CapacitySide::Right {
                    right.capacity_width_times_2 += plane_width - old_right_plane;
                }
                // Connect the segments
                sgm![left.segments.end - 1].next = Some(right.segments.start);
                sgm![right.segments.start].prev = Some(left.segments.end - 1);
                // Connect the sinks
                left.next = Some(right.idx);
                right.prev = Some(left.idx);
                // Recalculate the overflow times.
                left.overflow_time = (left.capacity - left.fill) / left.width() + left.fill_time;
                right.overflow_time =
                    (right.capacity - right.fill) / right.width() + right.fill_time;
                self.heap.push(SinkRef {
                    pos: left.idx,
                    ver: left.ver.unwrap(),
                    overflow_time: left.overflow_time,
                });
                self.heap.push(SinkRef {
                    pos: right.idx,
                    ver: right.ver.unwrap(),
                    overflow_time: right.overflow_time,
                });
            } else {
                let merged: &mut Sink;
                let update_fill = |merged: &mut Sink| {
                    merged.fill += (mid.overflow_time - merged.fill_time) * merged.width();
                    merged.fill_time = mid.overflow_time;
                };
                if capacity_side == CapacitySide::Left {
                    let left = left.unwrap();
                    // Calculate the current fill value.
                    update_fill(left);
                    // Extend the rightmost segment in `left` to cover the area of `mid`
                    // that overflowed.
                    sgm![left.segments.end - 1].width_times_2 += mid.capacity_width_times_2;
                    // NOTE: `mid.capacity_segments.end >= mid.segments.end` implies that
                    // `mid` is the rightmost sink. Since otherwise `capacity_side` would
                    // not be `Left` but `Both`.
                    if mid.capacity_segments.end < mid.segments.end {
                        // Connect the first non-capacity segment to the right with the
                        // last segment in `left`.
                        sgm![left.segments.end - 1].next = Some(mid.capacity_segments.end);
                        sgm![mid.capacity_segments.end].prev = Some(left.segments.end - 1);
                        left.segments.end = mid.segments.end;
                    } else {
                        // There are no segments after the last segment in `left`.
                        sgm![left.segments.end - 1].next = None;
                    }
                    if left.capacity_side != CapacitySide::Left {
                        // The rightmost segment in `left` is a capacity segment.
                        left.capacity_width_times_2 += mid.capacity_width_times_2;
                        left.capacity_segments.end = mid.capacity_segments.end;
                    }
                    // Connect `left` with the segment to the right of `mid`, if any.
                    if let Some(right) = right {
                        left.next = Some(right.idx);
                        right.prev = Some(left.idx);
                    } else {
                        left.next = None;
                    }
                    left.right_elevation = mid.right_elevation;
                    merged = left;
                } else {
                    let right = right.unwrap();
                    // Calculate the current fill value.
                    update_fill(right);
                    // Extend the leftmost segment in `right` to cover the area of `mid`
                    // that overflowed.
                    sgm![right.segments.start].width_times_2 += mid.capacity_width_times_2;
                    // Connect the leftmost segment in `right` with the first non-capacity
                    // segment in `mid`.
                    sgm![right.segments.start].prev = sgm![mid.capacity_segments.start].prev;
                    if let Some(prev) = sgm![right.segments.start].prev {
                        sgm![prev].next = Some(right.segments.start);
                        if prev >= mid.segments.start {
                            // If `mid` contained non-capacity segments, extend `segments`
                            // accordingly.
                            right.segments.start = mid.segments.start;
                        }
                    }
                    if right.capacity_side != CapacitySide::Right {
                        // The leftmost segment in `right` is a capacity segment.
                        right.capacity_width_times_2 += mid.capacity_width_times_2;
                        right.capacity_segments.start = mid.capacity_segments.start;
                    }
                    // Connect `right` with the segment to the left of `mid`, if any.
                    if let Some(left) = left {
                        left.next = Some(right.idx);
                        right.prev = Some(left.idx);
                    } else {
                        right.prev = None;
                    }
                    right.left_elevation = mid.left_elevation;
                    merged = right;
                }
                merged.width_times_2 += mid.width_times_2;
                // Update the capacity side of the merged segment.
                merged.capacity_side = match merged
                    .left_elevation
                    .partial_cmp(&merged.right_elevation)
                    .unwrap()
                {
                    Ordering::Less => CapacitySide::Left,
                    Ordering::Equal => CapacitySide::Both,
                    Ordering::Greater => CapacitySide::Right,
                };
                let min_boundary = merged.right_elevation.min(merged.left_elevation);
                // If the boundary height increased, we have to extend the capacity
                // segments and update the capacity.
                if min_boundary > merged.min_boundary {
                    // Add the uniform vertical increase in capacity above the old
                    // capacity segments.
                    merged.capacity +=
                        (min_boundary - merged.min_boundary) * merged.capacity_width();
                    // Extend the capacity segments to the left.
                    let mut i = merged.capacity_segments.start;
                    loop {
                        i = match sgm![i].prev {
                            Some(i) if i >= merged.segments.start => i,
                            _ => break,
                        };
                        let seg = sgm![i];
                        if seg.elevation > min_boundary {
                            break;
                        }
                        merged.capacity_segments.start = i;
                        merged.capacity_width_times_2 += seg.width_times_2;
                        merged.capacity += (min_boundary - seg.elevation) * seg.width();
                    }
                    // Extend the capacity segments to the right.
                    i = merged.capacity_segments.end;
                    while i < merged.segments.end {
                        let seg = sgm![i];
                        if seg.elevation > min_boundary {
                            break;
                        }
                        // NOTE: If `seg.next` is `None`, then `merged` is the rightmost
                        // sink. Therefore the invariant of `capacity_segments.end`
                        // continues to hold.
                        merged.capacity_segments.end = seg.next.unwrap_or(merged.segments.end);
                        merged.capacity_width_times_2 += seg.width_times_2;
                        merged.capacity += (min_boundary - seg.elevation) * seg.width();
                        i = merged.capacity_segments.end;
                    }
                    merged.min_boundary = min_boundary;
                }
                // Recalculate the overflow times.
                merged.overflow_time =
                    (merged.capacity - merged.fill) / merged.width() + merged.fill_time;
                merged.ver = Some(merged.ver.unwrap() + 1);
                self.heap.push(SinkRef {
                    pos: merged.idx,
                    ver: merged.ver.unwrap(),
                    overflow_time: merged.overflow_time,
                });
            }
        }
    }

    /// Implements the second loop from the algorithm.
    ///
    /// Returns the index of the first segment in the linked list of segments.
    ///
    /// After this function returns, the invariants of the sinks no longer hold and the
    /// sinks should no longer be used.
    fn fill_sinks(&mut self, target: f64) -> usize {
        macro_rules! sgm {
            ($pos:expr) => {
                <[Segment]>::get_mut(&mut self.segments, $pos).unwrap()
            };
        }
        let mut first_seg = None;
        for sink in &mut self.sinks {
            // Ignore sinks that are not in the linked list.
            if sink.ver.is_none() {
                continue;
            }
            if first_seg.is_none() {
                first_seg = Some(sink.segments.start);
            }
            sink.fill += (target - sink.fill_time) * sink.width();
            // Find the single local minimum of the sink `mid`, the segment to its left
            // `prev`, and the segment to its right `next`.
            // NOTE: If `mid` is at the left boundary of the sink, we set `prev = mid`.
            // Similarly for `next`.
            let mut mid = sink.segments.start;
            let mut prev = mid;
            let mut next = mid;
            loop {
                next = match sgm![mid].next {
                    Some(i) if i < sink.segments.end => i,
                    _ => break,
                };
                if sgm![next].elevation > sgm![mid].elevation {
                    break;
                }
                prev = mid;
                mid = next;
            }
            // Now we fill the sink.
            loop {
                // Set `left/right_elevation` to the heights of `prev` and `next`. If we
                // are at a boundary, simulate infinitely high walls.
                let mut left_elevation = f64::INFINITY;
                if prev < mid {
                    left_elevation = sgm![prev].elevation;
                }
                let mut right_elevation = f64::INFINITY;
                if next > mid {
                    right_elevation = sgm![next].elevation;
                }
                let wall_height = left_elevation.min(right_elevation);
                let capacity = (wall_height - sgm![mid].elevation) * sgm![mid].width();
                // If all of the remaining water fits into `mid`, we are done.
                if sink.fill < capacity {
                    sgm![mid].elevation += sink.fill / sgm![mid].width();
                    break;
                }
                if left_elevation < right_elevation {
                    // If `mid` overflows to the left, merge with `prev`.
                    sgm![prev].width_times_2 += sgm![mid].width_times_2;
                    // Note: Here we only update the link from prev to next. We will
                    // update the link in the other direction after the if-else chain.
                    sgm![prev].next = sgm![mid].next;
                    mid = prev;
                } else if left_elevation > right_elevation {
                    // If `mid` overflows to the left, merge with `next`.
                    sgm![next].width_times_2 += sgm![mid].width_times_2;
                    sgm![next].prev = sgm![mid].prev;
                    if Some(mid) == first_seg {
                        // If we are merging the first segment, we have to update its index.
                        first_seg = Some(next);
                    }
                    mid = next;
                } else {
                    // If `mid` overflows on both sides, merge `mid` and `next` into
                    // `prev`.  Note that, since `capacity < \infty`, `left_elevation <
                    // \infty` and `right_elevation < \infty`. Therefore `prev < mid <
                    // next`.
                    sgm![prev].width_times_2 += sgm![mid].width_times_2 + sgm![next].width_times_2;
                    sgm![prev].next = sgm![next].next;
                    mid = prev;
                }
                prev = sgm![mid].prev.unwrap_or(mid);
                next = sgm![mid].next.unwrap_or(mid);
                // Update the links in the other direction.
                if next > mid {
                    sgm![next].prev = Some(mid);
                }
                if prev < mid {
                    sgm![prev].next = Some(mid);
                }
                // Note: In an ideal world, this should never happen since the sink does
                // not overflow. However, due to floating point imprecision, it is
                // possible for `sink.fill` to be slightly larger than the capacity of the
                // sink. Therefore we have to simulate the segments in the sink being
                // disconnected from the other segments. Comment the next two if
                // statements out and run the tests to see what happens.
                if prev < sink.segments.start {
                    prev = mid;
                }
                if next >= sink.segments.end {
                    next = mid;
                }
                sink.fill -= capacity;
            }
        }
        first_seg.unwrap()
    }

    /// Extracts the elevation levels from the segments.
    fn extract_elevations(&self, first_segment: usize) -> Vec<f64> {
        // Note: If the segments were fully merged, then `width_times_2` would be an even
        // number for each segment. Alas they are not fully merged at the boundaries of
        // the sinks. However, the sum of the two boundary widths is always an even
        // number.  Therefore we carry the remainder from the left boundary over to the
        // right boundary before dividing by 2. This ensures that we don't lose one
        // element of `res` whenever we encounter such a boundary.
        let mut next = Some(first_segment);
        let mut rem = 0;
        let mut res = vec![];
        while let Some(cur) = next.take() {
            let sgm = &self.segments[cur];
            let div = (sgm.width_times_2 + rem) / 2;
            rem = (sgm.width_times_2 + rem) % 2;
            for _ in 0..div {
                res.push(sgm.elevation);
            }
            next = sgm.next;
        }
        res
    }
}
