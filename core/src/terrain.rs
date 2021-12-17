use crate::{CapacitySide, Segment, Sink, SinkRef, Terrain};
use anyhow::{bail, Result};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn valid_f64(f: f64) -> bool {
    f.is_finite() && !f.is_sign_negative() && f <= u32::MAX as f64
}

impl Terrain {
    pub fn new(levels: &[f64]) -> Result<Self> {
        if levels.len() as u64 > u32::MAX as u64 {
            bail!("There can be at most u32::MAX segments")
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
        let mut last_is_max = false;
        for &level in levels {
            if !valid_f64(level) {
                bail!("Each `level` must be in the range [0, u32::MAX]");
            }
            if let Some(prev) = segments.last_mut() {
                if level == prev.elevation {
                    prev.width_times_2 += 2;
                    continue;
                }
                let next = prev.idx + 1;
                prev.next = Some(next);
                if last_is_max && level < prev.elevation {
                    prev.width_times_2 /= 2;
                    let prev_sink = {
                        let prev_sink = sinks.last_mut().unwrap();
                        prev_sink.segments = prev_sink.segments.start..next;
                        prev_sink.right_elevation = prev.elevation;
                        prev_sink.next = Some(prev_sink.pos + 1);
                        prev_sink.pos
                    };
                    sinks.push(Sink {
                        pos: sinks.len(),
                        prev: Some(prev_sink),
                        left_elevation: prev.elevation,
                        right_elevation: f64::INFINITY,
                        segments: next..next + 1,
                        ..Default::default()
                    });
                    let seg = Segment {
                        idx: next,
                        width_times_2: prev.width_times_2,
                        elevation: prev.elevation,
                        next: Some(next + 1),
                        prev: Some(prev.idx),
                    };
                    segments.push(seg);
                    last_is_max = false;
                } else if !last_is_max && level > prev.elevation {
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
            let last_sink = sinks.last_mut().unwrap();
            last_sink.segments = last_sink.segments.start..segments.len();
        }
        let mut heap = BinaryHeap::new();
        for sink in &mut sinks {
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
            for i in sink.segments.clone() {
                sink.width_times_2 += segments[i].width_times_2;
                if sink.capacity_segments.contains(&i) {
                    sink.capacity_width_times_2 += segments[i].width_times_2;
                    sink.capacity +=
                        (sink.min_boundary - segments[i].elevation) * (segments[i].width());
                }
            }
            sink.overflow_unit = sink.capacity / (sink.width());
            sink.ver = Some(0);
            heap.push(SinkRef {
                pos: sink.pos,
                ver: 0,
                overflow_unit: sink.overflow_unit,
            });
        }
        Ok(Terrain {
            segments,
            sinks,
            heap,
        })
    }

    pub fn fill(mut self, hours: f64) -> Result<Vec<f64>> {
        if !valid_f64(hours) {
            bail!("`hours` must be in the range [0, u32::MAX]");
        }
        // NOTE: IntellijRust fails to infer the type if we use `self.segments[$pos]`.
        macro_rules! sgm {
            ($pos:expr) => {
                <[Segment]>::get_mut(&mut self.segments, $pos).unwrap()
            };
        }
        macro_rules! sgmi {
            ($pos:expr) => {
                <[Segment]>::get(&self.segments, $pos).unwrap()
            };
        }
        let target = hours;
        let mut current = 0.0;
        while current < target {
            let sink_ref = self.heap.pop().unwrap();
            let (capacity_side, prev, next) = {
                let sink = &mut self.sinks[sink_ref.pos];
                if sink.ver != Some(sink_ref.ver) {
                    continue;
                }
                if sink.overflow_unit >= target {
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
            let right = split!(next);
            let mid = split!(Some(sink_ref.pos)).unwrap();
            let left = split!(prev);
            current = mid.overflow_unit;
            mid.ver = None;
            if capacity_side == CapacitySide::Both {
                let right = right.unwrap();
                let left = left.unwrap();
                left.ver = Some(left.ver.unwrap() + 1);
                right.ver = Some(right.ver.unwrap() + 1);
                left.fill += (current - left.fill_units) * left.width();
                right.fill += (current - right.fill_units) * right.width();
                left.fill_units = current;
                right.fill_units = current;
                let old_left_plane = sgm![left.segments.end - 1].width_times_2;
                let old_right_plane = sgm![right.segments.start].width_times_2;
                let plane_width = (mid.width_times_2 + old_left_plane + old_right_plane) / 2;
                sgm![left.segments.end - 1].width_times_2 = plane_width;
                sgm![right.segments.start].width_times_2 = plane_width;
                left.width_times_2 += plane_width - old_left_plane;
                right.width_times_2 += plane_width - old_right_plane;
                if left.capacity_side != CapacitySide::Left {
                    left.capacity_width_times_2 += plane_width - old_left_plane;
                }
                if right.capacity_side != CapacitySide::Right {
                    right.capacity_width_times_2 += plane_width - old_right_plane;
                }
                sgm![left.segments.end - 1].next = Some(right.segments.start);
                sgm![right.segments.start].prev = Some(left.segments.end - 1);
                left.next = Some(right.pos);
                right.prev = Some(left.pos);
                left.overflow_unit = (left.capacity - left.fill) / left.width() + left.fill_units;
                right.overflow_unit =
                    (right.capacity - right.fill) / right.width() + right.fill_units;
                self.heap.push(SinkRef {
                    pos: left.pos,
                    ver: left.ver.unwrap(),
                    overflow_unit: left.overflow_unit,
                });
                self.heap.push(SinkRef {
                    pos: right.pos,
                    ver: right.ver.unwrap(),
                    overflow_unit: right.overflow_unit,
                });
            } else {
                let merged: &mut Sink;
                let update_fill = |merged: &mut Sink| {
                    merged.fill += (current - merged.fill_units) * merged.width();
                    merged.fill_units = current;
                };
                if capacity_side == CapacitySide::Left {
                    let left = left.unwrap();
                    update_fill(left);
                    sgm![left.segments.end - 1].width_times_2 += mid.capacity_width_times_2;
                    if mid.capacity_segments.end < mid.segments.end {
                        sgm![left.segments.end - 1].next = Some(mid.capacity_segments.end);
                        sgm![mid.capacity_segments.end].prev = Some(left.segments.end - 1);
                        left.segments.end = mid.segments.end;
                    } else {
                        sgm![left.segments.end - 1].next = None;
                    }
                    left.width_times_2 += mid.width_times_2;
                    if left.capacity_side != CapacitySide::Left {
                        left.capacity_width_times_2 += mid.capacity_width_times_2;
                        left.capacity_segments.end = mid.capacity_segments.end;
                    }
                    if let Some(right) = right {
                        left.next = Some(right.pos);
                        right.prev = Some(left.pos);
                    } else {
                        left.next = None;
                    }
                    left.right_elevation = mid.right_elevation;
                    merged = left;
                } else {
                    let right = right.unwrap();
                    update_fill(right);
                    sgm![right.segments.start].width_times_2 += mid.capacity_width_times_2;
                    sgm![right.segments.start].prev = sgm![mid.capacity_segments.start].prev;
                    if let Some(prev) = sgm![right.segments.start].prev {
                        sgm![prev].next = Some(right.segments.start);
                        if prev >= mid.segments.start {
                            right.segments.start = mid.segments.start;
                        }
                    }
                    right.width_times_2 += mid.width_times_2;
                    if right.capacity_side != CapacitySide::Right {
                        right.capacity_width_times_2 += mid.capacity_width_times_2;
                        right.capacity_segments.start = mid.capacity_segments.start;
                    }
                    if let Some(left) = left {
                        left.next = Some(right.pos);
                        right.prev = Some(left.pos);
                    } else {
                        right.prev = None;
                    }
                    right.left_elevation = mid.left_elevation;
                    merged = right;
                }
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
                if min_boundary > merged.min_boundary {
                    merged.capacity +=
                        (min_boundary - merged.min_boundary) * merged.capacity_width();
                    let mut i = merged.capacity_segments.start;
                    loop {
                        i = match sgm![i].prev {
                            Some(i) if i >= merged.segments.start => i,
                            _ => break,
                        };
                        let seg = &sgm![i];
                        if seg.elevation > min_boundary {
                            break;
                        }
                        merged.capacity_segments.start = i;
                        merged.capacity_width_times_2 += seg.width_times_2;
                        merged.capacity += (min_boundary - seg.elevation) * seg.width();
                    }
                    i = merged.capacity_segments.end;
                    while i < merged.segments.end {
                        let seg = sgmi![i];
                        if seg.elevation > min_boundary {
                            break;
                        }
                        merged.capacity_segments.end = sgmi![i].next.unwrap_or(merged.segments.end);
                        merged.capacity_width_times_2 += seg.width_times_2;
                        merged.capacity += (min_boundary - seg.elevation) * seg.width();
                        i = match sgmi![i].next {
                            Some(i) => i,
                            _ => break,
                        };
                    }
                    merged.min_boundary = min_boundary;
                }
                merged.overflow_unit =
                    (merged.capacity - merged.fill) / merged.width() + merged.fill_units;
                merged.ver = Some(merged.ver.unwrap() + 1);
                self.heap.push(SinkRef {
                    pos: merged.pos,
                    ver: merged.ver.unwrap(),
                    overflow_unit: merged.overflow_unit,
                });
            }
        }
        let mut first_seg = None;
        for sink in &mut self.sinks {
            if sink.ver.is_none() {
                continue;
            }
            if first_seg.is_none() {
                first_seg = Some(sink.segments.start);
            }
            sink.fill += (target - sink.fill_units) * sink.width();
            dbg!(&sink);
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
            let mut left_elevation = f64::INFINITY;
            if prev < mid {
                left_elevation = sgm![prev].elevation;
            }
            let mut right_elevation = f64::INFINITY;
            if next > mid {
                right_elevation = sgm![next].elevation;
            }
            loop {
                dbg!(prev);
                dbg!(mid);
                dbg!(next);
                dbg!(left_elevation);
                dbg!(sgm![mid].elevation);
                dbg!(right_elevation);
                let wall_height = left_elevation.min(right_elevation);
                let capacity = (wall_height - sgm![mid].elevation) * sgm![mid].width();
                if sink.fill < capacity {
                    sgm![mid].elevation += sink.fill / sgm![mid].width();
                    break;
                }
                if left_elevation < right_elevation {
                    sgm![prev].width_times_2 += sgm![mid].width_times_2;
                    sgm![prev].next = sgm![mid].next;
                    mid = prev;
                } else if left_elevation > right_elevation {
                    sgm![next].width_times_2 += sgm![mid].width_times_2;
                    sgm![next].prev = sgm![mid].prev;
                    if Some(mid) == first_seg {
                        first_seg = Some(next);
                    }
                    mid = next;
                } else {
                    sgm![prev].width_times_2 += sgm![mid].width_times_2 + sgm![next].width_times_2;
                    sgm![prev].next = sgm![next].next;
                    mid = prev;
                }
                prev = sgm![mid].prev.unwrap_or(mid);
                next = sgm![mid].next.unwrap_or(mid);
                if prev < sink.segments.start {
                    prev = mid;
                }
                if next >= sink.segments.end {
                    next = mid;
                }
                if next > mid {
                    sgm![next].prev = Some(mid);
                    right_elevation = sgm![next].elevation;
                } else {
                    right_elevation = f64::INFINITY;
                }
                if prev < mid {
                    sgm![prev].next = Some(mid);
                    left_elevation = sgm![prev].elevation;
                } else {
                    left_elevation = f64::INFINITY;
                }
                sink.fill -= capacity;
            }
        }
        let mut cur = first_seg.unwrap();
        let mut rem = 0;
        let mut res = vec![];
        loop {
            let sgm = sgm!(cur);
            let div = (sgm.width_times_2 + rem) / 2;
            rem = (sgm.width_times_2 + rem) % 2;
            for _ in 0..div {
                res.push(sgm.elevation);
            }
            cur = match sgm.next {
                Some(n) => n,
                _ => break,
            };
        }
        Ok(res)
    }
}
