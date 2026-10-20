use super::model::ResolvePagerModel;
use crate::components::app::components::shell::components::shared::footer_intent::FooterIntent;
use crate::components::app::components::shell::components::shared::snap_pager::{
    PagerCallbacks, SnapPager,
};
use crate::services::editor_state::context::use_editor_state;
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

// The card hosts of both kinds, the cards the pager snaps onto.
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

/// Pages through the moves and the unresolved abilities, one card per swipe,
/// the browser snapping onto each.
pub(super) fn use_resolve_pager(props: &ResolvePagerModel) -> ResolvePagerPresentation {
    let move_count = props
        .section
        .as_ref()
        .map(|section| section.moves().len())
        .unwrap_or(0);
    let card_count = move_count.saturating_add(props.unresolved.len());
    // Created on mount, once the pager exists; dropping it removes its listeners.
    let snap_pager = use_hook(|| Rc::new(RefCell::new(None::<SnapPager>)));
    // The callbacks can hold this cell themselves, so the pager is taken out on
    // unmount instead of waiting for the cell to be dropped.
    let dropped_snap_pager = snap_pager.clone();
    use_drop(move || {
        dropped_snap_pager.borrow_mut().take();
    });
    // The card the scrolling last came to rest on, so a list that changes under it
    // keeps the pager on the same place, or on the last card once it shrinks past.
    let rested_card_cell = use_hook(|| Rc::new(Cell::<usize>::new(0)));

    // Resolving a card removes it from the list, so the pager stays on the card
    // that took its place.
    let effect_snap_pager = snap_pager.clone();
    let effect_rested_card = rested_card_cell.clone();
    use_effect(use_reactive!(|card_count| {
        let Some(last_card) = card_count.checked_sub(1) else {
            return;
        };
        let kept_card = effect_rested_card.get().min(last_card);
        effect_rested_card.set(kept_card);
        if let Some(pager) = effect_snap_pager.borrow().as_ref() {
            pager.place(kept_card, 0.0);
        }
    }));

    // Scrolling on to later cards tucks the footer away for good; scrolling
    // back leaves it tucked away.
    let footer_hidden = use_editor_state().footer_hidden();
    let footer_intent = use_hook(|| Rc::new(Cell::new(FooterIntent::default())));
    let onmounted = EventHandler::new(move |event: MountedEvent| {
        let Some(element) = event.data().try_as_web_event() else {
            return;
        };
        let settled_card = rested_card_cell.clone();
        let scroll_footer_intent = footer_intent.clone();
        let callbacks = PagerCallbacks::new(
            move |card_index| settled_card.set(card_index),
            move |moved_distance| {
                let mut footer_hidden_signal = footer_hidden;
                let mut intent = scroll_footer_intent.get();
                let decision = intent.observe(moved_distance);
                scroll_footer_intent.set(intent);
                let Some(hidden) = decision else {
                    return;
                };
                if *footer_hidden_signal.peek() != hidden {
                    footer_hidden_signal.set(hidden);
                }
            },
            |_edge| {},
        );
        let pager = SnapPager::attach(element, CARD_SELECTOR, callbacks);
        *snap_pager.borrow_mut() = Some(pager);
    });
    ResolvePagerPresentation { onmounted }
}
