use dioxus::prelude::*;
use warcraft_api::{Race, WarcraftApi};

use super::model::PagerCardRaceThemeModel;

/// Resolves the race the card publishes its theme for.
///
/// The pager runs through every race in one list, so the theme cannot come from
/// the navigation's active race the way the desktop workspace does. It comes
/// from the unit on the card itself. Objects the database gives no race read as
/// Neutral, which is where tavern heroes and mercenaries belong anyway.
pub(super) fn use_pager_card_race_theme(props: &PagerCardRaceThemeModel) -> Race {
    let unit_id = props.unit_id;
    let race_memo = use_memo(move || {
        let api = WarcraftApi::default();
        let unit_view = api.unit().get(unit_id);
        let resolved_race = unit_view.and_then(|unit| unit.race());
        resolved_race.unwrap_or(Race::Neutral)
    });
    race_memo()
}
