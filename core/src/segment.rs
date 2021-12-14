use crate::Segment;

impl Segment {
    pub fn width(&self) -> f64 {
        self.width_times_2 as f64 / 2.0
    }
}
