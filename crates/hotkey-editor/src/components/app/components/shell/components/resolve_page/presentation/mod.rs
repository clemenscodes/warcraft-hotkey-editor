use super::model::ResolvePageModel;
use crate::components::app::components::shell::components::shared::breadcrumbs::BreadcrumbView;
use crate::components::app::components::shell::components::shared::icons::ResolvedIcon;
use crate::components::app::components::shell::components::toasts::{ToastOptions, use_toast};
use crate::services::customkeys::context::{use_custom_keys_service, use_loaded_keys};
use crate::services::navigation::app_view::AppView;
use crate::services::navigation::context::use_view_navigation;
use crate::services::navigation::view_navigation::ViewNavigationContext;
use crate::services::resolve_selection::ResolveSelection;
use crate::services::resolve_selection::context::use_resolve_selection;
use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
use std::collections::{HashMap, HashSet};
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::{CascadePlan, GridCoordinate, GridSlotId, MoveReason};
/// One ability resolved to an icon, display name, and object id for the plan.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AbilityDisplay {
    object_id: WarcraftObjectId,
    name: String,
    icon_url: Option<String>,
}

impl From<GridSlotId> for AbilityDisplay {
    fn from(slot_id: GridSlotId) -> Self {
        let object_id = slot_id.id();
        let resolved = ResolvedIcon::lookup(object_id);
        let icon_url = resolved.icon_url().map(str::to_owned);
        let name = match resolved.name() {
            Some(name) => name.to_owned(),
            None => slot_id.display_name(None, None),
        };
        Self {
            object_id,
            name,
            icon_url,
        }
    }
}

