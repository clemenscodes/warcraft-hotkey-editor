use crate::cascade::queue::{AssignmentQueue, GroupKind, PositionAssignmentGroup};
use crate::identity::slot::GridSlotId;
use crate::model::GridCoordinate;
use crate::unit::grids::GridRole;
use std::fmt;
use warcraft_api::WarcraftObjectId;

/// Why a particular `PlannedMove` happened.
///
/// Every move emitted by the cascade falls into exactly one of these four
/// categories.  The variant data points back at the conflict or anchor that
/// caused the move so the UI can render per-move rationale (tooltips,
/// journal lines, preview list).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MoveReason {
    /// Phase 1: lost a same-cell fight to a higher-carrier ability.
    /// `anchor_slot` is the winning ability that stayed in the cell;
    /// `anchor_carrier_unit_ids` are the units it ties together — the count
    /// of them is what caused it to outrank the mover, and the ids let the UI
    /// list exactly which units carry the winning ability.
    Fight {
        anchor_slot: GridSlotId,
        anchor_carrier_unit_ids: Vec<WarcraftObjectId>,
    },
    /// Phase 2: cross-row spill from a stuck cell.  `from_position` is the
    /// stuck cell the ability was sitting on before being rehomed.  Used
    /// when no swap partner was needed (clean move into a non-conflicting
    /// cell) or when the destination row differs from the stuck row.
    Spill { from_position: GridCoordinate },
    /// Phase 2: same-row swap during the spill phase.  `swapped_with` is
    /// the slot the move displaced: for a spill anchor that's the
    /// incumbent it pushed back to its old cell; for an incumbent that's
    /// the spilling ability now occupying its old cell.
    Swap { swapped_with: GridSlotId },
    /// Phase 1: pulled leftward to fill a row gap left by an earlier
    /// cascade.  `source_position` is the ability's pre-pull cell.
    GapPull { source_position: GridCoordinate },
}

/// One ability successfully relocated by the cascade solver.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlannedMove {
    slot_id: GridSlotId,
    grid_role: GridRole,
    old_position: GridCoordinate,
    new_position: GridCoordinate,
    carrier_unit_ids: Vec<WarcraftObjectId>,
    reason: MoveReason,
}

impl PlannedMove {
    pub fn new(
        slot_id: GridSlotId,
        grid_role: GridRole,
        old_position: GridCoordinate,
        new_position: GridCoordinate,
        carrier_unit_ids: Vec<WarcraftObjectId>,
        reason: MoveReason,
    ) -> Self {
        Self {
            slot_id,
            grid_role,
            old_position,
            new_position,
            carrier_unit_ids,
            reason,
        }
    }

    pub fn slot_id(&self) -> GridSlotId {
        self.slot_id
    }

    pub fn grid_role(&self) -> GridRole {
        self.grid_role
    }

    pub fn old_position(&self) -> GridCoordinate {
        self.old_position
    }

    pub fn new_position(&self) -> GridCoordinate {
        self.new_position
    }

    pub fn carrier_count(&self) -> usize {
        self.carrier_unit_ids.len()
    }

    pub fn carrier_unit_ids(&self) -> &[WarcraftObjectId] {
        &self.carrier_unit_ids
    }

    pub fn reason(&self) -> &MoveReason {
        &self.reason
    }
}

/// One ability the solver could not relocate — the queue ran out of valid
/// same-row slots while cascading rightward and the ability is stuck at the
/// position recorded here.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UnresolvedMover {
    slot_id: GridSlotId,
    grid_role: GridRole,
    collision_position: GridCoordinate,
    carrier_unit_ids: Vec<WarcraftObjectId>,
}

impl UnresolvedMover {
    pub fn slot_id(&self) -> GridSlotId {
        self.slot_id
    }

    pub fn grid_role(&self) -> GridRole {
        self.grid_role
    }

    pub fn collision_position(&self) -> GridCoordinate {
        self.collision_position
    }

    pub fn carrier_count(&self) -> usize {
        self.carrier_unit_ids.len()
    }

    pub fn carrier_unit_ids(&self) -> &[WarcraftObjectId] {
        &self.carrier_unit_ids
    }
}

/// The full output of the cascade position solver.
///
/// Contains every move that was successfully planned plus every mover that
/// could not be placed (same-row sacred, row full of higher-carrier
/// neighbors).  Unresolved movers are left at their last attempted position
/// and must be handled separately.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct CascadePlan {
    moves: Vec<PlannedMove>,
    unresolved: Vec<UnresolvedMover>,
}

