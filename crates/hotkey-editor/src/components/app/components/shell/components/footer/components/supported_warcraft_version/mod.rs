mod model;
mod view;

pub use view::SupportedWarcraftVersionView;
mod style;

use dioxus::prelude::*;
use model::SupportedWarcraftVersionModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn SupportedWarcraftVersion(props: SupportedWarcraftVersionModel) -> Element {
    let label = props.label;
    let version = props.version;
    rsx! {
        span {
            class: CLASS,
            "{label} {version}"
        }
    }
}

assert_component!(SupportedWarcraftVersion);
