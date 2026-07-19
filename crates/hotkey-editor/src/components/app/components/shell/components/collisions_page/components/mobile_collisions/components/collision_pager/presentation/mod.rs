use super::model::CollisionPagerModel;
use crate::components::app::components::shell::components::collisions_page::components::body::ContentModel;
use crate::services::collision_selection::context::use_collision_selection;
use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use std::cell::RefCell;
use std::rc::Rc;

pub(super) struct CollisionPagerPresentation {
    pub(super) onmounted: EventHandler<MountedEvent>,
    pub(super) onscrollend: EventHandler<ScrollEvent>,
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

    let viewport_px = use_signal::<i32>(|| 0);
    let active_index = use_signal::<usize>(|| 0);
    // Set while the pager scrolls itself to follow a selection made elsewhere, so
    // the scroll it causes is not mistaken for a swipe. See `onscrollend`.
    let following_navigation = use_signal::<bool>(|| false);
    let element_ref = use_hook(|| Rc::new(RefCell::new(None::<web_sys::Element>)));

    let mounted_element_ref = element_ref.clone();
    let mut mounted_viewport_px = viewport_px;
    let onmounted = EventHandler::new(move |event: MountedEvent| {
        let Some(element) = event.data().try_as_web_event() else {
            return;
        };
        let measured_height = element.client_height();
        *mounted_element_ref.borrow_mut() = Some(element);
        if *mounted_viewport_px.peek() != measured_height {
            mounted_viewport_px.set(measured_height);
        }
    });

    let scroll_element_ref = element_ref.clone();
    let scroll_entry_keys = entry_keys.clone();
    let scroll_selected_signal = selected_signal;
    let mut scroll_viewport_px = viewport_px;
    let mut scroll_active_index = active_index;
    let mut scroll_following_navigation = following_navigation;
    let onscrollend = EventHandler::new(move |_event: ScrollEvent| {
        let borrowed = scroll_element_ref.borrow();
        let Some(element) = borrowed.as_ref() else {
            return;
        };
        let measured_height = element.client_height();
        if measured_height <= 0 {
            return;
        }
        let scroll_top = element.scroll_top();
        let rounded_index = (scroll_top + measured_height / 2) / measured_height;
        let last_index = entry_count.saturating_sub(1);
        let clamped_index = usize::try_from(rounded_index).unwrap_or(0).min(last_index);
        if *scroll_viewport_px.peek() != measured_height {
            scroll_viewport_px.set(measured_height);
        }
        // The pager scrolls itself whenever a card is chosen elsewhere, and that
        // scroll settles here too. It must not be read as a swipe, or navigating
        // to a card would immediately overwrite the entry that was asked for.
        if *scroll_following_navigation.peek() {
            scroll_following_navigation.set(false);
            return;
        }
        if *scroll_active_index.peek() != clamped_index {
            scroll_active_index.set(clamped_index);
            // `select_collision_entry` derives the active kind from the current
            // view itself and replaces the navigation snapshot, so swiping keeps
            // the `entry` query parameter in step with the card on screen. The
            // guard against the already selected key stops a redundant replace.
            if let Some(key) = scroll_entry_keys.get(clamped_index).cloned() {
                let already_selected = *scroll_selected_signal.peek() == Some(key.clone());
                if !already_selected {
                    view_navigation.select_collision_entry(key);
                }
            }
        }
    });

    // The other direction: a card chosen anywhere but the pager itself, above all
    // by switching the kind or by browser back and forward, has to bring itself on
    // screen. The card the pager follows is the selected entry when it survives the
    // active kind, and the first entry otherwise, which keeps the active index
    // inside the list rather than dangling past a shorter one.
    let effect_entry_keys = entry_keys.clone();
    let selected_key = selected_signal.read().clone();
    let current_viewport_px = *viewport_px.read();
    let effect_element_ref = element_ref.clone();
    let mut effect_active_index = active_index;
    let mut effect_following_navigation = following_navigation;
    use_effect(use_reactive!(|(
        effect_entry_keys,
        selected_key,
        current_viewport_px,
    )| {
        let _ = current_viewport_px;
        if effect_entry_keys.is_empty() {
            return;
        }
        let matched_index = selected_key
            .as_ref()
            .and_then(|key| effect_entry_keys.iter().position(|entry| entry == key));
        let target_index = matched_index.unwrap_or(0);
        if *effect_active_index.peek() == target_index {
            return;
        }
        let borrowed = effect_element_ref.borrow();
        let Some(element) = borrowed.as_ref() else {
            return;
        };
        let measured_height = element.client_height();
        if measured_height <= 0 {
            return;
        }
        effect_active_index.set(target_index);
        effect_following_navigation.set(true);
        // Scrolling has to wait for the cards this index change re-renders. The
        // card at `target_index` only sits at `target_index * height` once the
        // new content has laid out, so scrolling in this same tick measures the
        // old layout and lands elsewhere. Deferring one tick lets it settle.
        let scroll_element = element.clone();
        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(0).await;
            let laid_out_height = scroll_element.client_height();
            if laid_out_height <= 0 {
                return;
            }
            let target_index_px = i32::try_from(target_index).unwrap_or(0);
            let target_scroll_top = target_index_px * laid_out_height;
            scroll_element.set_scroll_top(target_scroll_top);
        });
    }));

    CollisionPagerPresentation {
        onmounted,
        onscrollend,
        content,
    }
}
