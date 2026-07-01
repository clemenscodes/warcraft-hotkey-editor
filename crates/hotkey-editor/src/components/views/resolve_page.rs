use crate::components::dialogs::dialog::Dialog;
use crate::components::shared::icons::ICON_COLLISIONS_CLEAR;
use crate::model::icons::IconUrl;
use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;
use dioxus_primitives::toast::{ToastOptions, use_toast};
use gloo_timers::future::TimeoutFuture;
use std::collections::{HashMap, HashSet};
use warcraft_database::ObjectLookup;
use warcraft_keybinds::{COMMAND_GRID_COLUMNS, COMMAND_GRID_ROWS};
use warcraft_keybinds::{CustomKeys, GridSlotId, MoveReason};

/// One ability resolved to an icon, display name, and object id for the plan.
#[derive(Clone, PartialEq)]
struct AbilityDisplay {
    object_id: String,
    name: String,
    icon_url: Option<String>,
}

impl AbilityDisplay {
    fn resolve(slot_id: GridSlotId) -> Self {
        let id_value = slot_id.id().value();
        let object_option = ObjectLookup::by_id(id_value);
        let icon_url = object_option
            .and_then(|object| object.icons().first().copied())
            .map(IconUrl::from_database_path)
            .map(|icon| icon.to_string());
        let name_option = object_option.and_then(|object| object.names().first().copied());
        let name = match name_option {
            Some(resolved) => resolved.to_owned(),
            None => slot_id.display_name(None, None),
        };
        let object_id = slot_id.as_str().to_owned();
        Self {
            object_id,
            name,
            icon_url,
        }
    }
}

/// One unit resolved to its id, name, and icon for the carriers dialog.
#[derive(Clone, PartialEq)]
struct UnitDisplay {
    unit_id: String,
    name: String,
    icon_url: Option<String>,
}

impl UnitDisplay {
    fn resolve(unit_id_value: &str) -> Self {
        let object_option = ObjectLookup::by_id(unit_id_value);
        let icon_url = object_option
            .and_then(|object| object.icons().first().copied())
            .map(IconUrl::from_database_path)
            .map(|icon| icon.to_string());
        let name_option = object_option.and_then(|object| object.names().first().copied());
        let name = name_option.unwrap_or(unit_id_value).to_owned();
        let unit_id = unit_id_value.to_owned();
        Self {
            unit_id,
            name,
            icon_url,
        }
    }
}

/// Which kind of move this is. Drives both grouping into sections and the order
/// the sections render in (Fights first, then Gap pulls, Spills, Swaps).
#[derive(Clone, Copy, PartialEq, Eq)]
enum MoveCategory {
    Fight,
    GapPull,
    Spill,
    Swap,
}

impl MoveCategory {
    const ORDER: [Self; 4] = [Self::Fight, Self::GapPull, Self::Spill, Self::Swap];

    fn section_title(self) -> &'static str {
        match self {
            Self::Fight => "Fights",
            Self::GapPull => "Gap pulls",
            Self::Spill => "Spills",
            Self::Swap => "Swaps",
        }
    }

    fn data_breadcrumb(self) -> &'static str {
        match self {
            Self::Fight => "fights",
            Self::GapPull => "gap-pulls",
            Self::Spill => "spills",
            Self::Swap => "swaps",
        }
    }

    /// Parses the `data_breadcrumb` slug back into a category — used to restore
    /// the selected move section from the `?entry=` URL parameter. Unknown slugs
    /// yield `None` (the page then falls back to the first section).
    fn from_data_breadcrumb(slug: &str) -> Option<Self> {
        match slug {
            "fights" => Some(Self::Fight),
            "gap-pulls" => Some(Self::GapPull),
            "spills" => Some(Self::Spill),
            "swaps" => Some(Self::Swap),
            _ => None,
        }
    }
}

