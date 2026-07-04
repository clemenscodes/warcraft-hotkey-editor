mod document_head;
mod hooks;
mod nav_params;
mod route;
mod style;
mod workbench;

use crate::app::route::Route;
use dioxus::prelude::*;
use warcraft_api::Race;
use warcraft_database::UnitMode;

const TAILWIND_STYLES: Asset = asset!("/assets/tailwind.css");
const KEYBOARD_NAVIGATION_SCRIPT: Asset = asset!("/assets/keyboard-navigation.js");
const FAVICON: Asset = asset!("/assets/favicon.svg");

/// The history-significant slice of editor navigation state: race, mode,
/// selected unit, and search query. Changing any of these pushes a new browser
/// history entry (so the back button steps through editor selections), whereas
/// an entry-only change (a collision/cascade breadcrumb) merely replaces. Used
/// only to decide push-vs-replace when syncing the URL — it is not reactive.
#[derive(Clone, PartialEq, Eq)]
struct EditorNavKey {
    race: Race,
    unit_mode: UnitMode,
    unit_id: Option<String>,
    query: String,
}

/// The application root. It mounts the Dioxus `Router`, whose single
/// `Route::Workbench` renders the whole editor from the URL's query parameters —
/// so the router now owns history and URL synchronisation in place of the old
/// hand-rolled `UrlNavigationState`/`popstate` machinery.
#[component]
pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
