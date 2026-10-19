//! The snap pager: a feed of cards the browser scrolls natively and freely, by
//! finger, trackpad and wheel alike, with no snapping. Beyond that the
//! module only reports: the card the scrolling comes to rest on, every scroll's
//! distance, and the scrolling coming near an end of the mounted cards, so a
//! feed can mount more before it runs out.

mod pager_edge;
mod pager_window;
mod passive_input_listeners;

pub(crate) use pager_edge::PagerEdge;
pub(crate) use pager_window::{PagerAnchor, PagerWindow, RosterSlot};

use gloo_timers::callback::Timeout;
use passive_input_listeners::PassiveInputListeners;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

// How long the pager must go without a scroll event before its scrolling counts
// as come to rest. Momentum reports a scroll every frame, but a mouse wheel
// leaves gaps between its notches; this outlasts those gaps, so a run of notches
// settles once at its end instead of on every pause, each settle being a full
// navigation that every mounted card renders again.
const SETTLE_QUIET_MS: u32 = 400;
// How many pager heights of mounted cards must stay ahead of the screen in
// either direction; closer than this, the feed is asked to mount more.
const NEAR_EDGE_SCREENS: f64 = 2.0;
/// What a pager is told: the index of the card the scrolling came to rest on,
/// the distance of every scroll (positive when it moved on to later cards), and
/// the end of the mounted cards the scrolling came near.
pub(crate) struct PagerCallbacks {
    on_settle: Box<dyn Fn(usize)>,
    on_scroll: Box<dyn Fn(f64)>,
    on_near_edge: Box<dyn Fn(PagerEdge)>,
}

impl PagerCallbacks {
    pub(crate) fn new(
        on_settle: impl Fn(usize) + 'static,
        on_scroll: impl Fn(f64) + 'static,
        on_near_edge: impl Fn(PagerEdge) + 'static,
    ) -> Self {
        Self {
            on_settle: Box::new(on_settle),
            on_scroll: Box::new(on_scroll),
            on_near_edge: Box::new(on_near_edge),
        }
    }
}

/// What the pager's listeners share.
#[derive(Debug, Default)]
struct PagerState {
    touching: Cell<bool>,
    settle_timer: RefCell<Option<Timeout>>,
    last_scroll_top: Cell<f64>,
    held_card: RefCell<Option<HeldCard>>,
    // Set by the user's first touch, wheel, key or pointer. Only scrolling of the
    // user's own mounts more cards: the pager's own placing never does, so
    // opening and jumping never set off a chain of mounts.
    user_scrolled: Cell<bool>,
    mount_above_mark: RefCell<Option<MountAboveMark>>,
}

/// The card on top of the pager just before cards are mounted above it, and
/// where it sat on screen, so the pager can put it back after the mount.
#[derive(Clone, Debug, PartialEq)]
struct MountAboveMark {
    card: web_sys::Element,
    offset_from_top: f64,
}

/// A card the pager keeps in place until the user scrolls: the cards still grow
/// and shrink after a render while their icons and fonts load, and every such
/// change would otherwise carry the aligned card away. It is held by its element,
/// not its index, so cards mounted before it never make another card the held one.
#[derive(Clone, Debug, PartialEq)]
struct HeldCard {
    card: web_sys::Element,
    offset_from_top: f64,
}

/// A natively scrolled feed of cards. The listeners are removed when
/// it is dropped.
pub(crate) struct SnapPager {
    container: web_sys::Element,
    card_selector: &'static str,
    state: Rc<PagerState>,
    _listeners: PassiveInputListeners,
    resize_observer: Option<web_sys::ResizeObserver>,
    _on_resize: Closure<dyn FnMut()>,
}

