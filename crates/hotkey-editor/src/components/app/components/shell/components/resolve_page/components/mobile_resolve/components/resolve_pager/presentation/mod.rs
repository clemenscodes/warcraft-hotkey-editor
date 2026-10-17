use super::model::ResolvePagerModel;
use crate::components::app::components::shell::components::shared::spring_pager::{
    CardResizeWatch, PagerAnchor, PagerMotion, PagerWindow, PassiveInputListeners,
};
use crate::services::drag_state::context::use_drag_state;
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

// The card hosts of both kinds, the elements the pager's motion measures and moves.
const CARD_SELECTOR: &str = ".move-pager-card-host, .unresolved-pager-card-host";

pub(super) struct ResolvePagerPresentation {
    pub(super) onmounted: EventHandler<MountedEvent>,
}

impl ddd::Layered for ResolvePagerPresentation {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Presentation for ResolvePagerPresentation {
    type Model = ResolvePagerModel;
}

/// Pages through the moves and the unresolved abilities the way the editor
/// pages through units: one card per swipe, along the same spring. Every card
/// stays mounted, so the window always spans the whole list and a slot is the
/// card's own index.
pub(super) fn use_resolve_pager(props: &ResolvePagerModel) -> ResolvePagerPresentation {
    let move_count = props
        .section
        .as_ref()
        .map(|section| section.moves().len())
        .unwrap_or(0);
    let card_count = move_count.saturating_add(props.unresolved.len());
    let center_slot = isize::try_from(card_count / 2).unwrap_or(0);
    let window = use_signal(|| PagerWindow::centered_on(center_slot, card_count, card_count));
    let rested_anchor = use_signal(|| None::<PagerAnchor>);
    let dragging_slot = use_drag_state().dragging_slot();
    let element_ref = use_hook(|| Rc::new(RefCell::new(None::<web_sys::Element>)));
    let hook_element_ref = element_ref.clone();
    let motion = use_hook(move || {
        PagerMotion::new(
            hook_element_ref,
            window,
            rested_anchor,
            dragging_slot,
            None,
            CARD_SELECTOR,
        )
    });
    // Created on mount, once the pager exists; dropping them disconnects.
    let resize_watch = use_hook(|| Rc::new(RefCell::new(None::<CardResizeWatch>)));
    let input_listeners = use_hook(|| Rc::new(RefCell::new(None::<PassiveInputListeners>)));
    // The card the strip last came to rest on, so a list that changes under it
    // keeps the pager on the same place, or on the last card once it shrinks past.
    let rested_slot_cell = use_hook(|| Rc::new(Cell::<isize>::new(0)));

    let settle_rested_slot = rested_slot_cell.clone();
    let mut settle_rested_anchor = rested_anchor;
    use_effect(move || {
        let Some(anchor) = *settle_rested_anchor.read() else {
            return;
        };
        settle_rested_anchor.set(None);
        settle_rested_slot.set(anchor.slot());
    });

    // Resolving a card removes it from the list, so the window follows the new
    // length and the strip is held on the card that took its place.
    let effect_element_ref = element_ref.clone();
    let effect_motion = motion.clone();
    let effect_resize_watch = resize_watch.clone();
    let effect_rested_slot = rested_slot_cell.clone();
    let mut effect_window = window;
    use_effect(use_reactive!(|card_count| {
        if card_count == 0 {
            return;
        }
        let center_slot = isize::try_from(card_count / 2).unwrap_or(0);
        let spanning_window = PagerWindow::centered_on(center_slot, card_count, card_count);
        if *effect_window.peek() == spanning_window {
            return;
        }
        effect_window.set(spanning_window);
        let borrowed = effect_element_ref.borrow();
        let Some(element) = borrowed.as_ref() else {
            return;
        };
        if let Some(watch) = effect_resize_watch.borrow().as_ref() {
            watch.observe_cards(element, CARD_SELECTOR);
        }
        drop(borrowed);
        let last_slot = isize::try_from(card_count - 1).unwrap_or(0);
        let kept_slot = effect_rested_slot.get().min(last_slot);
        effect_rested_slot.set(kept_slot);
        let kept_card = PagerAnchor::aligned(kept_slot);
        effect_motion.hold(kept_card);
    }));

    let mounted_element_ref = element_ref.clone();
    let onmounted = EventHandler::new(move |event: MountedEvent| {
        let Some(element) = event.data().try_as_web_event() else {
            return;
        };
        *mounted_element_ref.borrow_mut() = Some(element.clone());
        let resized_motion = motion.clone();
        let created_watch = CardResizeWatch::new(move || {
            resized_motion.reapply();
        });
        if let Some(watch) = created_watch.as_ref() {
            watch.observe_cards(&element, CARD_SELECTOR);
        }
        *resize_watch.borrow_mut() = created_watch;
        let mut listeners = PassiveInputListeners::new(&element);
        let pointer_down_motion = motion.clone();
        listeners.listen("pointerdown", move |event| {
            pointer_down_motion.pointer_down(&event)
        });
        let pointer_move_motion = motion.clone();
        listeners.listen("pointermove", move |event| {
            pointer_move_motion.pointer_move(&event)
        });
        let pointer_up_motion = motion.clone();
        listeners.listen("pointerup", move |event| {
            pointer_up_motion.pointer_up(&event)
        });
        let pointer_cancel_motion = motion.clone();
        listeners.listen("pointercancel", move |event| {
            pointer_cancel_motion.pointer_cancel(&event)
        });
        let wheel_motion = motion.clone();
        listeners.listen("wheel", move |event| wheel_motion.wheel(&event));
        let key_motion = motion.clone();
        listeners.listen("keydown", move |event| key_motion.key_down(&event));
        let transition_motion = motion.clone();
        listeners.listen("transitionend", move |event| {
            transition_motion.transition_end(&event)
        });
        *input_listeners.borrow_mut() = Some(listeners);
        // The first card rests at the top. If the cards are not laid out yet,
        // the resize watch applies the held anchor once they are.
        let first_slot = 0;
        let first_card = PagerAnchor::aligned(first_slot);
        motion.hold(first_card);
    });
    ResolvePagerPresentation { onmounted }
}
