use crate::components::app::components::shell::components::shared::breadcrumbs::BreadcrumbsProps;
use crate::components::app::components::shell::components::shared::breadcrumbs::components::breadcrumb::BreadcrumbProps;
use super::components::plan_body::components::active_move_list::components::move_row::MoveRowProps;
use super::components::plan_body::components::unresolved_section::components::unresolved_row::UnresolvedRowProps;
use super::components::plan_body::{PlanBodyProps, PlanBodySection};
use crate::components::app::components::shell::components::shared::icons::ResolvedIcon;
use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;
use std::collections::{HashMap, HashSet};
use warcraft_keybinds::{CascadePlan, GridSlotId, MoveReason};

/// One ability resolved to an icon, display name, and object id for the plan.
#[derive(Clone, PartialEq)]
pub struct AbilityDisplay {
    object_id: String,
    name: String,
    icon_url: Option<String>,
}

impl From<GridSlotId> for AbilityDisplay {
    fn from(slot_id: GridSlotId) -> Self {
        let id_value = slot_id.id().value();
        let resolved = ResolvedIcon::lookup(id_value);
        let icon_url = resolved.icon_url().map(str::to_owned);
        let name = match resolved.name() {
            Some(name) => name.to_owned(),
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
    label: &'static str,
    other_ability: Option<AbilityDisplay>,
    other_carriers: Option<usize>,
    other_carrier_unit_ids: Vec<String>,
    is_swap: bool,
    category: MoveCategory,
}

impl From<&MoveReason> for ReasonParts {
    fn from(reason: &MoveReason) -> Self {
        match reason {
            MoveReason::Fight {
                anchor_slot,
                anchor_carrier_unit_ids,
            } => {
                let anchor = AbilityDisplay::from(*anchor_slot);
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
                let other = AbilityDisplay::from(*swapped_with);
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
    ability: AbilityDisplay,
    carrier_count: usize,
    carrier_unit_ids: Vec<String>,
    column: u8,
    row: u8,
}

/// One titled group of moves of the same category (e.g. all Fights), in render
/// order.
#[derive(Clone, PartialEq)]
pub struct MoveSection {
    category: MoveCategory,
    title: &'static str,
    moves: Vec<MoveView>,
}

/// The cascade preview grouped into titled move sections and unresolved entries.
#[derive(Clone, PartialEq)]
pub struct PlanView {
    sections: Vec<MoveSection>,
    unresolved: Vec<UnresolvedView>,
}

impl PlanView {
    pub fn move_count(&self) -> usize {
        let mut total: usize = 0;
        for section in &self.sections {
            total += section.moves.len();
        }
        total
    }

    pub fn build(plan: &CascadePlan) -> Self {
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
            let mover = AbilityDisplay::from(planned_move.slot_id());
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
            let mut reason = ReasonParts::from(planned_move.reason());
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
            let ability = AbilityDisplay::from(stuck.slot_id());
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
    column: u8,
    row: u8,
    icon_url: Option<String>,
    name: String,
}

impl MiniGridPlacement {
    pub fn new(column: u8, row: u8, icon_url: Option<String>, name: String) -> Self {
        Self {
            column,
            row,
            icon_url,
            name,
        }
    }

    pub fn column(&self) -> u8 {
        self.column
    }

    pub fn row(&self) -> u8 {
        self.row
    }

    pub fn icon_url(&self) -> Option<&str> {
        self.icon_url.as_deref()
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

/// The move counts derived from the cascade preview: how many slots the plan moves
/// and how many abilities it cannot place. The plan state tags its root element with
/// these and the header phrases them; both `0` is the all-clear state.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub(super) struct PlanCounts {
    pub(super) move_count: usize,
    pub(super) unresolved_count: usize,
}

impl From<Option<&PlanView>> for PlanCounts {
    fn from(plan: Option<&PlanView>) -> Self {
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

/// The inputs that shape an [`ActivePlanView`]: the plan to render, the selected
/// section slug, and the Copy selection + navigation signals its breadcrumbs and
/// rows close over.
pub(super) struct ActivePlanInputs<'a> {
    pub(super) plan: &'a PlanView,
    pub(super) selected_slug: Option<&'a str>,
    pub(super) selection: Signal<Option<String>>,
    pub(super) view_navigation: ViewNavigationContext,
}

impl From<ActivePlanInputs<'_>> for ActivePlanView {
    fn from(inputs: ActivePlanInputs<'_>) -> Self {
        let ActivePlanInputs {
            plan,
            selected_slug,
            selection,
            view_navigation,
        } = inputs;
        let active = plan.active_section(selected_slug);
        let active_category = active.map(|section| section.category);
        let mut breadcrumb_list: Vec<BreadcrumbProps> = Vec::with_capacity(plan.sections.len());
        for section in &plan.sections {
            let category = section.category;
            let is_active = active_category == Some(category);
            let data_breadcrumb = category.data_breadcrumb();
            let label = section.title.to_owned();
            let count = section.moves.len();
            let mut selection = selection;
            let onclick = EventHandler::new(move |_event: MouseEvent| {
                let slug = category.data_breadcrumb().to_owned();
                selection.set(Some(slug));
            });
            let breadcrumb = BreadcrumbProps {
                label,
                count,
                data_breadcrumb,
                active: is_active,
                onclick,
            };
            breadcrumb_list.push(breadcrumb);
        }
        let breadcrumbs = BreadcrumbsProps {
            breadcrumbs: breadcrumb_list,
            aria_label: "Move categories",
        };
        let section = active.map(|section| {
            let rows: Vec<MoveRowProps> = section
                .moves
                .iter()
                .map(|move_view| MoveRowProps {
                    move_view: move_view.clone(),
                    view_navigation,
                })
                .collect();
            let data_category = section.category.data_breadcrumb();
            PlanBodySection::new(data_category, rows)
        });
        let unresolved_rows: Vec<UnresolvedRowProps> = plan
            .unresolved
            .iter()
            .map(|unresolved_view| UnresolvedRowProps {
                unresolved_view: unresolved_view.clone(),
            })
            .collect();
        let body = PlanBodyProps {
            section,
            unresolved_rows,
        };
        Self { breadcrumbs, body }
    }
}

impl AbilityDisplay {
    pub fn object_id(&self) -> &str {
        &self.object_id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn icon_url(&self) -> Option<&str> {
        self.icon_url.as_deref()
    }
}

impl ReasonParts {
    pub fn label(&self) -> &'static str {
        self.label
    }
    pub fn other_ability(&self) -> Option<&AbilityDisplay> {
        self.other_ability.as_ref()
    }
    pub fn other_carriers(&self) -> Option<usize> {
        self.other_carriers
    }
    pub fn other_carrier_unit_ids(&self) -> &[String] {
        &self.other_carrier_unit_ids
    }
    pub fn is_swap(&self) -> bool {
        self.is_swap
    }
    pub fn category(&self) -> MoveCategory {
        self.category
    }
}

impl MoveView {
    pub fn mover(&self) -> &AbilityDisplay {
        &self.mover
    }
    pub fn mover_carriers(&self) -> usize {
        self.mover_carriers
    }
    pub fn mover_unit_id(&self) -> Option<&str> {
        self.mover_unit_id.as_deref()
    }
    pub fn mover_carrier_unit_ids(&self) -> &[String] {
        &self.mover_carrier_unit_ids
    }
    pub fn from_column(&self) -> u8 {
        self.from_column
    }
    pub fn from_row(&self) -> u8 {
        self.from_row
    }
    pub fn to_column(&self) -> u8 {
        self.to_column
    }
    pub fn to_row(&self) -> u8 {
        self.to_row
    }
    pub fn reason(&self) -> &ReasonParts {
        &self.reason
    }
}

impl UnresolvedView {
    pub fn ability(&self) -> &AbilityDisplay {
        &self.ability
    }
    pub fn carrier_count(&self) -> usize {
        self.carrier_count
    }
    pub fn carrier_unit_ids(&self) -> &[String] {
        &self.carrier_unit_ids
    }
    pub fn column(&self) -> u8 {
        self.column
    }
    pub fn row(&self) -> u8 {
        self.row
    }
}

impl MoveSection {
    pub fn category(&self) -> MoveCategory {
        self.category
    }
    pub fn title(&self) -> &'static str {
        self.title
    }
    pub fn moves(&self) -> &[MoveView] {
        &self.moves
    }
}

impl PlanView {
    pub fn sections(&self) -> &[MoveSection] {
        &self.sections
    }
    pub fn unresolved(&self) -> &[UnresolvedView] {
        &self.unresolved
    }
}
