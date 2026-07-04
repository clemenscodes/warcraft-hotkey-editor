use warcraft_api::Race;
use warcraft_database::{UnitKindHelpers, UnitMode};

/// The editor navigation state decoded from the `?race=`/`?mode=`/`?unit=`/`?q=`
/// query parameters every route carries. Each page reconciles its URL into the
/// shell's navigation signals through this, so the editor's race/mode/unit/search
/// survive switching views (they ride in every route's query string, not just the
/// editor's).
///
/// The fallbacks mirror the app's long-standing URL contract: an unknown or empty
/// race/mode falls back to Human/Melee, and an empty unit falls back to the
/// race+mode default unit.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DecodedEditorNav {
    pub race: Race,
    pub unit_mode: UnitMode,
    pub selected_unit_id: Option<String>,
    pub search_query: String,
}

impl DecodedEditorNav {
    /// Decode the four editor query parameters into typed navigation state.
    pub fn decode(
        race: Option<&str>,
        mode: Option<&str>,
        unit: Option<&str>,
        query: Option<&str>,
    ) -> Self {
        let race_param = race.unwrap_or_default();
        let race = Race::try_from(race_param).unwrap_or(Race::Human);
        let mode_param = mode.unwrap_or_default();
        let unit_mode = UnitMode::try_from(mode_param).unwrap_or(UnitMode::Melee);
        let unit_param = unit.unwrap_or_default();
        let selected_unit_id = if unit_param.is_empty() {
            UnitKindHelpers::default_unit_id_for(race, unit_mode)
        } else {
            Some(unit_param.to_string())
        };
        let search_query = query.unwrap_or_default().to_string();
        Self {
            race,
            unit_mode,
            selected_unit_id,
            search_query,
        }
    }
}
