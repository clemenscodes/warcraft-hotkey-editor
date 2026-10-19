// Scrolling on to later cards hides the footer quickly; scrolling back reveals it
// only on a clear intent, so a snap settling back a few pixels never brings it up.
const FOOTER_HIDE_DISTANCE: f64 = 8.0;
const FOOTER_REVEAL_DISTANCE: f64 = 64.0;

/// The way the cards have been scrolling, accumulated since the last change of
/// direction, so the footer follows an intent rather than every pixel.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
pub(crate) struct FooterIntent {
    accumulated_distance: f64,
}

impl FooterIntent {
    /// Whether the footer should now hide (`Some(true)`) or reveal
    /// (`Some(false)`) after the cards scrolled `distance`, positive when moving on.
    pub(crate) fn observe(&mut self, distance: f64) -> Option<bool> {
        let changed_direction = (distance > 0.0 && self.accumulated_distance < 0.0)
            || (distance < 0.0 && self.accumulated_distance > 0.0);
        if changed_direction {
            self.accumulated_distance = 0.0;
        }
        self.accumulated_distance += distance;
        if self.accumulated_distance > FOOTER_HIDE_DISTANCE {
            self.accumulated_distance = 0.0;
            return Some(true);
        }
        if self.accumulated_distance < -FOOTER_REVEAL_DISTANCE {
            self.accumulated_distance = 0.0;
            return Some(false);
        }
        None
    }
}
