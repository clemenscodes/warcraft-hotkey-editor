use super::{AssignmentQueue, AssignmentScope, GroupKind, PositionAssignmentGroup};
use crate::cascade::conflict_graph::ConflictGraph;
use crate::grid::layout::{COMMAND_GRID_COLUMNS, COMMAND_GRID_ROWS};
use crate::model::{ColumnIndex, GridCoordinate, RowIndex};
use crate::unit::grids::GridRole;
use std::collections::{HashMap, HashSet};
use warcraft_api::WarcraftObjectId;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct UnitRoleKey {
    unit_id: WarcraftObjectId,
    grid_role: GridRole,
}

/// Mutable state threaded through the raster sweep.
#[derive(Clone, PartialEq, Eq, Default)]
pub(super) struct QueueBuildState {
    live_positions: Vec<GridCoordinate>,
    unresolved: HashSet<usize>,
    groups: Vec<PositionAssignmentGroup>,
    unit_carries: HashMap<UnitRoleKey, Vec<usize>>,
}

impl QueueBuildState {
    /// Runs the full two-phase cascade — the phase-1 raster sweep followed by
    /// the phase-2 spill — over `graph` and materializes the finished
    /// [`AssignmentQueue`].  This is the sole entry point the public
    /// `AssignmentQueue::build*` surface delegates to; keeping it here lets the
    /// state struct stay entirely private to this module.
    pub(super) fn build(graph: ConflictGraph, scope: AssignmentScope) -> AssignmentQueue {
        let mut state = Self::new(&graph);
        let role_sweep_order = [
            GridRole::MainCommand,
            GridRole::BuildMenu,
            GridRole::UprootedForm,
            GridRole::HeroSkillTree,
        ];
        for row_byte in 0..COMMAND_GRID_ROWS {
            let Ok(row) = RowIndex::try_from(row_byte) else {
                continue;
            };
            for column_byte in 0..COMMAND_GRID_COLUMNS {
                let Ok(column) = ColumnIndex::try_from(column_byte) else {
                    continue;
                };
                let position = GridCoordinate::new(column, row);
                for &role in &role_sweep_order {
                    state.process_cell(position, role, &graph, scope);
                }
            }
        }
        state.spill_unresolved(&graph);
        let total_mover_count = state.groups.iter().map(|group| group.mover_count()).sum();
        let mut unresolved_sorted: Vec<usize> = state.unresolved.into_iter().collect();
        unresolved_sorted.sort();
        AssignmentQueue {
            graph,
            groups: state.groups,
            final_positions: state.live_positions,
            unresolved: unresolved_sorted,
            total_mover_count,
        }
    }

    fn new(graph: &ConflictGraph) -> Self {
        let live_positions: Vec<GridCoordinate> = graph
            .nodes()
            .iter()
            .map(|node| node.current_position())
            .collect();
        let mut unit_carries: HashMap<UnitRoleKey, Vec<usize>> = HashMap::new();
        for (index, node) in graph.nodes().iter().enumerate() {
            let grid_role = node.grid_role();
            for carrier_id in node.carrier_unit_ids() {
                let key = UnitRoleKey {
                    unit_id: *carrier_id,
                    grid_role,
                };
                unit_carries.entry(key).or_default().push(index);
            }
        }
        Self {
            live_positions,
            unresolved: HashSet::new(),
            groups: Vec::new(),
            unit_carries,
        }
    }

