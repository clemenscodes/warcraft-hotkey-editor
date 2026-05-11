use std::fmt;

use warcraft_api::WarcraftObjectId;

use crate::cascade_queue::AssignmentQueue;
use crate::model::GridCoordinate;
use crate::slot::GridSlotId;
use crate::unit_grids::GridRole;

/// One ability successfully relocated by the cascade solver.
#[derive(Clone)]
pub struct PlannedMove {
    slot_id: GridSlotId,
    grid_role: GridRole,
    old_position: GridCoordinate,
    new_position: GridCoordinate,
    carrier_unit_ids: Vec<WarcraftObjectId>,
}

impl PlannedMove {
    pub fn new(
        slot_id: GridSlotId,
        grid_role: GridRole,
        old_position: GridCoordinate,
        new_position: GridCoordinate,
        carrier_unit_ids: Vec<WarcraftObjectId>,
    ) -> Self {
        Self {
            slot_id,
            grid_role,
            old_position,
            new_position,
            carrier_unit_ids,
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
}

/// One ability the solver could not relocate — the queue ran out of valid
/// same-row slots while cascading rightward and the ability is stuck at the
/// position recorded here.
#[derive(Clone)]
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
pub fn solve(queue: &AssignmentQueue) -> CascadePlan {
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

        if original_position != final_position {
            let planned_move = PlannedMove {
                slot_id,
                grid_role,
                old_position: original_position,
                new_position: final_position,
                carrier_unit_ids,
            };
            moves.push(planned_move);
        }
    }

    CascadePlan { moves, unresolved }
}

fn grid_role_label(role: GridRole) -> &'static str {
    match role {
        GridRole::MainCommand => "main command",
        GridRole::BuildMenu => "build menu",
        GridRole::UprootedForm => "uprooted",
        GridRole::HeroSkillTree => "research",
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
                let role = grid_role_label(planned_move.grid_role);
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
                writeln!(
                    formatter,
                    "  {name} ({id})  [{role}]  ({old_col},{old_row}) → ({new_col},{new_row})  \
                     [{carrier_count} carriers: {carrier_ids}]"
                )?;
            }
        }

        if !self.unresolved.is_empty() {
            writeln!(formatter)?;
            writeln!(formatter, "Unresolved (no valid position found):")?;
            for mover in &self.unresolved {
                let name = mover.slot_id.display_name(None, None);
                let id = mover.slot_id.as_str();
                let role = grid_role_label(mover.grid_role);
                let col = u8::from(mover.collision_position.column());
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
                    "  {name} ({id})  [{role}]  stayed at ({col},{row})  \
                     [{carrier_count} carriers: {carrier_ids}]"
                )?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod cascade_planner_tests {
    use std::collections::HashSet;

    use super::*;
    use crate::cascade_queue::AssignmentQueue;
    use crate::conflict_graph::ConflictGraph;
    use crate::custom_keys::CustomKeys;
    use crate::grid_layout::{COMMAND_GRID_COLUMNS, COMMAND_GRID_ROWS};
    use crate::model::{AbilityBinding, ColumnIndex, GridCoordinate, RowIndex};

    fn default_plan() -> CascadePlan {
        let custom_keys = CustomKeys::from("").normalize();
        let graph = ConflictGraph::build(&custom_keys);
        let queue = AssignmentQueue::build(graph);
        solve(&queue)
    }

    #[test]
    fn default_keys_produce_at_least_one_move() {
        let plan = default_plan();
        assert!(
            plan.move_count() > 0,
            "default keys have known collisions so the plan must contain at least one move"
        );
    }

    #[test]
    fn no_two_moves_land_on_the_same_position_for_conflicting_abilities() {
        let custom_keys = CustomKeys::from("").normalize();
        let graph = ConflictGraph::build(&custom_keys);
        let queue = AssignmentQueue::build(graph);
        let plan = solve(&queue);

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
                // `AssignmentQueue::build` defaults to `CrossUnitOnly`.  In
                // that scope the planner only resolves cross-unit collisions
                // (carrier_count ≥ 2 on both endpoints); intra-unit
                // collisions are phase 2 of `CustomKeys::resolve_conflicts`
                // and not visible to a single-pass planner test.
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
                    grid_role_label(first_node.grid_role()),
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
                "a PlannedMove must move to a different position"
            );
        }
    }

    #[test]
    fn all_new_positions_are_within_grid_bounds() {
        let plan = default_plan();
        for planned_move in plan.moves() {
            let col = u8::from(planned_move.new_position().column());
            let row = u8::from(planned_move.new_position().row());
            assert!(col < COMMAND_GRID_COLUMNS, "column {col} is out of bounds");
            assert!(row < COMMAND_GRID_ROWS, "row {row} is out of bounds");
        }
    }

    #[test]
    fn most_moves_stay_in_their_original_row() {
        // Same-row is preferred but no longer absolute.  Phase 2 of the queue
        // rehomes unresolved abilities cross-row when their original row has
        // no usable cell — a deliberate exception, since a persistent
        // collision is worse than a hotkey change.  Cross-row should still be
        // rare, not the norm.
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
        let cross_row_ratio = (cross_row_moves as f64) / (total_moves as f64);
        assert!(
            cross_row_ratio < 0.30,
            "cross-row moves should be rare ({} of {} = {:.0}%) — spill phase may be overactive",
            cross_row_moves,
            total_moves,
            cross_row_ratio * 100.0,
        );
    }

    #[test]
    fn unresolved_mover_stays_on_its_original_row() {
        let plan = default_plan();
        let custom_keys = CustomKeys::from("").normalize();
        let graph = ConflictGraph::build(&custom_keys);
        for mover in plan.unresolved() {
            let original_row_value = graph
                .nodes()
                .iter()
                .find(|node| {
                    node.slot_id() == mover.slot_id() && node.grid_role() == mover.grid_role()
                })
                .map(|node| u8::from(node.current_position().row()))
                .expect("unresolved node must exist in the graph");
            let stuck_row_value = u8::from(mover.collision_position().row());
            assert_eq!(
                original_row_value,
                stuck_row_value,
                "unresolved mover {} ended on row {} but started on row {}",
                mover.slot_id().as_str(),
                stuck_row_value,
                original_row_value,
            );
        }
    }

    #[test]
    fn single_collision_pair_is_resolved() {
        let collision_position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
        let binding = AbilityBinding::builder()
            .button_position(collision_position)
            .build();
        let mut custom_keys = CustomKeys::from("").normalize();
        custom_keys.put_ability("AHhb", binding.clone());
        custom_keys.put_ability("AHds", binding);
        let graph = ConflictGraph::build(&custom_keys);
        let queue = AssignmentQueue::build(graph);
        let plan = solve(&queue);
        assert!(
            plan.move_count() >= 1,
            "a single Paladin collision must produce at least one move"
        );
    }
}
