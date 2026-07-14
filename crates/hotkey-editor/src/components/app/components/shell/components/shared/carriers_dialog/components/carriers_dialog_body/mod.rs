pub mod components;
mod model;
mod view;

pub use view::CarriersDialogBodyView;
mod style;

use components::carriers_grid::CarriersGrid;
use dioxus::prelude::*;
use model::CarriersDialogBodyModel;
use style::CLASS;
use tw_macro::assert_component;

/// The carriers dialog's scrolling content region between the header and the panel
/// edge, holding the scrollable grid of carrier cards.
#[component]
pub fn CarriersDialogBody(props: CarriersDialogBodyModel) -> Element {
    let carriers = props.carriers;
    rsx! {
        div {
            class: CLASS,
            CarriersGrid {
                carriers,
            }
        }
    }
}

assert_component!(CarriersDialogBody);