    fn process_cell(
        &mut self,
        position: GridCoordinate,
        grid_role: GridRole,
        graph: &ConflictGraph,
        scope: AssignmentScope,
    ) {
        let residents = self.residents_at(position, grid_role, graph);
        let fight_groups = PositionAssignmentGroup::fight_groups_at_cell(
            &residents, position, grid_role, graph, scope,
        );
        for fight_group in fight_groups {
            let mover_indices_for_relocation: Vec<usize> = fight_group.mover_indices().to_vec();
            self.groups.push(fight_group);
            for mover_index in mover_indices_for_relocation {
                self.relocate_mover_rightward(mover_index, position);
            }
        }
        if let Some(candidate_index) = self.find_gap_pull_candidate(position, grid_role, graph) {
            let source_position = self.live_positions[candidate_index];
            self.live_positions[candidate_index] = position;
            let kind = GroupKind::GapPull { source_position };
            let gap_pull_group = PositionAssignmentGroup {
                position,
                grid_role,
                anchor_index: candidate_index,
                mover_indices: Vec::new(),
                kind,
            };
            self.groups.push(gap_pull_group);
        }
    }

    fn residents_at(
        &self,
        position: GridCoordinate,
        grid_role: GridRole,
        graph: &ConflictGraph,
    ) -> Vec<usize> {
        let mut residents: Vec<usize> = Vec::new();
        for (index, node) in graph.nodes().iter().enumerate() {
            if node.grid_role() != grid_role {
                continue;
            }
            if self.unresolved.contains(&index) {
                continue;
            }
            if self.live_positions[index] == position {
                residents.push(index);
            }
        }
        residents
    }

    fn relocate_mover_rightward(&mut self, mover_index: usize, from_position: GridCoordinate) {
        let column = u8::from(from_position.column());
        let row = u8::from(from_position.row());
        let next_column = column + 1;
        if next_column < COMMAND_GRID_COLUMNS
            && let Ok(next_column_index) = ColumnIndex::try_from(next_column)
        {
            let new_position = GridCoordinate::new(next_column_index, from_position.row());
            self.live_positions[mover_index] = new_position;
            return;
        }
        let next_row = row + 1;
        if next_row >= COMMAND_GRID_ROWS {
            self.unresolved.insert(mover_index);
            return;
        }
        let Ok(next_row_index) = RowIndex::try_from(next_row) else {
            self.unresolved.insert(mover_index);
            return;
        };
        let wrapped_position = GridCoordinate::new(ColumnIndex::Zero, next_row_index);
        self.live_positions[mover_index] = wrapped_position;
    }

    fn find_gap_pull_candidate(
        &self,
        position: GridCoordinate,
        grid_role: GridRole,
        graph: &ConflictGraph,
    ) -> Option<usize> {
        let column = u8::from(position.column());
        let row = u8::from(position.row());
        let mut units_needing_gap_close: HashSet<WarcraftObjectId> = HashSet::new();
        for (key, node_indices) in &self.unit_carries {
            if key.grid_role != grid_role {
                continue;
            }
            let mut anyone_at_cell = false;
            let mut anyone_displaced_past_here = false;
            for &node_index in node_indices {
                if self.unresolved.contains(&node_index) {
                    continue;
                }
                let node_position = self.live_positions[node_index];
                let node_row = u8::from(node_position.row());
                if node_row != row {
                    continue;
                }
                let node_column = u8::from(node_position.column());
                if node_column == column {
                    anyone_at_cell = true;
                    break;
                }
                if node_column > column {
                    let original_column =
                        u8::from(graph.node(node_index).current_position().column());
                    if original_column <= column {
                        anyone_displaced_past_here = true;
                    }
                }
            }
            if !anyone_at_cell && anyone_displaced_past_here {
                units_needing_gap_close.insert(key.unit_id);
            }
        }
        if units_needing_gap_close.is_empty() {
            return None;
        }
        let residents = self.residents_at(position, grid_role, graph);
        let residents_set: HashSet<usize> = residents.iter().copied().collect();
        let mut candidates: Vec<GapPullCandidate> = Vec::new();
        for (index, node) in graph.nodes().iter().enumerate() {
            if node.grid_role() != grid_role {
                continue;
            }
            if self.unresolved.contains(&index) {
                continue;
            }
            if node.slot_id().is_pinned() {
                continue;
            }
            let node_position = self.live_positions[index];
            let node_row = u8::from(node_position.row());
            if node_row != row {
                continue;
            }
            let node_column = u8::from(node_position.column());
            if node_column <= column {
                continue;
            }
            let helps_at_least_one_gap = node
                .carrier_unit_ids()
                .iter()
                .any(|carrier_id| units_needing_gap_close.contains(carrier_id));
            if !helps_at_least_one_gap {
                continue;
            }
            let creates_collision = graph
                .neighbors(index)
                .iter()
                .any(|neighbor_index| residents_set.contains(neighbor_index));
            if creates_collision {
                continue;
            }
            let carrier_count = node.carrier_count();
            let candidate = GapPullCandidate {
                node_index: index,
                source_column: node_column,
                carrier_count,
            };
            candidates.push(candidate);
        }
        if candidates.is_empty() {
            return None;
        }
        candidates.sort_by(|left, right| {
            left.source_column
                .cmp(&right.source_column)
                .then_with(|| right.carrier_count.cmp(&left.carrier_count))
                .then_with(|| left.node_index.cmp(&right.node_index))
        });
        Some(candidates[0].node_index)
    }

