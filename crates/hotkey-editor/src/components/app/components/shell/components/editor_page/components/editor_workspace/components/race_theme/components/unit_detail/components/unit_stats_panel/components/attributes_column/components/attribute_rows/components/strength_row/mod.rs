pub mod components;
mod props;

use components::primary_strength_row::{PrimaryStrengthRow, PrimaryStrengthRowProps};
use components::regular_strength_row::{RegularStrengthRow, RegularStrengthRowProps};
use dioxus::prelude::*;
pub use props::StrengthRowProps;
use tw_macro::assert_component;
assert_component!(StrengthRow);

/// The hero's strength attribute row. A dispatcher: when strength is the hero's primary
/// attribute it renders the glowing primary row, otherwise the resting regular row — each
/// owns its own look, so there is no `data-primary` attribute.
#[component]
pub fn StrengthRow(props: StrengthRowProps) -> Element {
    if props.is_primary {
        let row = PrimaryStrengthRowProps::from(&props);
        rsx! {
            PrimaryStrengthRow { ..row }
        }
    } else {
        let row = RegularStrengthRowProps::from(&props);
        rsx! {
            RegularStrengthRow { ..row }
        }
    }
}
