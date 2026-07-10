pub mod components;
mod hooks;
mod props;
mod view;

pub use view::AgilityRowView;

use components::primary_agility_row::PrimaryAgilityRow;
use components::regular_agility_row::RegularAgilityRow;
use dioxus::prelude::*;
use hooks::{AgilityRowModel, use_agility_row};
use props::AgilityRowProps;
use tw_macro::assert_component;

/// The hero's agility attribute row. A dispatcher: when agility is the hero's primary
/// attribute it renders the glowing primary row, otherwise the resting regular row — each
/// owns its own look, so there is no `data-primary` attribute.
#[component]
pub fn AgilityRow(props: AgilityRowProps) -> Element {
    let AgilityRowModel {
        statistic,
        growth,
        label,
    } = use_agility_row(&props);
    if props.is_primary {
        rsx! {
            PrimaryAgilityRow { statistic, growth, label }
        }
    } else {
        rsx! {
            RegularAgilityRow { statistic, growth, label }
        }
    }
}

assert_component!(AgilityRow);