/// Which kind of move this is. Drives both grouping into sections and the order
/// the sections render in (Fights first, then Gap pulls, Spills, Swaps).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

    pub fn entry_slug(self) -> &'static str {
        match self {
            Self::Fight => "fights",
            Self::GapPull => "gap-pulls",
            Self::Spill => "spills",
            Self::Swap => "swaps",
        }
    }

    /// Parses the `?entry=` slug back into a category — used to restore the
    /// selected move section from the URL parameter. Unknown slugs yield `None`
    /// (the page then falls back to the first section).
    pub fn from_entry_slug(slug: &str) -> Option<Self> {
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ReasonParts {
    label: &'static str,
    other_ability: Option<AbilityDisplay>,
    other_carriers: Option<usize>,
    other_carrier_unit_ids: Vec<WarcraftObjectId>,
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
                let other_carrier_unit_ids: Vec<WarcraftObjectId> =
                    anchor_carrier_unit_ids.to_vec();
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
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MoveView {
    mover: AbilityDisplay,
    mover_carriers: usize,
    mover_unit_id: Option<WarcraftObjectId>,
    mover_carrier_unit_ids: Vec<WarcraftObjectId>,
    from: GridCoordinate,
    to: GridCoordinate,
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
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UnresolvedView {
    ability: AbilityDisplay,
    carrier_count: usize,
    carrier_unit_ids: Vec<WarcraftObjectId>,
    position: GridCoordinate,
}

/// One titled group of moves of the same category (e.g. all Fights), in render
/// order.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MoveSection {
    category: MoveCategory,
    title: &'static str,
    moves: Vec<MoveView>,
}

/// The cascade preview grouped into titled move sections and unresolved entries.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
            let slot_key = planned_move.slot_id().id();
            moves_by_slot.insert(slot_key, planned_move);
        }
        let mut consumed_swap_slots: HashSet<WarcraftObjectId> = HashSet::new();
        let mut moves: Vec<MoveView> = Vec::with_capacity(plan.move_count());
        for planned_move in plan_moves {
            let mover_slot_key = planned_move.slot_id().id();
            if consumed_swap_slots.contains(&mover_slot_key) {
                continue;
            }
            let mover = AbilityDisplay::from(planned_move.slot_id());
            let mover_carriers = planned_move.carrier_count();
            let carrier_objects = planned_move.carrier_unit_ids();
            let mover_carrier_unit_ids: Vec<WarcraftObjectId> = carrier_objects.to_vec();
            let mover_unit_id = mover_carrier_unit_ids.first().copied();
            let from = planned_move.old_position();
            let to = planned_move.new_position();
            let mut reason = ReasonParts::from(planned_move.reason());
            if let MoveReason::Swap { swapped_with } = planned_move.reason() {
                let partner_key = swapped_with.id();
                consumed_swap_slots.insert(partner_key);
                if let Some(partner_move) = moves_by_slot.get(&partner_key) {
                    let partner_carrier_objects = partner_move.carrier_unit_ids();
                    let partner_carrier_unit_ids: Vec<WarcraftObjectId> =
                        partner_carrier_objects.to_vec();
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
                from,
                to,
                reason,
            };
            moves.push(move_view);
        }
        let mut unresolved: Vec<UnresolvedView> = Vec::with_capacity(plan.unresolved_count());
        for stuck in plan.unresolved() {
            let ability = AbilityDisplay::from(stuck.slot_id());
            let position = stuck.collision_position();
            let carrier_count = stuck.carrier_count();
            let carrier_objects = stuck.carrier_unit_ids();
            let carrier_unit_ids: Vec<WarcraftObjectId> = carrier_objects.to_vec();
            let unresolved_view = UnresolvedView {
                ability,
                carrier_count,
                carrier_unit_ids,
                position,
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
        let selected = selected_slug.and_then(MoveCategory::from_entry_slug);
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
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MiniGridPlacement {
    coordinate: GridCoordinate,
    icon_url: Option<String>,
    name: String,
}

impl MiniGridPlacement {
    pub fn new(coordinate: GridCoordinate, icon_url: Option<String>, name: String) -> Self {
        Self {
            coordinate,
            icon_url,
            name,
        }
    }

    pub fn coordinate(&self) -> GridCoordinate {
        self.coordinate
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
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
/// a click reselects) and the domain the scrollable body renders (the active section's
/// moves plus every unresolved ability). Shaping the breadcrumbs needs the Copy
/// selection signal, which arrives as an input.
pub(super) struct ActivePlanView {
    pub(super) breadcrumbs: Vec<BreadcrumbView>,
    pub(super) section: Option<MoveSection>,
    pub(super) unresolved: Vec<UnresolvedView>,
}

/// The inputs that shape an [`ActivePlanView`]: the plan to render, the selected
/// section slug, and the navigation surface its breadcrumbs write to when clicked.
pub(super) struct ActivePlanInputs<'a> {
    pub(super) plan: &'a PlanView,
    pub(super) selected_slug: Option<&'a str>,
    pub(super) view_navigation: ViewNavigationContext,
}

impl From<ActivePlanInputs<'_>> for ActivePlanView {
    fn from(inputs: ActivePlanInputs<'_>) -> Self {
        let ActivePlanInputs {
            plan,
            selected_slug,
            view_navigation,
        } = inputs;
        let active = plan.active_section(selected_slug);
        let active_category = active.map(|section| section.category);
        let mut breadcrumb_list: Vec<BreadcrumbView> = Vec::with_capacity(plan.sections.len());
        for section in &plan.sections {
            let category = section.category;
            let is_active = active_category == Some(category);
            let label = section.title.to_owned();
            let count = section.moves.len();
            let onclick = EventHandler::new(move |_event: MouseEvent| {
                let slug = category.entry_slug().to_owned();
                view_navigation.select_move_category(slug);
            });
            let breadcrumb = BreadcrumbView {
                label,
                count,
                active: is_active,
                onclick,
            };
            breadcrumb_list.push(breadcrumb);
        }
        let breadcrumbs = breadcrumb_list;
        let section = active.cloned();
        let unresolved = plan.unresolved.clone();
        Self {
            breadcrumbs,
            section,
            unresolved,
        }
    }
}

impl AbilityDisplay {
    pub fn object_id(&self) -> WarcraftObjectId {
        self.object_id
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
    pub fn other_carrier_unit_ids(&self) -> &[WarcraftObjectId] {
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
    pub fn mover_unit_id(&self) -> Option<WarcraftObjectId> {
        self.mover_unit_id
    }
    pub fn mover_carrier_unit_ids(&self) -> &[WarcraftObjectId] {
        &self.mover_carrier_unit_ids
    }
    pub fn from(&self) -> GridCoordinate {
        self.from
    }
    pub fn to(&self) -> GridCoordinate {
        self.to
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
    pub fn carrier_unit_ids(&self) -> &[WarcraftObjectId] {
        &self.carrier_unit_ids
    }
    pub fn position(&self) -> GridCoordinate {
        self.position
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

/// The three states the Resolve page renders, each as already shaped data. The
/// component body matches on this and places the data; the hook never builds
/// markup.
pub(super) enum ResolvePagePresentation {
    /// No CustomKeys.txt is loaded yet — the upload prompt.
    NoFile,
    /// A file is loaded but has no conflicts — the all-clear state.
    Clear,
    /// A cascade plan to preview and apply.
    Plan(Box<ResolvePlanPresentation>),
}

/// Everything the plan state needs, fully shaped: the header's summary text and Apply
/// control, the breadcrumb bar, and the domain the body renders (the active section's
/// moves and the unresolved abilities).
pub(super) struct ResolvePlanPresentation {
    pub moves_text: String,
    pub unresolved_count: usize,
    pub running: bool,
    pub on_apply: EventHandler<MouseEvent>,
    pub breadcrumbs: Vec<BreadcrumbView>,
    pub section: Option<MoveSection>,
    pub unresolved: Vec<UnresolvedView>,
}

/// The Apply control's state: whether the cascade is currently running and the
/// handler that spawns it. Owns the `is_running` signal and routes the cascade
/// through the [`CustomKeysService`](crate::services::customkeys) commit boundary —
/// never mutating `CustomKeys` inline — then reports the result via a toast.
pub(super) struct ApplyPlan {
    pub(super) running: bool,
    pub(super) on_apply: EventHandler<MouseEvent>,
}

/// Reconcile the live route into the shell's signals — the read side of the URL
/// contract: announce the Resolve view and sync the selected move-category from the
/// `?entry=` parameter. Reactive on `entry`; the move-category pick that writes the route
/// lives at the breadcrumb mutation site.
fn use_route_reconcile(
    entry: Option<String>,
    view_navigation: ViewNavigationContext,
    resolve_selection: ResolveSelection,
) {
    use_effect(use_reactive!(|entry| {
        view_navigation.restore_view(AppView::Resolve);
        let mut selected = resolve_selection.selected_move_category();
        if *selected.peek() != entry {
            selected.set(entry.clone());
        }
    }));
}

/// Owns the running flag and builds the Apply handler. The handler yields a tick so
/// the running state paints, runs the cascade through the service commit boundary
/// (which normalizes and writes through to storage), and toasts the outcome.
fn use_apply_plan() -> ApplyPlan {
    let loaded_keys = use_loaded_keys();
    let custom_keys_service = use_custom_keys_service();
    let toast_api = use_toast();
    let mut is_running = use_signal(|| false);
    let running = *is_running.read();
    let on_apply = EventHandler::new(move |_event: MouseEvent| {
        if *is_running.read() {
            return;
        }
        let read_guard = loaded_keys.peek();
        if read_guard.as_ref().is_none() {
            return;
        }
        drop(read_guard);
        is_running.set(true);
        spawn(async move {
            TimeoutFuture::new(0).await;
            let plan = custom_keys_service.resolve_conflicts();
            let move_count = plan.move_count();
            let unresolved_count = plan.unresolved_count();
            is_running.set(false);
            let summary = if unresolved_count == 0 {
                format!("Moved {move_count} ability slot(s). No remaining conflicts.")
            } else {
                format!(
                    "Moved {move_count} ability slot(s). {unresolved_count} could not be placed."
                )
            };
            let title = String::from("Cascade applied");
            let toast_options = ToastOptions::new().description(summary);
            toast_api.success(title, toast_options);
        });
    });
    ApplyPlan { running, on_apply }
}

/// Computes the cascade preview (memoised on the loaded keys), reconciles the route,
/// wires the Apply handler, and shapes the active section, breadcrumbs, and header —
/// returning the state's data for the body to render.
pub(super) fn use_resolve_page(props: &ResolvePageModel) -> ResolvePagePresentation {
    let view_navigation = use_view_navigation();
    let resolve_selection = use_resolve_selection();
    let custom_keys_service = use_custom_keys_service();
    let loaded_keys = use_loaded_keys();
    let selected_move_category = resolve_selection.selected_move_category();
    let entry = props.entry.clone().filter(|value| !value.is_empty());
    use_route_reconcile(entry, view_navigation, resolve_selection);
    let plan_memo = use_memo(move || {
        if loaded_keys.read().is_none() {
            return None;
        }
        let plan = custom_keys_service.resolve_preview();
        let plan_view = PlanView::build(&plan);
        Some(plan_view)
    });
    let apply = use_apply_plan();
    let has_file = loaded_keys.read().is_some();
    let plan_option = plan_memo();
    let counts = PlanCounts::from(plan_option.as_ref());
    if !has_file {
        return ResolvePagePresentation::NoFile;
    }
    if counts.move_count == 0 && counts.unresolved_count == 0 {
        return ResolvePagePresentation::Clear;
    }
    let plan = plan_option.expect("plan present when a file is loaded");
    let selected_slug = selected_move_category.read().clone();
    let active_inputs = ActivePlanInputs {
        plan: &plan,
        selected_slug: selected_slug.as_deref(),
        view_navigation,
    };
    let active = ActivePlanView::from(active_inputs);
    let move_count = counts.move_count;
    let move_noun = if move_count == 1 { "move" } else { "moves" };
    let moves_text = format!("{move_count} {move_noun}");
    let presentation = ResolvePlanPresentation {
        moves_text,
        unresolved_count: counts.unresolved_count,
        running: apply.running,
        on_apply: apply.on_apply,
        breadcrumbs: active.breadcrumbs,
        section: active.section,
        unresolved: active.unresolved,
    };
    ResolvePagePresentation::Plan(Box::new(presentation))
}
