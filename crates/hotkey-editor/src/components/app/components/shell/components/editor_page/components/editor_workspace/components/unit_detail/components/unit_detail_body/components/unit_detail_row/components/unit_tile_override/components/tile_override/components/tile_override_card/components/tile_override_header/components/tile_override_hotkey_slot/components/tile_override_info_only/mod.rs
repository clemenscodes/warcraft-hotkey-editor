mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::TileOverrideInfoOnlyProps;

assert_component!(TileOverrideInfoOnly);

/// The muted note shown for a passive ability in place of a hotkey field.
#[component]
pub fn TileOverrideInfoOnly(props: TileOverrideInfoOnlyProps) -> Element {
    let text = props.text;
    rsx! {
        p { class: CLASS, {text} }
    }
}
