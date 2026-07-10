use super::props::{RaceTabBinding, RaceTabsProps};
use dioxus::prelude::*;
use warcraft_api::{Race, RaceLabels};

/// The five race tabs' finished bindings, in supported-race order. `RaceTabs` is the one
/// consumer of the active race: it reads it once, marks each tab active from it, and
/// bakes the select handler into each tab's handlers, so only these plain bindings flow
/// to the per-race wrappers — never the navigation signals.
pub(super) struct RaceTabBindings {
    pub(super) human: RaceTabBinding,
    pub(super) orc: RaceTabBinding,
    pub(super) nightelf: RaceTabBinding,
    pub(super) undead: RaceTabBinding,
    pub(super) neutral: RaceTabBinding,
}

impl RaceTabBindings {
    pub(super) fn build(props: &RaceTabsProps) -> Self {
        let human = race_tab(props, Race::Human);
        let orc = race_tab(props, Race::Orc);
        let nightelf = race_tab(props, Race::Nightelf);
        let undead = race_tab(props, Race::Undead);
        let neutral = race_tab(props, Race::Neutral);
        Self {
            human,
            orc,
            nightelf,
            undead,
            neutral,
        }
    }
}

/// Build one tab's binding: mark it active when it is the current race, and bake the
/// select handler into its pointer and keyboard handlers. The selection cascade itself
/// lives behind `on_select`, in the navigation service.
fn race_tab(props: &RaceTabsProps, race: Race) -> RaceTabBinding {
    let active_race = props.active_race;
    let on_select = props.on_select;
    let current_race = *active_race.read();
    let is_active = current_race == race;
    let display_name = RaceLabels::display_name(race);
    let label = display_name.to_string();
    let onclick = EventHandler::new(move |_event: MouseEvent| {
        on_select.call(race);
    });
    let onkeydown = EventHandler::new(move |event: KeyboardEvent| {
        let key = event.key();
        let key_value = key.to_string();
        let is_space = key_value == " ";
        let is_enter = key_value == "Enter";
        if is_space || is_enter {
            event.prevent_default();
            on_select.call(race);
        }
    });
    RaceTabBinding {
        is_active,
        label,
        onclick,
        onkeydown,
    }
}
