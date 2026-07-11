mod model;
mod view;

pub use view::HeroLevelBackdropView;
mod style;

use dioxus::prelude::*;
use model::HeroLevelBackdropModel;
use style::CLASS;
use tw_macro::assert_component;

/// A fixed, transparent backdrop that dismisses the open level menu on click.
#[component]
pub fn HeroLevelBackdrop(props: HeroLevelBackdropModel) -> Element {
    let onclick = props.onclick;
    rsx! {
        div {
            class: CLASS,
            onclick,
        }
    }
}

assert_component!(HeroLevelBackdrop);