impl CascadePlan {
    pub fn from_parts(moves: Vec<PlannedMove>, unresolved: Vec<UnresolvedMover>) -> Self {
        Self { moves, unresolved }
    }

    pub fn moves(&self) -> &[PlannedMove] {
        &self.moves
    }

    pub fn unresolved(&self) -> &[UnresolvedMover] {
        &self.unresolved
    }

    pub fn move_count(&self) -> usize {
        self.moves.len()
    }

    pub fn unresolved_count(&self) -> usize {
        self.unresolved.len()
    }

    pub fn is_fully_resolved(&self) -> bool {
        self.unresolved.is_empty()
    }
}

/// Translates the queue's final assignment into a plan of position changes.
///
/// The queue has already done the work: every node has a `final_position`,
/// every stuck node is in `unresolved_nodes()`.  The planner just diffs the
/// final state against each node's original position and emits one
/// `PlannedMove` per change and one `UnresolvedMover` per stuck node.
impl From<&AssignmentQueue> for CascadePlan {
    fn from(queue: &AssignmentQueue) -> Self {
        let graph = queue.graph();
        let mut moves: Vec<PlannedMove> = Vec::new();
        let mut unresolved: Vec<UnresolvedMover> = Vec::new();
        for (node_index, node) in graph.nodes().iter().enumerate() {
            let slot_id = node.slot_id();
            let grid_role = node.grid_role();
            let carrier_unit_ids: Vec<WarcraftObjectId> = node.carrier_unit_ids().to_vec();
            let original_position = node.current_position();
            let final_position = queue.final_position(node_index);
            if queue.is_unresolved(node_index) {
                let unresolved_mover = UnresolvedMover {
                    slot_id,
                    grid_role,
                    collision_position: final_position,
                    carrier_unit_ids,
                };
                unresolved.push(unresolved_mover);
                continue;
            }
            if original_position == final_position {
                continue;
            }
            let reason = queue
                .move_reason_for_node(node_index)
                .expect("a node whose position changed must have a queue event explaining it");
            let planned_move = PlannedMove {
                slot_id,
                grid_role,
                old_position: original_position,
                new_position: final_position,
                carrier_unit_ids,
                reason,
            };
            moves.push(planned_move);
        }
        Self { moves, unresolved }
    }
}

impl AssignmentQueue {
    /// Walk the queue's groups in reverse order to find the last event that
    /// determined this node's final position, and translate it into a
    /// `MoveReason`.
    ///
    /// A node may pass through multiple groups (a phase-1 fight mover may later
    /// be spill-relocated, etc.).  The *latest* group that touched the node is
    /// the one that placed it where it ended up — earlier events were
    /// superseded by it.
    fn move_reason_for_node(&self, node_index: usize) -> Option<MoveReason> {
        let groups = self.groups();
        for group in groups.iter().rev() {
            if let Some(reason) = group.move_reason_for_node(self, node_index) {
                return Some(reason);
            }
        }
        None
    }
}

impl PositionAssignmentGroup {
    fn move_reason_for_node(
        &self,
        queue: &AssignmentQueue,
        node_index: usize,
    ) -> Option<MoveReason> {
        let graph = queue.graph();
        let is_anchor = self.anchor_index() == node_index;
        let is_mover = self.mover_indices().contains(&node_index);
        if !is_anchor && !is_mover {
            return None;
        }
        match self.kind() {
            GroupKind::Fight => {
                if !is_mover {
                    return None;
                }
                let anchor_node = graph.node(self.anchor_index());
                let anchor_slot = anchor_node.slot_id();
                let anchor_carrier_unit_ids = anchor_node.carrier_unit_ids().to_vec();
                let reason = MoveReason::Fight {
                    anchor_slot,
                    anchor_carrier_unit_ids,
                };
                Some(reason)
            }
            GroupKind::GapPull { source_position } => {
                if !is_anchor {
                    return None;
                }
                let reason = MoveReason::GapPull { source_position };
                Some(reason)
            }
            GroupKind::Spill { stuck_position } => {
                if is_anchor {
                    let stays_in_row = self.position().row() == stuck_position.row();
                    if stays_in_row {
                        let has_swap_partner = !self.mover_indices().is_empty();
                        if has_swap_partner {
                            let first_incumbent = self.mover_indices()[0];
                            let swapped_with = graph.node(first_incumbent).slot_id();
                            let reason = MoveReason::Swap { swapped_with };
                            return Some(reason);
                        }
                        let reason = MoveReason::GapPull {
                            source_position: stuck_position,
                        };
                        return Some(reason);
                    }
                    let reason = MoveReason::Spill {
                        from_position: stuck_position,
                    };
                    return Some(reason);
                }
                let anchor_slot = graph.node(self.anchor_index()).slot_id();
                let reason = MoveReason::Swap {
                    swapped_with: anchor_slot,
                };
                Some(reason)
            }
        }
    }
}

