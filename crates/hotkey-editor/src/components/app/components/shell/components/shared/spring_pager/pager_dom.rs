use super::spring_curve::SpringCurve;
use std::str::FromStr;
use wasm_bindgen::JsCast;

/// The strip positions at which a card is at rest: its top at the pager's top,
/// and, for a card taller than the pager, every position down to its bottom at
/// the pager's bottom. A strip position is how far the strip is moved up.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
pub(crate) struct RestingRange {
    first_position: f64,
    last_position: f64,
}

impl RestingRange {
    pub(crate) fn first_position(&self) -> f64 {
        self.first_position
    }

    pub(crate) fn last_position(&self) -> f64 {
        self.last_position
    }

    pub(crate) fn contains(&self, strip_position: f64) -> bool {
        strip_position >= self.first_position && strip_position <= self.last_position
    }

    pub(crate) fn clamp(&self, strip_position: f64) -> f64 {
        strip_position.clamp(self.first_position, self.last_position)
    }
}

/// Where one mounted card sits on the strip.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
struct CardSpan {
    top: f64,
    height: f64,
}

/// The mounted cards' places on the strip and the height of the pager showing
/// them, read in one pass. Card places are layout offsets on the strip itself,
/// so they do not change while the strip moves.
#[derive(Clone, Debug, PartialEq, PartialOrd, Default)]
pub(crate) struct PagerLayout {
    cards: Vec<CardSpan>,
    viewport_height: f64,
}

impl PagerLayout {
    pub(crate) fn card_top(&self, offset: usize) -> Option<f64> {
        self.cards.get(offset).map(|card| card.top)
    }

    pub(crate) fn resting_range(&self, offset: usize) -> Option<RestingRange> {
        let card = self.cards.get(offset)?;
        let overflow = (card.height - self.viewport_height).max(0.0);
        let last_position = card.top + overflow;
        let range = RestingRange {
            first_position: card.top,
            last_position,
        };
        Some(range)
    }

    /// The card whose resting range lies closest to `strip_position`.
    pub(crate) fn nearest_card(&self, strip_position: f64) -> Option<usize> {
        (0..self.cards.len())
            .filter_map(|offset| {
                let range = self.resting_range(offset)?;
                let resting_position = range.clamp(strip_position);
                let distance = (resting_position - strip_position).abs();
                let candidate = NearestCandidate { offset, distance };
                Some(candidate)
            })
            .min_by(|first, second| first.distance.total_cmp(&second.distance))
            .map(|candidate| candidate.offset)
    }

    /// The span of strip positions any card can rest at, from the first card's
    /// top to the last card's lowest resting position.
    pub(crate) fn bounds(&self) -> Option<RestingRange> {
        let first_range = self.resting_range(0)?;
        let last_offset = self.cards.len().checked_sub(1)?;
        let last_range = self.resting_range(last_offset)?;
        let bounds = RestingRange {
            first_position: first_range.first_position,
            last_position: last_range.last_position,
        };
        Some(bounds)
    }

    pub(crate) fn viewport_height(&self) -> f64 {
        self.viewport_height
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
struct NearestCandidate {
    offset: usize,
    distance: f64,
}

// Where the vertical translation sits among a computed `matrix(…)`'s six
// arguments and a `matrix3d(…)`'s sixteen.
const MATRIX_VERTICAL_INDEX: usize = 5;
const MATRIX3D_VERTICAL_INDEX: usize = 13;

/// The vertical translation of a computed CSS `transform`, which the browser
/// reports as `none`, a `matrix(…)` or a `matrix3d(…)`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
struct ComputedTranslation {
    vertical: f64,
}

impl FromStr for ComputedTranslation {
    type Err = std::num::ParseFloatError;

    fn from_str(computed_transform: &str) -> Result<Self, Self::Err> {
        let trimmed = computed_transform.trim();
        let vertical_argument = if let Some(arguments) = trimmed.strip_prefix("matrix3d(") {
            arguments.split(',').nth(MATRIX3D_VERTICAL_INDEX)
        } else if let Some(arguments) = trimmed.strip_prefix("matrix(") {
            arguments.split(',').nth(MATRIX_VERTICAL_INDEX)
        } else {
            return Ok(Self::default());
        };
        let vertical_text = vertical_argument
            .unwrap_or("0")
            .trim_end_matches(')')
            .trim();
        let vertical = vertical_text.parse::<f64>()?;
        Ok(Self { vertical })
    }
}

/// Reads the pager's real layout and moves its strip. Every position the pager
/// acts on comes from here, never from an estimate, so a card of any height
/// lands exactly where it is meant to.
pub(crate) struct PagerDom<'container> {
    container: &'container web_sys::Element,
    card_selector: &'static str,
}

impl<'container> PagerDom<'container> {
    pub(crate) fn new(
        container: &'container web_sys::Element,
        card_selector: &'static str,
    ) -> Self {
        Self {
            container,
            card_selector,
        }
    }

    fn strip(&self) -> Option<web_sys::HtmlElement> {
        let first_child = self.container.first_element_child()?;
        first_child.dyn_into::<web_sys::HtmlElement>().ok()
    }

    fn cards(&self) -> Vec<web_sys::HtmlElement> {
        let Ok(nodes) = self.container.query_selector_all(self.card_selector) else {
            return Vec::new();
        };
        (0..nodes.length())
            .filter_map(|index| nodes.item(index))
            .filter_map(|node| node.dyn_into::<web_sys::HtmlElement>().ok())
            .collect()
    }

