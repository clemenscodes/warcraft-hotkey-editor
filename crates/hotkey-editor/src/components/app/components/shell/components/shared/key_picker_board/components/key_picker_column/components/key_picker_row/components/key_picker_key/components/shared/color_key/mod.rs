pub mod components;
mod model;
mod view;

pub use view::ColorKeyView;
mod state;

use components::available_key::AvailableKey;
use components::conflict_key::ConflictKey;
use components::current_key::CurrentKey;
use dioxus::prelude::*;
use model::ColorKeyModel;
use tw_macro::assert_component;

pub use state::ColorKeyState;

/// A picker key's color look, chosen from its state. A pure dispatcher that carries no
/// look of its own: from the key's visual state it renders the matching color
/// (`AvailableKey` xor `CurrentKey` xor `ConflictKey`), each of which owns its own
/// button, color styling, and conflict tooltip. Its width is owned by the slot that
/// renders it; this only chooses the color and fills the box it is given.
#[component]
pub fn ColorKey(props: ColorKeyModel) -> Element {
    let label = props.label;
    let disabled = props.disabled;
    let onclick = props.onclick;
    let tooltip_text = props.tooltip_text;
    let tooltip_placement = props.tooltip_placement;
    let tooltip_anchor = props.tooltip_anchor;
    match props.state {
        ColorKeyState::Available => {
            rsx! {
                AvailableKey {
                    label,
                    disabled,
                    onclick,
                    tooltip_text,
                    tooltip_placement,
                    tooltip_anchor,
                }
            }
        }
        ColorKeyState::Current => {
            rsx! {
                CurrentKey {
                    label,
                    disabled,
                    onclick,
                    tooltip_text,
                    tooltip_placement,
                    tooltip_anchor,
                }
            }
        }
        ColorKeyState::Conflict => {
            rsx! {
                ConflictKey {
                    label,
                    disabled,
                    onclick,
                    tooltip_text,
                    tooltip_placement,
                    tooltip_anchor,
                }
            }
        }
    }
}

assert_component!(ColorKey);
