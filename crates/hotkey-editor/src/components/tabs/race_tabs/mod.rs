mod state;
mod tab;

use dioxus::prelude::*;
use warcraft_api::{Race, RaceLabels, SUPPORTED_RACES};
use warcraft_database::UnitMode;

use crate::model::grid::GridSlotId;

use tab::RaceTab;

#[derive(Props, Clone, PartialEq)]
pub(crate) struct RaceTabsProps {
    pub(crate) active_race: Signal<Race>,
    pub(crate) unit_mode: Signal<UnitMode>,
    pub(crate) selected_unit_id: Signal<Option<String>>,
    pub(crate) selected_slot: Signal<Option<GridSlotId>>,
}

#[component]
pub(crate) fn RaceTabs(props: RaceTabsProps) -> Element {
    let active_race = props.active_race;
    let unit_mode = props.unit_mode;
    let selected_unit_id = props.selected_unit_id;
    let selected_slot = props.selected_slot;
    let current_race = *active_race.read();
    rsx! {
        nav { class: "flex gap-4 flex-nowrap w-full min-w-0 grow self-stretch",
            for race in SUPPORTED_RACES.iter().copied() {
                {
                    let race_key = RaceLabels::data_attribute(race);
                    let is_active = current_race == race;
                    rsx! {
                        RaceTab {
                            key: "{race_key}",
                            race,
                            is_active,
                            active_race,
                            unit_mode,
                            selected_unit_id,
                            selected_slot,
                        }
                    }
                }
            }
        }
    }
}
