mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use crate::components::actions::grid_layout_button::GridLayoutButton;
use style::CLASS;

pub use props::HeaderLayoutSlotProps;

assert_component!(HeaderLayoutSlot);

/// The centered slot that holds the global grid-layout button in the full header
/// layout. Hidden in the compact layout, where the drawer offers it instead.
#[component]
pub fn HeaderLayoutSlot(props: HeaderLayoutSlotProps) -> Element {
    let layout_dialog_open = props.layout_dialog_open;
    rsx! {
        div {
            class: CLASS,
            GridLayoutButton { layout_dialog_open }
        }
    }
}
