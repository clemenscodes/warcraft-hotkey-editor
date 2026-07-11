mod model;
mod view;

pub use view::TileOverrideEmptyView;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

use model::TileOverrideEmptyModel;

/// The placeholder shown in the override panel before a grid tile is selected.
#[component]
pub fn TileOverrideEmpty(props: TileOverrideEmptyModel) -> Element {
    let message = props.message;
    rsx! {
        div { class: CLASS,
            p { {message} }
        }
    }
}

assert_component!(TileOverrideEmpty);
