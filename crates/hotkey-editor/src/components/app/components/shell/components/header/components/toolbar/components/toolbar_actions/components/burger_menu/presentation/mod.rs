use super::components::burger_drawer::components::burger_drawer_body::components::shared::burger_menu_item::BurgerItemState;
use super::data::{self, BurgerRowContent};
use super::state::BurgerAction;
use crate::services::navigation::app_view::AppView;
use crate::services::navigation::context::use_view_navigation;
use crate::services::overlay_state::context::use_overlay_state;
use crate::services::undo::context::use_undo_history;
use dioxus::prelude::*;

/// A fully-resolved drawer row as plain data: the static content merged with the
/// live signal state and the wired click handler. It carries no component props —
/// each leaf reads these fields and names its own children. This is what the
/// builder produces and what the drawer subtree threads down.
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

/// The live, signal-derived inputs every row resolves against: undo/redo
/// availability, which toggles are currently active, and which dialogs are open.
/// Read once by the actions hook and handed to the builder.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub(super) struct RowDynamics {
    pub(super) can_undo: bool,
    pub(super) can_redo: bool,
    pub(super) system_hotkeys_active: bool,
    pub(super) preview_active: bool,
    pub(super) layout_expanded: bool,
    pub(super) templates_expanded: bool,
    pub(super) help_expanded: bool,
}

/// Every drawer action's click handler, wired once by the actions hook. The
/// builder selects one per row by its [`BurgerAction`].
#[derive(Clone, Copy)]
pub(super) struct BurgerActionHandlers {
    pub(super) toggle_layout: EventHandler<MouseEvent>,
    pub(super) trigger_undo: EventHandler<MouseEvent>,
    pub(super) trigger_redo: EventHandler<MouseEvent>,
    pub(super) open_upload: EventHandler<MouseEvent>,
    pub(super) toggle_templates: EventHandler<MouseEvent>,
    pub(super) toggle_system_hotkeys: EventHandler<MouseEvent>,
    pub(super) toggle_preview: EventHandler<MouseEvent>,
    pub(super) open_resolve: EventHandler<MouseEvent>,
    pub(super) open_download: EventHandler<MouseEvent>,
    pub(super) open_help: EventHandler<MouseEvent>,
}

impl BurgerActionHandlers {
    pub(super) fn onclick_for(&self, action: BurgerAction) -> EventHandler<MouseEvent> {
        match action {
            BurgerAction::Layout => self.toggle_layout,
            BurgerAction::Undo => self.trigger_undo,
            BurgerAction::Redo => self.trigger_redo,
            BurgerAction::Upload => self.open_upload,
            BurgerAction::Templates => self.toggle_templates,
            BurgerAction::SystemHotkeys => self.toggle_system_hotkeys,
            BurgerAction::Preview => self.toggle_preview,
            BurgerAction::Resolve => self.open_resolve,
            BurgerAction::Download => self.open_download,
            BurgerAction::Help => self.open_help,
        }
    }
}

/// Resolves each drawer row: it pairs the static [`BurgerRowContent`] with the
/// live [`RowDynamics`] and the wired [`BurgerActionHandlers`], producing a
/// finished [`BurgerMenuRow`]. This replaces the ten hand-built row literals with
/// one construction site driven by the `data.rs` table.
pub(super) struct MenuRowBuilder {
    pub(super) dynamics: RowDynamics,
    pub(super) handlers: BurgerActionHandlers,
}

impl MenuRowBuilder {
    /// The primary Grid Layout row.
    pub(super) fn layout(&self) -> BurgerMenuRow {
        self.row(&data::LAYOUT_ROW)
    }

    /// The grouped file-action rows, in render order.
    pub(super) fn items(&self) -> Vec<BurgerMenuRow> {
        data::ITEM_ROWS
            .iter()
            .map(|content| self.row(content))
            .collect()
    }

