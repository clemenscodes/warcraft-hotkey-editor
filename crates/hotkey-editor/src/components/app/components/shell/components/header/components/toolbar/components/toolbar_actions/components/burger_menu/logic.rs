use dioxus::prelude::*;

use super::components::burger_drawer::components::burger_drawer_body::components::shared::burger_menu_item::BurgerItemState;
use super::data::{self, BurgerRowContent};
use super::state::BurgerAction;

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
