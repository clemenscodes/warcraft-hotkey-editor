pub mod components;
mod props;
mod style;

use components::carriers_grid::{CarriersGrid, CarriersGridProps};
use dioxus::prelude::*;
pub use props::CarriersDialogBodyProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(CarriersDialogBody);

/// The carriers dialog's scrolling content region between the header and the panel
/// edge, holding the scrollable grid of carrier cards.
#[component]
pub fn CarriersDialogBody(props: CarriersDialogBodyProps) -> Element {
    let grid = CarriersGridProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            CarriersGrid { ..grid }
        }
    }
}
