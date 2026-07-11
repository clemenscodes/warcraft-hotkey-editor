mod model;
mod view;

pub use view::TileOverrideNameView;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

use model::TileOverrideNameModel;

/// The ability / unit name heading in the override panel.
#[component]
pub fn TileOverrideName(props: TileOverrideNameModel) -> Element {
    let text = props.text;
    rsx! {
        h3 { class: CLASS, {text} }
    }
}

assert_component!(TileOverrideName);
