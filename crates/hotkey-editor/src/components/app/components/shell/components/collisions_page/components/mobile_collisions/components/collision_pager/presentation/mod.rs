use super::model::CollisionPagerModel;
use crate::components::app::components::shell::components::collisions_page::components::body::ContentModel;
use crate::components::app::components::shell::components::shared::spring_pager::{
    CardResizeWatch, PagerAnchor, PagerMotion, PagerWindow, PassiveInputListeners,
};
use crate::services::collision_selection::context::use_collision_selection;
use crate::services::drag_state::context::use_drag_state;
use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use std::cell::RefCell;
use std::rc::Rc;

// The card hosts of every kind, the elements the pager's motion measures and moves.
const CARD_SELECTOR: &str =
    ".island-pager-card-host, .hotkey-pager-card-host, .unit-position-pager-card-host";

pub(super) struct CollisionPagerPresentation {
    pub(super) onmounted: EventHandler<MountedEvent>,
    pub(super) content: ContentModel,
}

pub(super) fn use_collision_pager(props: &CollisionPagerModel) -> CollisionPagerPresentation {
    let content = props.content.clone();
    // Every card carries the key its kind selects on. The keys are gathered in the
    // order the cards render so the scroll position maps straight to an entry.
    let entry_keys: Vec<String> = match &content {
        ContentModel::Positions(pane) => pane
            .islands()
            .iter()
            .map(|island| island.key().to_owned())
            .collect(),
        ContentModel::Hotkeys(pane) => pane
            .units()
            .iter()
            .map(|unit| unit.key().to_owned())
            .collect(),
        ContentModel::UnitPositions(pane) => pane
            .units()
            .iter()
            .map(|unit| unit.key().to_owned())
            .collect(),
        ContentModel::Empty(_) | ContentModel::Clear => Vec::new(),
    };
    let entry_count = entry_keys.len();

    // The pager reads and writes the selection of whichever kind is on screen.
    // The active kind is the one the current content carries, so the signal is
    // chosen from the content variant rather than from a separate flag.
    let selection = use_collision_selection();
    let selected_signal = match content {
        ContentModel::Positions(_) => selection.selected_island(),
        ContentModel::Hotkeys(_) => selection.selected_hotkey_unit(),
        ContentModel::UnitPositions(_) => selection.selected_unit_position(),
        ContentModel::Empty(_) | ContentModel::Clear => selection.selected_island(),
    };

    let view_navigation = use_view_navigation();

    // Every card stays mounted, so the window always spans the whole list and a
    // slot is the entry's own index. It follows the list whenever its length changes.
    let center_slot = isize::try_from(entry_count / 2).unwrap_or(0);
    let window = use_signal(|| PagerWindow::centered_on(center_slot, entry_count, entry_count));
    // The card the strip last came to rest on, reported by the motion after a
    // swipe, a wheel or a key and cleared by the settle below once acted on.
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

    // Commits the card the strip has come to rest on as the selected entry. The
    // motion reports a rest only after the user's own input, never after the
    // pager follows a selection made elsewhere, so a settle is always a swipe.
    let settle_entry_keys = entry_keys.clone();
    let mut settle_rested_anchor = rested_anchor;
    use_effect(move || {
        let Some(anchor) = *settle_rested_anchor.read() else {
            return;
        };
        settle_rested_anchor.set(None);
        let Some(key) = usize::try_from(anchor.slot())
            .ok()
            .and_then(|entry_index| settle_entry_keys.get(entry_index))
            .cloned()
        else {
            return;
        };
        // `select_collision_entry` derives the active kind from the current view
        // itself and replaces the navigation snapshot, so swiping keeps the
        // `entry` query parameter in step with the card on screen. The guard
        // against the already selected key stops a redundant replace.
        let already_selected = *selected_signal.peek() == Some(key.clone());
        if !already_selected {
            view_navigation.select_collision_entry(key);
        }
    });

    // The other direction: a card chosen anywhere but the pager itself, above all
    // by switching the kind or by browser back and forward, has to bring itself on
    // screen. The card the pager follows is the selected entry when it survives the
    // active kind, and the first entry otherwise. Holding the card a swipe just
    // settled on again is harmless, the strip already rests there.
    let effect_entry_keys = entry_keys.clone();
    let selected_key = selected_signal.read().clone();
    let effect_element_ref = element_ref.clone();
    let effect_motion = motion.clone();
    let effect_resize_watch = resize_watch.clone();
    let mut effect_window = window;
    use_effect(use_reactive!(|(effect_entry_keys, selected_key)| {
        let entry_count = effect_entry_keys.len();
        if entry_count == 0 {
            return;
        }
        let center_slot = isize::try_from(entry_count / 2).unwrap_or(0);
        let spanning_window = PagerWindow::centered_on(center_slot, entry_count, entry_count);
        if *effect_window.peek() != spanning_window {
            effect_window.set(spanning_window);
        }
        let matched_index = selected_key
            .as_ref()
            .and_then(|key| effect_entry_keys.iter().position(|entry| entry == key))
            .unwrap_or(0);
        let target_slot = isize::try_from(matched_index).unwrap_or(0);
        let borrowed = effect_element_ref.borrow();
        let Some(element) = borrowed.as_ref() else {
            return;
        };
        // The cards of a new kind replace the old ones, so the watch moves to them.
        if let Some(watch) = effect_resize_watch.borrow().as_ref() {
            watch.observe_cards(element, CARD_SELECTOR);
        }
        drop(borrowed);
        // If the cards are not laid out yet, the resize watch applies the held
        // anchor once they are.
        let target_card = PagerAnchor::aligned(target_slot);
        effect_motion.hold(target_card);
    }));

    let mounted_element_ref = element_ref.clone();
    let mounted_entry_keys = entry_keys.clone();
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
        // The follow effect ran before the pager existed, so the selected card is
        // placed here the first time.
        let matched_index = selected_signal
            .peek()
            .as_ref()
            .and_then(|key| mounted_entry_keys.iter().position(|entry| entry == key))
            .unwrap_or(0);
        let target_slot = isize::try_from(matched_index).unwrap_or(0);
        let target_card = PagerAnchor::aligned(target_slot);
        motion.hold(target_card);
    });

    CollisionPagerPresentation { onmounted, content }
}
