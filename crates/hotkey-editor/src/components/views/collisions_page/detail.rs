use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};

use crate::components::dialogs::dialog_header::DialogHeader;
use crate::services::navigation::view_navigation::ViewNavigationContext;

use super::mini_grid::IslandMiniGrid;
use super::{CarrierDialogData, IslandView};

#[derive(Props, Clone, PartialEq)]
struct ConflictUnitProps {
    unit_id: String,
    icon_url: Option<String>,
    name: String,
    view_navigation: ViewNavigationContext,
}

/// The affected unit heading a conflict card: a big icon + name + object id
/// that deep-links into the editor focused on that unit.
#[component]
fn ConflictUnit(props: ConflictUnitProps) -> Element {
    let unit_id = props.unit_id;
    let icon_url = props.icon_url;
    let name = props.name;
    let view_navigation = props.view_navigation;
    let unit_id_label = unit_id.clone();

    rsx! {
        button {
            class: "conflict-unit",
            r#type: "button",
            onclick: move |_| view_navigation.open_unit(&unit_id),
            if let Some(url) = icon_url {
                img {
                    class: "conflict-unit-icon",
                    src: "{url}",
                    alt: "{name}",
                    loading: "lazy",
                    decoding: "async",
                }
            }
            span { class: "conflict-unit-name", "{name}" }
            code { class: "conflict-object-id", "{unit_id_label}" }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ConflictAbilityProps {
    ability_name: String,
    ability_id: String,
    icon_url: Option<String>,
    extra_count: usize,
    carrier_unit_ids: Vec<String>,
    carrier_dialog: Signal<Option<CarrierDialogData>>,
}

/// One ability of a conflict: a big icon with the name + object id below and,
/// when carried by more units, a "+N more" link. Both the icon and the "+N
/// more" link open the same carriers dialog for this ability.
#[component]
fn ConflictAbility(props: ConflictAbilityProps) -> Element {
    let ability_name = props.ability_name;
    let ability_id = props.ability_id;
    let icon_url = props.icon_url;
    let extra_count = props.extra_count;
    let carrier_unit_ids = props.carrier_unit_ids;
    let mut carrier_dialog = props.carrier_dialog;

    let open_from_icon = {
        let dialog_name = ability_name.clone();
        let dialog_carrier_unit_ids = carrier_unit_ids.clone();
        move |_| {
            let data = CarrierDialogData::new(dialog_name.clone(), &dialog_carrier_unit_ids);
            carrier_dialog.set(Some(data));
        }
    };
    let open_from_more = {
        let dialog_name = ability_name.clone();
        let dialog_carrier_unit_ids = carrier_unit_ids.clone();
        move |_| {
            let data = CarrierDialogData::new(dialog_name.clone(), &dialog_carrier_unit_ids);
            carrier_dialog.set(Some(data));
        }
    };

    rsx! {
        div { class: "conflict-ability",
            button {
                class: "conflict-ability-trigger",
                r#type: "button",
                onclick: open_from_icon,
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
            if extra_count > 0 {
                button {
                    class: "conflict-more",
                    r#type: "button",
                    onclick: open_from_more,
                    "+{extra_count} more"
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub(super) struct IslandDetailProps {
    pub(super) islands: Vec<IslandView>,
    pub(super) selected_island: Signal<Option<String>>,
    pub(super) view_navigation: ViewNavigationContext,
}

/// Island detail pane: a header that mirrors the island card (the mini grid,
/// the coordinate, the collision count), then a scrollable list of conflicts —
/// one card per affected unit, showing the two abilities of that unit which
/// land on the same cell, each a button into the full carriers dialog.
#[component]
pub(super) fn IslandDetail(props: IslandDetailProps) -> Element {
    let islands = props.islands;
    let selected_island = props.selected_island;
    let view_navigation = props.view_navigation;

    let carrier_dialog = use_signal(|| None::<CarrierDialogData>);

    let selected_key = selected_island.read().clone();
    let selected = selected_key
        .as_ref()
        .and_then(|key| islands.iter().find(|island| island.key() == key).cloned());

    let Some(island) = selected else {
        return rsx! {
            section { class: "unit-detail island-detail empty",
                p { "Select a collision to inspect." }
            }
        };
    };

    let position_column = island.position_column();
    let position_row = island.position_row();
    let collision_count = island.collision_count();
    let collision_noun = if collision_count == 1 {
        "collision"
    } else {
        "collisions"
    };

    let carrier_dialog_state = carrier_dialog.read().clone();

    rsx! {
        section { class: "unit-detail island-detail",
            header { class: "island-detail-header",
                IslandMiniGrid { collision_column: position_column, collision_row: position_row }
                div { class: "island-row-meta",
                    div { class: "island-coord-group",
                        span { class: "island-coord", "Column {position_column}" }
                        span { class: "island-coord-sep", "\u{00b7}" }
                        span { class: "island-coord", "Row {position_row}" }
                    }
                    span { class: "island-collision-count", "{collision_count} {collision_noun}" }
                }
            }
            div { class: "conflict-grid",
                for (conflict_index, conflict) in island.conflicts().iter().enumerate() {
                    div { key: "conflict-{conflict_index}", class: "conflict-card",
                        ConflictUnit {
                            unit_id: conflict.unit().unit_id().to_owned(),
                            icon_url: conflict.unit().icon_url().map(str::to_owned),
                            name: conflict.unit().name().to_owned(),
                            view_navigation,
                        }
                        div { class: "conflict-ability-row",
                            ConflictAbility {
                                ability_name: conflict.own_ability().ability().name().to_owned(),
                                ability_id: conflict.own_ability().ability().object_id().to_owned(),
                                icon_url: conflict.own_ability().ability().icon_url().map(str::to_owned),
                                extra_count: conflict.own_ability().extra_count(),
                                carrier_unit_ids: conflict.own_ability().carrier_unit_ids().to_vec(),
                                carrier_dialog,
                            }
                            span { class: "conflict-separator", aria_hidden: "true", "\u{2715}" }
                            ConflictAbility {
                                ability_name: conflict.shared_ability().ability().name().to_owned(),
                                ability_id: conflict.shared_ability().ability().object_id().to_owned(),
                                icon_url: conflict.shared_ability().ability().icon_url().map(str::to_owned),
                                extra_count: conflict.shared_ability().extra_count(),
                                carrier_unit_ids: conflict.shared_ability().carrier_unit_ids().to_vec(),
                                carrier_dialog,
                            }
                        }
                    }
                }
            }
        }
        if let Some(dialog_data) = carrier_dialog_state {
            CarriersDialog { dialog_data, carrier_dialog, view_navigation }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct CarriersDialogProps {
    dialog_data: CarrierDialogData,
    carrier_dialog: Signal<Option<CarrierDialogData>>,
    view_navigation: ViewNavigationContext,
}

/// Lists every unit that carries the shared ability in a scrollable grid.
/// Each card deep-links into the editor focused on that unit.
#[component]
fn CarriersDialog(props: CarriersDialogProps) -> Element {
    let dialog_data = props.dialog_data;
    let mut carrier_dialog = props.carrier_dialog;
    let view_navigation = props.view_navigation;

    let title = dialog_data.ability_name().to_owned();
    let handle_open_change = move |is_open: bool| {
        if !is_open {
            carrier_dialog.set(None);
        }
    };
    let handle_close = move |_| carrier_dialog.set(None);

    rsx! {
        DialogRoot {
            class: "dialog-overlay",
            open: true,
            on_open_change: handle_open_change,
            DialogContent { class: "dialog-shell wc3-dialog carriers-dialog".to_string(),
                DialogHeader { title, on_close: handle_close }
                div { class: "wc3-dialog-body carriers-dialog-body",
                    div { class: "carriers-grid",
                        for (carrier_index, carrier) in dialog_data.carriers().iter().enumerate() {
                            {
                                let carrier_id = carrier.unit_id().to_owned();
                                let carrier_id_label = carrier_id.clone();
                                rsx! {
                                    button {
                                        key: "carrier-{carrier_index}",
                                        class: "carrier-card",
                                        r#type: "button",
                                        onclick: move |_| view_navigation.open_unit(&carrier_id),
                                        if let Some(url) = carrier.icon_url() {
                                            img {
                                                class: "carrier-card-icon",
                                                src: "{url}",
                                                alt: "{carrier.name()}",
                                                loading: "lazy",
                                                decoding: "async",
                                            }
                                        }
                                        span { class: "carrier-card-name", "{carrier.name()}" }
                                        code { class: "conflict-object-id", "{carrier_id_label}" }
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
