use super::data::{ARIA_LABEL, LABEL};
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::components::burger_drawer::components::burger_drawer_body::components::shared::burger_menu_item::BurgerItemState;
use crate::components::app::components::shell::components::shared::icons::ICON_GRID;
use dioxus::prelude::*;

pub(super) struct BurgerLayoutItemPresentation {
    pub(super) icon: &'static str,
    pub(super) label: String,
    pub(super) state: BurgerItemState,
    pub(super) aria_haspopup: Option<&'static str>,
    pub(super) aria_expanded: Option<&'static str>,
    pub(super) aria_label: Option<&'static str>,
    pub(super) open: bool,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) on_open_change: Callback<bool>,
}

pub(super) fn use_burger_layout_item() -> BurgerLayoutItemPresentation {
    let mut open_signal = use_signal::<bool>(|| false);
    let open = open_signal();
    let onclick = EventHandler::new(move |_event: MouseEvent| {
        let next = !*open_signal.read();
        open_signal.set(next);
    });
    let on_open_change = Callback::new(move |is_open: bool| open_signal.set(is_open));
    let aria_haspopup = Some("dialog");
    let aria_expanded = if open { Some("true") } else { Some("false") };
    let aria_label = Some(ARIA_LABEL);
    BurgerLayoutItemPresentation {
        icon: ICON_GRID,
        label: String::from(LABEL),
        state: BurgerItemState::Primary,
        aria_haspopup,
        aria_expanded,
        aria_label,
        open,
        onclick,
        on_open_change,
    }
}
