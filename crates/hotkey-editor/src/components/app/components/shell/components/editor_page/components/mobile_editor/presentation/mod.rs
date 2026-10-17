mod anchor_keeper;
mod card_resize_watch;
mod last_card_watch;
mod pager_dom;
mod passive_input_listeners;

use super::state::{PagerAnchor, PagerWindow, RosterSlot};
use crate::services::editor_state::context::use_editor_state;
use crate::services::navigation::context::use_view_navigation;
use crate::services::unit_catalog::context::use_unit_catalog;
use anchor_keeper::AnchorKeeper;
use card_resize_watch::CardResizeWatch;
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use last_card_watch::LastCardWatch;
use pager_dom::PagerDom;
use passive_input_listeners::PassiveInputListeners;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;

// Cards mounted on each side of the card on screen once a swipe settles. A swipe
// moves one card at a time (the hosts snap with `snap-always`). Quick swipes
// interrupt each other's snap, so the pager may not settle between them: going
// forward the window grows below on the fly (see `LastCardWatch`), going back it has
// this many cards above to swipe through before the next settle re-centres it.
const CARD_WINDOW_BUFFER: usize = 5;
// Cards appended below each time the last mounted card comes near. One at a
// time keeps each mount its own short task: the near-watch re-reports after every
// append, so the window keeps growing card by card while the last one is still in
// reach, instead of mounting a batch in one long frame mid-swipe.
const FORWARD_APPEND_STEP: usize = 1;
// The fewest cards that must stay mounted on either side of the card a swipe
// settles on. Re-centring drops cards above, which shifts the content and needs a
// layout-forcing scroll correction, so the window is left alone on most swipes
// and re-centred only when the card nears an edge or too many cards are mounted.
const EDGE_MARGIN: usize = 2;
// The most cards mounted at once before a settle trims the window back.
const MAX_MOUNTED_CARDS: usize = 17;
// How long after a finger lifts the pager waits for a scroll before it settles
// on its own. A swipe that keeps moving (snap or momentum) ends in `scrollend`
// instead; this only catches a finger lifted from a pager already at rest.
const TOUCH_RELEASE_SETTLE_MS: u32 = 150;
// How long the pager must go without a scroll frame before it counts as at rest.
// `scrollend` normally says so first, but not every engine fires it (older
// Safari), so a quiet spell after the last frame settles the pager there too.
const SCROLL_REST_SETTLE_MS: u32 = 150;

pub(super) struct MobileEditorPresentation {
    pub(super) onmounted: EventHandler<MountedEvent>,
    pub(super) onscroll: EventHandler<ScrollEvent>,
    pub(super) onscrollend: EventHandler<ScrollEvent>,
    pub(super) onpointerdown: EventHandler<PointerEvent>,
    pub(super) onkeydown: EventHandler<KeyboardEvent>,
    pub(super) window_unit_ids: Vec<WarcraftObjectId>,
}

