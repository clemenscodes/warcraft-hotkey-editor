mod props;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

pub use props::TileOverridePanelProps;

assert_component!(TileOverridePanel);

/// The right column of the unit-detail row holding the hotkey override.
#[component]
pub fn TileOverridePanel(props: TileOverridePanelProps) -> Element {
    let children = props.children;
    rsx! {
        aside { class: CLASS, {children} }
    }
}
