use super::model::RaceTabsModel;
use dioxus::prelude::*;
use warcraft_api::AllRaces;
use warcraft_api::Race;

/// The finished race tabs, one per race in the domain's `AllRaces` order. `RaceTabs` is
/// the one consumer of the active race: this reads it once, marks each tab active, and
/// bakes the select handler into each tab's pointer/keyboard handlers — so only plain,
/// navigation-free items flow to the dispatcher. The set and order of races come from
/// the domain (`AllRaces`), never hardcoded here.
pub(super) struct RaceTabsPresentation {
    pub(super) tabs: Vec<RaceTabItem>,
}

/// One race tab's finished item: which race it is (so the dispatcher picks that race's
/// themed wrapper), whether it is the active tab, its display label, and the baked
/// pointer/keyboard handlers. Plain data — no navigation signal.
pub(super) struct RaceTabItem {
    pub(super) race: Race,
    pub(super) is_active: bool,
    pub(super) label: String,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) onkeydown: EventHandler<KeyboardEvent>,
}

impl RaceTabItem {
    fn new(race: Race, current_race: Race, on_select: EventHandler<Race>) -> Self {
        let is_active = race == current_race;
        let label = race.to_string();
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
        Self {
            race,
            is_active,
            label,
            onclick,
            onkeydown,
        }
    }
}

impl From<&RaceTabsModel> for RaceTabsPresentation {
    fn from(model: &RaceTabsModel) -> Self {
        let active_race = model.active_race;
        let on_select = model.on_select;
        let current_race = *active_race.read();
        let all_races = AllRaces::ALL;
        let tabs = all_races
            .iter()
            .map(|race| RaceTabItem::new(race, current_race, on_select))
            .collect();
        Self { tabs }
    }
}
