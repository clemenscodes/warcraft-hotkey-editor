use warcraft_api::Race;
#[cfg(target_arch = "wasm32")]
use warcraft_api::RaceLabels;
use warcraft_database::{UnitKindHelpers, UnitMode};

use crate::services::navigation::app_view::AppView;

pub(crate) struct UrlNavigationState {
    race: Race,
    unit_mode: UnitMode,
    selected_unit_id: Option<String>,
    search_query: String,
    view: AppView,
}

impl UrlNavigationState {
    pub(crate) fn race(&self) -> Race {
        self.race
    }

    pub(crate) fn unit_mode(&self) -> UnitMode {
        self.unit_mode
    }

    pub(crate) fn selected_unit_id(&self) -> Option<&str> {
        self.selected_unit_id.as_deref()
    }

    pub(crate) fn search_query(&self) -> &str {
        &self.search_query
    }

    pub(crate) fn view(&self) -> AppView {
        self.view
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
            .and_then(|param| Race::try_from(param).ok())
            .unwrap_or(Race::Human);

        let mode_param = params.get("mode");
        let unit_mode = mode_param
            .as_deref()
            .and_then(|param| UnitMode::try_from(param).ok())
            .unwrap_or(UnitMode::Melee);

        let unit_param = params.get("unit");
        let selected_unit_id = unit_param
            .filter(|id| !id.is_empty())
            .or_else(|| UnitKindHelpers::default_unit_id_for(race, unit_mode));

        let search_query = params.get("q").unwrap_or_default();

        let view_param = params.get("view");
        let kind_param = params.get("kind");
        let view = AppView::from_query_params(view_param.as_deref(), kind_param.as_deref());

        Self {
            race,
            unit_mode,
            selected_unit_id,
            search_query,
            view,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn replace_in_url(
        _race: Race,
        _unit_mode: UnitMode,
        _unit_id: Option<&str>,
        _query: &str,
        _view: AppView,
    ) {
    }

    /// Update the URL in place (no history entry) — used for editor
    /// navigation like switching race/mode/unit/search.
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn replace_in_url(
        race: Race,
        unit_mode: UnitMode,
        unit_id: Option<&str>,
        query: &str,
        view: AppView,
    ) {
        let url = build_url(race, unit_mode, unit_id, query, view);
        let Some(window) = web_sys::window() else {
            return;
        };
        let Ok(history) = window.history() else {
            return;
        };
        let _ = history.replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&url));
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn push_view_to_url(
        _race: Race,
        _unit_mode: UnitMode,
        _unit_id: Option<&str>,
        _query: &str,
        _view: AppView,
    ) {
    }

    /// Push a new URL onto the history stack — used when the user
    /// switches between Editor / Collisions / Resolve, so browser
    /// back/forward navigates between views.
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn push_view_to_url(
        race: Race,
        unit_mode: UnitMode,
        unit_id: Option<&str>,
        query: &str,
        view: AppView,
    ) {
        let url = build_url(race, unit_mode, unit_id, query, view);
        let Some(window) = web_sys::window() else {
            return;
        };
        let Ok(history) = window.history() else {
            return;
        };
        let _ = history.push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&url));
    }

    /// Installs a one-shot `popstate` listener on the window that fires
    /// the supplied callback with a freshly parsed `UrlNavigationState`
    /// whenever the user navigates back/forward in browser history.
    /// The underlying `Closure` is leaked so the listener stays alive
    /// for the page's lifetime (the listener has no detach path —
    /// installation is per-app boot).
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn install_popstate_listener<F>(mut callback: F)
    where
        F: FnMut(UrlNavigationState) + 'static,
    {
        use wasm_bindgen::JsCast;
        use wasm_bindgen::closure::Closure;
        let Some(window) = web_sys::window() else {
            return;
        };
        let closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |_event: web_sys::Event| {
            let nav_state = UrlNavigationState::from_url();
            callback(nav_state);
        });
        let listener_ref = closure.as_ref().unchecked_ref();
        let _ = window.add_event_listener_with_callback("popstate", listener_ref);
        closure.forget();
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn install_popstate_listener<F>(_callback: F)
    where
        F: FnMut(UrlNavigationState) + 'static,
    {
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
            view: AppView::default_view(),
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn build_url(
    race: Race,
    unit_mode: UnitMode,
    unit_id: Option<&str>,
    query: &str,
    view: AppView,
) -> String {
    let race_param = RaceLabels::data_attribute(race);
    let mut url = format!("?race={race_param}&mode={unit_mode}");

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

    let view_param = view.view_param();
    if view_param != "editor" {
        url.push_str("&view=");
        url.push_str(view_param);
        if let Some(kind_param) = view.kind_param() {
            url.push_str("&kind=");
            url.push_str(kind_param);
        }
    }

    url
}
