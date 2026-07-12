pub mod components;
mod model;
mod view;

pub use view::UnitCardButtonView;

use components::idle_unit_card_button::IdleUnitCardButton;
use components::selected_unit_card_button::SelectedUnitCardButton;
use dioxus::prelude::*;
use model::UnitCardButtonModel;
use tw_macro::assert_component;

/// The unit card's selectable button. A pure dispatcher: from whether the card is the
/// selected unit it renders `SelectedUnitCardButton` xor `IdleUnitCardButton`. Each
/// owns its `<button>` and its own look — the selected one wears the generic
/// `--race-color` accent and publishes `--name-color`; there is no `data-selected`, the
/// look follows the component.
#[component]
pub fn UnitCardButton(props: UnitCardButtonModel) -> Element {
    let icon_path = props.icon_path.clone();
    let display_name = props.display_name.clone();
    let unit_id = props.unit_id;
    let onclick = props.onclick;
    let onkeydown = props.onkeydown;
    match props.is_selected {
        true => rsx! {
            SelectedUnitCardButton {
                icon_path,
                display_name,
                unit_id,
                onclick,
                onkeydown,
            }
        },
        false => rsx! {
            IdleUnitCardButton {
                icon_path,
                display_name,
                unit_id,
                onclick,
                onkeydown,
            }
        },
    }
}

assert_component!(UnitCardButton);
