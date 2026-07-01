mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::HeroLevelBackdropProps;
use style::CLASS;
assert_component!(HeroLevelBackdrop);

/// A fixed, transparent backdrop that dismisses the open level menu on click.
#[component]
pub fn HeroLevelBackdrop(props: HeroLevelBackdropProps) -> Element {
    let onclick = props.onclick;
    rsx! {
        div {
            class: CLASS,
            onclick,
        }
    }
}
