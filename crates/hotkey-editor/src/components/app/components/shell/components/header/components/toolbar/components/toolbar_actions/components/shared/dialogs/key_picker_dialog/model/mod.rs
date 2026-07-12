use super::state::KeyPickerCell;
use super::view::KeyPickerDialogView;
use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

/// The key picker's private internal model — the props the component receives. Mirrors
/// the published [`KeyPickerDialogView`] field-for-field (decoupling, not duplication); the
/// `From<&View>` is the boundary translation. Signal-free: the board shaping and the
/// open-signal live in the presentation builder.
#[derive(Props, Clone, PartialEq)]
pub struct KeyPickerDialogModel {
    #[props(into)]
    pub title: String,
    pub rows: Vec<Vec<KeyPickerCell>>,
    pub open: bool,
    #[props(default = false)]
    pub allow_conflict_pick: bool,
    pub on_pick: EventHandler<HotkeyToken>,
    pub on_close: EventHandler<()>,
}

impl From<&KeyPickerDialogView> for KeyPickerDialogModel {
    fn from(view: &KeyPickerDialogView) -> Self {
        let KeyPickerDialogView {
            title,
            rows,
            open,
            allow_conflict_pick,
            on_pick,
            on_close,
        } = view.clone();
        Self {
            title,
            rows,
            open,
            allow_conflict_pick,
            on_pick,
            on_close,
        }
    }
}

impl ddd::Model for KeyPickerDialogModel {
    type View = KeyPickerDialogView;
}
