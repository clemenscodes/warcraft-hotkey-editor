pub mod components;
mod props;
mod view;

pub use view::UnitCardSurfaceView;

use components::idle_unit_card_surface::IdleUnitCardSurface;
use components::selected_unit_card_surface::SelectedUnitCardSurface;
use dioxus::prelude::*;
use props::UnitCardSurfaceProps;
use tw_macro::assert_component;

/// The unit card's selectable button. A pure dispatcher: from whether the card is the
/// selected unit it renders `SelectedUnitCardSurface` xor `IdleUnitCardSurface`. Each
/// owns its `<button>` and its own look — the selected one wears the generic
/// `--race-color` accent and publishes `--name-color`; there is no `data-selected`, the
/// look follows the component.
#[component]
pub fn UnitCardSurface(props: UnitCardSurfaceProps) -> Element {
    let icon_path = props.icon_path.clone();
    let display_name = props.display_name.clone();
    let unit_id = props.unit_id;
    let onclick = props.onclick;
    let onkeydown = props.onkeydown;
    match props.is_selected {
        true => rsx! {
            SelectedUnitCardSurface {
                icon_path,
                display_name,
                unit_id,
                onclick,
                onkeydown,
            }
        },
        false => rsx! {
            IdleUnitCardSurface {
                icon_path,
                display_name,
                unit_id,
                onclick,
                onkeydown,
            }
        },
    }
}

assert_component!(UnitCardSurface);
