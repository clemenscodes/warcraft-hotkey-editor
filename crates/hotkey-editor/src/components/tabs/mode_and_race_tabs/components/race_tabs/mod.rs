pub mod components;
mod logic;
mod props;
mod style;

use crate::assert_component;
use components::race_tab::RaceTab;
use dioxus::prelude::*;
use logic::RaceTabList;
pub use props::RaceTabsProps;
use style::CLASS;
assert_component!(RaceTabs);

/// The race tabs: one banner button per supported race. The finished tab props,
/// each carrying its active flag and handlers, are built in `logic`.
#[component]
pub fn RaceTabs(props: RaceTabsProps) -> Element {
    let RaceTabList { tabs } = RaceTabList::from(&props);
    rsx! {
        nav {
            class: CLASS,
            for tab in tabs {
                RaceTab { ..tab }
            }
        }
    }
}
