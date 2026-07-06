pub mod components;
mod logic;
mod props;
mod style;

use components::mode_tab::ModeTab;
use crate::services::focus::context::use_focus_coordinator;
use dioxus::prelude::*;
use logic::ModeTabPair;
pub use props::ModeTabsProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ModeTabs);

/// The mode column: the Melee and Campaign buttons stacked (or laid in a row on
/// phones). Each button's label, active flag, and handlers are built by
/// conversion in `logic`.
#[component]
pub fn ModeTabs(props: ModeTabsProps) -> Element {
    let focus = use_focus_coordinator();
    let ModeTabPair { melee, campaign } = ModeTabPair::build(&props, focus);
    rsx! {
        div {
            class: CLASS,
            ModeTab { ..melee }
            ModeTab { ..campaign }
        }
    }
}
