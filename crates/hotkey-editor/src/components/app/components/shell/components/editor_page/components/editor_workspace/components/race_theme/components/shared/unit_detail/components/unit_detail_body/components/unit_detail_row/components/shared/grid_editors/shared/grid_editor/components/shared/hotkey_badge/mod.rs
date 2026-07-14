pub mod components;
mod model;
mod view;

pub use view::HotkeyBadgeView;
mod state;

use components::conflict_hotkey_badge::ConflictHotkeyBadge;
use components::normal_hotkey_badge::NormalHotkeyBadge;
use components::passive_hotkey_badge::PassiveHotkeyBadge;
use dioxus::prelude::*;
use model::HotkeyBadgeModel;
pub use state::HotkeyBadgeState;
use tw_macro::assert_component;

/// The hotkey-letter badge. A pure dispatcher: the domain-derived tone — an ordinary
/// binding, a passive ability, or a conflicting one — selects which per-tone badge
/// component draws the letter. Each tone owns its own look in its own `style.rs`; this
/// body only routes, it computes no look of its own.
#[component]
pub fn HotkeyBadge(props: HotkeyBadgeModel) -> Element {
    let letter = props.letter;
    match props.state {
        HotkeyBadgeState::Normal => {
            rsx! {
                NormalHotkeyBadge {
                    letter,
                }
            }
        }
        HotkeyBadgeState::Passive => {
            rsx! {
                PassiveHotkeyBadge {
                    letter,
                }
            }
        }
        HotkeyBadgeState::Conflict => {
            rsx! {
                ConflictHotkeyBadge {
                    letter,
                }
            }
        }
    }
}

assert_component!(HotkeyBadge);
