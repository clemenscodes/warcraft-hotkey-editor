pub mod components;
mod props;
mod state;

use components::conflict_hotkey_badge::{ConflictHotkeyBadge, ConflictHotkeyBadgeProps};
use components::normal_hotkey_badge::{NormalHotkeyBadge, NormalHotkeyBadgeProps};
use components::passive_hotkey_badge::{PassiveHotkeyBadge, PassiveHotkeyBadgeProps};
use dioxus::prelude::*;
pub use props::HotkeyBadgeProps;
pub use state::HotkeyBadgeState;
use tw_macro::assert_component;
assert_component!(HotkeyBadge);

/// The hotkey-letter badge. A pure dispatcher: the domain-derived tone — an ordinary
/// binding, a passive ability, or a conflicting one — selects which per-tone badge
/// component draws the letter. Each tone owns its own look in its own `style.rs`; this
/// body only routes, it computes no look of its own.
#[component]
pub fn HotkeyBadge(props: HotkeyBadgeProps) -> Element {
    match props.state {
        HotkeyBadgeState::Normal => {
            let badge = NormalHotkeyBadgeProps::from(&props);
            rsx! {
                NormalHotkeyBadge { ..badge }
            }
        }
        HotkeyBadgeState::Passive => {
            let badge = PassiveHotkeyBadgeProps::from(&props);
            rsx! {
                PassiveHotkeyBadge { ..badge }
            }
        }
        HotkeyBadgeState::Conflict => {
            let badge = ConflictHotkeyBadgeProps::from(&props);
            rsx! {
                ConflictHotkeyBadge { ..badge }
            }
        }
    }
}
