mod props;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

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
