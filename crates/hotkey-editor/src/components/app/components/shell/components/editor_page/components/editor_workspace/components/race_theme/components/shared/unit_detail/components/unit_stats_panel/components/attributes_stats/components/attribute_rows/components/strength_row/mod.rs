pub mod components;
mod model;
mod presentation;
mod view;

pub use view::StrengthRowView;

use components::primary_strength_row::PrimaryStrengthRow;
use components::regular_strength_row::RegularStrengthRow;
use dioxus::prelude::*;
use model::StrengthRowModel;
use presentation::{StrengthRowPresentation, use_strength_row};
use tw_macro::assert_component;

/// The hero's strength attribute row. A dispatcher: when strength is the hero's primary
/// attribute it renders the glowing primary row, otherwise the resting regular row — each
/// owns its own look, so there is no `data-primary` attribute.
#[component]
pub fn StrengthRow(props: StrengthRowModel) -> Element {
    let StrengthRowPresentation {
        statistic,
        growth,
        label,
    } = use_strength_row(&props);
    if props.is_primary {
        rsx! {
            PrimaryStrengthRow { statistic, growth, label }
        }
    } else {
        rsx! {
            RegularStrengthRow { statistic, growth, label }
        }
    }
}

assert_component!(StrengthRow);
