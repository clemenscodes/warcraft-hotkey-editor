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
