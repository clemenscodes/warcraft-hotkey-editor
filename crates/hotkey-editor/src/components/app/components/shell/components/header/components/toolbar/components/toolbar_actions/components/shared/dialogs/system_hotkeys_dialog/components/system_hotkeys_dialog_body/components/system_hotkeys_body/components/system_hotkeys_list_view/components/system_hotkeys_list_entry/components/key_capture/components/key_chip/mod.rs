pub mod components;
mod model;
mod view;

pub use view::KeyChipView;

use components::conflict_key_chip::ConflictKeyChip;
use components::normal_key_chip::NormalKeyChip;
use dioxus::prelude::*;
use model::KeyChipModel;
use tw_macro::assert_component;

/// A pure dispatcher for the system-hotkey chip: from the row's domain conflict
/// fact it renders the red `ConflictKeyChip` or the gold `NormalKeyChip`. It carries
/// no class of its own — each look owns its own classed button root.
#[component]
pub fn KeyChip(props: KeyChipModel) -> Element {
    if props.conflict {
        let label = props.label.clone();
        let onclick = props.onclick;
        let tooltip_text = props.tooltip_text.clone();
        let tooltip_placement = props.tooltip_placement;
        return rsx! {
            ConflictKeyChip {
                label,
                onclick,
                tooltip_text,
                tooltip_placement,
            }
        };
    }
    let label = props.label.clone();
    let onclick = props.onclick;
    let tooltip_text = props.tooltip_text.clone();
    let tooltip_placement = props.tooltip_placement;
    rsx! {
        NormalKeyChip {
            label,
            onclick,
            tooltip_text,
            tooltip_placement,
        }
    }
}

assert_component!(KeyChip);
