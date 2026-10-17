use wasm_bindgen::JsCast;

/// The mounted card the pager has snapped to, as its position among the mounted
/// cards and its top's distance from the pager's top.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
pub(super) struct AlignedCard {
    offset: usize,
    offset_from_top: f64,
}

impl AlignedCard {
    pub(super) fn offset(&self) -> usize {
        self.offset
    }

    pub(super) fn offset_from_top(&self) -> f64 {
        self.offset_from_top
    }
}

/// Reads the real on-screen positions of the pager's mounted cards. Every
/// position the pager acts on comes from here, never from an estimate, so a
/// card of any height lands exactly where it is meant to.
pub(super) struct PagerDom<'container> {
    container: &'container web_sys::Element,
}

impl<'container> PagerDom<'container> {
    pub(super) fn new(container: &'container web_sys::Element) -> Self {
        Self { container }
    }

    fn cards(&self) -> Vec<web_sys::Element> {
        let Ok(nodes) = self.container.query_selector_all(".pager-card-host") else {
            return Vec::new();
        };
        (0..nodes.length())
            .filter_map(|index| nodes.item(index))
            .filter_map(|node| node.dyn_into::<web_sys::Element>().ok())
            .collect()
    }

    fn container_top(&self) -> f64 {
        self.container.get_bounding_client_rect().top()
    }

    /// The top of the `offset`th mounted card, measured from the pager's top.
    pub(super) fn card_offset_from_top(&self, offset: usize) -> Option<f64> {
        let cards = self.cards();
        let card = cards.get(offset)?;
        let card_top = card.get_bounding_client_rect().top();
        let container_top = self.container_top();
        Some(card_top - container_top)
    }

    /// The mounted card whose top sits closest to the pager's top, the one the
    /// snap points have settled on.
    pub(super) fn aligned_card(&self) -> Option<AlignedCard> {
        let container_top = self.container_top();
        self.cards()
            .iter()
            .enumerate()
            .map(|(offset, card)| {
                let card_top = card.get_bounding_client_rect().top();
                let offset_from_top = card_top - container_top;
                AlignedCard {
                    offset,
                    offset_from_top,
                }
            })
            .min_by(|first, second| {
                let first_distance = first.offset_from_top.abs();
                let second_distance = second.offset_from_top.abs();
                first_distance.total_cmp(&second_distance)
            })
    }

    /// Whether the last mounted card's top is at most one pager height below the
    /// pager's visible area, the same reach the near-watch observes. A single
    /// layout read, made only when that watch reports a change.
    pub(super) fn last_card_is_near(&self) -> bool {
        let cards = self.cards();
        let Some(last_card) = cards.last() else {
            return false;
        };
        let container_box = self.container.get_bounding_client_rect();
        let reach_bottom = container_box.bottom() + container_box.height();
        let last_card_top = last_card.get_bounding_client_rect().top();
        last_card_top <= reach_bottom
    }

    /// Scrolls by `delta` pixels at once, with no smooth animation, so a
    /// correction is never visible.
    pub(super) fn scroll_by(&self, delta: f64) {
        let rounded_delta = delta.round();
        let Ok(delta_px) = i32::try_from(PixelDelta::from(rounded_delta)) else {
            return;
        };
        if delta_px == 0 {
            return;
        }
        let target_scroll_top = self.container.scroll_top().saturating_add(delta_px);
        self.container.set_scroll_top(target_scroll_top);
    }
}

/// A whole-pixel scroll distance, converted from a rounded `f64` measurement.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
struct PixelDelta {
    pixels: i64,
}

impl From<f64> for PixelDelta {
    fn from(rounded: f64) -> Self {
        Self {
            pixels: rounded as i64,
        }
    }
}

impl TryFrom<PixelDelta> for i32 {
    type Error = std::num::TryFromIntError;

    fn try_from(delta: PixelDelta) -> Result<Self, Self::Error> {
        i32::try_from(delta.pixels)
    }
}
