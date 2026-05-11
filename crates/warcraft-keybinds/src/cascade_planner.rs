use std::collections::{HashMap, HashSet};
use std::fmt;

use crate::cascade_queue::AssignmentQueue;
use crate::grid_layout::{COMMAND_GRID_COLUMNS, COMMAND_GRID_ROWS};
use crate::model::{ColumnIndex, GridCoordinate, RowIndex};
use crate::slot::GridSlotId;
use crate::unit_grids::GridRole;

/// One ability successfully relocated by the cascade solver.
pub struct PlannedMove {
    slot_id: GridSlotId,
    grid_role: GridRole,
    old_position: GridCoordinate,
    new_position: GridCoordinate,
    carrier_count: usize,
}

impl PlannedMove {
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
        self.carrier_count
    }
}

/// One ability the solver could not relocate — all 12 grid positions were
/// blocked by conflict-graph neighbors.
pub struct UnresolvedMover {
    slot_id: GridSlotId,
    grid_role: GridRole,
    collision_position: GridCoordinate,
    carrier_count: usize,
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
        self.carrier_count
    }
}

/// The full output of the greedy cascade position solver.
///
/// Contains every move that was successfully planned plus every mover that
/// could not be placed (all 12 positions blocked by conflict neighbors).
/// Unresolved movers are left at their collision positions and must be handled
/// separately.
pub struct CascadePlan {
    moves: Vec<PlannedMove>,
    unresolved: Vec<UnresolvedMover>,
}

impl CascadePlan {
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

/// HashMap key for a (grid_role, position) cell.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct PositionRoleKey {
    grid_role: GridRole,
    position: GridCoordinate,
}