pub(super) fn use_mobile_editor() -> MobileEditorPresentation {
    // The pager walks the whole roster — every unit of every race, in canonical
    // race order, melee before campaign — not the active race's filtered listing.
    // A swipe carries from the last unit of one race into the first of the next,
    // and `open_unit` (in `onscrollend`) switches the race tab to match.
    let catalog = use_unit_catalog();
    let unit_ids_memo = use_memo(move || {
        let roster = catalog.roster();
        let ids: Rc<[WarcraftObjectId]> = roster
            .entries()
            .iter()
            .map(|entry| entry.unit_id())
            .collect();
        ids
    });

    let editor = use_editor_state();
    let selected_slot = editor.selected_slot();
    let selected_from_research = editor.selected_from_research();
    let selected_from_uprooted = editor.selected_from_uprooted();
    let navigation = use_view_navigation();
    let selected_unit_id = navigation.selected_unit_id();

    // The window opens straight on the selected unit, so the very first render
    // already mounts the cards around it and nothing else.
    let opening_unit_ids = unit_ids_memo.peek().clone();
    let opening_unit_id = *selected_unit_id.peek();
    let opening_slot = opening_unit_id
        .and_then(|unit_id| opening_unit_ids.iter().position(|id| *id == unit_id))
        .and_then(|unit_index| isize::try_from(unit_index).ok())
        .unwrap_or(0);
    let opening_roster_len = opening_unit_ids.len();
    let window = use_signal(|| {
        PagerWindow::centered_on(opening_slot, CARD_WINDOW_BUFFER, opening_roster_len)
    });
    // Where a card must sit once the next render is committed. Set by a jump
    // (align the target) and by a re-centre (keep the card on screen still);
    // the effect below applies it against the real layout and clears it.
    let pending_anchor = use_signal(|| Some(PagerAnchor::aligned(opening_slot)));
    // Set once the pager element exists; the anchor effect waits for it, because
    // nothing can be aligned before the pager is in the DOM.
    let pager_mounted = use_signal(|| false);
    let element_ref = use_hook(|| Rc::new(RefCell::new(None::<web_sys::Element>)));
    // Last scroll position, so `onscroll` can tell which way the finger is going
    // and hide or reveal the footer accordingly.
    let previous_scroll_cell = use_hook(|| Rc::new(Cell::<i32>::new(0)));
    // Movement accumulated since the last direction change, so a brief snap-back
    // bounce during a downward swipe cannot be mistaken for an upward swipe.
    let scroll_intent_cell = use_hook(|| Rc::new(Cell::<i32>::new(0)));
    let hook_element_ref = element_ref.clone();
    let anchor_keeper = use_hook(move || AnchorKeeper::new(hook_element_ref, window));
    // Created on mount, once the pager element exists; dropping it disconnects.
    let resize_watch = use_hook(|| Rc::new(RefCell::new(None::<CardResizeWatch>)));
    // Whether a finger is on the pager. While it is, nothing re-centres or
    // navigates: re-centring adds and drops cards and corrects the scroll, and
    // doing that under a finger that has started the next swipe breaks it.
    let touch_active = use_hook(|| Rc::new(Cell::new(false)));
    // Bumped on every scroll frame, so a finger lifting can tell whether the
    // pager is still moving (a `scrollend` will follow) or already at rest.
    let scroll_generation = use_hook(|| Rc::new(Cell::<u32>::new(0)));
    // The slot the pager last settled on or jumped to, so a navigation that only
    // echoes the pager's own settle is not mistaken for a jump.
    let settled_slot_cell = use_hook(|| Rc::new(Cell::new(Some(opening_slot))));

    let scroll_search_dialog_open = editor.search_dialog_open();
    let scroll_unit_ids_memo = unit_ids_memo;
    let scroll_element_ref = element_ref.clone();
    let scroll_window = window;
    let scroll_pending_anchor = pending_anchor;
    let scroll_settled_slot = settled_slot_cell.clone();
    // Settles the pager on the card it has come to rest on: re-centres the window
    // around it and commits it as navigation. Runs when a scroll ends, and when a
    // finger lifts from a pager that is no longer moving.
    let settle: Rc<dyn Fn()> = Rc::new(move || {
        let borrowed = scroll_element_ref.borrow();
        let Some(element) = borrowed.as_ref() else {
            return;
        };
        let pager_dom = PagerDom::new(element);
        let Some(aligned_card) = pager_dom.aligned_card() else {
            return;
        };
        let current_window = *scroll_window.peek();
        let settled_slot = current_window.slot_at(aligned_card.offset());
        let unit_ids = scroll_unit_ids_memo.peek().clone();
        let roster_len = unit_ids.len();
        scroll_settled_slot.set(Some(settled_slot));
        // Re-centre the window on the card the swipe settled on when it has come
        // near an edge of the window, or the window has grown too large, so the
        // next swipe either way lands on a card that is already mounted. The card
        // on screen keeps exactly the offset it has now; the anchor effect
        // corrects the scroll for the cards added and dropped around it.
        let mounted_above = aligned_card.offset();
        let mounted_below = current_window
            .slot_count()
            .saturating_sub(mounted_above)
            .saturating_sub(1);
        let near_an_edge = mounted_above < EDGE_MARGIN || mounted_below < EDGE_MARGIN;
        let too_many_mounted = current_window.slot_count() > MAX_MOUNTED_CARDS;
        if near_an_edge || too_many_mounted {
            let recentered = PagerWindow::centered_on(settled_slot, CARD_WINDOW_BUFFER, roster_len);
            let keep_in_place = PagerAnchor::new(settled_slot, aligned_card.offset_from_top());
            let mut window_signal = scroll_window;
            let mut pending_anchor_signal = scroll_pending_anchor;
            window_signal.set(recentered);
            pending_anchor_signal.set(Some(keep_in_place));
        }
        // The search dialog owns the navigation while it is open, so a settle of
        // the pager underneath must not overwrite the unit it navigated to.
        if *scroll_search_dialog_open.peek() {
            return;
        }
        let Some(settled_unit_id) = RosterSlot::unit_index(settled_slot, roster_len)
            .and_then(|unit_index| unit_ids.get(unit_index).copied())
        else {
            return;
        };
        // Only a swipe onto a different unit is navigation. A jump the pager was
        // told to make settles on the unit it was told to show, which is already
        // current, so back and forward never get a duplicate history entry.
        if *selected_unit_id.peek() == Some(settled_unit_id) {
            return;
        }
        // A slot id is the ability itself, not the ability on this unit, so a
        // surviving selection would light up on every card that shares it. Each
        // is written only when it actually changes: every grid of every mounted
        // card recomputes its tiles on a write, even one that leaves it equal.
        let mut selected_slot_signal = selected_slot;
        let mut selected_from_research_signal = selected_from_research;
        let mut selected_from_uprooted_signal = selected_from_uprooted;
        if selected_slot_signal.peek().is_some() {
            selected_slot_signal.set(None);
        }
        if *selected_from_research_signal.peek() {
            selected_from_research_signal.set(false);
        }
        if *selected_from_uprooted_signal.peek() {
            selected_from_uprooted_signal.set(false);
        }
        // `open_unit` derives the race and the unit mode from the unit and
        // pushes a navigation snapshot, so swiping keeps the race theme, the
        // `unit` query parameter and the history entry in step with the card.
        navigation.open_unit(settled_unit_id);
    });
    let scrollend_settle = settle.clone();
    let scrollend_touch_active = touch_active.clone();
    let onscrollend = EventHandler::new(move |_event: ScrollEvent| {
        // A scroll interrupted by a finger starting the next swipe also ends;
        // that settle waits until the finger lifts.
        if scrollend_touch_active.get() {
            return;
        }
        scrollend_settle();
    });

    // Runs on every scroll frame, including mid-snap, so it stays cheap: it only
    // drives the footer and arms the rest settle, and never reads the layout.
    let footer_scroll_element_ref = element_ref.clone();
    let footer_scroll_previous = previous_scroll_cell.clone();
    let footer_scroll_intent = scroll_intent_cell.clone();
    let mut footer_hidden = editor.footer_hidden();
    let footer_scroll_generation = scroll_generation.clone();
    let rest_settle = settle.clone();
    let rest_touch_active = touch_active.clone();
    let rest_timer_pending = use_hook(|| Rc::new(Cell::new(false)));
    let onscroll = EventHandler::new(move |_event: ScrollEvent| {
        let next_generation = footer_scroll_generation.get().wrapping_add(1);
        footer_scroll_generation.set(next_generation);
        // Settle once the scroll has been quiet for a moment, for the engines
        // that never fire `scrollend`. One timer serves a whole scroll: it keeps
        // waiting while frames keep arriving and settles after the first quiet
        // spell, so a swipe arms one timer instead of one per frame. A finger on
        // the pager defers the settle to the release.
        if !rest_timer_pending.get() {
            rest_timer_pending.set(true);
            let delayed_settle = rest_settle.clone();
            let delayed_generation = footer_scroll_generation.clone();
            let delayed_touch_active = rest_touch_active.clone();
            let delayed_pending = rest_timer_pending.clone();
            spawn(async move {
                loop {
                    let generation_before_wait = delayed_generation.get();
                    gloo_timers::future::TimeoutFuture::new(SCROLL_REST_SETTLE_MS).await;
                    if delayed_generation.get() == generation_before_wait {
                        break;
                    }
                }
                delayed_pending.set(false);
                if delayed_touch_active.get() {
                    return;
                }
                delayed_settle();
            });
        }
        let borrowed = footer_scroll_element_ref.borrow();
        let Some(element) = borrowed.as_ref() else {
            return;
        };
        let scroll_top = element.scroll_top();
        // Scrolling the cards down hides the footer, up reveals it. Movement is
        // accumulated since the last direction change so a brief snap-back bounce
        // during a downward swipe never reaches the reveal threshold, while a
        // sustained upward swipe does. Hiding reacts quickly, revealing needs a
        // clear intent.
        let previous = footer_scroll_previous.get();
        footer_scroll_previous.set(scroll_top);
        let scroll_delta = scroll_top - previous;
        let mut scroll_intent = footer_scroll_intent.get();
        if (scroll_delta > 0 && scroll_intent < 0) || (scroll_delta < 0 && scroll_intent > 0) {
            scroll_intent = 0;
        }
        scroll_intent += scroll_delta;
        let want_hidden = if scroll_intent > 8 {
            scroll_intent = 0;
            true
        } else if scroll_intent < -64 {
            scroll_intent = 0;
            false
        } else {
            *footer_hidden.peek()
        };
        footer_scroll_intent.set(scroll_intent);
        if *footer_hidden.peek() != want_hidden {
            footer_hidden.set(want_hidden);
        }
    });

    // A unit chosen anywhere but a swipe — a race tab, the search dialog, back
    // and forward — re-centres the window straight on it and aligns its card.
    // Only that one window is ever mounted, so the pager never renders its way
    // through the units in between.
    let jump_unit_ids_memo = unit_ids_memo;
    let mut jump_window = window;
    let mut jump_pending_anchor = pending_anchor;
    let jump_settled_slot = settled_slot_cell.clone();
    use_effect(move || {
        let unit_ids = jump_unit_ids_memo();
        let roster_len = unit_ids.len();
        let Some(target_unit_id) = *selected_unit_id.read() else {
            return;
        };
        let Some(target_index) = unit_ids
            .iter()
            .position(|unit_id| *unit_id == target_unit_id)
        else {
            return;
        };
        let current_window = *jump_window.peek();
        let target_slot = current_window.nearest_slot_of(target_index, roster_len);
        // A swipe commits the unit it settled on, which is already on screen.
        if jump_settled_slot.get() == Some(target_slot) {
            return;
        }
        jump_settled_slot.set(Some(target_slot));
        let jumped = PagerWindow::centered_on(target_slot, CARD_WINDOW_BUFFER, roster_len);
        jump_window.set(jumped);
        jump_pending_anchor.set(Some(PagerAnchor::aligned(target_slot)));
    });

    // Applies the pending anchor once the window it belongs to is in the DOM, and
    // keeps holding it: the keeper re-applies it whenever a card changes size
    // until the user takes the scroll over, so the card lands exactly at its
    // anchor offset however the cards around it settle.
    let anchor_element_ref = element_ref.clone();
    let anchor_window = window;
    let mut anchor_pending = pending_anchor;
    let effect_keeper = anchor_keeper.clone();
    let effect_resize_watch = resize_watch.clone();
    use_effect(move || {
        let _ = pager_mounted.read();
        let _ = anchor_window.read();
        let Some(anchor) = *anchor_pending.read() else {
            return;
        };
        let borrowed = anchor_element_ref.borrow();
        let Some(element) = borrowed.as_ref() else {
            return;
        };
        if let Some(watch) = effect_resize_watch.borrow().as_ref() {
            watch.observe_cards(element);
        }
        drop(borrowed);
        if effect_keeper.hold(anchor) {
            anchor_pending.set(None);
        }
    });

    // The user taking the scroll over releases the held anchor, so a correction
    // never fights a finger, a wheel or a drag. The next settle holds a new one.
    let touch_keeper = anchor_keeper.clone();
    let touch_start_active = touch_active.clone();
    let start_touch: Rc<dyn Fn()> = Rc::new(move || {
        touch_keeper.release();
        touch_start_active.set(true);
    });
    // Lifting the finger hands the settle back. A pager still moving settles in
    // its `scrollend`; one already at rest gets no further scroll, so it settles
    // once a short wait shows no scroll followed. The native listener runs outside
    // the Dioxus runtime, so it only records the release (the way
    // `UndoHistory::install_keyboard_shortcuts` records a key) and the effect
    // below starts the wait inside the runtime.
    let touch_release_active = touch_active.clone();
    let touch_release_generation = scroll_generation.clone();
    let touch_release_request = use_signal(|| None::<u32>);
    let release_touch: Rc<dyn Fn()> = Rc::new(move || {
        touch_release_active.set(false);
        let released_generation = touch_release_generation.get();
        let mut release_request = touch_release_request;
        release_request.set(Some(released_generation));
    });
    let effect_release_settle = settle.clone();
    let effect_release_active = touch_active.clone();
    let effect_release_generation = scroll_generation.clone();
    use_effect(move || {
        let Some(released_generation) = *touch_release_request.read() else {
            return;
        };
        let delayed_settle = effect_release_settle.clone();
        let delayed_generation = effect_release_generation.clone();
        let delayed_active = effect_release_active.clone();
        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(TOUCH_RELEASE_SETTLE_MS).await;
            let pager_moved = delayed_generation.get() != released_generation;
            if pager_moved || delayed_active.get() {
                return;
            }
            delayed_settle();
        });
    });
    // Grows the window below when the last mounted card comes near, so the next
    // swipe forward always has a card to snap to, even when quick swipes never
    // let the pager settle in between. It asks only whether the last card is in
    // reach, never which card is on screen: the near-watch fires mid-swipe, while
    // the card on screen is still the previous one. Appending below never moves
    // what is on screen, so it is safe under a moving finger; the next settle
    // trims the window back once it holds too many cards.
    let lookahead_element_ref = element_ref.clone();
    let lookahead_unit_ids_memo = unit_ids_memo;
    let lookahead_window = window;
    let extend_forward: Rc<dyn Fn()> = Rc::new(move || {
        let borrowed = lookahead_element_ref.borrow();
        let Some(element) = borrowed.as_ref() else {
            return;
        };
        let pager_dom = PagerDom::new(element);
        if !pager_dom.last_card_is_near() {
            return;
        }
        let current_window = *lookahead_window.peek();
        let roster_len = lookahead_unit_ids_memo.peek().len();
        let extended = current_window.extended_forward(FORWARD_APPEND_STEP, roster_len);
        if extended != current_window {
            let mut window_signal = lookahead_window;
            window_signal.set(extended);
        }
    });
    // Created on mount, once the pager element exists; dropping them removes the
    // listeners and disconnects the observer.
    let input_listeners = use_hook(|| Rc::new(RefCell::new(None::<PassiveInputListeners>)));
    let last_card_watch = use_hook(|| Rc::new(RefCell::new(None::<LastCardWatch>)));
    let mounted_element_ref = element_ref.clone();
    let mut mounted_pager = pager_mounted;
    let mounted_keeper = anchor_keeper.clone();
    let mounted_resize_watch = resize_watch.clone();
    let mounted_input_listeners = input_listeners.clone();
    let mounted_last_card_watch = last_card_watch.clone();
    let mounted_start_touch = start_touch.clone();
    let mounted_release_touch = release_touch.clone();
    let mounted_extend_forward = extend_forward.clone();
    let onmounted = EventHandler::new(move |event: MountedEvent| {
        let Some(element) = event.data().try_as_web_event() else {
            return;
        };
        let resized_keeper = mounted_keeper.clone();
        let created_watch = CardResizeWatch::new(move || {
            resized_keeper.reapply();
        });
        if let Some(watch) = created_watch.as_ref() {
            watch.observe_cards(&element);
        }
        *mounted_resize_watch.borrow_mut() = created_watch;
        let mut listeners = PassiveInputListeners::new(&element);
        let touch_start_handler = mounted_start_touch.clone();
        listeners.listen("touchstart", move || touch_start_handler());
        let touch_end_handler = mounted_release_touch.clone();
        listeners.listen("touchend", move || touch_end_handler());
        let touch_cancel_handler = mounted_release_touch.clone();
        listeners.listen("touchcancel", move || touch_cancel_handler());
        let wheel_keeper = mounted_keeper.clone();
        listeners.listen("wheel", move || wheel_keeper.release());
        *mounted_input_listeners.borrow_mut() = Some(listeners);
        let near_handler = mounted_extend_forward.clone();
        let created_last_card_watch = LastCardWatch::new(&element, move || near_handler());
        if let Some(watch) = created_last_card_watch.as_ref() {
            watch.watch_last_card(&element);
        }
        *mounted_last_card_watch.borrow_mut() = created_last_card_watch;
        *mounted_element_ref.borrow_mut() = Some(element);
        mounted_pager.set(true);
    });

    // Every window change can mount a new last card, so the near-watch moves to it.
    let watch_element_ref = element_ref.clone();
    let watch_window = window;
    let effect_last_card_watch = last_card_watch.clone();
    use_effect(move || {
        let _ = pager_mounted.read();
        let _ = watch_window.read();
        let borrowed = watch_element_ref.borrow();
        let Some(element) = borrowed.as_ref() else {
            return;
        };
        if let Some(watch) = effect_last_card_watch.borrow().as_ref() {
            watch.watch_last_card(element);
        }
    });

    let pointer_keeper = anchor_keeper.clone();
    let onpointerdown = EventHandler::new(move |_event: PointerEvent| {
        pointer_keeper.release();
    });
    let keyboard_keeper = anchor_keeper.clone();
    let onkeydown = EventHandler::new(move |_event: KeyboardEvent| {
        keyboard_keeper.release();
    });

    let unit_ids = unit_ids_memo();
    let window_unit_ids = window.read().unit_ids(&unit_ids);

    MobileEditorPresentation {
        onmounted,
        onscroll,
        onscrollend,
        onpointerdown,
        onkeydown,
        window_unit_ids,
    }
}