impl SnapPager {
    /// Starts watching `container`, whose cards are the elements matching
    /// `card_selector`, and reports through `callbacks`.
    pub(crate) fn attach(
        container: web_sys::Element,
        card_selector: &'static str,
        callbacks: PagerCallbacks,
    ) -> Self {
        let state = Rc::new(PagerState::default());
        let scroll_top = container.scroll_top();
        state.last_scroll_top.set(f64::from(scroll_top));
        let callbacks = Rc::new(callbacks);
        let mut listeners = PassiveInputListeners::new(&container);

        let scroll_container = container.clone();
        let scroll_state = state.clone();
        let scroll_callbacks = callbacks.clone();
        listeners.listen("scroll", move |_event| {
            let scroll_top = f64::from(scroll_container.scroll_top());
            let moved_distance = scroll_top - scroll_state.last_scroll_top.get();
            scroll_state.last_scroll_top.set(scroll_top);
            (scroll_callbacks.on_scroll)(moved_distance);
            let user_driven =
                scroll_state.user_scrolled.get() && scroll_state.held_card.borrow().is_none();
            if user_driven {
                Self::report_near_edge(&scroll_container, &scroll_callbacks);
            }
            Self::restart_settle(
                &scroll_container,
                card_selector,
                &scroll_state,
                &scroll_callbacks,
            );
        });

        let touch_start_state = state.clone();
        listeners.listen("touchstart", move |_event| {
            touch_start_state.touching.set(true);
            touch_start_state.user_scrolled.set(true);
            touch_start_state.held_card.borrow_mut().take();
            touch_start_state.settle_timer.borrow_mut().take();
        });

        // Any scrolling of the user's own lets go of the held card.
        for user_input_type in ["keydown", "pointerdown", "wheel"] {
            let input_state = state.clone();
            listeners.listen(user_input_type, move |_event| {
                input_state.user_scrolled.set(true);
                input_state.held_card.borrow_mut().take();
            });
        }

        for touch_end_type in ["touchend", "touchcancel"] {
            let end_container = container.clone();
            let end_state = state.clone();
            let end_callbacks = callbacks.clone();
            listeners.listen(touch_end_type, move |_event| {
                end_state.touching.set(false);
                Self::restart_settle(&end_container, card_selector, &end_state, &end_callbacks);
            });
        }

        // Re-places the held card whenever the cards or the pager change size.
        let resize_container = container.clone();
        let resize_state = state.clone();
        let boxed_on_resize: Box<dyn FnMut()> = Box::new(move || {
            let held_card = resize_state.held_card.borrow().clone();
            let Some(held_card) = held_card else {
                return;
            };
            Self::place_card(&resize_container, &resize_state, &held_card);
        });
        let on_resize = Closure::wrap(boxed_on_resize);
        let resize_callback = on_resize.as_ref().unchecked_ref();
        let resize_observer = web_sys::ResizeObserver::new(resize_callback).ok();
        if let Some(observer) = resize_observer.as_ref() {
            observer.observe(&container);
            if let Some(track) = container.first_element_child() {
                observer.observe(&track);
            }
        }

        Self {
            container,
            card_selector,
            state,
            _listeners: listeners,
            resize_observer,
            _on_resize: on_resize,
        }
    }

    /// Tells the feed when fewer than a couple of screens of mounted cards are
    /// left ahead in either direction, on every scroll, so it mounts more while
    /// the scrolling is still under way.
    fn report_near_edge(container: &web_sys::Element, callbacks: &PagerCallbacks) {
        let viewport_height = f64::from(container.client_height());
        let scroll_top = f64::from(container.scroll_top());
        let scroll_height = f64::from(container.scroll_height());
        let near_distance = viewport_height * NEAR_EDGE_SCREENS;
        let distance_below = scroll_height - viewport_height - scroll_top;
        if scroll_top < near_distance {
            (callbacks.on_near_edge)(PagerEdge::Start);
        }
        if distance_below < near_distance {
            (callbacks.on_near_edge)(PagerEdge::End);
        }
    }

    /// Reports the resting card once the pager has been quiet for the settle
    /// time. A finger on the screen holds the report back until it lifts.
    fn restart_settle(
        container: &web_sys::Element,
        card_selector: &'static str,
        state: &Rc<PagerState>,
        callbacks: &Rc<PagerCallbacks>,
    ) {
        let timer_container = container.clone();
        let timer_state = state.clone();
        let timer_callbacks = callbacks.clone();
        let settle_timer = Timeout::new(SETTLE_QUIET_MS, move || {
            if timer_state.touching.get() {
                return;
            }
            let Some(resting_card) = Self::nearest_card(&timer_container, card_selector) else {
                return;
            };
            (timer_callbacks.on_settle)(resting_card);
        });
        *state.settle_timer.borrow_mut() = Some(settle_timer);
    }

    /// The index of the card whose top sits nearest the top of the pager.
    fn nearest_card(container: &web_sys::Element, card_selector: &str) -> Option<usize> {
        let pager_top = Self::content_top(container);
        let cards = Self::cards(container, card_selector);
        let mut nearest: Option<NearestCard> = None;
        for (index, card) in cards.iter().enumerate() {
            let card_top = card.get_bounding_client_rect().top();
            let distance = (card_top - pager_top).abs();
            let is_nearer = nearest.is_none_or(|current| distance < current.distance);
            if is_nearer {
                nearest = Some(NearestCard { index, distance });
            }
        }
        nearest.map(|card| card.index)
    }

