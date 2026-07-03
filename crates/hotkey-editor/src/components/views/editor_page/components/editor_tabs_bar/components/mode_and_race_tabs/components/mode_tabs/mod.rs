pub mod components;
mod logic;
mod props;
mod style;

use crate::assert_component;
use components::mode_tab::ModeTab;
use dioxus::prelude::*;
use logic::ModeTabPair;
pub use props::ModeTabsProps;
use style::CLASS;
assert_component!(ModeTabs);

/// The mode column: the Melee and Campaign buttons stacked (or laid in a row on
/// phones). Each button's label, active flag, and handlers are built by
/// conversion in `logic`.
#[component]
pub fn ModeTabs(props: ModeTabsProps) -> Element {
    let ModeTabPair { melee, campaign } = ModeTabPair::from(&props);
    rsx! {
        div {
            class: CLASS,
            ModeTab { ..melee }
            ModeTab { ..campaign }
        }
    }
}