    /// Final fallback for nodes that couldn't be placed by the raster sweep.
    ///
    /// For each still-unresolved node (processed in carrier-count-descending
    /// priority order), try to find a better home using a *best-fit* search:
    ///
    /// 1. **Same-row, with swap allowed**.  For every other column in the
    ///    node's row, count how many of the node's carriers already have an
    ///    ability at that cell (its "occupations").  Pick the lowest-
    ///    occupation cell where every incumbent is safely swappable —
    ///    not pinned, not unresolved, and won't itself collide if relocated
    ///    to the node's current stuck cell.  Swap the incumbent(s) into the
    ///    node's old slot and place the node at the new cell.
    /// 2. **Cross-row, with swap allowed**.  If same-row fails, repeat the
    ///    same search on other rows in ascending row-distance order.
    /// 3. **Leave unresolved**.  If neither phase finds a swappable cell,
    ///    the node stays where it is.  Cross-row movement is bad — but a
    ///    persistent collision is worse, so we only stay unresolved when no
    ///    cross-row option exists either.
    fn spill_unresolved(&mut self, graph: &ConflictGraph) {
        let mut spill_order: Vec<usize> = self.unresolved.iter().copied().collect();
        spill_order.sort_by(|&left, &right| {
            let left_carriers = graph.node(left).carrier_count();
            let right_carriers = graph.node(right).carrier_count();
            right_carriers
                .cmp(&left_carriers)
                .then_with(|| left.cmp(&right))
        });
        for node_index in spill_order {
            let decision = self.find_spill_decision(node_index, graph);
            if let Some(spill_decision) = decision {
                self.apply_spill_decision(node_index, spill_decision, graph);
            }
        }
    }

    fn find_spill_decision(
        &self,
        node_index: usize,
        graph: &ConflictGraph,
    ) -> Option<SpillDecision> {
        let node = graph.node(node_index);
        let role = node.grid_role();
        let stuck_position = self.live_positions[node_index];
        let stuck_column = u8::from(stuck_position.column());
        let stuck_row = u8::from(stuck_position.row());
        let mut row_order: Vec<u8> = (0..COMMAND_GRID_ROWS).collect();
        let stuck_row_signed = i32::from(stuck_row);
        row_order.sort_by(|&left_row, &right_row| {
            let left_row_signed = i32::from(left_row);
            let right_row_signed = i32::from(right_row);
            let left_distance = (left_row_signed - stuck_row_signed).unsigned_abs();
            let right_distance = (right_row_signed - stuck_row_signed).unsigned_abs();
            left_distance
                .cmp(&right_distance)
                .then_with(|| left_row.cmp(&right_row))
        });
        for candidate_row_byte in row_order {
            if let Some(decision) =
                self.best_fit_in_row(node_index, role, candidate_row_byte, stuck_column, graph)
            {
                return Some(decision);
            }
        }
        None
    }

