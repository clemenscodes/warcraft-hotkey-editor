use super::components::breadcrumbs::BreadcrumbsProps;
use super::components::breadcrumbs::components::breadcrumb::BreadcrumbProps;
use super::components::plan_body::components::active_move_list::components::move_row::MoveRowProps;
use super::components::plan_body::components::unresolved_section::components::unresolved_row::UnresolvedRowProps;
use super::components::plan_body::{PlanBodyProps, PlanBodySection};
use crate::components::app::components::shell::components::shared::icons::IconUrl;
use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;
use std::collections::{HashMap, HashSet};
use warcraft_database::ObjectLookup;
use warcraft_keybinds::{CustomKeys, GridSlotId, MoveReason};

/// One ability resolved to an icon, display name, and object id for the plan.
#[derive(Clone, PartialEq)]
pub struct AbilityDisplay {
    pub object_id: String,
    pub name: String,
    pub icon_url: Option<String>,
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
pub struct UnitDisplay {
    pub unit_id: String,
    pub name: String,
    pub icon_url: Option<String>,
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
pub enum MoveCategory {
    Fight,
    GapPull,
    Spill,
    Swap,
}

impl MoveCategory {
    const ORDER: [Self; 4] = [Self::Fight, Self::GapPull, Self::Spill, Self::Swap];

    pub fn section_title(self) -> &'static str {
        match self {
            Self::Fight => "Fights",
            Self::GapPull => "Gap pulls",
            Self::Spill => "Spills",
            Self::Swap => "Swaps",
        }
    }

    pub fn data_breadcrumb(self) -> &'static str {
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
    pub fn from_data_breadcrumb(slug: &str) -> Option<Self> {
        match slug {
            "fights" => Some(Self::Fight),
            "gap-pulls" => Some(Self::GapPull),
            "spills" => Some(Self::Spill),
            "swaps" => Some(Self::Swap),
            _ => None,
        }
    }
}

/// The visual kind of a move's reason badge — the four move categories plus the
/// "Stuck" badge shown on unresolved abilities. Selects the badge's colour.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ReasonKind {
    Fight,
    GapPull,
    Spill,
    Swap,
    Stuck,
}

impl From<MoveCategory> for ReasonKind {
    fn from(category: MoveCategory) -> Self {
        match category {
            MoveCategory::Fight => Self::Fight,
            MoveCategory::GapPull => Self::GapPull,
            MoveCategory::Spill => Self::Spill,
            MoveCategory::Swap => Self::Swap,
        }
    }
}