    fn cards(container: &web_sys::Element, card_selector: &str) -> Vec<web_sys::Element> {
        let Ok(nodes) = container.query_selector_all(card_selector) else {
            return Vec::new();
        };
        (0..nodes.length())
            .filter_map(|index| nodes.item(index))
            .filter_map(|node| node.dyn_into::<web_sys::Element>().ok())
            .collect()
    }

    /// The top of the pager's scrolling area, inside its border.
    fn content_top(container: &web_sys::Element) -> f64 {
        let container_top = container.get_bounding_client_rect().top();
        let border_top = f64::from(container.client_top());
        container_top + border_top
    }

    /// The index of the card nearest the top of the pager right now.
    fn top_card(&self) -> Option<usize> {
        Self::nearest_card(&self.container, self.card_selector)
    }

    /// How far the card at `index` sits below the top of the pager right now.
    pub(crate) fn offset_from_top(&self, index: usize) -> Option<f64> {
        let cards = Self::cards(&self.container, self.card_selector);
        let card = cards.get(index)?;
        let card_top = card.get_bounding_client_rect().top();
        let pager_top = Self::content_top(&self.container);
        Some(card_top - pager_top)
    }

    /// Scrolls at once so the card at `index` sits `offset_from_top` below the
    /// top of the pager, and keeps it there while the cards still change size,
    /// until the user scrolls. Returns whether that card exists.
    pub(crate) fn place(&self, index: usize, offset_from_top: f64) -> bool {
        let cards = Self::cards(&self.container, self.card_selector);
        let Some(card) = cards.get(index).cloned() else {
            return false;
        };
        let held_card = HeldCard {
            card,
            offset_from_top,
        };
        Self::place_card(&self.container, &self.state, &held_card);
        *self.state.held_card.borrow_mut() = Some(held_card);
        true
    }

    /// Marks the card on top of the pager just before cards are mounted above
    /// it. [`Self::end_mount_above`] puts it back once they are in.
    pub(crate) fn begin_mount_above(&self) {
        let Some(index) = self.top_card() else {
            return;
        };
        let cards = Self::cards(&self.container, self.card_selector);
        let Some(card) = cards.get(index).cloned() else {
            return;
        };
        let card_top = card.get_bounding_client_rect().top();
        let offset_from_top = card_top - Self::content_top(&self.container);
        let mark = MountAboveMark {
            card,
            offset_from_top,
        };
        *self.state.mount_above_mark.borrow_mut() = Some(mark);
    }

    /// Puts the marked card back where it sat on screen after cards were mounted
    /// above it. The browser's own scroll anchoring usually has already, and then
    /// the scroll position is left alone, so a scroll under way keeps running.
    pub(crate) fn end_mount_above(&self) {
        let Some(mark) = self.state.mount_above_mark.borrow_mut().take() else {
            return;
        };
        let card_top = mark.card.get_bounding_client_rect().top();
        let current_offset = card_top - Self::content_top(&self.container);
        if (current_offset - mark.offset_from_top).abs() >= 1.0 {
            let kept_card = HeldCard {
                card: mark.card.clone(),
                offset_from_top: mark.offset_from_top,
            };
            Self::place_card(&self.container, &self.state, &kept_card);
        }
        let scroll_top = f64::from(self.container.scroll_top());
        self.state.last_scroll_top.set(scroll_top);
    }

    fn place_card(container: &web_sys::Element, state: &PagerState, held_card: &HeldCard) {
        let card_top = held_card.card.get_bounding_client_rect().top();
        let current_offset = card_top - Self::content_top(container);
        let scroll_top = f64::from(container.scroll_top());
        let placed_scroll_top = scroll_top + current_offset - held_card.offset_from_top;
        let scroll_pixels = ScrollPixels::from(placed_scroll_top);
        container.set_scroll_top(scroll_pixels.pixels);
        let applied_scroll_top = f64::from(container.scroll_top());
        state.last_scroll_top.set(applied_scroll_top);
    }
}

impl Drop for SnapPager {
    fn drop(&mut self) {
        if let Some(observer) = self.resize_observer.as_ref() {
            observer.disconnect();
        }
        // A pending timer holds the state and the callbacks, so it would still
        // fire into the unmounted component that owned this pager.
        self.state.settle_timer.borrow_mut().take();
    }
}

/// A card and how far its top sits from the top of the pager.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
struct NearestCard {
    index: usize,
    distance: f64,
}

/// A scroll position as the whole pixels the pager's scroll takes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
struct ScrollPixels {
    pixels: i32,
}

impl From<f64> for ScrollPixels {
    fn from(scroll_top: f64) -> Self {
        let rounded = scroll_top.round();
        Self {
            pixels: rounded as i32,
        }
    }
}