    fn row(&self, content: &BurgerRowContent) -> BurgerMenuRow {
        let action = content.action;
        let icon = content.icon;
        let role = content.role;
        let aria_haspopup = content.aria_haspopup;
        let aria_label = content.aria_label;
        let label = self.label(action, content.label);
        let state = self.weight(action);
        let disabled = self.disabled(action);
        let aria_expanded = self.aria_expanded(action);
        let aria_pressed = self.aria_pressed(action);
        let onclick = self.handlers.onclick_for(action);
        BurgerMenuRow {
            icon,
            label,
            state,
            disabled,
            role,
            aria_haspopup,
            aria_expanded,
            aria_pressed,
            aria_label,
            onclick,
        }
    }

    fn disabled(&self, action: BurgerAction) -> bool {
        match action {
            BurgerAction::Undo => !self.dynamics.can_undo,
            BurgerAction::Redo => !self.dynamics.can_redo,
            _ => false,
        }
    }

    fn weight(&self, action: BurgerAction) -> BurgerItemState {
        match action {
            BurgerAction::Layout => BurgerItemState::Primary,
            BurgerAction::SystemHotkeys => item_state(self.dynamics.system_hotkeys_active),
            BurgerAction::Preview => item_state(self.dynamics.preview_active),
            _ => BurgerItemState::Idle,
        }
    }

    fn label(&self, action: BurgerAction, content_label: &'static str) -> String {
        match action {
            BurgerAction::Preview if self.dynamics.preview_active => String::from("Hide Preview"),
            BurgerAction::Preview => String::from("Preview"),
            _ => String::from(content_label),
        }
    }

    fn aria_expanded(&self, action: BurgerAction) -> Option<&'static str> {
        match action {
            BurgerAction::Layout => aria_flag(self.dynamics.layout_expanded),
            BurgerAction::Templates => aria_flag(self.dynamics.templates_expanded),
            BurgerAction::SystemHotkeys => aria_flag(self.dynamics.system_hotkeys_active),
            BurgerAction::Help => aria_flag(self.dynamics.help_expanded),
            _ => None,
        }
    }

    fn aria_pressed(&self, action: BurgerAction) -> Option<&'static str> {
        match action {
            BurgerAction::Preview => aria_flag(self.dynamics.preview_active),
            _ => None,
        }
    }
}

/// A live boolean rendered as an `aria-*` attribute value, or omitted when the
/// row does not carry that attribute.
fn aria_flag(value: bool) -> Option<&'static str> {
    if value { Some("true") } else { Some("false") }
}

/// A toggle row is styled active while its target (dialog/preview) is open.
fn item_state(active: bool) -> BurgerItemState {
    if active {
        BurgerItemState::Active
    } else {
        BurgerItemState::Idle
    }
}

/// The already-shaped controller state the body renders: the drawer open flags,
/// the toggle and close handlers, and the fully-built drawer rows (the primary
/// Grid Layout row plus the grouped action rows, each with its handler and state).
pub struct BurgerMenuView {
    pub(super) burger_open: Signal<bool>,
    pub(super) upload_info_open: Signal<bool>,
    pub(super) download_info_open: Signal<bool>,
    pub(super) toggle: EventHandler<MouseEvent>,
    pub(super) on_close: EventHandler<MouseEvent>,
    pub(super) layout: BurgerMenuRow,
    pub(super) items: Vec<BurgerMenuRow>,
}

/// The drawer's open state and the body-scroll lock it drives: owns the
/// `burger_open` signal, the effect that pins the page while the drawer is open,
/// and the toggle/close handlers.
pub(super) struct BurgerOpen {
    pub(super) burger_open: Signal<bool>,
    pub(super) toggle: EventHandler<MouseEvent>,
    pub(super) close: EventHandler<MouseEvent>,
}

