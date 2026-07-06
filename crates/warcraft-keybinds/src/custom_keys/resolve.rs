//! The cascade orchestration driver: the domain service that runs the
//! two-phase conflict-resolution algorithm over the aggregate (preview and
//! apply). Split out of the aggregate root as an `impl CustomKeys` continuation.

use super::CustomKeys;
use crate::cascade::conflict_graph::ConflictGraph;
use crate::cascade::planner::{CascadePlan, MoveReason, PlannedMove, UnresolvedMover};
use crate::cascade::queue::{AssignmentQueue, AssignmentScope};
use crate::identity::slot::GridSlotId;
use crate::model::GridCoordinate;
use crate::unit::grids::GridRole;
use std::collections::BTreeMap;
use warcraft_api::WarcraftObjectId;

impl CustomKeys {
    /// Computes the cascade conflict-resolution plan **without mutating
    /// `self`**.  Runs the same two-phase algorithm as `resolve_conflicts`
    /// on a clone and returns the resulting `CascadePlan`.
    ///
    /// This is the entry point for "preview before apply" UI flows: render
    /// the plan to the user, let them confirm, then call
    /// `resolve_conflicts` to actually apply it.  The returned plan
    /// includes the per-move rationale (`MoveReason`) so the UI can show
    /// *why* each move would happen.
    ///
    /// See `resolve_conflicts` for the algorithm description.
    pub fn preview_resolve(&self) -> CascadePlan {
        let mut working_copy = self.clone();
        working_copy.run_iterative_cascade()
    }

    /// Runs the cascade conflict-resolution algorithm and applies its plan to
    /// this `CustomKeys`.  This is a user-triggered, opt-in operation — it is
    /// **not** called from `normalize()` or the boot path.  Use it when the
    /// user explicitly asks the app to try resolving collisions (typically
    /// before export).
    ///
    /// Only **positions** are written back; hotkeys are untouched.  Hotkeys
    /// belong to `assign_position` and `apply_grid_to_all_bindings`; the
    /// cascade just redistributes geometry to remove cross-unit collisions
    /// (and pack rows left where it can).
    ///
    /// **Two phases**:
    ///   1. **Cross-unit cascade** (`AssignmentScope::CrossUnitOnly`) — the
    ///      classic cascade, treating only multi-carrier and pinned slots
    ///      as anchor candidates.  Settles all cross-unit collisions first.
    ///   2. **Intra-unit cleanup** (`AssignmentScope::IncludingIntraUnit`)
    ///      — a second pass with single-carrier abilities also eligible.
    ///      Resolves the remaining "two shop items on the same Goblin
    ///      Merchant slot" style collisions that phase 1 deliberately left
    ///      alone.
    ///
    /// Each phase loops to a fixed point because the spill step can create
    /// new gap-pull opportunities that a follow-up pass closes.  The returned
    /// `CascadePlan` aggregates every net position change from the starting
    /// state to the final state so the caller sees a single `(old → new)` per
    /// ability.  Unresolved nodes are the ones still stuck after both phases.
    ///
    /// Implemented in terms of `preview_resolve` so the algorithm logic
    /// stays single-sourced — this method runs `preview_resolve` on `self`
    /// and then applies the resulting plan back to `self`.
    pub fn resolve_conflicts(&mut self) -> CascadePlan {
        let cascade_plan = self.preview_resolve();
        for planned_move in cascade_plan.moves() {
            let application = MoveApplication::from_planned_move(planned_move);
            self.apply_resolved_position(application);
        }
        cascade_plan
    }

    /// Runs the iterative two-phase cascade on `self`, mutating positions
    /// in place as moves are emitted.  Returns the net plan across both
    /// phases.  Internal helper for `preview_resolve` (clone + run) and the
    /// implementation backbone of `resolve_conflicts`.
    fn run_iterative_cascade(&mut self) -> CascadePlan {
        let mut net_moves: BTreeMap<MoveKey, AccumulatedMove> = BTreeMap::new();
        let _phase_one_unresolved =
            self.run_cascade_phase(AssignmentScope::CrossUnitOnly, &mut net_moves);
        let last_unresolved =
            self.run_cascade_phase(AssignmentScope::IncludingIntraUnit, &mut net_moves);
        let mut combined_moves: Vec<PlannedMove> = Vec::new();
        for (key, accumulated) in net_moves {
            if accumulated.old_position == accumulated.new_position {
                continue;
            }
            let planned_move = PlannedMove::new(
                key.slot_id,
                key.grid_role,
                accumulated.old_position,
                accumulated.new_position,
                accumulated.carrier_unit_ids,
                accumulated.reason,
            );
            combined_moves.push(planned_move);
        }
        CascadePlan::from_parts(combined_moves, last_unresolved)
    }

