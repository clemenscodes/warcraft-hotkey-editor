mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::TileOverrideNameProps;

assert_component!(TileOverrideName);

/// The ability / unit name heading in the override panel.
#[component]
pub fn TileOverrideName(props: TileOverrideNameProps) -> Element {
    let text = props.text;
    rsx! {
        h3 { class: CLASS, {text} }
    }
}
