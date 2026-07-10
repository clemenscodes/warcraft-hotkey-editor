mod props;
mod view;

pub use view::TileOverrideInfoOnlyView;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

use props::TileOverrideInfoOnlyProps;

/// The muted note shown for a passive ability in place of a hotkey field.
#[component]
pub fn TileOverrideInfoOnly(props: TileOverrideInfoOnlyProps) -> Element {
    let text = props.text;
    rsx! {
        p { class: CLASS, {text} }
    }
}

assert_component!(TileOverrideInfoOnly);