    /// Drives one cascade phase to a fixed point under the given
    /// `AssignmentScope`.  Each iteration rebuilds the conflict graph,
    /// builds the queue with that scope, applies every planned move, and
    /// merges the moves into `net_moves` (so a single ability that moves
    /// across multiple iterations collapses into one `(old → new)` entry).
    /// Returns the unresolved set from the final iteration.
    fn run_cascade_phase(
        &mut self,
        scope: AssignmentScope,
        net_moves: &mut BTreeMap<MoveKey, AccumulatedMove>,
    ) -> Vec<UnresolvedMover> {
        const MAX_ITERATIONS_PER_PHASE: usize = 32;
        let mut last_unresolved: Vec<UnresolvedMover> = Vec::new();
        for _ in 0..MAX_ITERATIONS_PER_PHASE {
            let graph = ConflictGraph::build(self);
            let queue = AssignmentQueue::build_with_scope(graph, scope);
            let pass_plan = CascadePlan::from(&queue);
            last_unresolved = pass_plan.unresolved().to_vec();
            if pass_plan.move_count() == 0 {
                break;
            }
            for planned_move in pass_plan.moves() {
                let key = MoveKey {
                    slot_id: planned_move.slot_id(),
                    grid_role: planned_move.grid_role(),
                };
                let new_position = planned_move.new_position();
                let carrier_unit_ids: Vec<WarcraftObjectId> =
                    planned_move.carrier_unit_ids().to_vec();
                let move_reason: MoveReason = planned_move.reason().clone();
                let fresh_reason = move_reason.clone();
                net_moves
                    .entry(key)
                    .and_modify(|accumulated| {
                        accumulated.new_position = new_position;
                        accumulated.reason = move_reason;
                    })
                    .or_insert_with(|| AccumulatedMove {
                        old_position: planned_move.old_position(),
                        new_position,
                        carrier_unit_ids,
                        reason: fresh_reason,
                    });
                let application = MoveApplication::from_planned_move(planned_move);
                self.apply_resolved_position(application);
            }
        }
        last_unresolved
    }

    fn apply_resolved_position(&mut self, application: MoveApplication) {
        let is_research_context = application.grid_role.is_research_context();
        let new_position = application.new_position;
        match application.slot_id {
            GridSlotId::Ability(ability_id) => {
                let Some(binding) = self.binding_or_default_mut(ability_id) else {
                    return;
                };
                if is_research_context {
                    binding.set_research_button_position(Some(new_position));
                } else {
                    let old_button_position = binding.button_position().copied();
                    let old_unbutton_position = binding.unbutton_position().copied();
                    let off_was_colocated = old_unbutton_position.is_some()
                        && old_unbutton_position == old_button_position;
                    binding.set_button_position(Some(new_position));
                    if off_was_colocated {
                        binding.set_unbutton_position(Some(new_position));
                    }
                }
            }
            GridSlotId::AbilityOff(ability_id) => {
                let Some(binding) = self.binding_or_default_mut(ability_id) else {
                    return;
                };
                binding.set_unbutton_position(Some(new_position));
            }
            GridSlotId::Command(command_id) => {
                let Some(binding) = self.command_or_default_mut(command_id) else {
                    return;
                };
                binding.set_button_position(Some(new_position));
                binding.set_unbutton_position(Some(new_position));
            }
        }
    }
}

/// Snapshot of a single `PlannedMove` decoupled from the plan's borrow, so
/// `resolve_conflicts` can release its read of `&self` before mutating.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct MoveApplication {
    slot_id: GridSlotId,
    grid_role: GridRole,
    new_position: GridCoordinate,
}

impl MoveApplication {
    fn from_planned_move(planned_move: &PlannedMove) -> Self {
        Self {
            slot_id: planned_move.slot_id(),
            grid_role: planned_move.grid_role(),
            new_position: planned_move.new_position(),
        }
    }
}

/// Identifies a slot/role pair across multiple `resolve_conflicts` iterations
/// so we can collapse repeated moves of the same ability into a single
/// `(original → final)` entry.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct MoveKey {
    slot_id: GridSlotId,
    grid_role: GridRole,
}

/// Net movement of a single slot accumulated across iterations.  The
/// `old_position` is the first one we saw (before any mutation), the
/// `new_position` is updated on each subsequent move so the final value
/// reflects where the slot ended up.  `reason` is overwritten on each
/// update so it always reflects the *last* event that placed the slot —
/// earlier iterations were superseded by the most recent move.
#[derive(Clone, Debug, PartialEq, Eq)]
struct AccumulatedMove {
    old_position: GridCoordinate,
    new_position: GridCoordinate,
    carrier_unit_ids: Vec<WarcraftObjectId>,
    reason: MoveReason,
}