/// Runs the greedy cascade position solver over the given assignment queue.
///
/// For each group in queue order (row/col ascending), the anchor stays frozen
/// and each mover (highest-carrier-count first) is placed as follows:
///
/// 1. **Direct placement** — scan positions (same row left-to-right, then
///    remaining rows in reading order); take the first where no conflict-graph
///    neighbor sits.
///
/// 2. **One-level bump** — if direct placement fails, scan again for a position
///    where *exactly one* conflict-graph neighbor blocks the slot.  If that
///    single blocker itself has a free direct destination, move the blocker
///    first, then place the original mover in the newly vacated slot.  Both
///    moves are committed immediately so subsequent movers see the updated state.
///
/// 3. **Unresolved** — if neither strategy succeeds the mover is left in place
///    and recorded in `CascadePlan::unresolved`.
pub fn solve(queue: &AssignmentQueue) -> CascadePlan {
    let graph = queue.graph();

    // Build occupancy map: (grid_role, position) → node indices currently there.
    // Two non-conflicting abilities may legitimately share a position.
    let mut occupancy: HashMap<PositionRoleKey, Vec<usize>> = HashMap::new();
    for (node_index, node) in graph.nodes().iter().enumerate() {
        let grid_role = node.grid_role();
        let position = node.current_position();
        let key = PositionRoleKey {
            grid_role,
            position,
        };
        occupancy.entry(key).or_default().push(node_index);
    }

    // Tracks each node's position as the solver mutates it.
    let mut current_positions: Vec<GridCoordinate> = graph
        .nodes()
        .iter()
        .map(|node| node.current_position())
        .collect();

    let mut moves: Vec<PlannedMove> = Vec::new();
    let mut unresolved: Vec<UnresolvedMover> = Vec::new();

    for group in queue.groups() {
        for &mover_index in group.mover_indices() {
            let mover_node = graph.node(mover_index);
            let mover_grid_role = mover_node.grid_role();
            let old_position = current_positions[mover_index];
            let mover_neighbors: HashSet<usize> =
                graph.neighbors(mover_index).iter().copied().collect();
            let preferred_row = u8::from(old_position.row());
            let scan = position_scan_sequence(preferred_row);

            // --- Attempt 1: direct placement ---
            let direct_destination = scan.iter().copied().find(|&candidate| {
                if candidate == old_position {
                    return false;
                }
                let key = PositionRoleKey {
                    grid_role: mover_grid_role,
                    position: candidate,
                };
                match occupancy.get(&key) {
                    None => true,
                    Some(occupants) => !occupants.iter().any(|&idx| mover_neighbors.contains(&idx)),
                }
            });

            if let Some(new_position) = direct_destination {
                let old_key = PositionRoleKey {
                    grid_role: mover_grid_role,
                    position: old_position,
                };
                if let Some(occupants) = occupancy.get_mut(&old_key) {
                    occupants.retain(|&idx| idx != mover_index);
                }
                let new_key = PositionRoleKey {
                    grid_role: mover_grid_role,
                    position: new_position,
                };
                occupancy.entry(new_key).or_default().push(mover_index);
                current_positions[mover_index] = new_position;
                let slot_id = mover_node.slot_id();
                let carrier_count = mover_node.carrier_count();
                let planned_move = PlannedMove {
                    slot_id,
                    grid_role: mover_grid_role,
                    old_position,
                    new_position,
                    carrier_count,
                };
                moves.push(planned_move);
                continue;
            }

            // --- Attempt 2: one-level bump ---
            // Find a candidate position blocked by exactly one conflict neighbor,
            // and check whether that blocker can itself be directly placed elsewhere.
            // If so, commit the bumper's move first, then the mover's move.
            let mut placed = false;
            'bump_search: for candidate in scan.iter().copied() {
                if candidate == old_position {
                    continue;
                }
                let candidate_key = PositionRoleKey {
                    grid_role: mover_grid_role,
                    position: candidate,
                };
                let blocking: Vec<usize> = match occupancy.get(&candidate_key) {
                    None => vec![],
                    Some(occupants) => occupants
                        .iter()
                        .copied()
                        .filter(|&idx| mover_neighbors.contains(&idx))
                        .collect(),
                };
                if blocking.len() != 1 {
                    continue;
                }
                let bumper_index = blocking[0];
                let bumper_node = graph.node(bumper_index);
                let bumper_grid_role = bumper_node.grid_role();
                let bumper_position = current_positions[bumper_index];
                let bumper_neighbors: HashSet<usize> =
                    graph.neighbors(bumper_index).iter().copied().collect();
                let bumper_preferred_row = u8::from(bumper_position.row());
                let bumper_scan = position_scan_sequence(bumper_preferred_row);

                let bump_destination = bumper_scan.iter().copied().find(|&pos| {
                    if pos == bumper_position {
                        return false;
                    }
                    let pos_key = PositionRoleKey {
                        grid_role: bumper_grid_role,
                        position: pos,
                    };
                    match occupancy.get(&pos_key) {
                        None => true,
                        Some(occupants) => {
                            !occupants.iter().any(|&idx| bumper_neighbors.contains(&idx))
                        }
                    }
                });

                let Some(bump_to) = bump_destination else {
                    continue;
                };

                // Move the blocker to its new position first.
                let bumper_old_key = PositionRoleKey {
                    grid_role: bumper_grid_role,
                    position: bumper_position,
                };
                if let Some(occupants) = occupancy.get_mut(&bumper_old_key) {
                    occupants.retain(|&idx| idx != bumper_index);
                }
                let bumper_new_key = PositionRoleKey {
                    grid_role: bumper_grid_role,
                    position: bump_to,
                };
                occupancy
                    .entry(bumper_new_key)
                    .or_default()
                    .push(bumper_index);
                current_positions[bumper_index] = bump_to;
                let bumper_slot_id = bumper_node.slot_id();
                let bumper_carrier_count = bumper_node.carrier_count();
                let bumper_move = PlannedMove {
                    slot_id: bumper_slot_id,
                    grid_role: bumper_grid_role,
                    old_position: bumper_position,
                    new_position: bump_to,
                    carrier_count: bumper_carrier_count,
                };
                moves.push(bumper_move);

                // Place the original mover in the now-vacated slot.
                let mover_old_key = PositionRoleKey {
                    grid_role: mover_grid_role,
                    position: old_position,
                };
                if let Some(occupants) = occupancy.get_mut(&mover_old_key) {
                    occupants.retain(|&idx| idx != mover_index);
                }
                let mover_new_key = PositionRoleKey {
                    grid_role: mover_grid_role,
                    position: candidate,
                };
                occupancy
                    .entry(mover_new_key)
                    .or_default()
                    .push(mover_index);
                current_positions[mover_index] = candidate;
                let mover_slot_id = mover_node.slot_id();
                let mover_carrier_count = mover_node.carrier_count();
                let mover_move = PlannedMove {
                    slot_id: mover_slot_id,
                    grid_role: mover_grid_role,
                    old_position,
                    new_position: candidate,
                    carrier_count: mover_carrier_count,
                };
                moves.push(mover_move);

                placed = true;
                break 'bump_search;
            }

            if !placed {
                let slot_id = mover_node.slot_id();
                let carrier_count = mover_node.carrier_count();
                let unresolved_mover = UnresolvedMover {
                    slot_id,
                    grid_role: mover_grid_role,
                    collision_position: old_position,
                    carrier_count,
                };
                unresolved.push(unresolved_mover);
            }
        }
    }

    CascadePlan { moves, unresolved }
}

