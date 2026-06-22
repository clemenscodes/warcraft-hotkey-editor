use dioxus::prelude::*;

use super::UnitPositionUnitView;

#[derive(Props, Clone, PartialEq)]
pub(super) struct UnitPositionSidebarProps {
    pub(super) units: Vec<UnitPositionUnitView>,
    pub(super) selected_unit: Signal<Option<String>>,
}

/// Unit sidebar for per-unit position collisions: a flat list of units whose
/// own abilities clash on a command-card cell, sized like the editor's unit
/// cards. Each card shows the unit's icon, name and object id, and how many
/// cell clashes it has. Sorted by clash count descending.
#[component]
pub(super) fn UnitPositionSidebar(props: UnitPositionSidebarProps) -> Element {
    let units = props.units;
    let mut selected_unit = props.selected_unit;

    let selected_key = selected_unit.read().clone();

    rsx! {
        aside { class: "unit-list island-list",
            div { class: "unit-list-scroll",
                div { class: "unit-list-track",
                    for unit_view in units.iter() {
                        {
                            let key = unit_view.key().to_owned();
                            let is_selected = selected_key.as_deref() == Some(unit_view.key());
                            let row_class = if is_selected {
                                "unit-card island-row hotkey-unit-row selected"
                            } else {
                                "unit-card island-row hotkey-unit-row"
                            };
                            let unit = unit_view.unit();
                            let icon_url = unit.icon_url().map(str::to_owned);
                            let name = unit.name().to_owned();
                            let unit_id_label = unit.unit_id().to_owned();
                            let collision_count = unit_view.collision_count();
                            let collision_noun = if collision_count == 1 {
                                "collision"
                            } else {
                                "collisions"
                            };
                            rsx! {
                                button {
                                    key: "{unit_view.key()}",
                                    class: row_class,
                                    "data-unit-position-key": "{unit_view.key()}",
                                    onclick: move |_| selected_unit.set(Some(key.clone())),
                                    if let Some(url) = icon_url {
                                        img {
                                            class: "hotkey-unit-row-icon",
                                            src: "{url}",
                                            alt: "{name}",
                                            loading: "lazy",
                                            decoding: "async",
                                        }
                                    }
                                    div { class: "island-row-meta",
                                        span { class: "island-coord hotkey-unit-name", "{name}" }
                                        code { class: "conflict-object-id", "{unit_id_label}" }
                                        span { class: "island-collision-count",
                                            "{collision_count} {collision_noun}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