impl fmt::Display for MoveReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fight {
                anchor_slot,
                anchor_carrier_unit_ids,
            } => {
                let anchor_id = anchor_slot.as_str();
                let anchor_carrier_count = anchor_carrier_unit_ids.len();
                write!(
                    formatter,
                    "lost fight to {anchor_id} ({anchor_carrier_count} carriers)",
                )
            }
            Self::Spill { from_position } => {
                let column = u8::from(from_position.column());
                let row = u8::from(from_position.row());
                write!(formatter, "spilled from stuck cell ({column},{row})")
            }
            Self::Swap { swapped_with } => {
                let swap_id = swapped_with.as_str();
                write!(formatter, "swapped with {swap_id}")
            }
            Self::GapPull { source_position } => {
                let column = u8::from(source_position.column());
                let row = u8::from(source_position.row());
                write!(formatter, "gap-pulled from ({column},{row})")
            }
        }
    }
}

impl fmt::Display for CascadePlan {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.moves.is_empty() && self.unresolved.is_empty() {
            return writeln!(formatter, "Cascade plan: no moves — queue was empty.");
        }
        writeln!(
            formatter,
            "Cascade plan: {} move(s), {} unresolved\n",
            self.moves.len(),
            self.unresolved.len(),
        )?;
        if !self.moves.is_empty() {
            writeln!(formatter, "Moves:")?;
            for planned_move in &self.moves {
                let name = planned_move.slot_id.display_name(None, None);
                let id = planned_move.slot_id.as_str();
                let role = planned_move.grid_role.label();
                let old_col = u8::from(planned_move.old_position.column());
                let old_row = u8::from(planned_move.old_position.row());
                let new_col = u8::from(planned_move.new_position.column());
                let new_row = u8::from(planned_move.new_position.row());
                let carrier_count = planned_move.carrier_count();
                let carrier_ids = planned_move
                    .carrier_unit_ids
                    .iter()
                    .map(|carrier_id| carrier_id.value())
                    .collect::<Vec<_>>()
                    .join(", ");
                let reason = &planned_move.reason;
                writeln!(
                    formatter,
                    "  {name} ({id})  [{role}]  ({old_col},{old_row}) → ({new_col},{new_row})  \
                     [{carrier_count} carriers: {carrier_ids}]  — {reason}",
                )?;
            }
        }
        if !self.unresolved.is_empty() {
            writeln!(formatter)?;
            writeln!(formatter, "Unresolved (no valid position found):")?;
            for mover in &self.unresolved {
                let name = mover.slot_id.display_name(None, None);
                let id = mover.slot_id.as_str();
                let role = mover.grid_role.label();
                let column = u8::from(mover.collision_position.column());
                let row = u8::from(mover.collision_position.row());
                let carrier_count = mover.carrier_count();
                let carrier_ids = mover
                    .carrier_unit_ids
                    .iter()
                    .map(|carrier_id| carrier_id.value())
                    .collect::<Vec<_>>()
                    .join(", ");
                writeln!(
                    formatter,
                    "  {name} ({id})  [{role}]  stayed at ({column},{row})  \
                     [{carrier_count} carriers: {carrier_ids}]",
                )?;
            }
        }
        Ok(())
    }
}

impl ddd::Layered for MoveReason {
    type Layer = ddd::DomainLayer;
}

impl ddd::ValueObject for MoveReason {}

impl ddd::Layered for PlannedMove {
    type Layer = ddd::DomainLayer;
}

impl ddd::ValueObject for PlannedMove {}

impl ddd::Layered for UnresolvedMover {
    type Layer = ddd::DomainLayer;
}

impl ddd::ValueObject for UnresolvedMover {}

impl ddd::Layered for CascadePlan {
    type Layer = ddd::DomainLayer;
}

impl ddd::ValueObject for CascadePlan {}

#[cfg(test)]
mod ddd_marker_tests {
    use super::CascadePlan;
    use super::MoveReason;
    use super::PlannedMove;
    use super::UnresolvedMover;
    use crate::ddd_conformance::assert_value_object;

