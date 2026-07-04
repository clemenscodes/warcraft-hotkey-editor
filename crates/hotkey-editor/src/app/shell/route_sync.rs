use crate::app::route::Route;
use crate::services::navigation::app_view::CollisionKind;
use crate::services::navigation::editor_nav::DecodedEditorNav;
use crate::services::navigation::nav_snapshot::NavSnapshot;
use warcraft_api::RaceLabels;

impl From<&Route> for NavSnapshot {
    fn from(route: &Route) -> Self {
        match route {
            Route::Editor {
                race,
                mode,
                unit,
                q,
            } => {
                let nav = DecodedEditorNav::decode(
                    race.as_deref(),
                    mode.as_deref(),
                    unit.as_deref(),
                    q.as_deref(),
                );
                Self::Editor(nav)
            }
            Route::Collisions { kind, entry } => {
                let collision_kind = CollisionKind::from_query_param(kind.as_deref());
                let selected_entry = entry.clone().filter(|value| !value.is_empty());
                Self::Collisions {
                    kind: collision_kind,
                    entry: selected_entry,
                }
            }
            Route::Resolve { entry } => {
                let selected_entry = entry.clone().filter(|value| !value.is_empty());
                Self::Resolve {
                    entry: selected_entry,
                }
            }
        }
    }
}

impl From<&NavSnapshot> for Route {
    fn from(snapshot: &NavSnapshot) -> Self {
        match snapshot {
            NavSnapshot::Editor(nav) => {
                let race_label = RaceLabels::data_attribute(nav.race).to_string();
                let race = Some(race_label);
                let mode = Some(nav.unit_mode.to_string());
                let unit = nav
                    .selected_unit_id
                    .clone()
                    .filter(|value| !value.is_empty());
                let q = if nav.search_query.is_empty() {
                    None
                } else {
                    Some(nav.search_query.clone())
                };
                Self::Editor {
                    race,
                    mode,
                    unit,
                    q,
                }
            }
            NavSnapshot::Collisions { kind, entry } => {
                let kind_slug = kind.kind_param().to_string();
                let selected_entry = entry.clone().filter(|value| !value.is_empty());
                Self::Collisions {
                    kind: Some(kind_slug),
                    entry: selected_entry,
                }
            }
            NavSnapshot::Resolve { entry } => {
                let selected_entry = entry.clone().filter(|value| !value.is_empty());
                Self::Resolve {
                    entry: selected_entry,
                }
            }
        }
    }
}

/// How the shell should reconcile the URL after a state change: leave it, push a new
/// history entry, replace the current one, or treat it as an in-flight search-typing
/// session (push the first keystroke, replace the rest).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NavDecision {
    Skip,
    Push,
    Replace,
    SessionQuery,
}

impl NavDecision {
    /// Decide how to move from the snapshot currently in the address bar (`live`) to
    /// the one the shell's signals now describe (`target`).
    ///
    /// - identical → `Skip` (this is what absorbs the echo when a page reconciles the
    ///   route back into the signals; the guard is peeked, never subscribed);
    /// - same editor selection but a different search query → `SessionQuery`;
    /// - same collision kind (or the same resolve page) but a different list entry →
    ///   `Replace` (picking a collision/cascade entry must not spam history);
    /// - anything else (a page switch, a race/mode/unit change, a collision kind
    ///   change) → `Push`.
    pub fn between(live: &NavSnapshot, target: &NavSnapshot) -> Self {
        if live == target {
            return Self::Skip;
        }
        match (live, target) {
            (NavSnapshot::Editor(live_nav), NavSnapshot::Editor(target_nav)) => {
                let same_race_mode_unit = live_nav.race == target_nav.race
                    && live_nav.unit_mode == target_nav.unit_mode
                    && live_nav.selected_unit_id == target_nav.selected_unit_id;
                if same_race_mode_unit {
                    Self::SessionQuery
                } else {
                    Self::Push
                }
            }
            (
                NavSnapshot::Collisions {
                    kind: live_kind, ..
                },
                NavSnapshot::Collisions {
                    kind: target_kind, ..
                },
            ) => {
                if live_kind == target_kind {
                    Self::Replace
                } else {
                    Self::Push
                }
            }
            (NavSnapshot::Resolve { .. }, NavSnapshot::Resolve { .. }) => Self::Replace,
            _ => Self::Push,
        }
    }
}
