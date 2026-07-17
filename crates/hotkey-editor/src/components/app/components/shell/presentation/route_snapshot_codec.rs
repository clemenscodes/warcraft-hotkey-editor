use crate::components::app::route::Route;
use crate::services::navigation::app_view::CollisionKind;
use crate::services::navigation::editor_navigation::DecodedEditorNavigation;
use crate::services::navigation::navigation_snapshot::NavigationSnapshot;

impl From<&Route> for NavigationSnapshot {
    fn from(route: &Route) -> Self {
        match route {
            Route::Editor {
                race,
                mode,
                unit,
                search_query,
            } => {
                let navigation = DecodedEditorNavigation::decode(
                    race.as_deref(),
                    mode.as_deref(),
                    unit.as_deref(),
                    search_query.as_deref(),
                );
                Self::Editor(navigation)
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

impl From<&NavigationSnapshot> for Route {
    fn from(snapshot: &NavigationSnapshot) -> Self {
        match snapshot {
            NavigationSnapshot::Editor(navigation) => {
                let race = navigation.race();
                let race_label = race.slug().to_string();
                let race = Some(race_label);
                let mode = Some(navigation.unit_modes().to_string());
                let unit = navigation
                    .selected_unit_id()
                    .map(|unit_id| unit_id.value().to_string());
                let search_query = if navigation.search_query().is_empty() {
                    None
                } else {
                    Some(navigation.search_query().to_owned())
                };
                Self::Editor {
                    race,
                    mode,
                    unit,
                    search_query,
                }
            }
            NavigationSnapshot::Collisions { kind, entry } => {
                let kind_slug = kind.kind_param().to_string();
                let selected_entry = entry.clone().filter(|value| !value.is_empty());
                Self::Collisions {
                    kind: Some(kind_slug),
                    entry: selected_entry,
                }
            }
            NavigationSnapshot::Resolve { entry } => {
                let selected_entry = entry.clone().filter(|value| !value.is_empty());
                Self::Resolve {
                    entry: selected_entry,
                }
            }
        }
    }
}
