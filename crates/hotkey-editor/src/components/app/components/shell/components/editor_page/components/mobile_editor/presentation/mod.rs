use crate::components::app::components::shell::components::shared::spring_pager::{
    CardResizeWatch, LastCardWatch, PagerAnchor, PagerDom, PagerMotion, PagerWindow,
    PassiveInputListeners, RosterSlot,
};
use crate::services::drag_state::context::use_drag_state;
use crate::services::editor_state::context::use_editor_state;
use crate::services::navigation::context::use_view_navigation;
use crate::services::unit_catalog::context::use_unit_catalog;
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;

// Cards mounted on each side of the card on screen once a swipe settles. A swipe
// moves one card at a time. Quick swipes catch the strip before it comes to
// rest, so the pager may not settle between them: going forward the window
// grows below on the fly (see `LastCardWatch`), going back it has this many
// cards above to swipe through before the next settle re-centres it.
const CARD_WINDOW_BUFFER: usize = 5;
// Cards appended below each time the last mounted card comes near. One at a
// time keeps each mount its own short task: the near-watch re-reports after every
// append, so the window keeps growing card by card while the last one is still in
// reach, instead of mounting a batch in one long frame mid-swipe.
const FORWARD_APPEND_STEP: usize = 1;
// The fewest cards that must stay mounted on either side of the card a swipe
// settles on. Re-centring mounts and drops cards, which is main-thread work, so
// the window is left alone on most swipes and re-centred only when the card nears
// an edge or too many cards are mounted. Three keeps the last mounted card off
// screen during the next swipe, so the fallback growth in `LastCardWatch` never
// has to mount a card under a finger.
const EDGE_MARGIN: usize = 3;
// The pager's card hosts, the elements its motion measures and moves.
const CARD_SELECTOR: &str = ".pager-card-host";
// The most cards mounted at once before a settle trims the window back.
const MAX_MOUNTED_CARDS: usize = 17;

pub(super) struct MobileEditorPresentation {
    pub(super) onmounted: EventHandler<MountedEvent>,
    pub(super) window_unit_ids: Vec<WarcraftObjectId>,
}

