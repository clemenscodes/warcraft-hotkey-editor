pub mod components;
mod logic;
mod props;

use components::idle_unit_card_surface::{IdleUnitCardSurface, IdleUnitCardSurfaceProps};
use components::selected_unit_card_surface::{
    SelectedUnitCardSurface, SelectedUnitCardSurfaceProps,
};
use dioxus::prelude::*;
pub use props::UnitCardSurfaceProps;
use tw_macro::assert_component;

/// The unit card's selectable button. A pure dispatcher: from whether the card is the
/// selected unit it renders `SelectedUnitCardSurface` xor `IdleUnitCardSurface`. Each
/// owns its `<button>` and its own look — the selected one wears the generic
/// `--race-color` accent and publishes `--name-color`; there is no `data-selected`, the
/// look follows the component.
#[component]
pub fn UnitCardSurface(props: UnitCardSurfaceProps) -> Element {
    match props.is_selected {
        true => {
            let surface = SelectedUnitCardSurfaceProps::from(&props);
            rsx! {
                SelectedUnitCardSurface { ..surface }
            }
        }
        false => {
            let surface = IdleUnitCardSurfaceProps::from(&props);
            rsx! {
                IdleUnitCardSurface { ..surface }
            }
        }
    }
}

assert_component!(UnitCardSurface);
