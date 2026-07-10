pub mod components;
mod props;

use components::primary_agility_row::{PrimaryAgilityRow, PrimaryAgilityRowProps};
use components::regular_agility_row::{RegularAgilityRow, RegularAgilityRowProps};
use dioxus::prelude::*;
pub use props::AgilityRowProps;
use tw_macro::assert_component;
assert_component!(AgilityRow);

/// The hero's agility attribute row. A dispatcher: when agility is the hero's primary
/// attribute it renders the glowing primary row, otherwise the resting regular row — each
/// owns its own look, so there is no `data-primary` attribute.
#[component]
pub fn AgilityRow(props: AgilityRowProps) -> Element {
    if props.is_primary {
        let row = PrimaryAgilityRowProps::from(&props);
        rsx! {
            PrimaryAgilityRow { ..row }
        }
    } else {
        let row = RegularAgilityRowProps::from(&props);
        rsx! {
            RegularAgilityRow { ..row }
        }
    }
}
