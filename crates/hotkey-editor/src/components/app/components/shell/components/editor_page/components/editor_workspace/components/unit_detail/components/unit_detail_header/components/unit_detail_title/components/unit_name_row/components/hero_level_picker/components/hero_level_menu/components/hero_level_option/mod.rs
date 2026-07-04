mod logic;
mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
use logic::HeroLevelOptionPresentation;
pub use props::HeroLevelOptionProps;
use style::CLASS;
assert_component!(HeroLevelOption);

/// One selectable hero level in the dropdown menu. Its active look is driven by the
/// `data-active` attribute; selecting it sets the level and closes the menu.
#[component]
pub fn HeroLevelOption(props: HeroLevelOptionProps) -> Element {
    let HeroLevelOptionPresentation {
        is_active,
        label,
        onclick,
    } = HeroLevelOptionPresentation::from(&props);
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            "data-active": is_active,
            onclick,
            {label}
        }
    }
}
