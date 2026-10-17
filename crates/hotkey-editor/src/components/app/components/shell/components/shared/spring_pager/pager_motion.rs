use super::pager_dom::{PagerDom, PagerLayout, RestingRange};
use super::pager_window::{PagerAnchor, PagerWindow};
use super::spring_curve::{SpringCharacter, SpringCurve};
use crate::services::drag_state::DraggingSlot;
use dioxus::prelude::*;
use gloo_timers::callback::Timeout;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;

// How far a finger must travel before it drags the strip. The same distance a
// grid tile waits before it gives up its long-press drag, so a finger that holds
// still on a tile lifts the tile and never the strip.
const DRAG_START_DISTANCE: f64 = 12.0;
// How much of a drag's most recent movement measures the finger's speed on
// release. A finger that rests longer than this before lifting has no speed.
const VELOCITY_WINDOW_MS: f64 = 100.0;
// How far ahead a release looks: the strip is taken to keep the finger's speed
// for this long, and the card nearest that point is where it comes to rest.
const PROJECTION_MS: f64 = 300.0;
// The speed, in pixels per millisecond, past which a release always moves on to
// the next card in its direction, however short the drag was.
const FLICK_SPEED: f64 = 0.35;
// The spring a card snaps into place with: damped just enough that it overshoots
// a little and swings back once.
const SNAP_SPRING: SpringCharacter = SpringCharacter::new(0.7, 500.0);
// The spring a tall card glides with when a release keeps it inside its own
// range: no overshoot, it just runs out.
const GLIDE_SPRING: SpringCharacter = SpringCharacter::new(1.0, 700.0);
// How much of a drag past the first or last mounted card the strip follows.
const RUBBER_BAND_RESISTANCE: f64 = 0.35;
// The wheel travel, in pixels, that turns the page by one card.
const WHEEL_STEP_DISTANCE: f64 = 40.0;
// How long the wheel must stay quiet before it may turn another card, so one
// trackpad swipe with its trailing inertia turns exactly one card.
const WHEEL_QUIET_MS: u32 = 200;
// The pixels one wheel line stands for, for wheels that report lines.
const WHEEL_LINE_PIXELS: f64 = 16.0;
// The share of the pager a key moves a card taller than the pager by.
const KEYBOARD_PAGE_SHARE: f64 = 0.85;
// Moving the cards on hides the footer quickly; moving back reveals it only on
// a clear intent, so a spring swinging back never brings it up.
const FOOTER_HIDE_DISTANCE: f64 = 8.0;
const FOOTER_REVEAL_DISTANCE: f64 = 64.0;
// What marks a dialog, whose input belongs to that dialog and not to a pager
// outside it.
const DIALOG_SELECTOR: &str = "[role=\"dialog\"]";
// The elements a key press types into, which keep their keys for themselves.
const TYPING_TAGS: [&str; 3] = ["INPUT", "TEXTAREA", "SELECT"];
// How long past its curve's end an animation waits for `transitionend` before
// it settles anyway.
const REST_FALLBACK_GRACE_MS: u32 = 100;
// Closer than this to its target, a card is already there.
const SETTLED_DISTANCE: f64 = 0.5;

/// Which way the cards turn: on to the next unit, or back to the previous one.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
enum PagerDirection {
    #[default]
    Forward,
    Backward,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
struct PointerPoint {
    horizontal: f64,
    vertical: f64,
}

/// Where a finger came down: on which card, with the strip where.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
struct GestureStart {
    pointer_id: i32,
    point: PointerPoint,
    strip_position: f64,
    base_slot: isize,
    caught_animation: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
struct PositionSample {
    time_ms: f64,
    strip_position: f64,
}

/// A finger dragging the strip, with its recent positions to measure its speed.
#[derive(Clone, Debug, PartialEq, PartialOrd, Default)]
struct DragGesture {
    start: GestureStart,
    samples: Vec<PositionSample>,
    layout: PagerLayout,
}

impl DragGesture {
    fn current_position(&self) -> f64 {
        self.samples
            .last()
            .map(|sample| sample.strip_position)
            .unwrap_or(self.start.strip_position)
    }

