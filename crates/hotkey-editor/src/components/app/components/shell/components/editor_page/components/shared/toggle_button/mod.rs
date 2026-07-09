pub mod components;
mod logic;
mod props;

use components::active_toggle_button::{ActiveToggleButton, ActiveToggleButtonProps};
use components::idle_toggle_button::{IdleToggleButton, IdleToggleButtonProps};
use dioxus::prelude::*;
pub use props::ToggleButtonProps;
use tw_macro::assert_component;
assert_component!(ToggleButton);

/// The shared labeled pill button (mode, search-field, catalog-visibility). A pure
/// dispatcher: from whether it is the active button in its group it renders the one
/// matching look — `ActiveToggleButton` xor `IdleToggleButton`. Each look owns its own
/// `<button>` and writes the shared pill chrome values plus its own state accent; this
/// dispatcher owns no class and there is no `data-active`, so the look follows the
/// component, not an attribute.
#[component]
pub fn ToggleButton(props: ToggleButtonProps) -> Element {
    match props.active {
        true => {
            let active = ActiveToggleButtonProps::from(&props);
            rsx! {
                ActiveToggleButton { ..active }
            }
        }
        false => {
            let idle = IdleToggleButtonProps::from(&props);
            rsx! {
                IdleToggleButton { ..idle }
            }
        }
    }
}
