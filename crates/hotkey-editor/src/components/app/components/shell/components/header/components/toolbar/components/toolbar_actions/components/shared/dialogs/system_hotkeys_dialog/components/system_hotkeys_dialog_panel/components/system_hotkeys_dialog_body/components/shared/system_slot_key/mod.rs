pub mod components;
mod logic;
mod props;

use components::conflict_slot_key::{ConflictSlotKey, ConflictSlotKeyProps};
use components::plain_slot_key::{PlainSlotKey, PlainSlotKeyProps};
use dioxus::prelude::*;
pub use props::SystemSlotKeyProps;
use tw_macro::assert_component;
assert_component!(SystemSlotKey);

/// The bound-key glyph shown on a system hotkey slot. A pure dispatcher: from the
/// slot's conflict flag it renders the matching look — `PlainSlotKey` (gold) xor
/// `ConflictSlotKey` (danger-red). The glyph size is inherited from the parent size
/// container, so both looks are size-agnostic.
#[component]
pub fn SystemSlotKey(props: SystemSlotKeyProps) -> Element {
    match props.conflict {
        false => {
            let plain = PlainSlotKeyProps::from(&props);
            rsx! {
                PlainSlotKey { ..plain }
            }
        }
        true => {
            let conflict = ConflictSlotKeyProps::from(&props);
            rsx! {
                ConflictSlotKey { ..conflict }
            }
        }
    }
}
