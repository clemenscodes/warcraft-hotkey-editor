use warcraft_api::Race;

use crate::races::RaceLabels;
use warcraft_database::{UnitKindHelpers, UnitMode};

pub(crate) struct UrlNavigationState {
    race: Race,
    unit_mode: UnitMode,
    selected_unit_id: Option<String>,
    search_query: String,
}

impl UrlNavigationState {
    pub(crate) fn race(&self) -> Race {
        self.race
    }

    pub(crate) fn unit_mode(&self) -> UnitMode {
        self.unit_mode
    }

    pub(crate) fn selected_unit_id(&self) -> Option<String> {
        self.selected_unit_id.clone()
    }

    pub(crate) fn search_query(&self) -> String {
        self.search_query.clone()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn from_url() -> Self {
        Self::default_state()
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn from_url() -> Self {
        let search_string = web_sys::window()
            .and_then(|window| window.location().search().ok())
            .unwrap_or_default();

        let params = match web_sys::UrlSearchParams::new_with_str(&search_string) {
            Ok(parsed) => parsed,
            Err(_) => return Self::default_state(),
        };

        let race_param = params.get("race");
        let race = race_param
            .as_deref()
            .and_then(Self::race_from_param)
            .unwrap_or(Race::Human);

        let mode_param = params.get("mode");
        let unit_mode = mode_param
            .as_deref()
            .and_then(Self::mode_from_param)
            .unwrap_or(UnitMode::Melee);

        let unit_param = params.get("unit");
        let selected_unit_id = unit_param
            .filter(|id| !id.is_empty())
            .or_else(|| UnitKindHelpers::default_unit_id_for(race, unit_mode));

        let search_query = params.get("q").unwrap_or_default();

        Self {
            race,
            unit_mode,
            selected_unit_id,
            search_query,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn push_to_url(
        _race: Race,
        _unit_mode: UnitMode,
        _unit_id: Option<&str>,
        _query: &str,
    ) {
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn push_to_url(race: Race, unit_mode: UnitMode, unit_id: Option<&str>, query: &str) {
        let Some(window) = web_sys::window() else {
            return;
        };
        let Ok(history) = window.history() else {
            return;
        };

        let race_param = Self::race_to_param(race);
        let mode_param = Self::mode_to_param(unit_mode);
        let mut url = format!("?race={race_param}&mode={mode_param}");

        if let Some(id) = unit_id {
            url.push_str("&unit=");
            url.push_str(id);
        }

        if !query.is_empty() {
            let encoded = js_sys::encode_uri_component(query);
            let encoded_str = encoded.as_string().unwrap_or_default();
            url.push_str("&q=");
            url.push_str(&encoded_str);
        }

        let _ = history.replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&url));
    }

    fn default_state() -> Self {
        let race = Race::Human;
        let unit_mode = UnitMode::Melee;
        let selected_unit_id = UnitKindHelpers::default_unit_id_for(race, unit_mode);
        Self {
            race,
            unit_mode,
            selected_unit_id,
            search_query: String::new(),
        }
    }

    fn race_to_param(race: Race) -> &'static str {
        RaceLabels::data_attribute(race)
    }

    fn race_from_param(param: &str) -> Option<Race> {
        match param {
            "human" => Some(Race::Human),
            "orc" => Some(Race::Orc),
            "nightelf" => Some(Race::Nightelf),
            "undead" => Some(Race::Undead),
            "neutral" => Some(Race::Neutral),
            _ => None,
        }
    }

    fn mode_to_param(mode: UnitMode) -> &'static str {
        match mode {
            UnitMode::Melee => "melee",
            UnitMode::Campaign => "campaign",
        }
    }

    fn mode_from_param(param: &str) -> Option<UnitMode> {
        match param {
            "melee" => Some(UnitMode::Melee),
            "campaign" => Some(UnitMode::Campaign),
            _ => None,
        }
    }
}
