use super::model::CollisionPagerModel;
use crate::components::app::components::shell::components::collisions_page::components::body::ContentModel;
use crate::components::app::components::shell::components::shared::footer_intent::FooterIntent;
use crate::components::app::components::shell::components::shared::snap_pager::{
    PagerCallbacks, SnapPager,
};
use crate::services::collision_selection::context::use_collision_selection;
use crate::services::editor_state::context::use_editor_state;
use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

// The card hosts of every kind, the cards the pager scrolls through.
const CARD_SELECTOR: &str =
    ".island-pager-card-host, .hotkey-pager-card-host, .unit-position-pager-card-host";

pub(super) struct CollisionPagerPresentation {
    pub(super) onmounted: EventHandler<MountedEvent>,
    pub(super) content: ContentModel,
}

pub(super) fn use_collision_pager(props: &CollisionPagerModel) -> CollisionPagerPresentation {
    let content = props.content.clone();
    // Every card carries the key its kind selects on. The keys are gathered in the
    // order the cards render so a card's index maps straight to an entry.
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
    // Created on mount, once the pager exists; dropping it removes its listeners.
    let snap_pager = use_hook(|| Rc::new(RefCell::new(None::<SnapPager>)));
    // The callbacks can hold this cell themselves, so the pager is taken out on
    // unmount instead of waiting for the cell to be dropped.
    let dropped_snap_pager = snap_pager.clone();
    use_drop(move || {
        dropped_snap_pager.borrow_mut().take();
    });
    // The card the scrolling last came to rest on, cleared by the settle below
    // once it has acted on it.
    let rested_card = use_signal(|| None::<usize>);
    // The entry the pager itself last selected by coming to rest on it, so the
    // follow effect below tells that echo apart from a selection made elsewhere.
    let settled_key = use_hook(|| Rc::new(RefCell::new(None::<String>)));

    // Commits the card the scrolling has come to rest on as the selected entry.
    let settle_entry_keys = entry_keys.clone();
    let mut settle_rested_card = rested_card;
    let settle_settled_key = settled_key.clone();
    use_effect(move || {
        let Some(card_index) = *settle_rested_card.read() else {
            return;
        };
        settle_rested_card.set(None);
        let Some(key) = settle_entry_keys.get(card_index).cloned() else {
            return;
        };
        // `select_collision_entry` derives the active kind from the current view
        // itself and replaces the navigation snapshot, so scrolling keeps the
        // `entry` query parameter in step with the card on screen. The guard
        // against the already selected key stops a redundant replace.
        let already_selected = *selected_signal.peek() == Some(key.clone());
        *settle_settled_key.borrow_mut() = Some(key.clone());
        if !already_selected {
            view_navigation.select_collision_entry(key);
        }
    });

    // The other direction: a card chosen anywhere but the pager itself, above all
    // by switching the kind or by browser back and forward, brings itself on
    // screen. The card the pager follows is the selected entry when it survives the
    // active kind, and the first entry otherwise. The entry the scrolling itself
    // just rested on is left where the scrolling put it: the list scrolls freely,
    // so placing it again would move the list.
    let effect_entry_keys = entry_keys.clone();
    let selected_key = selected_signal.read().clone();
    let effect_snap_pager = snap_pager.clone();
    let effect_settled_key = settled_key.clone();
    use_effect(use_reactive!(|(effect_entry_keys, selected_key)| {
        let echoes_settle = selected_key.is_some() && *effect_settled_key.borrow() == selected_key;
        if echoes_settle {
            return;
        }
        let matched_index = selected_key
            .as_ref()
            .and_then(|key| effect_entry_keys.iter().position(|entry| entry == key))
            .unwrap_or(0);
        if let Some(pager) = effect_snap_pager.borrow().as_ref() {
            pager.place(matched_index, 0.0);
        }
    }));

    let mounted_entry_keys = entry_keys;
    // Scrolling on to later cards tucks the footer away, scrolling back brings
    // it up again.
    let footer_hidden = use_editor_state().footer_hidden();
    let footer_intent = use_hook(|| Rc::new(Cell::new(FooterIntent::default())));
    let onmounted = EventHandler::new(move |event: MountedEvent| {
        let Some(element) = event.data().try_as_web_event() else {
            return;
        };
        let scroll_footer_intent = footer_intent.clone();
        let callbacks = PagerCallbacks::new(
            move |card_index| {
                let mut settled_card = rested_card;
                settled_card.set(Some(card_index));
            },
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
        // The follow effect ran before the pager existed, so the selected card is
        // aligned here the first time.
        let matched_index = selected_signal
            .peek()
            .as_ref()
            .and_then(|key| mounted_entry_keys.iter().position(|entry| entry == key))
            .unwrap_or(0);
        pager.place(matched_index, 0.0);
        *snap_pager.borrow_mut() = Some(pager);
    });

    CollisionPagerPresentation { onmounted, content }
}
