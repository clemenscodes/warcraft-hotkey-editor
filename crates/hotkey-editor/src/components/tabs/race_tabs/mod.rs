mod state;
mod tab;

use dioxus::prelude::*;
use warcraft_api::{Race, RaceLabels, SUPPORTED_RACES};
use warcraft_database::UnitMode;

use crate::model::grid::GridSlotId;

use tab::RaceTab;

#[component]
pub(crate) fn RaceTabs(
    active_race: Signal<Race>,
    unit_mode: Signal<UnitMode>,
    selected_unit_id: Signal<Option<String>>,
    selected_slot: Signal<Option<GridSlotId>>,
) -> Element {
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