pub(super) fn use_mobile_editor() -> MobileEditorPresentation {
    // The pager walks the whole roster — every unit of every race, in canonical
    // race order, melee before campaign — not the active race's filtered listing.
    // A swipe carries from the last unit of one race into the first of the next,
    // and `open_unit` (in the settle) switches the race tab to match.
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
    let dragging_slot = use_drag_state().dragging_slot();
    let footer_hidden = editor.footer_hidden();

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
    // The card the strip last came to rest on, reported by the motion and
    // cleared by the settle below once it has acted on it.
    let rested_anchor = use_signal(|| None::<PagerAnchor>);
    let element_ref = use_hook(|| Rc::new(RefCell::new(None::<web_sys::Element>)));
    let hook_element_ref = element_ref.clone();
    let motion = use_hook(move || {
        PagerMotion::new(
            hook_element_ref,
            window,
            rested_anchor,
            dragging_slot,
            Some(footer_hidden),
            CARD_SELECTOR,
        )
    });
    // Created on mount, once the pager element exists; dropping it disconnects.
    let resize_watch = use_hook(|| Rc::new(RefCell::new(None::<CardResizeWatch>)));
    // The slot the pager last settled on or jumped to, so a navigation that only
    // echoes the pager's own settle is not mistaken for a jump.
    let settled_slot_cell = use_hook(|| Rc::new(Cell::new(Some(opening_slot))));

    // Settles the pager on the card the strip has come to rest on: re-centres the
    // window around it and commits it as navigation. The motion reports a rest
    // only after a swipe, a wheel or a key, never after a jump or a re-centre,
    // so a settle is always the user's own navigation.
    let search_dialog_open = editor.search_dialog_open();
    let settle_unit_ids_memo = unit_ids_memo;
    let mut settle_rested_anchor = rested_anchor;
    let mut settle_window = window;
    let mut settle_pending_anchor = pending_anchor;
    let settle_settled_slot = settled_slot_cell.clone();
    use_effect(move || {
        let Some(anchor) = *settle_rested_anchor.read() else {
            return;
        };
        settle_rested_anchor.set(None);
        let settled_slot = anchor.slot();
        let current_window = *settle_window.peek();
        let unit_ids = settle_unit_ids_memo.peek().clone();
        let roster_len = unit_ids.len();
        settle_settled_slot.set(Some(settled_slot));
        // Re-centre the window on the card the swipe settled on when it has come
        // near an edge of the window, or the window has grown too large, so the
        // next swipe either way lands on a card that is already mounted. The card
        // on screen keeps exactly the offset it has now; the anchor effect
        // places the strip again for the cards added and dropped around it.
        let first_slot = current_window.first_slot();
        let slots_above = settled_slot - first_slot;
        let Ok(mounted_above) = usize::try_from(slots_above) else {
            return;
        };
        let mounted_below = current_window
            .slot_count()
            .saturating_sub(mounted_above)
            .saturating_sub(1);
        let near_an_edge = mounted_above < EDGE_MARGIN || mounted_below < EDGE_MARGIN;
        let too_many_mounted = current_window.slot_count() > MAX_MOUNTED_CARDS;
        if near_an_edge || too_many_mounted {
            let recentered = PagerWindow::centered_on(settled_slot, CARD_WINDOW_BUFFER, roster_len);
            settle_window.set(recentered);
            settle_pending_anchor.set(Some(anchor));
        }
        // The search dialog owns the navigation while it is open, so a settle of
        // the pager underneath must not overwrite the unit it navigated to.
        if *search_dialog_open.peek() {
            return;
        }
        let Some(settled_unit_id) = RosterSlot::unit_index(settled_slot, roster_len)
            .and_then(|unit_index| unit_ids.get(unit_index).copied())
        else {
            return;
        };
        // Only a swipe onto a different unit is navigation. A swipe that springs
        // back onto the unit it started on is already current, so back and
        // forward never get a duplicate history entry.
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
    // keeps holding it: the motion re-applies it whenever a card changes size
    // until a finger takes the strip, so the card lands exactly at its anchor
    // offset however the cards around it settle.
    let anchor_element_ref = element_ref.clone();
    let anchor_window = window;
    let mut anchor_pending = pending_anchor;
    let effect_motion = motion.clone();
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
            watch.observe_cards(element, CARD_SELECTOR);
        }
        drop(borrowed);
        if effect_motion.hold(anchor) {
            anchor_pending.set(None);
        }
    });

    // Grows the window below when the last mounted card comes into view, so the
    // next swipe forward always has a card to snap to, even when quick swipes
    // never let the pager settle in between. Appending below never moves the
    // cards above, so it is safe under a moving finger; the next settle trims the
    // window back once it holds too many cards.
    let lookahead_element_ref = element_ref.clone();
    let lookahead_unit_ids_memo = unit_ids_memo;
    let lookahead_window = window;
    let extend_forward: Rc<dyn Fn()> = Rc::new(move || {
        let borrowed = lookahead_element_ref.borrow();
        let Some(element) = borrowed.as_ref() else {
            return;
        };
        let pager_dom = PagerDom::new(element, CARD_SELECTOR);
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
    let mounted_motion = motion.clone();
    let mounted_resize_watch = resize_watch.clone();
    let mounted_input_listeners = input_listeners.clone();
    let mounted_last_card_watch = last_card_watch.clone();
    let mounted_extend_forward = extend_forward.clone();
    let onmounted = EventHandler::new(move |event: MountedEvent| {
        let Some(element) = event.data().try_as_web_event() else {
            return;
        };
        let resized_motion = mounted_motion.clone();
        let created_watch = CardResizeWatch::new(move || {
            resized_motion.reapply();
        });
        if let Some(watch) = created_watch.as_ref() {
            watch.observe_cards(&element, CARD_SELECTOR);
        }
        *mounted_resize_watch.borrow_mut() = created_watch;
        let mut listeners = PassiveInputListeners::new(&element);
        let pointer_down_motion = mounted_motion.clone();
        listeners.listen("pointerdown", move |event| {
            pointer_down_motion.pointer_down(&event)
        });
        let pointer_move_motion = mounted_motion.clone();
        listeners.listen("pointermove", move |event| {
            pointer_move_motion.pointer_move(&event)
        });
        let pointer_up_motion = mounted_motion.clone();
        listeners.listen("pointerup", move |event| {
            pointer_up_motion.pointer_up(&event)
        });
        let pointer_cancel_motion = mounted_motion.clone();
        listeners.listen("pointercancel", move |event| {
            pointer_cancel_motion.pointer_cancel(&event)
        });
        let wheel_motion = mounted_motion.clone();
        listeners.listen("wheel", move |event| wheel_motion.wheel(&event));
        let key_motion = mounted_motion.clone();
        listeners.listen("keydown", move |event| key_motion.key_down(&event));
        let transition_motion = mounted_motion.clone();
        listeners.listen("transitionend", move |event| {
            transition_motion.transition_end(&event)
        });
        *mounted_input_listeners.borrow_mut() = Some(listeners);
        let near_handler = mounted_extend_forward.clone();
        let created_last_card_watch = LastCardWatch::new(&element, move || near_handler());
        if let Some(watch) = created_last_card_watch.as_ref() {
            watch.watch_last_card(&element, CARD_SELECTOR);
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
            watch.watch_last_card(element, CARD_SELECTOR);
        }
    });

    let unit_ids = unit_ids_memo();
    let window_unit_ids = window.read().unit_ids(&unit_ids);

    MobileEditorPresentation {
        onmounted,
        window_unit_ids,
    }
}