/// The action rows' live state, their own info-dialog open signals, and every
/// row handler — reading the undo history, navigation, and overlay dialogs once
/// and wiring each row to close the drawer plus perform its action.
pub(super) struct BurgerActions {
    pub(super) upload_info_open: Signal<bool>,
    pub(super) download_info_open: Signal<bool>,
    pub(super) dynamics: RowDynamics,
    pub(super) handlers: BurgerActionHandlers,
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

fn use_burger_actions(burger_open: Signal<bool>) -> BurgerActions {
    let navigation = use_view_navigation();
    let overlay = use_overlay_state();
    let history = use_undo_history();
    let mut burger_open = burger_open;
    let mut system_hotkeys_open = overlay.system_hotkeys_open();
    let mut help_open = overlay.help_open();
    let mut layout_dialog_open = overlay.layout_dialog_open();
    let mut templates_dialog_open = overlay.templates_dialog_open();
    let mut preview_open = overlay.preview_open();
    let mut upload_info_open = use_signal::<bool>(|| false);
    let mut download_info_open = use_signal::<bool>(|| false);

    let preview_active = preview_open();
    let system_hotkeys_active = system_hotkeys_open();
    let layout_expanded = layout_dialog_open();
    let templates_expanded = templates_dialog_open();
    let help_expanded = help_open();
    let can_undo = history.can_undo();
    let can_redo = history.can_redo();

    let toggle_layout = EventHandler::new(move |_event: MouseEvent| {
        let next = !*layout_dialog_open.read();
        layout_dialog_open.set(next);
        burger_open.set(false);
    });
    let open_upload = EventHandler::new(move |_event: MouseEvent| {
        upload_info_open.set(true);
        burger_open.set(false);
    });
    let toggle_templates = EventHandler::new(move |_event: MouseEvent| {
        let next = !*templates_dialog_open.read();
        templates_dialog_open.set(next);
        burger_open.set(false);
    });
    let toggle_system_hotkeys = EventHandler::new(move |_event: MouseEvent| {
        let next = !*system_hotkeys_open.read();
        system_hotkeys_open.set(next);
        burger_open.set(false);
    });
    let open_help = EventHandler::new(move |_event: MouseEvent| {
        help_open.set(true);
        burger_open.set(false);
    });
    let toggle_preview = EventHandler::new(move |_event: MouseEvent| {
        let next = !*preview_open.read();
        preview_open.set(next);
        burger_open.set(false);
    });
    let open_download = EventHandler::new(move |_event: MouseEvent| {
        download_info_open.set(true);
        burger_open.set(false);
    });
    let open_resolve = EventHandler::new(move |_event: MouseEvent| {
        burger_open.set(false);
        navigation.apply(AppView::Resolve);
    });
    let trigger_undo = EventHandler::new(move |_event: MouseEvent| {
        burger_open.set(false);
        history.undo();
    });
    let trigger_redo = EventHandler::new(move |_event: MouseEvent| {
        burger_open.set(false);
        history.redo();
    });

    let dynamics = RowDynamics {
        can_undo,
        can_redo,
        system_hotkeys_active,
        preview_active,
        layout_expanded,
        templates_expanded,
        help_expanded,
    };
    let handlers = BurgerActionHandlers {
        toggle_layout,
        trigger_undo,
        trigger_redo,
        open_upload,
        toggle_templates,
        toggle_system_hotkeys,
        toggle_preview,
        open_resolve,
        open_download,
        open_help,
    };
    BurgerActions {
        upload_info_open,
        download_info_open,
        dynamics,
        handlers,
    }
}

/// The composed hook: owns the drawer's local open state, reads the undo history
/// and the live config, and wires every row's handler and state. The body only
/// names the result.
pub fn use_burger_menu() -> BurgerMenuView {
    let drawer = use_burger_open();
    let actions = use_burger_actions(drawer.burger_open);
    let builder = MenuRowBuilder {
        dynamics: actions.dynamics,
        handlers: actions.handlers,
    };
    let layout = builder.layout();
    let items = builder.items();
    let on_close = drawer.close;
    BurgerMenuView {
        burger_open: drawer.burger_open,
        upload_info_open: actions.upload_info_open,
        download_info_open: actions.download_info_open,
        toggle: drawer.toggle,
        on_close,
        layout,
        items,
    }
}