    #[test]
    fn cascade_plan_types_are_value_objects() {
        assert_value_object::<MoveReason>();
        assert_value_object::<PlannedMove>();
        assert_value_object::<UnresolvedMover>();
        assert_value_object::<CascadePlan>();
    }
}

#[cfg(test)]
mod cascade_planner_tests {
    use super::*;
    use crate::cascade::conflict_graph::ConflictGraph;
    use crate::cascade::queue::AssignmentQueue;
    use crate::custom_keys::CustomKeys;
    use crate::grid::layout::{COMMAND_GRID_COLUMNS, COMMAND_GRID_ROWS};
    use crate::model::{AbilityBinding, ColumnIndex, GridCoordinate, RowIndex};
    use std::collections::HashSet;

    fn default_plan() -> CascadePlan {
        let custom_keys = CustomKeys::from_text("");
        let graph = ConflictGraph::build(&custom_keys);
        let queue = AssignmentQueue::build(graph);
        CascadePlan::from(&queue)
    }

    #[test]
    fn default_keys_produce_at_least_one_move() {
        let plan = default_plan();
        assert!(
            plan.move_count() > 0,
            "default keys have known collisions so the plan must contain at least one move",
        );
    }

    #[test]
    fn no_two_moves_land_on_the_same_position_for_conflicting_abilities() {
        let custom_keys = CustomKeys::from_text("");
        let graph = ConflictGraph::build(&custom_keys);
        let queue = AssignmentQueue::build(graph);
        let plan = CascadePlan::from(&queue);
        let mut final_positions: Vec<GridCoordinate> = queue
            .graph()
            .nodes()
            .iter()
            .map(|node| node.current_position())
            .collect();
        for planned_move in plan.moves() {
            let node_index = queue
                .graph()
                .nodes()
                .iter()
                .position(|node| {
                    node.slot_id() == planned_move.slot_id()
                        && node.grid_role() == planned_move.grid_role()
                })
                .expect("moved node must exist in graph");
            final_positions[node_index] = planned_move.new_position();
        }
        let unresolved_indices: HashSet<usize> = plan
            .unresolved()
            .iter()
            .filter_map(|mover| {
                queue.graph().nodes().iter().position(|node| {
                    node.slot_id() == mover.slot_id() && node.grid_role() == mover.grid_role()
                })
            })
            .collect();
        for (first_index, first_node) in queue.graph().nodes().iter().enumerate() {
            for &second_index in queue.graph().neighbors(first_index) {
                if second_index <= first_index {
                    continue;
                }
                let second_node = queue.graph().node(second_index);
                if first_node.carrier_count() < 2 || second_node.carrier_count() < 2 {
                    continue;
                }
                if unresolved_indices.contains(&first_index)
                    || unresolved_indices.contains(&second_index)
                {
                    continue;
                }
                let first_pos = final_positions[first_index];
                let second_pos = final_positions[second_index];
                assert!(
                    first_pos != second_pos || first_node.grid_role() != second_node.grid_role(),
                    "after planning, {} and {} share position ({},{}) on [{}] — still colliding",
                    first_node.slot_id().as_str(),
                    second_node.slot_id().as_str(),
                    u8::from(first_pos.column()),
                    u8::from(first_pos.row()),
                    first_node.grid_role().label(),
                );
            }
        }
    }

    #[test]
    fn same_row_reflow_is_classified_as_gap_pull_not_spill() {
        let plan = default_plan();
        let acfr_move = plan
            .moves()
            .iter()
            .find(|planned_move| planned_move.slot_id().as_str() == "ACfr")
            .expect("ACfr must be moved in the default plan");
        assert_eq!(
            acfr_move.old_position().row(),
            acfr_move.new_position().row(),
            "precondition: ACfr never leaves its row",
        );
        let reason = acfr_move.reason();
        assert!(
            matches!(reason, MoveReason::GapPull { .. }),
            "a same-row reflow must be a GapPull, got {reason:?}",
        );
    }

    #[test]
    fn every_spill_move_changes_row() {
        let plan = default_plan();
        for planned_move in plan.moves() {
            let reason = planned_move.reason();
            if matches!(reason, MoveReason::Spill { .. }) {
                let old_row = u8::from(planned_move.old_position().row());
                let new_row = u8::from(planned_move.new_position().row());
                assert_ne!(
                    old_row,
                    new_row,
                    "{} is labeled Spill but stays in row {old_row}",
                    planned_move.slot_id().as_str(),
                );
            }
        }
    }

