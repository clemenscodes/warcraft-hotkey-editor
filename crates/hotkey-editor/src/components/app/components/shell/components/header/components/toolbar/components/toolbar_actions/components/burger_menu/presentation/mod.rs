use super::components::burger_drawer::components::burger_drawer_body::components::shared::burger_menu_item::BurgerItemState;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::presentation::{ToolbarActionModel, use_toolbar_actions};
use crate::components::app::components::shell::components::shared::icons::ICON_GRID;
use crate::services::overlay_state::context::use_overlay_state;
use dioxus::prelude::*;

/// A fully-resolved drawer row as plain data: the shared toolbar action shaped into the
/// drawer's look, with the drawer-close baked onto its click. Each leaf reads these fields
/// and names its own children.
#[derive(Clone, PartialEq)]
pub struct BurgerMenuRow {
    pub icon: &'static str,
    pub label: String,
    pub state: BurgerItemState,
    pub disabled: bool,
    pub role: Option<&'static str>,
    pub aria_haspopup: Option<&'static str>,
    pub aria_expanded: Option<&'static str>,
    pub aria_pressed: Option<&'static str>,
    pub aria_label: Option<&'static str>,
    pub onclick: EventHandler<MouseEvent>,
}

/// The already-shaped controller state the body renders: the drawer open flags, the
/// toggle and close handlers, and the drawer rows (the primary Grid Layout row plus the
/// shared file-action rows, each closing the drawer and performing its action).
pub struct BurgerMenuView {
    pub(super) burger_open: Signal<bool>,
    pub(super) toggle: EventHandler<MouseEvent>,
    pub(super) on_close: EventHandler<MouseEvent>,
    pub(super) layout: BurgerMenuRow,
    pub(super) items: Vec<BurgerMenuRow>,
}

/// The drawer's open state and the body-scroll lock it drives.
pub(super) struct BurgerOpen {
    pub(super) burger_open: Signal<bool>,
    pub(super) toggle: EventHandler<MouseEvent>,
    pub(super) close: EventHandler<MouseEvent>,
}

fn use_burger_open() -> BurgerOpen {
    let mut burger_open = use_signal::<bool>(|| false);
    use_effect(move || {
        let is_open = burger_open();
        let Some(body) = web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.body())
        else {
            return;
        };
        let style = body.style();
        if is_open {
            let _ = style.set_property("overflow", "hidden");
            let _ = style.set_property("overscroll-behavior", "contain");
        } else {
            let _ = style.remove_property("overflow");
            let _ = style.remove_property("overscroll-behavior");
        }
    });
    let toggle = EventHandler::new(move |_event: MouseEvent| {
        let next = !*burger_open.read();
        burger_open.set(next);
    });
    let close = EventHandler::new(move |_event: MouseEvent| burger_open.set(false));
    BurgerOpen {
        burger_open,
        toggle,
        close,
    }
}

/// A live boolean rendered as an `aria-*` attribute value, or omitted.
fn aria_flag(value: Option<bool>) -> Option<&'static str> {
    value.map(|flag| if flag { "true" } else { "false" })
}

/// One shared file action shaped into a drawer row: the drawer closes and the action runs
/// on click, toggles render their active weight, and aria state is carried as strings.
fn action_row(action: ToolbarActionModel, burger_open: Signal<bool>) -> BurgerMenuRow {
    let mut close_open = burger_open;
    let base = action.onclick;
    let onclick = EventHandler::new(move |event: MouseEvent| {
        close_open.set(false);
        base.call(event);
    });
    let state = if action.active {
        BurgerItemState::Active
    } else {
        BurgerItemState::Idle
    };
    let aria_expanded = aria_flag(action.expanded);
    let aria_pressed = aria_flag(action.pressed);
    BurgerMenuRow {
        icon: action.icon,
        label: action.label,
        state,
        disabled: action.disabled,
        role: Some("menuitem"),
        aria_haspopup: action.aria_haspopup,
        aria_expanded,
        aria_pressed,
        aria_label: None,
        onclick,
    }
}

/// The primary Grid Layout row — burger-only, since the inline header carries the centered
/// grid-layout button instead. Toggles the layout dialog and closes the drawer.
fn layout_row(
    burger_open: Signal<bool>,
    layout_open: Signal<bool>,
    expanded: bool,
) -> BurgerMenuRow {
    let mut close_open = burger_open;
    let mut toggle_open = layout_open;
    let onclick = EventHandler::new(move |_event: MouseEvent| {
        let next = !*toggle_open.read();
        toggle_open.set(next);
        close_open.set(false);
    });
    let aria_expanded = aria_flag(Some(expanded));
    BurgerMenuRow {
        icon: ICON_GRID,
        label: String::from("Grid Layout"),
        state: BurgerItemState::Primary,
        disabled: false,
        role: None,
        aria_haspopup: Some("dialog"),
        aria_expanded,
        aria_pressed: None,
        aria_label: Some("Edit global hotkey layout"),
        onclick,
    }
}

/// The composed hook: owns the drawer's local open state and builds every drawer row from
/// the shared toolbar-action set (plus the primary Grid Layout row). The body only names
/// the result.
pub fn use_burger_menu() -> BurgerMenuView {
    let drawer = use_burger_open();
    let actions = use_toolbar_actions();
    let overlay = use_overlay_state();
    let layout_open = overlay.layout_dialog_open();
    let layout_expanded = *layout_open.read();
    let burger_open = drawer.burger_open;
    let layout = layout_row(burger_open, layout_open, layout_expanded);
    let items = actions
        .items()
        .into_iter()
        .filter(|action| !action.hidden)
        .map(|action| action_row(action, burger_open))
        .collect();
    let on_close = drawer.close;
    BurgerMenuView {
        burger_open: drawer.burger_open,
        toggle: drawer.toggle,
        on_close,
        layout,
        items,
    }
}