/// The display-ready pieces of a move's rationale: a short badge label + colour
/// and, for Fight/Swap, the rival ability and its carrier count.
#[derive(Clone, PartialEq)]
struct ReasonParts {
    label: &'static str,
    badge_class: &'static str,
    other_ability: Option<AbilityDisplay>,
    other_carriers: Option<usize>,
    other_carrier_unit_ids: Vec<String>,
    is_swap: bool,
    category: MoveCategory,
}

impl ReasonParts {
    fn resolve(reason: &MoveReason) -> Self {
        match reason {
            MoveReason::Fight {
                anchor_slot,
                anchor_carrier_unit_ids,
            } => {
                let anchor = AbilityDisplay::resolve(*anchor_slot);
                let mut other_carrier_unit_ids: Vec<String> =
                    Vec::with_capacity(anchor_carrier_unit_ids.len());
                for anchor_carrier_object in anchor_carrier_unit_ids {
                    let anchor_carrier_value = anchor_carrier_object.value().to_owned();
                    other_carrier_unit_ids.push(anchor_carrier_value);
                }
                let anchor_carrier_count = other_carrier_unit_ids.len();
                Self {
                    label: "Fight",
                    badge_class: "resolve-reason-fight",
                    other_ability: Some(anchor),
                    other_carriers: Some(anchor_carrier_count),
                    other_carrier_unit_ids,
                    is_swap: false,
                    category: MoveCategory::Fight,
                }
            }
            MoveReason::Swap { swapped_with } => {
                let other = AbilityDisplay::resolve(*swapped_with);
                Self {
                    label: "Swap",
                    badge_class: "resolve-reason-swap",
                    other_ability: Some(other),
                    other_carriers: None,
                    other_carrier_unit_ids: Vec::new(),
                    is_swap: true,
                    category: MoveCategory::Swap,
                }
            }
            MoveReason::Spill { .. } => Self {
                label: "Spill",
                badge_class: "resolve-reason-spill",
                other_ability: None,
                other_carriers: None,
                other_carrier_unit_ids: Vec::new(),
                is_swap: false,
                category: MoveCategory::Spill,
            },
            MoveReason::GapPull { .. } => Self {
                label: "Gap pull",
                badge_class: "resolve-reason-gappull",
                other_ability: None,
                other_carriers: None,
                other_carrier_unit_ids: Vec::new(),
                is_swap: false,
                category: MoveCategory::GapPull,
            },
        }
    }
}

/// One planned move, display-ready: the moved ability (with carriers + a unit to
/// link to), the old → new cell, and the rival ability that displaced it.
#[derive(Clone, PartialEq)]
struct ResolveMoveView {
    mover: AbilityDisplay,
    mover_carriers: usize,
    mover_unit_id: Option<String>,
    mover_carrier_unit_ids: Vec<String>,
    from_column: u8,
    from_row: u8,
    to_column: u8,
    to_row: u8,
    reason: ReasonParts,
}

impl ResolveMoveView {
    /// How many units this move ties together — the larger of the moved ability's
    /// carriers and the rival's. Used to rank Fights by impact.
    fn contributor_count(&self) -> usize {
        let rival_carriers = self.reason.other_carriers.unwrap_or(0);
        self.mover_carriers.max(rival_carriers)
    }
}

/// One ability the cascade could not place, with the cell it is stuck on.
#[derive(Clone, PartialEq)]
struct ResolveUnresolvedView {
    ability: AbilityDisplay,
    carrier_count: usize,
    carrier_unit_ids: Vec<String>,
    column: u8,
    row: u8,
}

/// The data backing the carriers dialog: an ability's name and every unit that
/// carries it, resolved to icons and names.
#[derive(Clone, PartialEq)]
struct CarriersDialogData {
    ability_name: String,
    carriers: Vec<UnitDisplay>,
}

impl CarriersDialogData {
    fn new(ability_name: String, carrier_unit_ids: &[String]) -> Self {
        let mut carriers: Vec<UnitDisplay> = Vec::with_capacity(carrier_unit_ids.len());
        for carrier_unit_id in carrier_unit_ids {
            let carrier = UnitDisplay::resolve(carrier_unit_id);
            carriers.push(carrier);
        }
        Self {
            ability_name,
            carriers,
        }
    }
}