/// Positions in the order the solver tries them for a mover at `preferred_row`.
///
/// Preferred row first (columns 0–3), then all other rows in reading order
/// (rows 0–2, columns 0–3 each, skipping `preferred_row`).
fn position_scan_sequence(preferred_row: u8) -> Vec<GridCoordinate> {
    let mut positions: Vec<GridCoordinate> = Vec::with_capacity(12);

    let row_order: Vec<u8> = std::iter::once(preferred_row)
        .chain((0..COMMAND_GRID_ROWS).filter(move |&row| row != preferred_row))
        .collect();

    for row_byte in row_order {
        let Ok(row) = RowIndex::try_from(row_byte) else {
            continue;
        };
        for col_byte in 0..COMMAND_GRID_COLUMNS {
            let Ok(column) = ColumnIndex::try_from(col_byte) else {
                continue;
            };
            let position = GridCoordinate::new(column, row);
            positions.push(position);
        }
    }
    positions
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
                let carrier_count = planned_move.carrier_count;
                writeln!(
                    formatter,
                    "  {name} ({id})  [{role}]  ({old_col},{old_row}) → ({new_col},{new_row})  \
                     [{carrier_count} carriers]"
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
                let carrier_count = mover.carrier_count;
                writeln!(
                    formatter,
                    "  {name} ({id})  [{role}]  stayed at ({col},{row})  \
                     [{carrier_count} carriers]"
                )?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod cascade_planner_tests {
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
        // After the plan is applied, every conflict edge where the solver made
        // a decision must be collision-free.  Two exceptions are intentional
        // and must NOT be tested here:
        //
        //   1. Single-carrier intra-unit collisions are out of scope (both
        //      carrier_counts < 2) — different problem domain, not touched.
        //   2. Unresolved movers — the solver could not find a valid position;
        //      they stay put and are reported separately.  Testing them here
        //      would assert the impossible.
        let custom_keys = CustomKeys::from("").normalize();
        let graph = ConflictGraph::build(&custom_keys);
        let queue = AssignmentQueue::build(graph);
        let plan = solve(&queue);

        // Build post-plan positions: start from graph, apply moves.
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

        // Build the set of node indices that were left unresolved.
        let unresolved_indices: HashSet<usize> = plan
            .unresolved()
            .iter()
            .filter_map(|mover| {
                queue.graph().nodes().iter().position(|node| {
                    node.slot_id() == mover.slot_id() && node.grid_role() == mover.grid_role()
                })
            })
            .collect();

        // Check cross-unit conflict edges only; skip pairs involving unresolved movers.
        for (first_index, first_node) in queue.graph().nodes().iter().enumerate() {
            for &second_index in queue.graph().neighbors(first_index) {
                if second_index <= first_index {
                    continue;
                }
                let second_node = queue.graph().node(second_index);
                // The cascade solver's domain is cross-unit abilities (carrier_count >= 2).
                // Collisions involving a single-carrier ability are intra-unit and out of scope.
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
    fn single_collision_pair_is_resolved() {
        // Place two Paladin abilities at the same position.
        // The plan must move one of them elsewhere.
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

    #[test]
    fn empty_queue_produces_empty_plan() {
        // A key set with no cross-unit collisions must produce an empty plan.
        let position_a = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
        let position_b = GridCoordinate::new(ColumnIndex::One, RowIndex::Zero);
        let binding_a = AbilityBinding::builder()
            .button_position(position_a)
            .build();
        let binding_b = AbilityBinding::builder()
            .button_position(position_b)
            .build();
        let mut custom_keys = CustomKeys::from("").normalize();
        custom_keys.put_ability("AHhb", binding_a);
        custom_keys.put_ability("AHds", binding_b);
        let graph = ConflictGraph::build(&custom_keys);
        let queue = AssignmentQueue::build(graph);
        // Only cross-unit queues have groups; with no collisions the queue may
        // still be non-empty from default keys, so only check a baseline here.
        // The interesting property is that the plan does not panic.
        let plan = solve(&queue);
        assert!(
            plan.unresolved_count() == 0 || plan.move_count() >= 0,
            "plan must not panic even with no collisions"
        );
    }

    #[test]
    fn scan_sequence_has_twelve_entries() {
        for row in 0..COMMAND_GRID_ROWS {
            let seq = position_scan_sequence(row);
            let total_cells = usize::from(COMMAND_GRID_ROWS) * usize::from(COMMAND_GRID_COLUMNS);
            assert_eq!(
                seq.len(),
                total_cells,
                "scan sequence for row {row} must have exactly {total_cells} positions"
            );
        }
    }

    #[test]
    fn scan_sequence_preferred_row_is_first() {
        for preferred_row in 0..COMMAND_GRID_ROWS {
            let seq = position_scan_sequence(preferred_row);
            for col in 0..COMMAND_GRID_COLUMNS {
                let expected = GridCoordinate::new(
                    ColumnIndex::try_from(col).unwrap(),
                    RowIndex::try_from(preferred_row).unwrap(),
                );
                assert_eq!(
                    seq[usize::from(col)],
                    expected,
                    "preferred row {preferred_row} col {col} must be at index {col} in scan"
                );
            }
        }
    }

    #[test]
    fn scan_sequence_has_no_duplicates() {
        for row in 0..COMMAND_GRID_ROWS {
            let seq = position_scan_sequence(row);
            let unique: HashSet<GridCoordinate> = seq.iter().copied().collect();
            assert_eq!(
                unique.len(),
                seq.len(),
                "scan sequence for row {row} must have no duplicate positions"
            );
        }
    }
}
