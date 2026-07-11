pub mod components;
mod model;
mod view;

pub use view::ToggleButtonView;

use components::active_toggle_button::ActiveToggleButton;
use components::idle_toggle_button::IdleToggleButton;
use dioxus::prelude::*;
use model::ToggleButtonModel;
use tw_macro::assert_component;

/// The shared labeled pill button (mode, search-field, catalog-visibility). A pure
/// dispatcher: from whether it is the active button in its group it renders the one
/// matching look — `ActiveToggleButton` xor `IdleToggleButton`. Each look owns its own
/// `<button>` and writes the shared pill chrome values plus its own state accent; this
/// dispatcher owns no class and there is no `data-active`, so the look follows the
/// component, not an attribute.
#[component]
pub fn ToggleButton(props: ToggleButtonModel) -> Element {
    let label = props.label;
    let title = props.title;
    let onclick = props.onclick;
    let onkeydown = props.onkeydown;
    match props.active {
        true => rsx! {
            ActiveToggleButton { label, title, onclick, onkeydown }
        },
        false => rsx! {
            IdleToggleButton { label, title, onclick, onkeydown }
        },
    }
}

assert_component!(ToggleButton);