    /// Every card's place on the strip, measured from the strip's own top so the
    /// strip's movement cancels out. Bounding boxes keep the fractions of a
    /// pixel that whole-pixel offsets round away; a pager resting a fraction
    /// off a card's whole-pixel top would otherwise read as sitting inside it.
    pub(crate) fn layout(&self) -> PagerLayout {
        let strip_top = self
            .strip()
            .map(|strip| strip.get_bounding_client_rect().top())
            .unwrap_or_default();
        let cards = self
            .cards()
            .iter()
            .map(|card| {
                let card_box = card.get_bounding_client_rect();
                let top = card_box.top() - strip_top;
                let height = card_box.height();
                CardSpan { top, height }
            })
            .collect();
        let client_height = self.container.client_height();
        let viewport_height = f64::from(client_height);
        PagerLayout {
            cards,
            viewport_height,
        }
    }

    /// Whether `target` is the strip itself, so a transition ending on a card
    /// inside it is not mistaken for the strip's own.
    pub(crate) fn is_strip(&self, target: &web_sys::EventTarget) -> bool {
        let Some(strip) = self.strip() else {
            return false;
        };
        let strip_target: &web_sys::EventTarget = strip.as_ref();
        strip_target == target
    }

    /// Whether the last mounted card's top is inside the pager's visible area,
    /// the same reach the near-watch observes. A single layout read, made only
    /// when that watch reports a change.
    pub(crate) fn last_card_is_near(&self) -> bool {
        let cards = self.cards();
        let Some(last_card) = cards.last() else {
            return false;
        };
        let container_box = self.container.get_bounding_client_rect();
        let last_card_top = last_card.get_bounding_client_rect().top();
        last_card_top <= container_box.bottom()
    }

    /// Where the strip is right now, mid-animation included: how far its top
    /// sits above the top of the pager's content.
    pub(crate) fn current_strip_position(&self) -> Option<f64> {
        let strip = self.strip()?;
        let strip_top = strip.get_bounding_client_rect().top();
        let container_top = self.container.get_bounding_client_rect().top();
        let border_top = self.container.client_top();
        let border_width = f64::from(border_top);
        let content_top = container_top + border_width;
        Some(content_top - strip_top)
    }

    /// Where the pager's own scroll has the strip, with its transform taken back
    /// out: the strip's position minus what its transform currently adds. Read
    /// from the layout rather than `scrollTop`, which only reports whole pixels
    /// while an engine may rest the scroll a fraction off one.
    fn resting_scroll_position(&self) -> Option<f64> {
        let strip = self.strip()?;
        let current_position = self.current_strip_position()?;
        let window = web_sys::window()?;
        let computed_style = window.get_computed_style(&strip).ok()??;
        let computed_transform = computed_style.get_property_value("transform").ok()?;
        let translation = computed_transform.parse::<ComputedTranslation>().ok()?;
        Some(current_position + translation.vertical)
    }

    /// Brings the strip to rest at `strip_position` as a plain scroll of the
    /// pager, with no transform left on the strip. A transformed element becomes
    /// the containing block of every `position: fixed` element inside it, so a
    /// card's dialog would be pinned to the strip instead of the screen; at rest
    /// nothing inside the cards may have a transformed ancestor.
    pub(crate) fn rest_strip(&self, strip_position: f64) {
        let Some(strip) = self.strip() else {
            return;
        };
        let style = strip.style();
        let _ = style.set_property("transition", "none");
        let _ = style.remove_property("transform");
        let _ = style.remove_property("will-change");
        let scroll_pixels = ScrollPixels::from(strip_position);
        self.container.set_scroll_top(scroll_pixels.pixels);
    }

    /// Moves the strip to `strip_position` at once, stopping any animation.
    pub(crate) fn place_strip(&self, strip_position: f64) {
        let no_transition = "none";
        self.style_strip(strip_position, no_transition);
    }

    /// Moves the strip to `strip_position` along `curve`.
    pub(crate) fn animate_strip(&self, strip_position: f64, curve: &SpringCurve) {
        let transition = curve.to_string();
        self.style_strip(strip_position, &transition);
    }

    fn style_strip(&self, strip_position: f64, transition: &str) {
        let Some(strip) = self.strip() else {
            return;
        };
        let Some(scroll_position) = self.resting_scroll_position() else {
            return;
        };
        let style = strip.style();
        // The pager's scroll stays where the strip last rested; while it moves,
        // the transform carries only the distance from there.
        let vertical_translation = scroll_position - strip_position;
        let transform = format!("translate3d(0, {vertical_translation:.2}px, 0)");
        let _ = style.set_property("will-change", "transform");
        let _ = style.set_property("transition", transition);
        let _ = style.set_property("transform", &transform);
    }
}

/// A strip position as the whole pixels the pager's scroll takes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
struct ScrollPixels {
    pixels: i32,
}

impl From<f64> for ScrollPixels {
    fn from(strip_position: f64) -> Self {
        let rounded = strip_position.round();
        Self {
            pixels: rounded as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ComputedTranslation;

    #[test]
    fn reads_the_vertical_translation_of_a_matrix() {
        let translation = "matrix(1, 0, 0, 1, 0, -1234.5)".parse::<ComputedTranslation>();
        let vertical = translation.map(|translation| translation.vertical);
        assert_eq!(vertical, Ok(-1234.5));
    }

    #[test]
    fn reads_the_vertical_translation_of_a_three_dimensional_matrix() {
        let computed = "matrix3d(1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, -88, 0, 1)";
        let translation = computed.parse::<ComputedTranslation>();
        let vertical = translation.map(|translation| translation.vertical);
        assert_eq!(vertical, Ok(-88.0));
    }

    #[test]
    fn reads_no_transform_as_no_translation() {
        let translation = "none".parse::<ComputedTranslation>();
        let vertical = translation.map(|translation| translation.vertical);
        assert_eq!(vertical, Ok(0.0));
    }
}
