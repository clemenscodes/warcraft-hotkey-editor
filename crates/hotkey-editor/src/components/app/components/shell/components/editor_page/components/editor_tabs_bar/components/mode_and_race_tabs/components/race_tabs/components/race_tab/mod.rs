pub mod components;
mod logic;
mod props;
mod style;

use components::race_tab_label::{RaceTabLabel, RaceTabLabelProps};
use dioxus::prelude::*;
use logic::RaceTabChrome;
pub use props::RaceTabProps;
use tw_macro::assert_component;
assert_component!(RaceTab);

/// One race banner button. Its per-race banner and accent come from
/// `style::class(race)`; its active look is driven by `data-active`; the label is
/// a child, and the handlers are built in `logic`.
#[component]
pub fn RaceTab(props: RaceTabProps) -> Element {
    let label = RaceTabLabelProps::from(&props);
    let class = style::class(props.race);
    let RaceTabChrome {
        race_attribute,
        is_active,
        onclick,
        onkeydown,
    } = RaceTabChrome::from(&props);
    rsx! {
        button {
            class,
            "data-race": race_attribute,
            "data-active": is_active,
            onclick,
            onkeydown,
            RaceTabLabel { ..label }
        }
    }
}