    #[test]
    fn all_moves_change_position() {
        let plan = default_plan();
        for planned_move in plan.moves() {
            assert_ne!(
                planned_move.old_position(),
                planned_move.new_position(),
                "a PlannedMove must move to a different position",
            );
        }
    }

    #[test]
    fn all_new_positions_are_within_grid_bounds() {
        let plan = default_plan();
        for planned_move in plan.moves() {
            let column = u8::from(planned_move.new_position().column());
            let row = u8::from(planned_move.new_position().row());
            assert!(
                column < COMMAND_GRID_COLUMNS,
                "column {column} is out of bounds"
            );
            assert!(row < COMMAND_GRID_ROWS, "row {row} is out of bounds");
        }
    }

    #[test]
    fn most_moves_stay_in_their_original_row() {
        let plan = default_plan();
        let total_moves = plan.move_count();
        if total_moves == 0 {
            return;
        }
        let cross_row_moves = plan
            .moves()
            .iter()
            .filter(|planned_move| {
                planned_move.old_position().row() != planned_move.new_position().row()
            })
            .count();
        let cross_row_share_basis_points = cross_row_moves * 100;
        let allowed_share_basis_points = total_moves * 30;
        assert!(
            cross_row_share_basis_points < allowed_share_basis_points,
            "cross-row moves should be rare ({cross_row_moves} of {total_moves}) — \
             spill phase may be overactive",
        );
    }

    #[test]
    fn unresolved_mover_stays_on_its_original_row() {
        let plan = default_plan();
        let custom_keys = CustomKeys::from_text("");
        let graph = ConflictGraph::build(&custom_keys);
        for mover in plan.unresolved() {
            let original_row = graph
                .nodes()
                .iter()
                .find(|node| {
                    node.slot_id() == mover.slot_id() && node.grid_role() == mover.grid_role()
                })
                .map(|node| u8::from(node.current_position().row()))
                .expect("unresolved node must exist in the graph");
            let stuck_row = u8::from(mover.collision_position().row());
            assert_eq!(
                original_row,
                stuck_row,
                "unresolved mover {} ended on row {} but started on row {}",
                mover.slot_id().as_str(),
                stuck_row,
                original_row,
            );
        }
    }

    #[test]
    fn every_move_has_a_documented_reason() {
        let plan = default_plan();
        assert!(
            plan.move_count() > 0,
            "default keys must produce moves for this test to be meaningful",
        );
        for planned_move in plan.moves() {
            match planned_move.reason() {
                MoveReason::Fight {
                    anchor_carrier_unit_ids,
                    ..
                } => {
                    let anchor_carrier_count = anchor_carrier_unit_ids.len();
                    assert!(
                        anchor_carrier_count >= 1,
                        "Fight anchor must carry at least one unit, got {anchor_carrier_count}",
                    );
                }
                MoveReason::Spill { .. } | MoveReason::Swap { .. } | MoveReason::GapPull { .. } => {
                }
            }
        }
    }

    #[test]
    fn fight_mover_reason_points_at_winning_anchor() {
        let collision_position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
        let binding = AbilityBinding::builder()
            .button_position(collision_position)
            .build();
        let mut custom_keys = CustomKeys::from_text("");
        custom_keys.put_ability("AHhb", binding.clone());
        custom_keys.put_ability("AHds", binding);
        let graph = ConflictGraph::build(&custom_keys);
        let queue = AssignmentQueue::build(graph);
        let plan = CascadePlan::from(&queue);
        let fight_reason = plan
            .moves()
            .iter()
            .map(|planned_move| planned_move.reason())
            .find(|reason| matches!(reason, MoveReason::Fight { .. }));
        let Some(MoveReason::Fight {
            anchor_carrier_unit_ids,
            ..
        }) = fight_reason
        else {
            panic!("a same-cell Paladin collision must produce at least one Fight-reason move",);
        };
        assert!(
            !anchor_carrier_unit_ids.is_empty(),
            "the winning anchor must expose the units that carry it, not just a count",
        );
    }

    #[test]
    fn single_collision_pair_is_resolved() {
        let collision_position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
        let binding = AbilityBinding::builder()
            .button_position(collision_position)
            .build();
        let mut custom_keys = CustomKeys::from_text("");
        custom_keys.put_ability("AHhb", binding.clone());
        custom_keys.put_ability("AHds", binding);
        let graph = ConflictGraph::build(&custom_keys);
        let queue = AssignmentQueue::build(graph);
        let plan = CascadePlan::from(&queue);
        assert!(
            plan.move_count() >= 1,
            "a single Paladin collision must produce at least one move",
        );
    }
}
