pub mod components;
mod logic;
mod props;
mod style;

use components::race_tab_label::{RaceTabLabel, RaceTabLabelProps};
use crate::services::focus::context::use_focus_coordinator;
use dioxus::prelude::*;
use logic::RaceTabChrome;
pub use props::RaceTabProps;
use std::rc::Rc;
use tw_macro::assert_component;
assert_component!(RaceTab);

/// One race banner button. Its per-race banner and accent come from
/// `style::class(race)`; its active look is driven by `data-active`; the label is
/// a child, and the handlers are built in `logic`.
#[component]
pub fn RaceTab(props: RaceTabProps) -> Element {
    let label = RaceTabLabelProps::from(&props);
    let class = style::class(props.race);
    let focus = use_focus_coordinator();
    let mut mounted_handle = use_signal(|| None::<Rc<MountedData>>);
    let active_race = props.active_race;
    let race = props.race;
    // Register this tab as the race-tabs focus target exactly while it is the active
    // race — read from the active-race signal, never from a `data-active` DOM query.
    use_effect(move || {
        if *active_race.read() == race {
            let handle = mounted_handle.read().clone();
            focus.set_race_tabs_handle(handle);
        }
    });
    let RaceTabChrome {
        race_attribute,
        is_active,
        onclick,
        onkeydown,
    } = RaceTabChrome::build(&props, focus);
    rsx! {
        button {
            class,
            "data-race": race_attribute,
            "data-active": is_active,
            onmounted: move |event: Event<MountedData>| mounted_handle.set(Some(event.data())),
            onclick,
            onkeydown,
            RaceTabLabel { ..label }
        }
    }
}