/// One titled group of moves of the same category (e.g. all Fights), in render
/// order.
#[derive(Clone, PartialEq)]
struct MoveSection {
    category: MoveCategory,
    title: &'static str,
    moves: Vec<ResolveMoveView>,
}

/// The cascade preview grouped into titled move sections and unresolved entries.
#[derive(Clone, PartialEq)]
struct ResolvePlanView {
    sections: Vec<MoveSection>,
    unresolved: Vec<ResolveUnresolvedView>,
}

impl ResolvePlanView {
    fn move_count(&self) -> usize {
        let mut total: usize = 0;
        for section in &self.sections {
            total += section.moves.len();
        }
        total
    }
}

impl ResolvePlanView {
    fn build(custom_keys: &CustomKeys) -> Self {
        let plan = custom_keys.preview_resolve();
        let plan_moves = plan.moves();
        let mut moves_by_slot = HashMap::new();
        for planned_move in plan_moves {
            let slot_key = planned_move.slot_id().as_str().to_owned();
            moves_by_slot.insert(slot_key, planned_move);
        }
        let mut consumed_swap_slots: HashSet<String> = HashSet::new();
        let mut moves: Vec<ResolveMoveView> = Vec::with_capacity(plan.move_count());
        for planned_move in plan_moves {
            let mover_slot_key = planned_move.slot_id().as_str().to_owned();
            if consumed_swap_slots.contains(&mover_slot_key) {
                continue;
            }
            let mover = AbilityDisplay::resolve(planned_move.slot_id());
            let mover_carriers = planned_move.carrier_count();
            let carrier_objects = planned_move.carrier_unit_ids();
            let mut mover_carrier_unit_ids: Vec<String> = Vec::with_capacity(carrier_objects.len());
            for carrier_object in carrier_objects {
                let carrier_value = carrier_object.value().to_owned();
                mover_carrier_unit_ids.push(carrier_value);
            }
            let mover_unit_id = mover_carrier_unit_ids.first().cloned();
            let old_position = planned_move.old_position();
            let new_position = planned_move.new_position();
            let from_column = u8::from(old_position.column());
            let from_row = u8::from(old_position.row());
            let to_column = u8::from(new_position.column());
            let to_row = u8::from(new_position.row());
            let mut reason = ReasonParts::resolve(planned_move.reason());
            if let MoveReason::Swap { swapped_with } = planned_move.reason() {
                let partner_key = swapped_with.as_str().to_owned();
                consumed_swap_slots.insert(partner_key.clone());
                if let Some(partner_move) = moves_by_slot.get(&partner_key) {
                    let partner_carrier_objects = partner_move.carrier_unit_ids();
                    let mut partner_carrier_unit_ids: Vec<String> =
                        Vec::with_capacity(partner_carrier_objects.len());
                    for partner_carrier_object in partner_carrier_objects {
                        let partner_carrier_value = partner_carrier_object.value().to_owned();
                        partner_carrier_unit_ids.push(partner_carrier_value);
                    }
                    let partner_carrier_count = partner_move.carrier_count();
                    reason.other_carriers = Some(partner_carrier_count);
                    reason.other_carrier_unit_ids = partner_carrier_unit_ids;
                }
            }
            let move_view = ResolveMoveView {
                mover,
                mover_carriers,
                mover_unit_id,
                mover_carrier_unit_ids,
                from_column,
                from_row,
                to_column,
                to_row,
                reason,
            };
            moves.push(move_view);
        }
        let mut unresolved: Vec<ResolveUnresolvedView> =
            Vec::with_capacity(plan.unresolved_count());
        for stuck in plan.unresolved() {
            let ability = AbilityDisplay::resolve(stuck.slot_id());
            let position = stuck.collision_position();
            let column = u8::from(position.column());
            let row = u8::from(position.row());
            let carrier_count = stuck.carrier_count();
            let carrier_objects = stuck.carrier_unit_ids();
            let mut carrier_unit_ids: Vec<String> = Vec::with_capacity(carrier_objects.len());
            for carrier_object in carrier_objects {
                let carrier_value = carrier_object.value().to_owned();
                carrier_unit_ids.push(carrier_value);
            }
            let unresolved_view = ResolveUnresolvedView {
                ability,
                carrier_count,
                carrier_unit_ids,
                column,
                row,
            };
            unresolved.push(unresolved_view);
        }
        let sections = Self::group_into_sections(moves);
        Self {
            sections,
            unresolved,
        }
    }

