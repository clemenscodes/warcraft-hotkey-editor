use crate::app::route::Route;
use crate::services::navigation::app_view::AppView;
use warcraft_api::{Race, RaceLabels};
use warcraft_database::{UnitKindHelpers, UnitMode};

/// The navigable state decoded from — or encoded back into — the URL's query
/// parameters. This is the router's replacement for the old `UrlNavigationState`
/// parse (`from_url`) and serialize (`build_url`): the same `?race=…&mode=…&unit=…
/// &q=…&view=…&kind=…&entry=…` shape, expressed as a typed value the workbench
/// initialises its signals from, and rebuilds from those signals to drive
/// `navigator().push/replace`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NavState {
    race: Race,
    unit_mode: UnitMode,
    selected_unit_id: Option<String>,
    search_query: String,
    view: AppView,
    selected_entry: Option<String>,
}

impl NavState {
    /// Assemble navigation state from the workbench's live signals, ready to encode
    /// into a `Route` for `navigator().push/replace`.
    pub fn new(
        race: Race,
        unit_mode: UnitMode,
        selected_unit_id: Option<String>,
        search_query: String,
        view: AppView,
        selected_entry: Option<String>,
    ) -> Self {
        Self {
            race,
            unit_mode,
            selected_unit_id,
            search_query,
            view,
            selected_entry,
        }
    }

    pub fn race(&self) -> Race {
        self.race
    }

    pub fn unit_mode(&self) -> UnitMode {
        self.unit_mode
    }

    pub fn selected_unit_id(&self) -> Option<&str> {
        self.selected_unit_id.as_deref()
    }

    pub fn search_query(&self) -> &str {
        &self.search_query
    }

    pub fn view(&self) -> AppView {
        self.view
    }

    pub fn selected_entry(&self) -> Option<&str> {
        self.selected_entry.as_deref()
    }

    /// Decode the seven query-parameter strings the router hands the workbench into
    /// typed navigation state, mirroring the old `from_url` fallbacks: an unknown or
    /// empty race/mode falls back to Human/Melee, an empty unit falls back to the
    /// race+mode default unit, and an empty view is the editor.
    pub fn decode(params: &RouteParams) -> Self {
        let race_param = params.race.as_deref().unwrap_or_default();
        let race = Race::try_from(race_param).unwrap_or(Race::Human);
        let mode_param = params.mode.as_deref().unwrap_or_default();
        let unit_mode = UnitMode::try_from(mode_param).unwrap_or(UnitMode::Melee);
        let unit_param = params.unit.as_deref().unwrap_or_default();
        let selected_unit_id = if unit_param.is_empty() {
            UnitKindHelpers::default_unit_id_for(race, unit_mode)
        } else {
            Some(unit_param.to_string())
        };
        let search_query = params.q.clone().unwrap_or_default();
        let view_param = params.view.as_deref().filter(|value| !value.is_empty());
        let kind_param = params.kind.as_deref().filter(|value| !value.is_empty());
        let view = AppView::from_query_params(view_param, kind_param);
        let selected_entry = params.entry.clone().filter(|value| !value.is_empty());
        Self {
            race,
            unit_mode,
            selected_unit_id,
            search_query,
            view,
            selected_entry,
        }
    }

    /// Encode this navigation state back into a `Route`, ready for
    /// `navigator().push(route)` / `.replace(route)`. Empty strings stand in for
    /// absent parameters (the editor view, no selected unit/entry, empty search),
    /// so the rendered URL keeps its historical minimal shape.
    pub fn to_route(&self) -> Route {
        let race = Some(RaceLabels::data_attribute(self.race).to_string());
        let mode = Some(self.unit_mode.to_string());
        let unit = self
            .selected_unit_id
            .clone()
            .filter(|value| !value.is_empty());
        let q = if self.search_query.is_empty() {
            None
        } else {
            Some(self.search_query.clone())
        };
        let view_param = self.view.view_param();
        let view = if view_param == "editor" {
            None
        } else {
            Some(view_param.to_string())
        };
        let kind = self.view.kind_param().map(str::to_string);
        let entry = self
            .selected_entry
            .clone()
            .filter(|value| !value.is_empty());
        Route::Workbench {
            race,
            mode,
            unit,
            q,
            view,
            kind,
            entry,
        }
    }
}

/// The seven query-parameter strings the `#[derive(Routable)]` `Route::Workbench`
/// variant carries. Grouped so the workbench component can take them as one bundle
/// and decode them in a single step.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct RouteParams {
    pub race: Option<String>,
    pub mode: Option<String>,
    pub unit: Option<String>,
    pub q: Option<String>,
    pub view: Option<String>,
    pub kind: Option<String>,
    pub entry: Option<String>,
}
