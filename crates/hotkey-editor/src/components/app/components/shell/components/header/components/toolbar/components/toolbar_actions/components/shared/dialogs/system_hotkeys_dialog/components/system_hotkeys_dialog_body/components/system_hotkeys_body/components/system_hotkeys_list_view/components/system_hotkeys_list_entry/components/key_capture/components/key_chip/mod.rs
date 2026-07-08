pub mod components;
mod props;

use components::conflict_key_chip::{ConflictKeyChip, ConflictKeyChipProps};
use components::normal_key_chip::{NormalKeyChip, NormalKeyChipProps};
use dioxus::prelude::*;
pub use props::KeyChipProps;
use tw_macro::assert_component;
assert_component!(KeyChip);

/// A pure dispatcher for the system-hotkey chip: from the row's domain conflict
/// fact it renders the red `ConflictKeyChip` or the gold `NormalKeyChip`. It carries
/// no class of its own — each look owns its own classed button root.
#[component]
pub fn KeyChip(props: KeyChipProps) -> Element {
    if props.conflict {
        let look = ConflictKeyChipProps::from(&props);
        return rsx! {
            ConflictKeyChip { ..look }
        };
    }
    let look = NormalKeyChipProps::from(&props);
    rsx! {
        NormalKeyChip { ..look }
    }
}
