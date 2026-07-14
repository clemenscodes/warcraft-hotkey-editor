use super::default_unit::DefaultUnit;
use warcraft_api::{Race, WarcraftObjectId};
use warcraft_api::{UnitMode, WarcraftApi};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DecodedEditorNavigation {
    race: Race,
    unit_mode: UnitMode,
    selected_unit_id: Option<WarcraftObjectId>,
    search_query: String,
}

impl DecodedEditorNavigation {
    pub fn race(&self) -> Race {
        self.race
    }

    pub fn unit_mode(&self) -> UnitMode {
        self.unit_mode
    }

    pub fn selected_unit_id(&self) -> Option<WarcraftObjectId> {
        self.selected_unit_id
    }

    pub fn search_query(&self) -> &str {
        &self.search_query
    }

    pub fn new(
        race: Race,
        unit_mode: UnitMode,
        selected_unit_id: Option<WarcraftObjectId>,
        search_query: String,
    ) -> Self {
        Self {
            race,
            unit_mode,
            selected_unit_id,
            search_query,
        }
    }

    pub fn decode(
        race: Option<&str>,
        mode: Option<&str>,
        unit: Option<&str>,
        query: Option<&str>,
    ) -> Self {
        let api = WarcraftApi::default();
        let race_param = race.unwrap_or_default();
        let race = Race::try_from(race_param).unwrap_or(Race::Human);
        let mode_param = mode.unwrap_or_default();
        let unit_mode = UnitMode::try_from(mode_param).unwrap_or(UnitMode::Melee);
        let unit_param = unit.unwrap_or_default();
        let selected_unit_id = if unit_param.is_empty() {
            let default_unit = DefaultUnit::new(race, unit_mode);
            default_unit.resolve()
        } else {
            api.resolve(unit_param)
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