/// The display-ready pieces of a move's rationale: a short badge label + kind
/// and, for Fight/Swap, the rival ability and its carrier count.
#[derive(Clone, PartialEq)]
pub struct ReasonParts {
    pub label: &'static str,
    pub other_ability: Option<AbilityDisplay>,
    pub other_carriers: Option<usize>,
    pub other_carrier_unit_ids: Vec<String>,
    pub is_swap: bool,
    pub category: MoveCategory,
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
                    other_ability: Some(other),
                    other_carriers: None,
                    other_carrier_unit_ids: Vec::new(),
                    is_swap: true,
                    category: MoveCategory::Swap,
                }
            }
            MoveReason::Spill { .. } => Self {
                label: "Spill",
                other_ability: None,
                other_carriers: None,
                other_carrier_unit_ids: Vec::new(),
                is_swap: false,
                category: MoveCategory::Spill,
            },
            MoveReason::GapPull { .. } => Self {
                label: "Gap pull",
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
pub struct MoveView {
    pub mover: AbilityDisplay,
    pub mover_carriers: usize,
    pub mover_unit_id: Option<String>,
    pub mover_carrier_unit_ids: Vec<String>,
    pub from_column: u8,
    pub from_row: u8,
    pub to_column: u8,
    pub to_row: u8,
    pub reason: ReasonParts,
}

impl MoveView {
    /// How many units this move ties together — the larger of the moved ability's
    /// carriers and the rival's. Used to rank Fights by impact.
    fn contributor_count(&self) -> usize {
        let rival_carriers = self.reason.other_carriers.unwrap_or(0);
        self.mover_carriers.max(rival_carriers)
    }
}

/// One ability the cascade could not place, with the cell it is stuck on.
#[derive(Clone, PartialEq)]
pub struct UnresolvedView {
    pub ability: AbilityDisplay,
    pub carrier_count: usize,
    pub carrier_unit_ids: Vec<String>,
    pub column: u8,
    pub row: u8,
}

/// The data backing the carriers dialog: an ability's name and every unit that
/// carries it, resolved to icons and names.
#[derive(Clone, PartialEq)]
pub struct CarriersDialogData {
    pub ability_name: String,
    pub carriers: Vec<UnitDisplay>,
}

impl CarriersDialogData {
    pub fn new(ability_name: String, carrier_unit_ids: &[String]) -> Self {
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
pub struct MoveSection {
    pub category: MoveCategory,
    pub title: &'static str,
    pub moves: Vec<MoveView>,
}

/// The cascade preview grouped into titled move sections and unresolved entries.
#[derive(Clone, PartialEq)]
pub struct PlanView {
    pub sections: Vec<MoveSection>,
    pub unresolved: Vec<UnresolvedView>,
}

impl PlanView {
    pub fn move_count(&self) -> usize {
        let mut total: usize = 0;
        for section in &self.sections {
            total += section.moves.len();
        }
        total
    }

    pub fn build(custom_keys: &CustomKeys) -> Self {
        let plan = custom_keys.preview_resolve();
        let plan_moves = plan.moves();
        let mut moves_by_slot = HashMap::new();
        for planned_move in plan_moves {
            let slot_key = planned_move.slot_id().as_str().to_owned();
            moves_by_slot.insert(slot_key, planned_move);
        }
        let mut consumed_swap_slots: HashSet<String> = HashSet::new();
        let mut moves: Vec<MoveView> = Vec::with_capacity(plan.move_count());
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
            let move_view = MoveView {
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
        let mut unresolved: Vec<UnresolvedView> = Vec::with_capacity(plan.unresolved_count());
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
            let unresolved_view = UnresolvedView {
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
    fn group_into_sections(moves: Vec<MoveView>) -> Vec<MoveSection> {
        let mut sections: Vec<MoveSection> = Vec::new();
        for category in MoveCategory::ORDER {
            let mut group: Vec<MoveView> = Vec::new();
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

    /// The section matching the selected breadcrumb slug, falling back to the
    /// first section when the slug is missing or names an absent category.
    pub fn active_section(&self, selected_slug: Option<&str>) -> Option<&MoveSection> {
        let selected = selected_slug.and_then(MoveCategory::from_data_breadcrumb);
        let selected_exists = selected
            .map(|category| {
                self.sections
                    .iter()
                    .any(|section| section.category == category)
            })
            .unwrap_or(false);
        let active_category = if selected_exists {
            selected
        } else {
            self.sections.first().map(|section| section.category)
        };
        active_category.and_then(|category| {
            self.sections
                .iter()
                .find(|section| section.category == category)
        })
    }
}

/// One ability icon pinned to a cell inside a `MiniGrid`.
#[derive(Clone, PartialEq)]
pub struct MiniGridPlacement {
    pub column: u8,
    pub row: u8,
    pub icon_url: Option<String>,
    pub name: String,
}

/// The move counts derived from the cascade preview: how many slots the plan moves
/// and how many abilities it cannot place. The plan state tags its root element with
/// these and the header phrases them; both `0` is the all-clear state.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub(super) struct PlanCounts {
    pub(super) move_count: usize,
    pub(super) unresolved_count: usize,
}

impl PlanCounts {
    pub(super) fn resolve(plan: Option<&PlanView>) -> Self {
        let move_count = plan.map(PlanView::move_count).unwrap_or(0);
        let unresolved_count = plan.map(|view| view.unresolved.len()).unwrap_or(0);
        Self {
            move_count,
            unresolved_count,
        }
    }
}

/// The active section of the plan shaped for the body: the breadcrumb bar (one tab
/// per section, the selected one flagged, each closing over the selection signal so
/// a click reselects) and the scrollable body (the active section's move rows plus
/// every unresolved row). Shaping the breadcrumbs and rows needs the Copy navigation
/// and dialog signals, which arrive as inputs.
pub(super) struct ActivePlanView {
    pub(super) breadcrumbs: BreadcrumbsProps,
    pub(super) body: PlanBodyProps,
}

impl ActivePlanView {
    pub(super) fn resolve(
        plan: &PlanView,
        selected_slug: Option<&str>,
        selection: Signal<Option<String>>,
        view_navigation: ViewNavigationContext,
        carriers_dialog: Signal<Option<CarriersDialogData>>,
    ) -> Self {
        let active = plan.active_section(selected_slug);
        let active_category = active.map(|section| section.category);
        let mut breadcrumb_list: Vec<BreadcrumbProps> = Vec::with_capacity(plan.sections.len());
        for section in &plan.sections {
            let category = section.category;
            let is_active = active_category == Some(category);
            let data_breadcrumb = category.data_breadcrumb();
            let title = section.title.to_owned();
            let count = section.moves.len();
            let mut selection = selection;
            let onclick = EventHandler::new(move |_event: MouseEvent| {
                let slug = category.data_breadcrumb().to_owned();
                selection.set(Some(slug));
            });
            let breadcrumb = BreadcrumbProps {
                title,
                count,
                data_breadcrumb,
                active: is_active,
                onclick,
            };
            breadcrumb_list.push(breadcrumb);
        }
        let breadcrumbs = BreadcrumbsProps {
            breadcrumbs: breadcrumb_list,
        };
        let section = active.map(|section| {
            let rows: Vec<MoveRowProps> = section
                .moves
                .iter()
                .map(|move_view| MoveRowProps {
                    move_view: move_view.clone(),
                    view_navigation,
                    carriers_dialog,
                })
                .collect();
            PlanBodySection {
                data_category: section.category.data_breadcrumb(),
                rows,
            }
        });
        let unresolved_rows: Vec<UnresolvedRowProps> = plan
            .unresolved
            .iter()
            .map(|unresolved_view| UnresolvedRowProps {
                unresolved_view: unresolved_view.clone(),
                carriers_dialog,
            })
            .collect();
        let body = PlanBodyProps {
            section,
            unresolved_rows,
        };
        Self { breadcrumbs, body }
    }
}