    fn best_fit_in_row(
        &self,
        node_index: usize,
        role: GridRole,
        candidate_row_byte: u8,
        origin_column: u8,
        graph: &ConflictGraph,
    ) -> Option<SpillDecision> {
        let Ok(candidate_row) = RowIndex::try_from(candidate_row_byte) else {
            return None;
        };
        let stuck_position = self.live_positions[node_index];
        let stuck_row_byte = u8::from(stuck_position.row());
        let stuck_column_byte = u8::from(stuck_position.column());
        let mut best: Option<SpillDecision> = None;
        let mut best_occupation_count: usize = usize::MAX;
        let mut best_column_distance: u32 = u32::MAX;
        for column_byte in 0..COMMAND_GRID_COLUMNS {
            let Ok(column) = ColumnIndex::try_from(column_byte) else {
                continue;
            };
            let candidate = GridCoordinate::new(column, candidate_row);
            if candidate_row_byte == stuck_row_byte && column_byte == stuck_column_byte {
                continue;
            }
            let mut incumbents: Vec<usize> = Vec::new();
            for &neighbor_index in graph.neighbors(node_index) {
                if self.live_positions[neighbor_index] != candidate {
                    continue;
                }
                if graph.node(neighbor_index).grid_role() != role {
                    continue;
                }
                incumbents.push(neighbor_index);
            }
            let occupation_count = incumbents.len();
            if occupation_count > best_occupation_count {
                continue;
            }
            let all_swappable = incumbents
                .iter()
                .all(|&inc| self.is_swap_safe(inc, stuck_position, node_index, graph));
            if !all_swappable {
                continue;
            }
            let column_byte_signed = i32::from(column_byte);
            let origin_column_signed = i32::from(origin_column);
            let column_distance_signed = column_byte_signed - origin_column_signed;
            let column_distance = column_distance_signed.unsigned_abs();
            let beats_best = occupation_count < best_occupation_count
                || (occupation_count == best_occupation_count
                    && column_distance < best_column_distance);
            if !beats_best {
                continue;
            }
            best_occupation_count = occupation_count;
            best_column_distance = column_distance;
            let new_best = SpillDecision {
                destination: candidate,
                incumbents,
            };
            best = Some(new_best);
            if best_occupation_count == 0 && best_column_distance == 0 {
                break;
            }
        }
        best
    }

    fn is_swap_safe(
        &self,
        incumbent_index: usize,
        destination: GridCoordinate,
        spilling_node_index: usize,
        graph: &ConflictGraph,
    ) -> bool {
        let incumbent = graph.node(incumbent_index);
        if incumbent.slot_id().is_pinned() {
            return false;
        }
        if self.unresolved.contains(&incumbent_index) {
            return false;
        }
        let role = incumbent.grid_role();
        for &neighbor_index in graph.neighbors(incumbent_index) {
            if neighbor_index == spilling_node_index {
                continue;
            }
            if self.live_positions[neighbor_index] == destination
                && graph.node(neighbor_index).grid_role() == role
            {
                return false;
            }
        }
        true
    }

    fn apply_spill_decision(
        &mut self,
        node_index: usize,
        decision: SpillDecision,
        graph: &ConflictGraph,
    ) {
        let stuck_position = self.live_positions[node_index];
        let role = graph.node(node_index).grid_role();
        self.live_positions[node_index] = decision.destination;
        self.unresolved.remove(&node_index);
        for &incumbent_index in &decision.incumbents {
            self.live_positions[incumbent_index] = stuck_position;
        }
        let kind = GroupKind::Spill { stuck_position };
        let spill_group = PositionAssignmentGroup {
            position: decision.destination,
            grid_role: role,
            anchor_index: node_index,
            mover_indices: decision.incumbents,
            kind,
        };
        self.groups.push(spill_group);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct SpillDecision {
    destination: GridCoordinate,
    incumbents: Vec<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
struct GapPullCandidate {
    node_index: usize,
    source_column: u8,
    carrier_count: usize,
}
