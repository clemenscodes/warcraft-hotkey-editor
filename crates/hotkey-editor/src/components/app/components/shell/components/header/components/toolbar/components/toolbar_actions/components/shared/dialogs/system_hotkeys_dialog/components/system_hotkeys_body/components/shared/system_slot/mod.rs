mod logic;
mod props;
mod state;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::shared::system_slot_key::{
    SystemSlotKey, SystemSlotKeyProps,
};
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_body::components::shared::system_slot_label::{
    SystemSlotLabel, SystemSlotLabelProps,
};
use crate::components::app::components::shell::components::shared::tooltip::{Tooltip, TooltipProps};
use dioxus::prelude::*;
pub use props::SystemSlotProps;
pub use state::SystemSlotState;
use tw_macro::assert_component;
assert_component!(SystemSlot);

/// The framed WC3 slot cell shared by the inventory grid and the hero/control-group
/// layouts: it draws the gold border-image frame and composes the caption, bound
/// key, and conflict tooltip from typed props. Purely presentational — it fills the
/// box its host gives it and reacts to the host's hover/keyboard-focus for its
/// glows; the host owns the outer interactive element, size, and behaviour.
#[component]
pub fn SystemSlot(props: SystemSlotProps) -> Element {
    let label = SystemSlotLabelProps::from(&props);
    let key = SystemSlotKeyProps::from(&props);
    let tooltip = TooltipProps::from(&props);
    let class = style::class(props.state);
    rsx! {
        div {
            class,
            "data-compact": props.compact,
            "data-dragging": props.dragging,
            SystemSlotLabel { ..label }
            SystemSlotKey { ..key }
            Tooltip { ..tooltip }
        }
    }
}
