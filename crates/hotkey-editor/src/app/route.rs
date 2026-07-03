use crate::app::workbench::Workbench;
use dioxus::prelude::*;

/// The application's single client-side route.
///
/// The workbench is a one-page app whose entire navigable state — which race, unit
/// mode, and unit are selected; the live search query; which top-level view
/// (editor / collisions / resolve) is showing, its collision kind, and the selected
/// list entry — rides in the URL's query string. Keeping every field a query
/// parameter preserves the exact URL shape the app has always used
/// (`?race=…&mode=…&unit=…&q=…&view=…&kind=…&entry=…`), so deep links, the
/// back/forward buttons, and the e2e suite all keep working unchanged — while the
/// Dioxus `Router` now owns history and URL synchronisation in place of the old
/// hand-rolled `UrlNavigationState` + `popstate` machinery.
///
/// Every field is a `String`: an absent parameter arrives as the empty string, and
/// the `Workbench` component normalises each into the typed domain/UI value it
/// needs (`Race`, `UnitMode`, `AppView`, …), exactly as the old `from_url()` parser
/// did.
#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/?:race&:mode&:unit&:q&:view&:kind&:entry")]
    Workbench {
        race: Option<String>,
        mode: Option<String>,
        unit: Option<String>,
        q: Option<String>,
        view: Option<String>,
        kind: Option<String>,
        entry: Option<String>,
    },
}
