use dioxus::prelude::*;

use crate::services::navigation::view_navigation::ViewNavigationContext;

use super::mini_grid::IslandMiniGrid;
use super::{UnitPositionConflictView, UnitPositionUnitView};

#[derive(Props, Clone, PartialEq)]
struct UnitPositionAbilityProps {
    ability_name: String,
    ability_id: String,
    icon_url: Option<String>,
    unit_id: String,
    view_navigation: ViewNavigationContext,
}

/// One ability of a position conflict: a big icon (a button that deep-links into
/// the editor focused on the unit) with the name and object id below it.
#[component]
fn UnitPositionAbility(props: UnitPositionAbilityProps) -> Element {
    let ability_name = props.ability_name;
    let ability_id = props.ability_id;
    let icon_url = props.icon_url;
    let unit_id = props.unit_id;
    let view_navigation = props.view_navigation;

    rsx! {
        div { class: "conflict-ability",
            button {
                class: "conflict-ability-trigger",
                r#type: "button",
                onclick: move |_| view_navigation.open_unit(&unit_id),
                if let Some(url) = icon_url {
                    img {
                        class: "conflict-ability-icon",
                        src: "{url}",
                        alt: "{ability_name}",
                        loading: "lazy",
                        decoding: "async",
                    }
                }
            }
            span { class: "conflict-ability-name", "{ability_name}" }
            code { class: "conflict-object-id", "{ability_id}" }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct UnitPositionConflictCardProps {
    conflict: UnitPositionConflictView,
    unit_id: String,
    view_navigation: ViewNavigationContext,
}

/// One conflict card: the unit's abilities that land on the same command-card
/// cell, flanking a mini grid that flags the colliding cell. The caption names
/// the command card. A pair sits either side of the grid; a rarer 3+ way clash
/// stacks the grid above a centered row of all the abilities.
#[component]
fn UnitPositionConflictCard(props: UnitPositionConflictCardProps) -> Element {
    let conflict = props.conflict;
    let unit_id = props.unit_id;
    let view_navigation = props.view_navigation;

    let role_label = conflict.role_label().to_owned();
    let position_column = conflict.position_column();
    let position_row = conflict.position_row();
    let abilities = conflict.abilities();
    let is_pair = abilities.len() == 2;

    rsx! {
        div { class: "conflict-card hotkey-conflict-card",
            span { class: "conflict-card-caption", "{role_label}" }
            if is_pair {
                div { class: "conflict-ability-row",
                    UnitPositionAbility {
                        ability_name: abilities[0].name().to_owned(),
                        ability_id: abilities[0].object_id().to_owned(),
                        icon_url: abilities[0].icon_url().map(str::to_owned),
                        unit_id: unit_id.clone(),
                        view_navigation,
                    }
                    span { class: "conflict-position-cell",
                        IslandMiniGrid {
                            collision_column: position_column,
                            collision_row: position_row,
                        }
                    }
                    UnitPositionAbility {
                        ability_name: abilities[1].name().to_owned(),
                        ability_id: abilities[1].object_id().to_owned(),
                        icon_url: abilities[1].icon_url().map(str::to_owned),
                        unit_id: unit_id.clone(),
                        view_navigation,
                    }
                }
            } else {
                span { class: "conflict-position-cell conflict-position-cell-top",
                    IslandMiniGrid {
                        collision_column: position_column,
                        collision_row: position_row,
                    }
                }
                div { class: "conflict-ability-row conflict-ability-row-multi",
                    for ability in abilities.iter() {
                        UnitPositionAbility {
                            key: "{ability.object_id()}",
                            ability_name: ability.name().to_owned(),
                            ability_id: ability.object_id().to_owned(),
                            icon_url: ability.icon_url().map(str::to_owned),
                            unit_id: unit_id.clone(),
                            view_navigation,
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub(super) struct UnitPositionDetailProps {
    pub(super) units: Vec<UnitPositionUnitView>,
    pub(super) selected_unit: Signal<Option<String>>,
    pub(super) view_navigation: ViewNavigationContext,
}

/// Per-unit position-collision detail pane: a header naming the selected unit
/// (a button into the editor) with its clash count, then one card per cell where
/// the unit's own abilities collide.
#[component]
pub(super) fn UnitPositionDetail(props: UnitPositionDetailProps) -> Element {
    let units = props.units;
    let selected_unit = props.selected_unit;
    let view_navigation = props.view_navigation;

    let selected_key = selected_unit.read().clone();
    let selected = selected_key.as_ref().and_then(|key| {
        units
            .iter()
            .find(|unit_view| unit_view.key() == key)
            .cloned()
    });

    let Some(unit_view) = selected else {
        return rsx! {
            section { class: "unit-detail island-detail empty",
                p { "Select a unit to inspect." }
            }
        };
    };

    let unit = unit_view.unit();
    let unit_name = unit.name().to_owned();
    let unit_id_label = unit.unit_id().to_owned();
    let unit_icon = unit.icon_url().map(str::to_owned);
    let unit_id_for_nav = unit.unit_id().to_owned();
    let collision_count = unit_view.collision_count();
    let collision_noun = if collision_count == 1 {
        "collision"
    } else {
        "collisions"
    };

    rsx! {
        section { class: "unit-detail island-detail",
            header { class: "island-detail-header",
                button {
                    class: "hotkey-detail-unit",
                    r#type: "button",
                    onclick: move |_| view_navigation.open_unit(&unit_id_for_nav),
                    if let Some(url) = unit_icon {
                        img {
                            class: "hotkey-detail-unit-icon",
                            src: "{url}",
                            alt: "{unit_name}",
                            loading: "lazy",
                            decoding: "async",
                        }
                    }
                }
                div { class: "island-row-meta",
                    span { class: "island-coord", "{unit_name}" }
                    code { class: "conflict-object-id", "{unit_id_label}" }
                    span { class: "island-collision-count", "{collision_count} {collision_noun}" }
                }
            }
            div { class: "conflict-grid",
                for (conflict_index, conflict) in unit_view.conflicts().iter().enumerate() {
                    UnitPositionConflictCard {
                        key: "unit-position-conflict-{conflict_index}",
                        conflict: conflict.clone(),
                        unit_id: unit_view.key().to_owned(),
                        view_navigation,
                    }
                }
            }
        }
    }
}
