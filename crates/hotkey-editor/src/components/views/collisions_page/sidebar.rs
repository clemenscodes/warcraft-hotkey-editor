use dioxus::prelude::*;

use super::IslandView;
use super::mini_grid::IslandMiniGrid;

#[derive(Props, Clone, PartialEq)]
pub(super) struct IslandSidebarProps {
    pub(super) islands: Vec<IslandView>,
    pub(super) selected_island: Signal<Option<String>>,
}

/// Island sidebar: a flat list of islands sized exactly like the unit cards.
/// Each card carries a tiny empty command grid with only the island's
/// conflicting button highlighted, plus the cell letter and unit count.
#[component]
pub(super) fn IslandSidebar(props: IslandSidebarProps) -> Element {
    let islands = props.islands;
    let mut selected_island = props.selected_island;

    let selected_key = selected_island.read().clone();

    rsx! {
        aside {
            class: "unit-list island-list",
            div {
                class: "unit-list-scroll",
                div {
                    class: "unit-list-track",
                    for island in islands.iter() {
                        {
                            let key = island.key().to_owned();
                            let is_selected = selected_key.as_deref() == Some(island.key());
                            let row_class = if is_selected {
                                "unit-card island-row selected"
                            } else {
                                "unit-card island-row"
                            };
                            let collision_column = island.position_column();
                            let collision_row = island.position_row();
                            let collision_count = island.collision_count();
                            let collision_noun = if collision_count == 1 {
                                "collision"
                            } else {
                                "collisions"
                            };
                            rsx! {
                                button {
                                    key: "{island.key()}",
                                    class: row_class,
                                    "data-island-key": "{island.key()}",
                                    onclick: move |_| selected_island.set(Some(key.clone())),
                                    IslandMiniGrid { collision_column, collision_row }
                                    div { class: "island-row-meta",
                                        div { class: "island-coord-group",
                                            span { class: "island-coord", "Column {collision_column}" }
                                            span { class: "island-coord-sep", "\u{00b7}" }
                                            span { class: "island-coord", "Row {collision_row}" }
                                        }
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
