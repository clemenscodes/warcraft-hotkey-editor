mod props;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_panel::components::system_hotkeys_dialog_body::components::shared::system_slot_key::SystemSlotKey;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_panel::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::shared::system_slot_label::SystemSlotLabel;
use crate::components::app::components::shell::components::shared::tooltip::Tooltip;
use dioxus::prelude::*;
pub use props::IdleSlotProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(IdleSlot);

/// The idle (unglowed) look of a system slot: the gold border-image frame composing
/// the caption, bound key, and conflict tooltip. Presentational — the dispatcher
/// builds its props and renders it when the slot's glow state is idle.
#[component]
pub fn IdleSlot(props: IdleSlotProps) -> Element {
    let label = props.label;
    let slot_key = props.slot_key;
    let tooltip = props.tooltip;
    rsx! {
        div {
            class: CLASS,
            "data-compact": props.compact,
            "data-dragging": props.dragging,
            SystemSlotLabel { ..label }
            SystemSlotKey { ..slot_key }
            Tooltip { ..tooltip }
        }
    }
}
