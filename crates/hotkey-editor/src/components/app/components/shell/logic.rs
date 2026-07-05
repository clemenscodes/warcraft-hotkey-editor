use crate::components::app::route::Route;
use crate::services::navigation::app_view::{AppView, CollisionKind};
use crate::services::navigation::editor_nav::DecodedEditorNav;
use crate::services::navigation::nav_snapshot::NavSnapshot;

/// The app's opening state, decoded once from the entry URL: the canonical route the
/// address bar should show, whether the entry URL needs canonicalizing, and every
/// initial value the shell's signals seed from.
///
/// A bare or partial URL (`/`, `/collisions`) decodes to the same state as its
/// fully-materialized form (`/?race=human&mode=melee&unit=…`); `canonical_route` is
/// that materialized form and `needs_canonicalize` is whether the entry differed, so
/// the shell can replace the address bar once on entry.
#[derive(Clone)]
pub(super) struct RouteBootstrap {
    pub(super) snapshot: NavSnapshot,
    pub(super) canonical_route: Route,
    pub(super) needs_canonicalize: bool,
    pub(super) view: AppView,
    pub(super) nav: DecodedEditorNav,
    pub(super) selected_island: Option<String>,
    pub(super) selected_hotkey_unit: Option<String>,
    pub(super) selected_unit_position: Option<String>,
    pub(super) selected_move_category: Option<String>,
}

impl From<&Route> for RouteBootstrap {
    fn from(initial_route: &Route) -> Self {
        let snapshot = NavSnapshot::from(initial_route);
        let canonical_route = Route::from(&snapshot);
        let needs_canonicalize = *initial_route != canonical_route;
        let view = match &snapshot {
            NavSnapshot::Editor(_) => AppView::Editor,
            NavSnapshot::Collisions { kind, .. } => AppView::Collisions { kind: *kind },
            NavSnapshot::Resolve { .. } => AppView::Resolve,
        };
        let nav = match &snapshot {
            NavSnapshot::Editor(nav) => nav.clone(),
            _ => DecodedEditorNav::decode(None, None, None, None),
        };
        let selected_island = match &snapshot {
            NavSnapshot::Collisions {
                kind: CollisionKind::Positions,
                entry,
            } => entry.clone(),
            _ => None,
        };
        let selected_hotkey_unit = match &snapshot {
            NavSnapshot::Collisions {
                kind: CollisionKind::Hotkeys,
                entry,
            } => entry.clone(),
            _ => None,
        };
        let selected_unit_position = match &snapshot {
            NavSnapshot::Collisions {
                kind: CollisionKind::UnitPositions,
                entry,
            } => entry.clone(),
            _ => None,
        };
        let selected_move_category = match &snapshot {
            NavSnapshot::Resolve { entry } => entry.clone(),
            _ => None,
        };
        Self {
            snapshot,
            canonical_route,
            needs_canonicalize,
            view,
            nav,
            selected_island,
            selected_hotkey_unit,
            selected_unit_position,
            selected_move_category,
        }
    }
}
