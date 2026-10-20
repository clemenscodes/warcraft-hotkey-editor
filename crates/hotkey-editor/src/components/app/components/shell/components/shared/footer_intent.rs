// Scrolling on to later cards hides the footer for good: on a phone every line of
// the screen belongs to the cards, so scrolling back never brings it up again.
const FOOTER_HIDE_DISTANCE: f64 = 8.0;

/// The way the cards have been scrolling, accumulated since the last change of
/// direction, so the footer follows an intent rather than every pixel.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
pub(crate) struct FooterIntent {
    accumulated_distance: f64,
}

impl FooterIntent {
    /// Whether the footer should now hide (`Some(true)`) after the cards
    /// scrolled `distance`, positive when moving on. It never asks to reveal it.
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
        None
    }
}