    fn record(&mut self, sample: PositionSample) {
        let oldest_kept_ms = sample.time_ms - VELOCITY_WINDOW_MS;
        self.samples.push(sample);
        let first_kept = self
            .samples
            .iter()
            .position(|kept| kept.time_ms >= oldest_kept_ms)
            .unwrap_or(0);
        let last_pair_start = self.samples.len().saturating_sub(2);
        let first_kept_bounded = first_kept.min(last_pair_start);
        self.samples.drain(..first_kept_bounded);
    }

    /// The finger's speed at `release_time_ms`, in pixels per millisecond,
    /// positive when it moves the cards on.
    fn velocity_at(&self, release_time_ms: f64) -> f64 {
        let Some(first) = self.samples.first() else {
            return 0.0;
        };
        let Some(last) = self.samples.last() else {
            return 0.0;
        };
        let resting_ms = release_time_ms - last.time_ms;
        let span_ms = last.time_ms - first.time_ms;
        if resting_ms > VELOCITY_WINDOW_MS || span_ms <= 0.0 {
            return 0.0;
        }
        (last.strip_position - first.strip_position) / span_ms
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Default)]
enum PagerGesture {
    #[default]
    Idle,
    Pressed(GestureStart),
    Dragging(DragGesture),
}

/// How a motion leaves: from where, to which card, how fast, along which spring.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
struct MotionLaunch {
    current_position: f64,
    target: MotionTarget,
    velocity: f64,
    spring: SpringCharacter,
}

/// Which card a motion comes to rest on, and where on the strip.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
struct MotionTarget {
    offset: usize,
    strip_position: f64,
}

/// The way the cards have been moving, accumulated since the last change of
/// direction, so the footer follows an intent rather than every pixel.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
struct FooterIntent {
    accumulated_distance: f64,
}

impl FooterIntent {
    /// Whether the footer should now hide (`Some(true)`) or reveal
    /// (`Some(false)`) after the cards moved `distance`, positive when moving on.
    fn observe(&mut self, distance: f64) -> Option<bool> {
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

/// The wheel travel since the wheel last went quiet, and whether it has already
/// turned a card in that time.
#[derive(Debug, Default)]
struct WheelState {
    accumulated_distance: f64,
    turned_card: bool,
    quiet_timer: Option<Timeout>,
}

#[derive(Debug, Default)]
struct MotionState {
    held_anchor: Option<PagerAnchor>,
    heading_to: Option<PagerAnchor>,
    // Settles the strip once its animation must have ended, for when the
    // browser never reports `transitionend` (an interrupted style, a hidden
    // tab). Replaced by every new animation; a stale one finds nothing to do.
    rest_timer: Option<Timeout>,
    gesture: PagerGesture,
    wheel: WheelState,
    footer_intent: FooterIntent,
}

/// Moves the pager's strip of cards. A finger drags it one to one; letting go
/// throws it to the next card, or back to the one it came from, along a spring
/// that leaves at the finger's speed and settles with a small bounce. A wheel
/// or a key turns one card at a time along the same spring.
///
/// At rest it holds the card on screen in place: the cards around it keep
/// changing height after a render, and every such change re-applies the held
/// anchor. The anchor is let go the moment a finger drags the strip, and a new
/// one is held wherever the strip comes to rest, which is also when
/// `rested_anchor` reports it so the pager can settle on it.
#[derive(Clone)]
pub(crate) struct PagerMotion {
    element_ref: Rc<RefCell<Option<web_sys::Element>>>,
    window: Signal<PagerWindow>,
    rested_anchor: Signal<Option<PagerAnchor>>,
    dragging_slot: Signal<Option<DraggingSlot>>,
    footer_hidden: Option<Signal<bool>>,
    card_selector: &'static str,
    state: Rc<RefCell<MotionState>>,
}

impl PagerMotion {
    pub(crate) fn new(
        element_ref: Rc<RefCell<Option<web_sys::Element>>>,
        window: Signal<PagerWindow>,
        rested_anchor: Signal<Option<PagerAnchor>>,
        dragging_slot: Signal<Option<DraggingSlot>>,
        footer_hidden: Option<Signal<bool>>,
        card_selector: &'static str,
    ) -> Self {
        let state = Rc::new(RefCell::new(MotionState::default()));
        Self {
            element_ref,
            window,
            rested_anchor,
            dragging_slot,
            footer_hidden,
            card_selector,
            state,
        }
    }

    /// Starts holding `anchor` and applies it now, ending any gesture or
    /// animation. Returns whether the anchored card was in the DOM, so a caller
    /// can retry once it is.
    pub(crate) fn hold(&self, anchor: PagerAnchor) -> bool {
        {
            let mut state = self.state.borrow_mut();
            state.held_anchor = Some(anchor);
            state.heading_to = None;
            state.gesture = PagerGesture::Idle;
        }
        self.reapply()
    }

    /// Puts the held anchor's card back at its anchor offset. Does nothing when
    /// no anchor is held.
    pub(crate) fn reapply(&self) -> bool {
        let Some(anchor) = self.state.borrow().held_anchor else {
            return false;
        };
        let borrowed = self.element_ref.borrow();
        let Some(element) = borrowed.as_ref() else {
            return false;
        };
        let pager_dom = PagerDom::new(element, self.card_selector);
        let layout = pager_dom.layout();
        let window = *self.window.peek();
        let Some(strip_position) = Self::anchor_position(&layout, window, anchor) else {
            return false;
        };
        pager_dom.rest_strip(strip_position);
        true
    }

    pub(crate) fn pointer_down(&self, event: &web_sys::Event) {
        let Some(pointer) = event.dyn_ref::<web_sys::PointerEvent>() else {
            return;
        };
        if pointer.pointer_type() == "mouse" || self.comes_from_another_dialog(event) {
            return;
        }
        let borrowed = self.element_ref.borrow();
        let Some(element) = borrowed.as_ref() else {
            return;
        };
        let mut state = self.state.borrow_mut();
        if state.gesture != PagerGesture::Idle {
            return;
        }
        let pager_dom = PagerDom::new(element, self.card_selector);
        let Some(strip_position) = pager_dom.current_strip_position() else {
            return;
        };
        // A finger landing on a moving strip catches it where it is.
        let caught_animation = state.heading_to.is_some();
        if caught_animation {
            pager_dom.place_strip(strip_position);
        }
        let layout = pager_dom.layout();
        let window = *self.window.peek();
        let Some(base_slot) = Self::base_slot(&state, &layout, window, strip_position) else {
            return;
        };
        let client_x = pointer.client_x();
        let client_y = pointer.client_y();
        let point = PointerPoint {
            horizontal: f64::from(client_x),
            vertical: f64::from(client_y),
        };
        let start = GestureStart {
            pointer_id: pointer.pointer_id(),
            point,
            strip_position,
            base_slot,
            caught_animation,
        };
        state.gesture = PagerGesture::Pressed(start);
    }

    pub(crate) fn pointer_move(&self, event: &web_sys::Event) {
        let Some(pointer) = event.dyn_ref::<web_sys::PointerEvent>() else {
            return;
        };
        let borrowed = self.element_ref.borrow();
        let Some(element) = borrowed.as_ref() else {
            return;
        };
        let pager_dom = PagerDom::new(element, self.card_selector);
        let mut state = self.state.borrow_mut();
        let gesture = state.gesture.clone();
        let client_x = pointer.client_x();
        let client_y = pointer.client_y();
        let horizontal = f64::from(client_x);
        let vertical = f64::from(client_y);
        let time_ms = event.time_stamp();
        let mut drag = match gesture {
            PagerGesture::Idle => return,
            PagerGesture::Pressed(start) => {
                if pointer.pointer_id() != start.pointer_id {
                    return;
                }
                // A grid tile lifted by a long press owns the finger now.
                if self.dragging_slot.peek().is_some() {
                    self.abandon(&mut state, &pager_dom, start);
                    return;
                }
                let horizontal_distance = (horizontal - start.point.horizontal).abs();
                let vertical_distance = (vertical - start.point.vertical).abs();
                if horizontal_distance > DRAG_START_DISTANCE
                    && horizontal_distance > vertical_distance
                {
                    self.abandon(&mut state, &pager_dom, start);
                    return;
                }
                if vertical_distance <= DRAG_START_DISTANCE {
                    return;
                }
                state.held_anchor = None;
                state.heading_to = None;
                let first_sample = PositionSample {
                    time_ms,
                    strip_position: start.strip_position,
                };
                let layout = pager_dom.layout();
                DragGesture {
                    start,
                    samples: vec![first_sample],
                    layout,
                }
            }
            PagerGesture::Dragging(drag) => {
                if pointer.pointer_id() != drag.start.pointer_id {
                    return;
                }
                drag
            }
        };
        let finger_travel = vertical - drag.start.point.vertical;
        let dragged_position = drag.start.strip_position - finger_travel;
        // The window grows below while a drag runs, so a drag past the last card
        // known so far reads the layout again before it resists.
        let known_last_position = drag
            .layout
            .bounds()
            .map(|bounds| bounds.last_position())
            .unwrap_or(dragged_position);
        if dragged_position > known_last_position {
            drag.layout = pager_dom.layout();
        }
        let bounds = drag.layout.bounds();
        let strip_position = Self::rubber_band(dragged_position, bounds);
        let previous_position = drag.current_position();
        pager_dom.place_strip(strip_position);
        let sample = PositionSample {
            time_ms,
            strip_position,
        };
        drag.record(sample);
        let moved_distance = strip_position - previous_position;
        self.observe_footer(&mut state, moved_distance);
        state.gesture = PagerGesture::Dragging(drag);
    }

    pub(crate) fn pointer_up(&self, event: &web_sys::Event) {
        let release_time_ms = event.time_stamp();
        self.end_pointer(event, release_time_ms, true);
    }

    pub(crate) fn pointer_cancel(&self, event: &web_sys::Event) {
        let release_time_ms = event.time_stamp();
        self.end_pointer(event, release_time_ms, false);
    }

    fn end_pointer(&self, event: &web_sys::Event, release_time_ms: f64, keeps_speed: bool) {
        let Some(pointer) = event.dyn_ref::<web_sys::PointerEvent>() else {
            return;
        };
        let borrowed = self.element_ref.borrow();
        let Some(element) = borrowed.as_ref() else {
            return;
        };
        let pager_dom = PagerDom::new(element, self.card_selector);
        let mut state = self.state.borrow_mut();
        let gesture = state.gesture.clone();
        match gesture {
            PagerGesture::Idle => {}
            PagerGesture::Pressed(start) => {
                if pointer.pointer_id() == start.pointer_id {
                    self.abandon(&mut state, &pager_dom, start);
                }
            }
            PagerGesture::Dragging(drag) => {
                if pointer.pointer_id() != drag.start.pointer_id {
                    return;
                }
                state.gesture = PagerGesture::Idle;
                let velocity = if keeps_speed {
                    drag.velocity_at(release_time_ms)
                } else {
                    0.0
                };
                self.release(&mut state, &pager_dom, &drag, velocity);
            }
        }
    }

    /// The arrow and page keys turn one card, unless they are typing into a
    /// field on a card, and never with a modifier held.
    pub(crate) fn key_down(&self, event: &web_sys::Event) {
        let Some(keyboard_event) = event.dyn_ref::<web_sys::KeyboardEvent>() else {
            return;
        };
        let modified = keyboard_event.alt_key()
            || keyboard_event.ctrl_key()
            || keyboard_event.meta_key()
            || keyboard_event.shift_key();
        if modified || self.comes_from_another_dialog(event) {
            return;
        }
        let typing = event
            .target()
            .and_then(|target| target.dyn_into::<web_sys::HtmlElement>().ok())
            .is_some_and(|target| {
                let tag_name = target.tag_name();
                target.is_content_editable() || TYPING_TAGS.contains(&tag_name.as_str())
            });
        if typing {
            return;
        }
        let direction = match keyboard_event.key().as_str() {
            "ArrowDown" | "PageDown" => PagerDirection::Forward,
            "ArrowUp" | "PageUp" => PagerDirection::Backward,
            _ => return,
        };
        self.step(direction);
    }

    /// Turns one card in `direction` along the snap spring, or, inside a card
    /// taller than the pager, moves most of a pager height within it first.
    fn step(&self, direction: PagerDirection) {
        let borrowed = self.element_ref.borrow();
        let Some(element) = borrowed.as_ref() else {
            return;
        };
        let pager_dom = PagerDom::new(element, self.card_selector);
        let mut state = self.state.borrow_mut();
        if state.gesture != PagerGesture::Idle {
            return;
        }
        let Some(current_position) = pager_dom.current_strip_position() else {
            return;
        };
        let layout = pager_dom.layout();
        let window = *self.window.peek();
        let Some(base_slot) = Self::base_slot(&state, &layout, window, current_position) else {
            return;
        };
        let Some(base_offset) = Self::slot_offset(window, base_slot) else {
            return;
        };
        let Some(base_range) = layout.resting_range(base_offset) else {
            return;
        };
        let page_distance = layout.viewport_height() * KEYBOARD_PAGE_SHARE;
        let target = match direction {
            PagerDirection::Forward => {
                if current_position < base_range.last_position() - SETTLED_DISTANCE {
                    let paged_position = current_position + page_distance;
                    let strip_position = paged_position.min(base_range.last_position());
                    Some(MotionTarget {
                        offset: base_offset,
                        strip_position,
                    })
                } else {
                    let next_offset = base_offset.saturating_add(1);
                    layout
                        .resting_range(next_offset)
                        .map(|next_range| MotionTarget {
                            offset: next_offset,
                            strip_position: next_range.first_position(),
                        })
                }
            }
            PagerDirection::Backward => {
                if current_position > base_range.first_position() + SETTLED_DISTANCE {
                    let paged_position = current_position - page_distance;
                    let strip_position = paged_position.max(base_range.first_position());
                    Some(MotionTarget {
                        offset: base_offset,
                        strip_position,
                    })
                } else {
                    base_offset.checked_sub(1).and_then(|previous_offset| {
                        layout
                            .resting_range(previous_offset)
                            .map(|previous_range| MotionTarget {
                                offset: previous_offset,
                                strip_position: previous_range.last_position(),
                            })
                    })
                }
            }
        };
        let Some(target) = target else {
            return;
        };
        let footer_hidden = direction == PagerDirection::Forward;
        self.set_footer_hidden(footer_hidden);
        let launch = MotionLaunch {
            current_position,
            target,
            velocity: 0.0,
            spring: SNAP_SPRING,
        };
        self.animate_to(&mut state, &pager_dom, &layout, launch);
    }

    /// A wheel scrolls a tall card within itself and otherwise turns one card
    /// per burst of wheel travel.
    pub(crate) fn wheel(&self, event: &web_sys::Event) {
        let Some(wheel_event) = event.dyn_ref::<web_sys::WheelEvent>() else {
            return;
        };
        if self.comes_from_another_dialog(event) {
            return;
        }
        let direction = {
            let borrowed = self.element_ref.borrow();
            let Some(element) = borrowed.as_ref() else {
                return;
            };
            let pager_dom = PagerDom::new(element, self.card_selector);
            let layout = pager_dom.layout();
            let delta_y = wheel_event.delta_y();
            let wheel_distance = match wheel_event.delta_mode() {
                web_sys::WheelEvent::DOM_DELTA_LINE => delta_y * WHEEL_LINE_PIXELS,
                web_sys::WheelEvent::DOM_DELTA_PAGE => delta_y * layout.viewport_height(),
                _ => delta_y,
            };
            let mut state = self.state.borrow_mut();
            if state.gesture != PagerGesture::Idle {
                return;
            }
            if self.scroll_within_tall_card(&mut state, &pager_dom, &layout, wheel_distance) {
                return;
            }
            let quiet_state = self.state.clone();
            let quiet_timer = Timeout::new(WHEEL_QUIET_MS, move || {
                if let Ok(mut state) = quiet_state.try_borrow_mut() {
                    state.wheel.accumulated_distance = 0.0;
                    state.wheel.turned_card = false;
                }
            });
            state.wheel.quiet_timer = Some(quiet_timer);
            if state.wheel.turned_card {
                return;
            }
            state.wheel.accumulated_distance += wheel_distance;
            let accumulated_distance = state.wheel.accumulated_distance;
            if accumulated_distance.abs() < WHEEL_STEP_DISTANCE {
                return;
            }
            state.wheel.accumulated_distance = 0.0;
            state.wheel.turned_card = true;
            if accumulated_distance > 0.0 {
                PagerDirection::Forward
            } else {
                PagerDirection::Backward
            }
        };
        self.step(direction);
    }

    /// Moves a resting tall card by `wheel_distance` within itself. Returns
    /// whether it did, which it does not once the card's edge in that direction
    /// is already on screen: then the wheel turns the card instead.
    fn scroll_within_tall_card(
        &self,
        state: &mut MotionState,
        pager_dom: &PagerDom,
        layout: &PagerLayout,
        wheel_distance: f64,
    ) -> bool {
        if state.heading_to.is_some() {
            return false;
        }
        let Some(anchor) = state.held_anchor else {
            return false;
        };
        let window = *self.window.peek();
        let Some(offset) = Self::slot_offset(window, anchor.slot()) else {
            return false;
        };
        let Some(range) = layout.resting_range(offset) else {
            return false;
        };
        let Some(current_position) = Self::anchor_position(layout, window, anchor) else {
            return false;
        };
        let at_edge = if wheel_distance > 0.0 {
            current_position >= range.last_position() - SETTLED_DISTANCE
        } else {
            current_position <= range.first_position() + SETTLED_DISTANCE
        };
        if at_edge {
            return false;
        }
        let scrolled_position = current_position + wheel_distance;
        let strip_position = range.clamp(scrolled_position);
        pager_dom.rest_strip(strip_position);
        let Some(card_top) = layout.card_top(offset) else {
            return false;
        };
        let offset_from_top = card_top - strip_position;
        let moved_anchor = PagerAnchor::new(anchor.slot(), offset_from_top);
        state.held_anchor = Some(moved_anchor);
        self.observe_footer(state, wheel_distance);
        true
    }

    /// The strip finished an animation: hold the card it came to rest on and
    /// report it.
    pub(crate) fn transition_end(&self, event: &web_sys::Event) {
        let Some(target) = event.target() else {
            return;
        };
        {
            let borrowed = self.element_ref.borrow();
            let Some(element) = borrowed.as_ref() else {
                return;
            };
            let pager_dom = PagerDom::new(element, self.card_selector);
            if !pager_dom.is_strip(&target) {
                return;
            }
        }
        let Some(heading_to) = self.state.borrow().heading_to else {
            return;
        };
        self.finish_animation(heading_to);
    }

    /// Brings an animation heading to `anchor` to rest, unless the strip has
    /// since been sent elsewhere or a finger holds it.
    fn finish_animation(&self, anchor: PagerAnchor) {
        {
            let Ok(mut state) = self.state.try_borrow_mut() else {
                return;
            };
            if state.gesture != PagerGesture::Idle || state.heading_to != Some(anchor) {
                return;
            }
            state.heading_to = None;
        }
        self.come_to_rest(anchor);
    }

    /// Decides where a released drag comes to rest and sends it there: the card
    /// nearest the point the finger's speed would carry it to, at most one card
    /// away from the card the drag started on.
    fn release(
        &self,
        state: &mut MotionState,
        pager_dom: &PagerDom,
        drag: &DragGesture,
        velocity: f64,
    ) {
        let current_position = drag.current_position();
        let layout = pager_dom.layout();
        let window = *self.window.peek();
        let base_offset = Self::slot_offset(window, drag.start.base_slot)
            .or_else(|| layout.nearest_card(current_position));
        let Some(base_offset) = base_offset else {
            return;
        };
        let Some(base_range) = layout.resting_range(base_offset) else {
            return;
        };
        let projected_position = current_position + velocity * PROJECTION_MS;
        let next_offset = base_offset.saturating_add(1);
        let next_target = layout
            .resting_range(next_offset)
            .map(|next_range| MotionTarget {
                offset: next_offset,
                strip_position: next_range.first_position(),
            });
        let previous_target = base_offset.checked_sub(1).and_then(|previous_offset| {
            layout
                .resting_range(previous_offset)
                .map(|previous_range| MotionTarget {
                    offset: previous_offset,
                    strip_position: previous_range.last_position(),
                })
        });
        let stay_target = MotionTarget {
            offset: base_offset,
            strip_position: base_range.clamp(projected_position),
        };
        let flicked_forward =
            velocity > FLICK_SPEED && projected_position > base_range.last_position();
        let flicked_backward =
            velocity < -FLICK_SPEED && projected_position < base_range.first_position();
        let flicked_target = if flicked_forward {
            next_target
        } else if flicked_backward {
            previous_target
        } else {
            None
        };
        let nearest_target = [Some(stay_target), next_target, previous_target]
            .into_iter()
            .flatten()
            .min_by(|first, second| {
                let first_distance = (first.strip_position - projected_position).abs();
                let second_distance = (second.strip_position - projected_position).abs();
                first_distance.total_cmp(&second_distance)
            })
            .unwrap_or(stay_target);
        let target = flicked_target.unwrap_or(nearest_target);
        let glides_within_card = target.offset == base_offset
            && Self::is_tall(base_range)
            && base_range.contains(projected_position);
        let spring = if glides_within_card {
            GLIDE_SPRING
        } else {
            SNAP_SPRING
        };
        let launch = MotionLaunch {
            current_position,
            target,
            velocity,
            spring,
        };
        self.animate_to(state, pager_dom, &layout, launch);
    }

    /// Lets go of a press that never became a drag. A strip the finger caught
    /// mid-animation springs on to where it was heading.
    fn abandon(&self, state: &mut MotionState, pager_dom: &PagerDom, start: GestureStart) {
        state.gesture = PagerGesture::Idle;
        if !start.caught_animation {
            return;
        }
        let Some(heading_to) = state.heading_to else {
            return;
        };
        let layout = pager_dom.layout();
        let window = *self.window.peek();
        let Some(offset) = Self::slot_offset(window, heading_to.slot()) else {
            return;
        };
        let Some(strip_position) = Self::anchor_position(&layout, window, heading_to) else {
            return;
        };
        let target = MotionTarget {
            offset,
            strip_position,
        };
        let launch = MotionLaunch {
            current_position: start.strip_position,
            target,
            velocity: 0.0,
            spring: SNAP_SPRING,
        };
        self.animate_to(state, pager_dom, &layout, launch);
    }

    /// Sends the strip from the launch's position to its target along its
    /// spring, leaving at its velocity in pixels per millisecond.
    fn animate_to(
        &self,
        state: &mut MotionState,
        pager_dom: &PagerDom,
        layout: &PagerLayout,
        launch: MotionLaunch,
    ) {
        let MotionLaunch {
            current_position,
            target,
            velocity,
            spring,
        } = launch;
        let window = *self.window.peek();
        let target_slot = window.slot_at(target.offset);
        let Some(card_top) = layout.card_top(target.offset) else {
            return;
        };
        let offset_from_top = card_top - target.strip_position;
        let anchor = PagerAnchor::new(target_slot, offset_from_top);
        state.held_anchor = None;
        let distance = target.strip_position - current_position;
        if distance.abs() < SETTLED_DISTANCE {
            pager_dom.rest_strip(target.strip_position);
            state.heading_to = None;
            state.held_anchor = Some(anchor);
            let mut rested_anchor = self.rested_anchor;
            rested_anchor.set(Some(anchor));
            return;
        }
        let initial_velocity = velocity / distance;
        let curve = SpringCurve::new(spring, initial_velocity);
        pager_dom.animate_strip(target.strip_position, &curve);
        state.heading_to = Some(anchor);
        let curve_duration = curve.duration();
        let rest_delay_ms = curve_duration
            .milliseconds()
            .saturating_add(REST_FALLBACK_GRACE_MS);
        let resting_motion = self.clone();
        let rest_timer = Timeout::new(rest_delay_ms, move || {
            resting_motion.finish_animation(anchor);
        });
        state.rest_timer = Some(rest_timer);
    }

    fn come_to_rest(&self, anchor: PagerAnchor) {
        self.hold(anchor);
        let mut rested_anchor = self.rested_anchor;
        rested_anchor.set(Some(anchor));
    }

    /// The card a gesture or a key belongs to: the one the strip is heading to,
    /// else the one it holds, else the one nearest the strip's position.
    fn base_slot(
        state: &MotionState,
        layout: &PagerLayout,
        window: PagerWindow,
        strip_position: f64,
    ) -> Option<isize> {
        let known_anchor = state.heading_to.or(state.held_anchor);
        if let Some(anchor) = known_anchor
            && window.contains(anchor.slot())
        {
            return Some(anchor.slot());
        }
        let nearest_offset = layout.nearest_card(strip_position)?;
        Some(window.slot_at(nearest_offset))
    }

    /// Whether `event` happened inside a dialog other than the one the pager
    /// itself sits in. A card's own dialog is mounted inside the card, so its
    /// input reaches the pager, but it is not the pager's to act on; a pager that
    /// lives inside a dialog still acts on its own dialog's input.
    fn comes_from_another_dialog(&self, event: &web_sys::Event) -> bool {
        let target_dialog = event
            .target()
            .and_then(|target| target.dyn_into::<web_sys::Element>().ok())
            .and_then(|target| target.closest(DIALOG_SELECTOR).ok().flatten());
        let Some(target_dialog) = target_dialog else {
            return false;
        };
        let borrowed = self.element_ref.borrow();
        let own_dialog = borrowed
            .as_ref()
            .and_then(|element| element.closest(DIALOG_SELECTOR).ok().flatten());
        own_dialog.as_ref() != Some(&target_dialog)
    }

    fn slot_offset(window: PagerWindow, slot: isize) -> Option<usize> {
        if !window.contains(slot) {
            return None;
        }
        let first_slot = window.first_slot();
        let slot_distance = slot - first_slot;
        usize::try_from(slot_distance).ok()
    }

    fn anchor_position(
        layout: &PagerLayout,
        window: PagerWindow,
        anchor: PagerAnchor,
    ) -> Option<f64> {
        let anchor_slot = anchor.slot();
        let offset = Self::slot_offset(window, anchor_slot)?;
        let card_top = layout.card_top(offset)?;
        Some(card_top - anchor.offset_from_top())
    }

    fn is_tall(range: RestingRange) -> bool {
        range.last_position() > range.first_position() + SETTLED_DISTANCE
    }

    /// Lets a drag past the strip's ends follow only partly, so the end is felt.
    fn rubber_band(strip_position: f64, bounds: Option<RestingRange>) -> f64 {
        let Some(bounds) = bounds else {
            return strip_position;
        };
        if strip_position < bounds.first_position() {
            let overshoot = bounds.first_position() - strip_position;
            return bounds.first_position() - overshoot * RUBBER_BAND_RESISTANCE;
        }
        if strip_position > bounds.last_position() {
            let overshoot = strip_position - bounds.last_position();
            return bounds.last_position() + overshoot * RUBBER_BAND_RESISTANCE;
        }
        strip_position
    }

    fn observe_footer(&self, state: &mut MotionState, moved_distance: f64) {
        let Some(footer_hidden) = state.footer_intent.observe(moved_distance) else {
            return;
        };
        self.set_footer_hidden(footer_hidden);
    }

    fn set_footer_hidden(&self, footer_hidden: bool) {
        let Some(mut footer_hidden_signal) = self.footer_hidden else {
            return;
        };
        if *footer_hidden_signal.peek() != footer_hidden {
            footer_hidden_signal.set(footer_hidden);
        }
    }
}