    /// Partition the flat move list into titled sections in a fixed order, and
    /// within Fights put the ones with the most contributors first.
    fn group_into_sections(moves: Vec<ResolveMoveView>) -> Vec<MoveSection> {
        let mut sections: Vec<MoveSection> = Vec::new();
        for category in MoveCategory::ORDER {
            let mut group: Vec<ResolveMoveView> = Vec::new();
            for move_view in &moves {
                if move_view.reason.category == category {
                    let cloned = move_view.clone();
                    group.push(cloned);
                }
            }
            if category == MoveCategory::Fight {
                group.sort_by(|left, right| {
                    let left_contributors = left.contributor_count();
                    let right_contributors = right.contributor_count();
                    right_contributors.cmp(&left_contributors)
                });
            }
            if !group.is_empty() {
                let title = category.section_title();
                let section = MoveSection {
                    category,
                    title,
                    moves: group,
                };
                sections.push(section);
            }
        }
        sections
    }
}

/// One ability icon pinned to a cell inside a `ResolveMiniGrid`.
#[derive(Clone, PartialEq)]
struct MiniGridPlacement {
    column: u8,
    row: u8,
    icon_url: Option<String>,
    name: String,
}

/// A tiny 4x3 command grid that renders each placed ability's icon into its
/// cell, so the move reads as "this ability ends up here".
#[component]
fn ResolveMiniGrid(placements: Vec<MiniGridPlacement>) -> Element {
    rsx! {
        div { class: "resolve-mini-grid",
            for grid_row in 0..COMMAND_GRID_ROWS {
                for grid_column in 0..COMMAND_GRID_COLUMNS {
                    {
                        let placement = placements
                            .iter()
                            .find(|placed| placed.column == grid_column && placed.row == grid_row);
                        let cell_class = if placement.is_some() {
                            "island-mini-cell collision"
                        } else {
                            "island-mini-cell"
                        };
                        rsx! {
                            div { key: "{grid_row}-{grid_column}", class: cell_class,
                                if let Some(placed) = placement {
                                    if let Some(url) = placed.icon_url.as_ref() {
                                        img {
                                            class: "resolve-mini-icon",
                                            src: "{url}",
                                            alt: "{placed.name}",
                                            loading: "lazy",
                                            decoding: "async",
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

#[derive(Props, Clone, PartialEq)]
struct ResolveAbilityIconProps {
    name: String,
    icon_url: Option<String>,
    carrier_count: usize,
    carrier_unit_ids: Vec<String>,
    is_winner: bool,
    carriers_dialog: Signal<Option<CarriersDialogData>>,
}

/// One ability icon with its carrier-count badge (top-right). Clicking it opens
/// the carriers dialog. The winner of a Fight is ringed gold.
#[component]
fn ResolveAbilityIcon(props: ResolveAbilityIconProps) -> Element {
    let name = props.name;
    let icon_url = props.icon_url;
    let carrier_count = props.carrier_count;
    let carrier_unit_ids = props.carrier_unit_ids;
    let is_winner = props.is_winner;
    let mut carriers_dialog = props.carriers_dialog;
    let dialog_name = name.clone();
    let has_carriers = !carrier_unit_ids.is_empty();
    let class_name = "resolve-fight-ability";
    let badge_class = if is_winner {
        "resolve-carrier-badge resolve-carrier-badge-win"
    } else {
        "resolve-carrier-badge"
    };
    rsx! {
        button {
            class: class_name,
            r#type: "button",
            disabled: !has_carriers,
            title: "{name} — {carrier_count} carriers",
            onclick: move |_| {
                if carrier_unit_ids.is_empty() {
                    return;
                }
                let data = CarriersDialogData::new(dialog_name.clone(), &carrier_unit_ids);
                carriers_dialog.set(Some(data));
            },
            if let Some(url) = icon_url {
                img {
                    class: "resolve-fight-icon",
                    src: "{url}",
                    alt: "{name}",
                    loading: "lazy",
                    decoding: "async",
                }
            }
            span { class: badge_class, "{carrier_count}" }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ResolveMoveRowProps {
    move_view: ResolveMoveView,
    view_navigation: ViewNavigationContext,
    carriers_dialog: Signal<Option<CarriersDialogData>>,
}

/// One move card: the reason badge (top-left), a row naming both fighting
/// abilities (gold, with object id) above their icons, then the from → to grids
/// with the abilities drawn into their cells. Everything is symmetric; no prose.
#[component]
fn ResolveMoveRow(props: ResolveMoveRowProps) -> Element {
    let move_view = props.move_view;
    let view_navigation = props.view_navigation;
    let carriers_dialog = props.carriers_dialog;
    let mover = move_view.mover;
    let reason = move_view.reason;
    let mover_unit_id = move_view.mover_unit_id;
    let mover_name = mover.name;
    let mover_object_id = mover.object_id;
    let mover_icon = mover.icon_url;
    let mover_carriers = move_view.mover_carriers;
    let mover_carrier_unit_ids = move_view.mover_carrier_unit_ids;
    let reason_label = reason.label;
    let badge_class = format!("resolve-reason {}", reason.badge_class);
    let has_unit = mover_unit_id.is_some();
    let name_class = if has_unit {
        "resolve-move-name resolve-move-name-link"
    } else {
        "resolve-move-name"
    };
    let from_column = move_view.from_column;
    let from_row = move_view.from_row;
    let to_column = move_view.to_column;
    let to_row = move_view.to_row;
    let anchor = reason.other_ability;
    let anchor_carriers = reason.other_carriers.unwrap_or(0);
    let anchor_carrier_unit_ids = reason.other_carrier_unit_ids;
    let is_swap = reason.is_swap;
    let anchor_is_winner = !is_swap;
    let mover_from_placement = MiniGridPlacement {
        column: from_column,
        row: from_row,
        icon_url: mover_icon.clone(),
        name: mover_name.clone(),
    };
    let mover_to_placement = MiniGridPlacement {
        column: to_column,
        row: to_row,
        icon_url: mover_icon.clone(),
        name: mover_name.clone(),
    };
    let mut from_placements: Vec<MiniGridPlacement> = vec![mover_from_placement];
    let mut to_placements: Vec<MiniGridPlacement> = vec![mover_to_placement];
    if let Some(anchor_ability) = anchor.as_ref() {
        let anchor_after_placement = MiniGridPlacement {
            column: from_column,
            row: from_row,
            icon_url: anchor_ability.icon_url.clone(),
            name: anchor_ability.name.clone(),
        };
        to_placements.push(anchor_after_placement);
        if is_swap {
            let anchor_before_placement = MiniGridPlacement {
                column: to_column,
                row: to_row,
                icon_url: anchor_ability.icon_url.clone(),
                name: anchor_ability.name.clone(),
            };
            from_placements.push(anchor_before_placement);
        }
    }
    let open_mover = move |_| {
        if let Some(unit_id) = mover_unit_id.as_ref() {
            view_navigation.open_unit(unit_id);
        }
    };
    rsx! {
        div { class: "resolve-move-row",
            div { class: "resolve-move-reasonrow",
                span { class: badge_class, "{reason_label}" }
            }
            div { class: "resolve-fight-row",
                div { class: "resolve-fight-col",
                    button {
                        class: "resolve-fight-namebtn",
                        r#type: "button",
                        disabled: !has_unit,
                        onclick: open_mover,
                        span { class: name_class, "{mover_name}" }
                        code { class: "conflict-object-id", "{mover_object_id}" }
                    }
                    ResolveAbilityIcon {
                        name: mover_name.clone(),
                        icon_url: mover_icon.clone(),
                        carrier_count: mover_carriers,
                        carrier_unit_ids: mover_carrier_unit_ids
                                .clone(),
                        is_winner: false,
                        carriers_dialog,
                    }
                }
                if let Some(anchor_ability) = anchor {
                    div { class: "resolve-fight-col",
                        div { class: "resolve-fight-nameplate",
                            span { class: "resolve-move-name", "{anchor_ability.name}" }
                            code { class: "conflict-object-id", "{anchor_ability.object_id}" }
                        }
                        ResolveAbilityIcon {
                            name: anchor_ability.name
                                    .clone(),
                            icon_url: anchor_ability.icon_url.clone(),
                            carrier_count: anchor_carriers,
                            carrier_unit_ids: anchor_carrier_unit_ids.clone(),
                            is_winner: anchor_is_winner,
                            carriers_dialog,
                        }
                    }
                }
            }
            div { class: "resolve-move-transition",
                div { class: "resolve-grid-col",
                    ResolveMiniGrid { placements: from_placements }
                }
                span { class: "resolve-move-arrow", aria_hidden: "true", "\u{2192}" }
                div { class: "resolve-grid-col",
                    ResolveMiniGrid { placements: to_placements }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct CarriersDialogProps {
    dialog_data: CarriersDialogData,
    carriers_dialog: Signal<Option<CarriersDialogData>>,
    view_navigation: ViewNavigationContext,
}

/// Lists every unit that carries an ability in a scrollable grid. Each card
/// deep-links into the editor focused on that unit.
#[component]
fn CarriersDialog(props: CarriersDialogProps) -> Element {
    let dialog_data = props.dialog_data;
    let mut carriers_dialog = props.carriers_dialog;
    let view_navigation = props.view_navigation;
    let title = dialog_data.ability_name.clone();
    let open = use_signal(|| true);
    use_effect(move || {
        if !open() {
            carriers_dialog.set(None);
        }
    });
    rsx! {
        Dialog { open, title,
            div { class: "carriers-grid",
                for (carrier_index, carrier) in dialog_data.carriers.iter().enumerate() {
                    {
                        let carrier_id = carrier.unit_id.clone();
                        let carrier_id_label = carrier_id.clone();
                        let carrier_name = carrier.name.clone();
                        let carrier_icon = carrier.icon_url.clone();
                        rsx! {
                            button {
                                key: "carrier-{carrier_index}",
                                class: "carrier-card",
                                r#type: "button",
                                onclick: move |_| view_navigation.open_unit(&carrier_id),
                                if let Some(url) = carrier_icon {
                                    img {
                                        class: "carrier-card-icon",
                                        src: "{url}",
                                        alt: "{carrier_name}",
                                        loading: "lazy",
                                        decoding: "async",
                                    }
                                }
                                span { class: "carrier-card-name", "{carrier_name}" }
                                code { class: "conflict-object-id", "{carrier_id_label}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

const CENTERED_STATE_CLASS: &str = "resolve-page flex flex-col items-center justify-center \
    gap-[1.25rem] [flex:1_1_0] [min-height:0] text-center p-[2rem] text-warcraft-text-secondary";

#[derive(Props, Clone, PartialEq)]
pub struct ResolvePageProps {
    pub loaded_keys: Signal<Option<CustomKeys>>,
    /// The selected move-category breadcrumb, backed by the `?entry=` URL
    /// parameter (its `data_breadcrumb` slug) so the viewed section deep-links
    /// and survives browser back/forward — mirroring the collisions page.
    pub selected_move_category: Signal<Option<String>>,
}

/// The Resolve page: a transparent preview of the cascade plan — every move the
/// algorithm would make (from → to cell + the fighting abilities) and any
/// unresolved abilities — with an Apply button that runs the cascade.
#[component]
pub fn ResolvePage(props: ResolvePageProps) -> Element {
    let mut loaded_keys = props.loaded_keys;
    let view_navigation = use_context::<ViewNavigationContext>();
    let toast_api = use_toast();
    let mut is_running = use_signal(|| false);
    let carriers_dialog = use_signal(|| None::<CarriersDialogData>);
    let mut selected_move_category = props.selected_move_category;
    let plan_memo = use_memo(move || {
        let guard = loaded_keys.read();
        guard.as_ref().map(ResolvePlanView::build)
    });
    let has_file = loaded_keys.read().is_some();
    let plan = plan_memo();
    let move_count = plan.as_ref().map(|view| view.move_count()).unwrap_or(0);
    let unresolved_count = plan.as_ref().map(|view| view.unresolved.len()).unwrap_or(0);
    let move_noun = if move_count == 1 { "move" } else { "moves" };
    let running_now = *is_running.read();
    let carriers_dialog_state = carriers_dialog.read().clone();
    let handle_apply = move |_| {
        if *is_running.read() {
            return;
        }
        let working_copy: CustomKeys = {
            let read_guard = loaded_keys.read();
            let Some(file) = read_guard.as_ref() else {
                return;
            };
            file.clone()
        };
        is_running.set(true);
        spawn(async move {
            TimeoutFuture::new(0).await;
            let mut working_copy = working_copy;
            let plan = working_copy.resolve_conflicts();
            let move_count = plan.move_count();
            let unresolved_count = plan.unresolved_count();
            let normalized = working_copy.normalize();
            loaded_keys.set(Some(normalized));
            is_running.set(false);
            let summary = if unresolved_count == 0 {
                format!("Moved {move_count} ability slot(s). No remaining conflicts.")
            } else {
                format!(
                    "Moved {move_count} ability slot(s). {unresolved_count} could not be placed.",
                )
            };
            let title = String::from("Cascade applied");
            let toast_options = ToastOptions::new().description(summary);
            toast_api.success(title, toast_options);
        });
    };
    if !has_file {
        return rsx! {
            section { class: CENTERED_STATE_CLASS, "data-resolve-state": "no-file",
                p { class: "m-0", "Upload your CustomKeys.txt to preview the cascade plan." }
            }
        };
    }
    if move_count == 0 && unresolved_count == 0 {
        return rsx! {
            section { class: CENTERED_STATE_CLASS, "data-resolve-state": "clear",
                span {
                    class: "inline-flex w-[3.5rem] h-[3.5rem] text-warcraft-gold \
                            [&_svg]:w-full [&_svg]:h-full [filter:drop-shadow(0_0_10px_rgba(255,206,99,0.45))]",
                    aria_hidden: "true",
                    dangerous_inner_html: ICON_COLLISIONS_CLEAR,
                }
                p { class: "m-0 font-friz-quadrata uppercase tracking-[0.12em] text-warcraft-gold [text-shadow:1px_1px_0_#000]",
                    "Nothing to resolve."
                }
            }
        };
    }
    let plan = plan.expect("plan present when a file is loaded");
    let selected_slug = selected_move_category.read().clone();
    let selected = selected_slug
        .as_deref()
        .and_then(MoveCategory::from_data_breadcrumb);
    let selected_exists = selected
        .map(|category| {
            plan.sections
                .iter()
                .any(|section| section.category == category)
        })
        .unwrap_or(false);
    let active_category = if selected_exists {
        selected
    } else {
        plan.sections.first().map(|section| section.category)
    };
    let active_section = active_category.and_then(|category| {
        plan.sections
            .iter()
            .find(|section| section.category == category)
    });
    rsx! {
        section {
            class: "resolve-page resolve-plan",
            "data-resolve-state": "plan",
            "data-move-count": "{move_count}",
            "data-unresolved-count": "{unresolved_count}",
            header { class: "resolve-plan-header",
                div { class: "resolve-plan-summary",
                    span { class: "resolve-plan-title", "Cascade Plan" }
                    span { class: "resolve-plan-counts",
                        "{move_count} {move_noun}"
                        if unresolved_count > 0 {
                            " · "
                            span { class: "resolve-plan-unresolved", "{unresolved_count} unresolved" }
                        }
                    }
                }
                button {
                    class: "resolve-apply-button",
                    r#type: "button",
                    disabled: running_now,
                    "data-action": "apply-cascade",
                    onclick: handle_apply,
                    if running_now {
                        "Applying…"
                    } else {
                        "Apply"
                    }
                }
            }
            nav {
                class: "collision-breadcrumbs resolve-breadcrumbs",
                aria_label: "Move categories",
                for (section_index, section) in plan.sections.iter().enumerate() {
                    {
                        let category = section.category;
                        let is_active = active_category == Some(category);
                        let class_name = if is_active {
                            "collision-breadcrumb active"
                        } else {
                            "collision-breadcrumb"
                        };
                        let aria_current = if is_active { "page" } else { "false" };
                        let data_breadcrumb = category.data_breadcrumb();
                        let title = section.title;
                        let count = section.moves.len();
                        let select = move |_| {
                            let slug = category.data_breadcrumb().to_owned();
                            selected_move_category.set(Some(slug));
                        };
                        rsx! {
                            if section_index > 0 {
                                span { class: "collision-breadcrumb-separator", aria_hidden: "true", "|" }
                            }
                            button {
                                class: class_name,
                                r#type: "button",
                                "aria-current": aria_current,
                                "data-breadcrumb": data_breadcrumb,
                                onclick: select,
                                span { class: "collision-breadcrumb-label", "{title}" }
                                span { class: "collision-breadcrumb-count", "{count}" }
                            }
                        }
                    }
                }
            }
            div { class: "resolve-plan-body",
                if let Some(section) = active_section {
                    div {
                        class: "resolve-move-list",
                        "data-category": "{section.category.data_breadcrumb()}",
                        for (move_index, move_view) in section.moves.iter().enumerate() {
                            ResolveMoveRow {
                                key: "{section.category.data_breadcrumb()}-move-{move_index}",
                                move_view: move_view
                                        .clone(),
                                view_navigation,
                                carriers_dialog,
                            }
                        }
                    }
                }
                if !plan.unresolved.is_empty() {
                    div { class: "resolve-unresolved",
                        span { class: "resolve-unresolved-title", "Unresolved" }
                        div { class: "resolve-move-list",
                            for (stuck_index, stuck) in plan.unresolved.iter().enumerate() {
                                {
                                    let stuck_placement = MiniGridPlacement {
                                        column: stuck.column,
                                        row: stuck.row,
                                        icon_url: stuck.ability.icon_url.clone(),
                                        name: stuck.ability.name.clone(),
                                    };
                                    let stuck_placements: Vec<MiniGridPlacement> = vec![stuck_placement];
                                    rsx! {
                                        div {
                                            key: "stuck-{stuck_index}",
                                            class: "resolve-move-row resolve-move-row-stuck",
                                            div { class: "resolve-move-reasonrow",
                                                span { class: "resolve-reason resolve-reason-stuck", "Stuck" }
                                            }
                                            div { class: "resolve-fight-row",
                                                div { class: "resolve-fight-col",
                                                    div { class: "resolve-fight-nameplate",
                                                        span { class: "resolve-move-name", "{stuck.ability.name}" }
                                                        code { class: "conflict-object-id", "{stuck.ability.object_id}" }
                                                    }
                                                    ResolveAbilityIcon {
                                                        name: stuck.ability.name
                                                                .clone(),
                                                        icon_url: stuck.ability.icon_url.clone(),
                                                        carrier_count: stuck
                                                                .carrier_count,
                                                        carrier_unit_ids: stuck.carrier_unit_ids.clone(),
                                                        is_winner: false,
                                                        carriers_dialog,
                                                    }
                                                }
                                            }
                                            div { class: "resolve-move-transition",
                                                ResolveMiniGrid { placements: stuck_placements }
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
        if let Some(dialog_data) = carriers_dialog_state {
            CarriersDialog { dialog_data, carriers_dialog, view_navigation }
        }
    }
}
