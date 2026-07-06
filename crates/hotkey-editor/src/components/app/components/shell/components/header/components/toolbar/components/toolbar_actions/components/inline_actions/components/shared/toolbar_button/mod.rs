pub mod components;
mod props;
mod style;

use components::toolbar_button_surface::{ToolbarButtonSurface, ToolbarButtonSurfaceProps};
use dioxus::prelude::*;
pub use props::ToolbarButtonProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ToolbarButton);

/// The container for a toolbar action button. It owns only the button's box: it fills
/// the height it is given, locks itself square (`aspect-square`, so it is never
/// stretched), and marks itself a query container. The parent that places it decides
/// how large the box is — a single length on the toolbar sizes the whole row, a Host
/// container resizes one button — and the surface inside scales its entire look off
/// this box in `cqi`, like one drawing.
#[component]
pub fn ToolbarButton(props: ToolbarButtonProps) -> Element {
    let surface = ToolbarButtonSurfaceProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            ToolbarButtonSurface { ..surface }
        }
    }
}
