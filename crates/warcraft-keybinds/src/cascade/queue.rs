use std::collections::{HashMap, HashSet};
use std::fmt;

use warcraft_api::WarcraftObjectId;

use crate::cascade::conflict_graph::ConflictGraph;
use crate::grid::layout::{COMMAND_GRID_COLUMNS, COMMAND_GRID_ROWS};
use crate::model::{ColumnIndex, GridCoordinate, RowIndex};
use crate::unit::grids::GridRole;

/// What sort of event produced a `PositionAssignmentGroup`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GroupKind {
    /// Multiple residents at the cell mutually conflict.  The anchor wins;
    /// movers slide rightward one column in the same row (or unresolved if
    /// the row is exhausted).
    Fight,
    /// The cell was empty on a unit with a left-gap and a non-conflicting
    /// rightward neighbor was pulled in.  Anchor = the puller, no movers.
    /// `source_position` records where the puller came from before the pull.
    GapPull { source_position: GridCoordinate },
    /// An ability that could not stay at its phase-1 final cell (every
    /// candidate there was claimed by a higher-priority conflict) was rehomed
    /// to a different cell by phase 2.  Anchor = the rehomed ability;
    /// movers (if any) are the incumbents of the new cell that were swapped
    /// back to the anchor's stuck cell.  `stuck_position` records the
    /// anchor's pre-spill cell.  A cross-row spill is the last-resort
    /// fallback before unresolved — better than leaving the ability stacked
    /// on top of another at the same cell.
    Spill { stuck_position: GridCoordinate },
}

/// One anchor decision at a single grid cell.  See [`GroupKind`] for the
/// three flavors.  After the queue finishes,
/// `AssignmentQueue::final_position(group.anchor_index())` always equals
/// `group.position()`.
pub struct PositionAssignmentGroup {
    position: GridCoordinate,
    grid_role: GridRole,
    anchor_index: usize,
    mover_indices: Vec<usize>,
    kind: GroupKind,
}

impl PositionAssignmentGroup {
    pub fn position(&self) -> GridCoordinate {
        self.position
    }

    pub fn grid_role(&self) -> GridRole {
        self.grid_role
    }

    pub fn anchor_index(&self) -> usize {
        self.anchor_index
    }

    pub fn mover_indices(&self) -> &[usize] {
        &self.mover_indices
    }

    pub fn mover_count(&self) -> usize {
        self.mover_indices.len()
    }

    pub fn kind(&self) -> GroupKind {
        self.kind
    }

    pub fn is_fight(&self) -> bool {
        matches!(self.kind, GroupKind::Fight)
    }

    pub fn is_gap_pull(&self) -> bool {
        matches!(self.kind, GroupKind::GapPull { .. })
    }

    pub fn is_spill(&self) -> bool {
        matches!(self.kind, GroupKind::Spill { .. })
    }

    pub fn gap_pull_source_position(&self) -> Option<GridCoordinate> {
        match self.kind {
            GroupKind::GapPull { source_position } => Some(source_position),
            _ => None,
        }
    }

    pub fn spill_stuck_position(&self) -> Option<GridCoordinate> {
        match self.kind {
            GroupKind::Spill { stuck_position } => Some(stuck_position),
            _ => None,
        }
    }
}

/// The ordered plan for resolving the cascade.
///
/// `AssignmentQueue::build` runs in two phases:
///
/// **Phase 1 — Raster sweep over every grid cell** (`row` asc, `col` asc,
/// `grid_role` in display order).  At each cell:
///
///   1. **Conflict fights**: residents currently assigned to the cell are
///      decomposed into anchor + direct-mover groups.  Each loser slides one
///      column to the right (same row).  Losers already at column 3 with no
///      open slot are tentatively marked *unresolved*.
///   2. **Gap-pull**: if any unit has a left-gap at this cell (something of
///      theirs further right in this row but nothing here), the leftmost
///      rightward candidate that doesn't conflict with the cell's current
///      residents is pulled in.
///
/// Phase 1 strictly preserves same-row placement.  An ability never crosses
/// rows in phase 1, because cross-row movement changes its hotkey.
///
/// **Phase 2 — Best-fit spill for still-unresolved nodes**.  Once the raster
/// sweep finishes, each unresolved node tries to find a real home: same row
/// first (with swap allowed), then other rows in distance order.  For each
/// candidate cell, the node counts how many of its carriers already have an
/// ability there ("occupations") and picks the lowest-occupation cell whose
/// incumbents can be safely swapped into the node's stuck slot.  Cross-row
/// movement is bad, but a persistent collision is worse — phase 2 makes
/// exactly that trade.  A node that finds no swap candidate stays unresolved.
pub struct AssignmentQueue {
    graph: ConflictGraph,
    groups: Vec<PositionAssignmentGroup>,
    final_positions: Vec<GridCoordinate>,
    unresolved: Vec<usize>,
    total_mover_count: usize,
}

/// Which conflicts the cascade is allowed to resolve in this pass.
///
/// `resolve_conflicts` runs a `CrossUnitOnly` pass first (the classic cascade
/// that ignores intra-unit collisions) and a follow-up `IncludingIntraUnit`
/// pass to clean up the remaining same-unit collisions (e.g. two shop items
/// on a Goblin Merchant claiming the same slot).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AssignmentScope {
    /// Anchor candidates: nodes with carrier count ≥ 2, plus pinned slots.
    /// Pure intra-unit collisions are left untouched.
    CrossUnitOnly,
    /// Every node in the conflict component is an anchor candidate.  Used
    /// for the second pass after cross-unit cascading has settled.
    IncludingIntraUnit,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct UnitRoleKey {
    unit_id: WarcraftObjectId,
    grid_role: GridRole,
}

impl AssignmentQueue {
    pub fn build(graph: ConflictGraph) -> Self {
        Self::build_with_scope(graph, AssignmentScope::CrossUnitOnly)
    }

    pub fn build_with_scope(graph: ConflictGraph, scope: AssignmentScope) -> Self {
        let mut state = QueueBuildState::new(&graph);
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
            for col_byte in 0..COMMAND_GRID_COLUMNS {
                let Ok(column) = ColumnIndex::try_from(col_byte) else {
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
        Self {
            graph,
            groups: state.groups,
            final_positions: state.live_positions,
            unresolved: unresolved_sorted,
            total_mover_count,
        }
    }

    pub fn graph(&self) -> &ConflictGraph {
        &self.graph
    }

    pub fn groups(&self) -> &[PositionAssignmentGroup] {
        &self.groups
    }

    pub fn group_count(&self) -> usize {
        self.groups.len()
    }

    pub fn total_mover_count(&self) -> usize {
        self.total_mover_count
    }

    pub fn is_empty(&self) -> bool {
        self.groups.is_empty()
    }

    pub fn final_position(&self, node_index: usize) -> GridCoordinate {
        self.final_positions[node_index]
    }

    pub fn unresolved_nodes(&self) -> &[usize] {
        &self.unresolved
    }

    pub fn is_unresolved(&self, node_index: usize) -> bool {
        self.unresolved.binary_search(&node_index).is_ok()
    }
}

/// Mutable state threaded through the raster sweep.
struct QueueBuildState {
    live_positions: Vec<GridCoordinate>,
    unresolved: HashSet<usize>,
    groups: Vec<PositionAssignmentGroup>,
    unit_carries: HashMap<UnitRoleKey, Vec<usize>>,
}

impl QueueBuildState {
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
            let mover_indices_for_relocation: Vec<usize> = fight_group.mover_indices.clone();
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
        let column_value = u8::from(from_position.column());
        let row_value = u8::from(from_position.row());
        let next_column_value = column_value + 1;
        // Within the row: push one column to the right.
        if next_column_value < COMMAND_GRID_COLUMNS
            && let Ok(next_column) = ColumnIndex::try_from(next_column_value)
        {
            let new_position = GridCoordinate::new(next_column, from_position.row());
            self.live_positions[mover_index] = new_position;
            return;
        }
        // Row overflow: wrap to the leftmost column of the next row down.
        let next_row_value = row_value + 1;
        if next_row_value >= COMMAND_GRID_ROWS {
            self.unresolved.insert(mover_index);
            return;
        }
        let Ok(next_row) = RowIndex::try_from(next_row_value) else {
            self.unresolved.insert(mover_index);
            return;
        };
        let wrapped_position = GridCoordinate::new(ColumnIndex::Zero, next_row);
        self.live_positions[mover_index] = wrapped_position;
    }

    fn find_gap_pull_candidate(
        &self,
        position: GridCoordinate,
        grid_role: GridRole,
        graph: &ConflictGraph,
    ) -> Option<usize> {
        let column_value = u8::from(position.column());
        let row_value = u8::from(position.row());

        let mut units_needing_gap_close: HashSet<WarcraftObjectId> = HashSet::new();
        for (key, node_indices) in &self.unit_carries {
            if key.grid_role != grid_role {
                continue;
            }
            let mut anyone_at_cell = false;
            // True only when an ability was originally at ≤ current column but the
            // cascade displaced it past the current column — a real gap.  An ability
            // that was always to the right of this cell (intentional design) does not
            // qualify and must not trigger a gap-pull.
            let mut anyone_displaced_past_here = false;
            for &node_index in node_indices {
                if self.unresolved.contains(&node_index) {
                    continue;
                }
                let node_position = self.live_positions[node_index];
                let node_row_value = u8::from(node_position.row());
                if node_row_value != row_value {
                    continue;
                }
                let node_column_value = u8::from(node_position.column());
                if node_column_value == column_value {
                    anyone_at_cell = true;
                    break;
                }
                if node_column_value > column_value {
                    let original_column_value =
                        u8::from(graph.node(node_index).current_position().column());
                    if original_column_value <= column_value {
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
            let node_row_value = u8::from(node_position.row());
            if node_row_value != row_value {
                continue;
            }
            let node_column_value = u8::from(node_position.column());
            if node_column_value <= column_value {
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
                source_column: node_column_value,
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

        // Pinned slots (system commands, Ancient root/uproot) normally win
        // every fight and never reach this list. They only land here when the
        // user has manually dragged commands so that two pinned slots collide
        // and one overflows its row. When that happens the overflowed pinned
        // slot still needs a real home, so it gets the same full-grid best-fit
        // search as everything else. `is_swap_safe` keeps it from displacing
        // another pinned slot, so a pinned spill can only take a free cell or
        // swap with a movable ability — never shuffle another command.
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

        // Same row first, then other rows in distance order, with stable
        // tie-break by row number ascending.
        let mut row_order: Vec<u8> = (0..COMMAND_GRID_ROWS).collect();
        let stuck_row_value = i32::from(stuck_row);
        row_order.sort_by(|&left_row, &right_row| {
            let left_row_value = i32::from(left_row);
            let right_row_value = i32::from(right_row);
            let left_distance = (left_row_value - stuck_row_value).unsigned_abs();
            let right_distance = (right_row_value - stuck_row_value).unsigned_abs();
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

        for col_byte in 0..COMMAND_GRID_COLUMNS {
            let Ok(column) = ColumnIndex::try_from(col_byte) else {
                continue;
            };
            let candidate = GridCoordinate::new(column, candidate_row);
            if candidate_row_byte == stuck_row_byte && col_byte == stuck_column_byte {
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

            let col_byte_value = i32::from(col_byte);
            let origin_column_value = i32::from(origin_column);
            let column_distance_signed = col_byte_value - origin_column_value;
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

struct SpillDecision {
    destination: GridCoordinate,
    incumbents: Vec<usize>,
}

struct GapPullCandidate {
    node_index: usize,
    source_column: u8,
    carrier_count: usize,
}

impl PositionAssignmentGroup {
    /// Decomposes the conflict subgraph among current residents at one cell
    /// into one or more anchor+movers groups.  The cell's position and grid
    /// role are carried into each emitted group.
    fn fight_groups_at_cell(
        residents: &[usize],
        position: GridCoordinate,
        grid_role: GridRole,
        graph: &ConflictGraph,
        scope: AssignmentScope,
    ) -> Vec<PositionAssignmentGroup> {
        let resident_set: HashSet<usize> = residents.iter().copied().collect();
        let mut position_adjacency: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut any_edge_in_residents = false;
        for &index in residents {
            let mut in_set_neighbors: Vec<usize> = Vec::new();
            for &neighbor_index in graph.neighbors(index) {
                if resident_set.contains(&neighbor_index) {
                    in_set_neighbors.push(neighbor_index);
                    any_edge_in_residents = true;
                }
            }
            if !in_set_neighbors.is_empty() {
                position_adjacency.insert(index, in_set_neighbors);
            }
        }
        if !any_edge_in_residents {
            return Vec::new();
        }

        let mut groups: Vec<PositionAssignmentGroup> = Vec::new();
        let mut visited: HashSet<usize> = HashSet::new();
        for &start_node in residents {
            if visited.contains(&start_node) {
                continue;
            }
            if !position_adjacency.contains_key(&start_node) {
                visited.insert(start_node);
                continue;
            }
            let mut component: Vec<usize> = Vec::new();
            let mut pending: Vec<usize> = vec![start_node];
            while let Some(current) = pending.pop() {
                if !visited.insert(current) {
                    continue;
                }
                component.push(current);
                if let Some(neighbors) = position_adjacency.get(&current) {
                    pending.extend(neighbors.iter().copied());
                }
            }
            let component_groups = Self::groups_for_component(
                &component,
                &position_adjacency,
                graph,
                position,
                grid_role,
                scope,
            );
            groups.extend(component_groups);
        }
        groups
    }

    /// Recursively splits one connected conflict component at a cell into
    /// direct-conflict groups.
    ///
    /// Each group contains exactly one anchor (the highest-carrier cross-unit
    /// node) and its *direct* conflict neighbours as movers.  Nodes connected
    /// to the anchor only through a chain — but sharing no carrier unit with
    /// the anchor — are handled in a sub-group at the next recursion level.
    /// This keeps Wind Walk from being forced to move just because Abolish
    /// Magic is the anchor when they only meet through Dispel Magic.
    fn groups_for_component(
        component: &[usize],
        position_adjacency: &HashMap<usize, Vec<usize>>,
        graph: &ConflictGraph,
        position: GridCoordinate,
        grid_role: GridRole,
        scope: AssignmentScope,
    ) -> Vec<PositionAssignmentGroup> {
        // Anchor candidates are filtered by the scope passed to
        // `AssignmentQueue::build`:
        //
        //   - `CrossUnitOnly` (phase 1): cross-unit nodes (carriers ≥ 2) plus
        //     any pinned node.  This is the existing cascade — intra-unit
        //     collisions (single-carrier abilities competing for one slot on
        //     one unit) are out of scope and silently ignored.
        //   - `IncludingIntraUnit` (phase 2): every node in the component is
        //     a candidate.  Cross-unit abilities still beat intra-unit ones
        //     via the carrier-count comparator below, but pure single-carrier
        //     collisions (e.g. two shop items on a Goblin Merchant) finally
        //     get resolved.
        let anchor_candidates: Vec<usize> = match scope {
            AssignmentScope::CrossUnitOnly => component
                .iter()
                .copied()
                .filter(|&index| {
                    let node = graph.node(index);
                    let slot_id = node.slot_id();
                    node.carrier_count() >= 2 || slot_id.is_pinned()
                })
                .collect(),
            AssignmentScope::IncludingIntraUnit => component.to_vec(),
        };

        if anchor_candidates.len() < 2 {
            return Vec::new();
        }

        // Anchor preference: pinned first, then highest carrier count, then
        // stable tiebreak.  For intra-unit ties (carrier_count == 1) the
        // ability that appears earlier in its unit's abilList wins: it is the
        // game's intended priority order, and the later-list loser cascades
        // through same-row incumbents without displacing them, keeping the
        // total count of moved abilities low.  The cross-unit tiebreak
        // (carrier_count > 1) is unchanged: lower node index (alphabetically
        // earlier) wins, preserving the existing cascade order.
        let anchor_index = anchor_candidates
            .iter()
            .copied()
            .max_by(|&left, &right| {
                let left_slot = graph.node(left).slot_id();
                let right_slot = graph.node(right).slot_id();
                let left_pinned = left_slot.is_pinned();
                let right_pinned = right_slot.is_pinned();
                let left_carriers = graph.node(left).carrier_count();
                let right_carriers = graph.node(right).carrier_count();
                left_pinned
                    .cmp(&right_pinned)
                    .then_with(|| left_carriers.cmp(&right_carriers))
                    .then_with(|| {
                        if left_carriers == 1 {
                            let left_priority = graph.node(left).ability_list_priority();
                            let right_priority = graph.node(right).ability_list_priority();
                            right_priority
                                .cmp(&left_priority)
                                .then_with(|| left.cmp(&right))
                        } else {
                            right.cmp(&left)
                        }
                    })
            })
            .expect("anchor_candidates is non-empty");

        let empty_neighbors: Vec<usize> = Vec::new();
        let anchor_position_neighbors: &Vec<usize> = position_adjacency
            .get(&anchor_index)
            .unwrap_or(&empty_neighbors);
        let anchor_neighbor_set: HashSet<usize> =
            anchor_position_neighbors.iter().copied().collect();

        let mut direct_mover_indices: Vec<usize> = anchor_candidates
            .iter()
            .copied()
            .filter(|&index| index != anchor_index && anchor_neighbor_set.contains(&index))
            .collect();

        if direct_mover_indices.is_empty() {
            let without_anchor: Vec<usize> = component
                .iter()
                .copied()
                .filter(|&index| index != anchor_index)
                .collect();
            return Self::groups_for_component(
                &without_anchor,
                position_adjacency,
                graph,
                position,
                grid_role,
                scope,
            );
        }

        direct_mover_indices.sort_by(|&left, &right| {
            let left_carriers = graph.node(left).carrier_count();
            let right_carriers = graph.node(right).carrier_count();
            right_carriers
                .cmp(&left_carriers)
                .then_with(|| left.cmp(&right))
        });

        let excluded_from_remaining: HashSet<usize> = std::iter::once(anchor_index)
            .chain(direct_mover_indices.iter().copied())
            .collect();
        let first_group = PositionAssignmentGroup {
            position,
            grid_role,
            anchor_index,
            mover_indices: direct_mover_indices,
            kind: GroupKind::Fight,
        };
        let mut groups: Vec<PositionAssignmentGroup> = vec![first_group];

        let remaining_nodes: Vec<usize> = component
            .iter()
            .copied()
            .filter(|&index| !excluded_from_remaining.contains(&index))
            .collect();

        if remaining_nodes.is_empty() {
            return groups;
        }

        let remaining_node_set: HashSet<usize> = remaining_nodes.iter().copied().collect();
        let mut remaining_adjacency: HashMap<usize, Vec<usize>> = HashMap::new();
        for &node in &remaining_nodes {
            let restricted_neighbors: Vec<usize> = position_adjacency
                .get(&node)
                .map(|neighbors| {
                    neighbors
                        .iter()
                        .copied()
                        .filter(|&neighbor| remaining_node_set.contains(&neighbor))
                        .collect()
                })
                .unwrap_or_default();
            if !restricted_neighbors.is_empty() {
                remaining_adjacency.insert(node, restricted_neighbors);
            }
        }

        let mut visited: HashSet<usize> = HashSet::new();
        for &start_node in &remaining_nodes {
            if visited.contains(&start_node) {
                continue;
            }
            let mut sub_component: Vec<usize> = Vec::new();
            let mut pending: Vec<usize> = vec![start_node];
            while let Some(current) = pending.pop() {
                if !visited.insert(current) {
                    continue;
                }
                sub_component.push(current);
                if let Some(neighbors) = remaining_adjacency.get(&current) {
                    for &neighbor in neighbors {
                        if !visited.contains(&neighbor) {
                            pending.push(neighbor);
                        }
                    }
                }
            }
            let sub_groups = Self::groups_for_component(
                &sub_component,
                &remaining_adjacency,
                graph,
                position,
                grid_role,
                scope,
            );
            groups.extend(sub_groups);
        }

        groups
    }
}

impl fmt::Display for AssignmentQueue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.groups.is_empty() && self.unresolved.is_empty() {
            return writeln!(
                formatter,
                "Assignment queue: empty — no collisions or gaps to resolve."
            );
        }
        writeln!(
            formatter,
            "Assignment queue: {} group(s), {} mover(s) total, {} unresolved\n",
            self.groups.len(),
            self.total_mover_count,
            self.unresolved.len(),
        )?;
        for (ordinal, group) in self.groups.iter().enumerate() {
            let column = u8::from(group.position.column());
            let row = u8::from(group.position.row());
            let role = group.grid_role.label();
            let kind = match group.kind {
                GroupKind::Fight => "fight",
                GroupKind::GapPull { .. } => "gap-pull",
                GroupKind::Spill { .. } => "spill",
            };
            writeln!(
                formatter,
                "[{}] ({},{}) [{}]  {}  — {} mover(s)",
                ordinal + 1,
                column,
                row,
                role,
                kind,
                group.mover_count(),
            )?;
            let anchor_node = self.graph.node(group.anchor_index);
            let anchor_name = anchor_node.slot_id().display_name(None, None);
            let anchor_id = anchor_node.slot_id().as_str();
            let anchor_carriers = anchor_node.carrier_count();
            let anchor_carrier_ids = anchor_node
                .carrier_unit_ids()
                .iter()
                .map(|carrier_id| carrier_id.value())
                .collect::<Vec<_>>()
                .join(", ");
            writeln!(
                formatter,
                "    ANCHOR  {anchor_name} ({anchor_id})  [{anchor_carriers} carriers: \
                 {anchor_carrier_ids}]"
            )?;
            for &mover_index in &group.mover_indices {
                let mover_node = self.graph.node(mover_index);
                let mover_name = mover_node.slot_id().display_name(None, None);
                let mover_id = mover_node.slot_id().as_str();
                let mover_carriers = mover_node.carrier_count();
                let mover_carrier_ids = mover_node
                    .carrier_unit_ids()
                    .iter()
                    .map(|carrier_id| carrier_id.value())
                    .collect::<Vec<_>>()
                    .join(", ");
                writeln!(
                    formatter,
                    "    MOVE    {mover_name} ({mover_id})  [{mover_carriers} carriers: \
                     {mover_carrier_ids}]"
                )?;
            }
            writeln!(formatter)?;
        }
        if !self.unresolved.is_empty() {
            writeln!(formatter, "Unresolved:")?;
            for &node_index in &self.unresolved {
                let node = self.graph.node(node_index);
                let name = node.slot_id().display_name(None, None);
                let id = node.slot_id().as_str();
                let position = self.final_positions[node_index];
                let column = u8::from(position.column());
                let row = u8::from(position.row());
                let role = node.grid_role().label();
                writeln!(
                    formatter,
                    "  {name} ({id})  [{role}]  stuck at ({column},{row})"
                )?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod cascade_queue_tests {
    use super::*;
    use crate::cascade::conflict_graph::ConflictGraph;
    use crate::custom_keys::CustomKeys;
    use crate::identity::slot::GridSlotId;
    use crate::model::{AbilityBinding, ColumnIndex, CommandBinding, GridCoordinate, RowIndex};

    /// Replays the global-command half of the QWER/ASDF/YXCV drag-drop
    /// rearrange on top of the default keys: the four bottom-row cells fill up
    /// with pinned, high-carrier system commands.
    fn rearranged_default_keys() -> CustomKeys {
        let mut custom_keys = CustomKeys::from("").normalize();
        let mut put_command_at = |command_id: &'static str, column: ColumnIndex, row: RowIndex| {
            let position = GridCoordinate::new(column, row);
            let binding = CommandBinding::builder().button_position(position).build();
            custom_keys.put_command(command_id, binding);
        };
        put_command_at("CmdMove", ColumnIndex::Three, RowIndex::Two);
        put_command_at("CmdStop", ColumnIndex::Zero, RowIndex::Two);
        put_command_at("CmdHoldPos", ColumnIndex::One, RowIndex::Two);
        put_command_at("CmdPatrol", ColumnIndex::Two, RowIndex::Two);
        put_command_at("CmdAttack", ColumnIndex::Zero, RowIndex::One);
        custom_keys
    }

    fn default_queue() -> AssignmentQueue {
        let custom_keys = CustomKeys::from("").normalize();
        let graph = ConflictGraph::build(&custom_keys);
        AssignmentQueue::build(graph)
    }

    #[test]
    fn queue_is_nonempty_for_default_keys() {
        let queue = default_queue();
        assert!(
            !queue.is_empty(),
            "default keys have known collisions so the queue must be non-empty"
        );
    }

    #[test]
    fn every_fight_mover_has_a_conflict_edge_within_its_group() {
        let queue = default_queue();
        for group in queue.groups() {
            if group.is_gap_pull() {
                continue;
            }
            let anchor_index = group.anchor_index();
            let mut group_nodes: HashSet<usize> = group.mover_indices().iter().copied().collect();
            group_nodes.insert(anchor_index);
            for &mover_index in group.mover_indices() {
                let mover_neighbors: HashSet<usize> = queue
                    .graph()
                    .neighbors(mover_index)
                    .iter()
                    .copied()
                    .collect();
                let shares_edge_with_group = group_nodes
                    .iter()
                    .any(|&other| other != mover_index && mover_neighbors.contains(&other));
                assert!(
                    shares_edge_with_group,
                    "mover {} has no conflict edge with any other node in its group",
                    queue.graph().node(mover_index).slot_id().as_str(),
                );
            }
        }
    }

    #[test]
    fn raster_phase_groups_are_sorted_left_to_right_top_to_bottom() {
        // Only Fight and GapPull groups belong to phase 1's raster sweep.
        // Spill groups happen in phase 2 and land at arbitrary cells, so they
        // are excluded from this ordering invariant.
        let queue = default_queue();
        let positions: Vec<(u8, u8)> = queue
            .groups()
            .iter()
            .filter(|group| !group.is_spill())
            .map(|group| {
                let column_value = u8::from(group.position().column());
                let row_value = u8::from(group.position().row());
                (row_value, column_value)
            })
            .collect();
        let mut sorted = positions.clone();
        sorted.sort();
        assert_eq!(
            positions, sorted,
            "phase-1 groups must be sorted row-then-column ascending"
        );
    }

    #[test]
    fn fight_anchor_has_highest_carrier_count_in_group() {
        let queue = default_queue();
        for group in queue.groups() {
            if group.is_gap_pull() {
                continue;
            }
            let anchor_carriers = queue.graph().node(group.anchor_index()).carrier_count();
            for &mover_index in group.mover_indices() {
                let mover_carriers = queue.graph().node(mover_index).carrier_count();
                assert!(
                    anchor_carriers >= mover_carriers,
                    "anchor must have at least as many carriers as any mover in the group"
                );
            }
        }
    }

    #[test]
    fn fight_movers_are_sorted_most_carriers_first_within_group() {
        let queue = default_queue();
        for group in queue.groups() {
            if group.is_gap_pull() {
                continue;
            }
            let mover_counts: Vec<usize> = group
                .mover_indices()
                .iter()
                .map(|&index| queue.graph().node(index).carrier_count())
                .collect();
            let mut sorted = mover_counts.clone();
            sorted.sort_by(|left, right| right.cmp(left));
            assert_eq!(
                mover_counts, sorted,
                "movers within a fight group must be sorted by carrier count descending"
            );
        }
    }

    #[test]
    fn anchor_final_position_equals_group_position() {
        let queue = default_queue();
        for group in queue.groups() {
            let anchor_final = queue.final_position(group.anchor_index());
            assert_eq!(
                anchor_final,
                group.position(),
                "anchor of every group must end up at the group's position"
            );
        }
    }

    #[test]
    fn every_fight_group_has_at_least_two_members() {
        // The cascade resolves both cross-unit collisions (carrier_count ≥ 2)
        // and pure intra-unit collisions (single-carrier abilities competing
        // for one slot on one unit, e.g. shop items on a Goblin Merchant).
        // The only invariant left is that a fight group must have an anchor
        // plus at least one mover — otherwise there's no fight to resolve.
        let queue = default_queue();
        for group in queue.groups() {
            if !group.is_fight() {
                continue;
            }
            let total_members = group.mover_count() + 1;
            assert!(
                total_members >= 2,
                "a fight group must have an anchor plus ≥ 1 mover, got {total_members}",
            );
        }
    }

    #[test]
    fn three_way_collision_produces_at_least_two_movers() {
        let position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
        let binding = AbilityBinding::builder().button_position(position).build();
        let mut custom_keys = CustomKeys::from("").normalize();
        custom_keys.put_ability("AHhb", binding.clone());
        custom_keys.put_ability("AHds", binding.clone());
        custom_keys.put_ability("AHad", binding);
        let graph = ConflictGraph::build(&custom_keys);
        let queue = AssignmentQueue::build(graph);
        let combined_movers: usize = queue
            .groups()
            .iter()
            .filter(|group| {
                group.position() == position && group.grid_role() == GridRole::MainCommand
            })
            .map(|group| group.mover_count())
            .sum();
        assert!(
            combined_movers >= 2,
            "three Paladin abilities at the same position must produce at least 2 movers across \
             groups at (0,0) main command, got {combined_movers}"
        );
    }

    #[test]
    fn four_way_collision_produces_at_least_three_movers() {
        let position = GridCoordinate::new(ColumnIndex::One, RowIndex::One);
        let binding = AbilityBinding::builder().button_position(position).build();
        let mut custom_keys = CustomKeys::from("").normalize();
        custom_keys.put_ability("AHhb", binding.clone());
        custom_keys.put_ability("AHds", binding.clone());
        custom_keys.put_ability("AHad", binding.clone());
        custom_keys.put_ability("AHre", binding);
        let graph = ConflictGraph::build(&custom_keys);
        let queue = AssignmentQueue::build(graph);
        let combined_movers: usize = queue
            .groups()
            .iter()
            .filter(|group| {
                group.position() == position && group.grid_role() == GridRole::MainCommand
            })
            .map(|group| group.mover_count())
            .sum();
        assert!(
            combined_movers >= 3,
            "four Paladin abilities at the same position must produce at least 3 movers across \
             groups at (1,1) main command, got {combined_movers}"
        );
    }

    #[test]
    fn every_fight_mover_never_moves_to_an_earlier_row() {
        // Fight movers slide rightward within the same row, wrapping to the
        // leftmost column of the next row when they overflow the row's right
        // edge.  They never move *backward* (to a row earlier than the fight
        // group's row).  A mover whose final fight slot ran out becomes
        // unresolved and may then be rehomed by phase 2 (spill); those
        // spill-anchored nodes are exempt — their final row reflects the spill.
        let queue = default_queue();
        let spilled_anchors: HashSet<usize> = queue
            .groups()
            .iter()
            .filter(|group| group.is_spill())
            .map(|group| group.anchor_index())
            .collect();
        for group in queue.groups() {
            if !group.is_fight() {
                continue;
            }
            let group_row_value = u8::from(group.position().row());
            for &mover_index in group.mover_indices() {
                if spilled_anchors.contains(&mover_index) {
                    continue;
                }
                let final_position = queue.final_position(mover_index);
                let mover_row_value = u8::from(final_position.row());
                assert!(
                    mover_row_value >= group_row_value,
                    "mover {} ended on row {} but its fight group was on row {} — \
                     cascade may wrap forward to a later row, never backward",
                    queue.graph().node(mover_index).slot_id().as_str(),
                    mover_row_value,
                    group_row_value,
                );
            }
        }
    }

    #[test]
    fn no_post_queue_collisions_for_resolved_cross_unit_nodes() {
        // `default_queue()` uses `AssignmentScope::CrossUnitOnly`.  Intra-unit
        // collisions (both endpoints have carrier_count == 1) are not the
        // queue's domain in that scope — they are resolved in phase 2 of
        // `CustomKeys::resolve_conflicts`.  Here we only check cross-unit
        // pairs.
        let queue = default_queue();
        let graph = queue.graph();
        for (first_index, first_node) in graph.nodes().iter().enumerate() {
            if queue.is_unresolved(first_index) {
                continue;
            }
            if first_node.carrier_count() < 2 {
                continue;
            }
            for &second_index in graph.neighbors(first_index) {
                if second_index <= first_index {
                    continue;
                }
                if queue.is_unresolved(second_index) {
                    continue;
                }
                let second_node = graph.node(second_index);
                if second_node.carrier_count() < 2 {
                    continue;
                }
                let first_final = queue.final_position(first_index);
                let second_final = queue.final_position(second_index);
                let same_role = first_node.grid_role() == second_node.grid_role();
                assert!(
                    first_final != second_final || !same_role,
                    "post-queue collision between {} and {} at ({},{}) on [{}]",
                    first_node.slot_id().as_str(),
                    second_node.slot_id().as_str(),
                    u8::from(first_final.column()),
                    u8::from(first_final.row()),
                    first_node.grid_role().label(),
                );
            }
        }
    }

    #[test]
    fn cascade_chain_emits_a_fight_group_at_the_displacement_destination() {
        // Three Paladin abilities at (0,0) plus one already at (1,0).  After the
        // (0,0) fight, the two losers slide to (1,0) where another Paladin
        // ability sits — a second fight must happen there.
        let collision_position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
        let next_position = GridCoordinate::new(ColumnIndex::One, RowIndex::Zero);
        let binding_collision = AbilityBinding::builder()
            .button_position(collision_position)
            .build();
        let binding_next = AbilityBinding::builder()
            .button_position(next_position)
            .build();
        let mut custom_keys = CustomKeys::from("").normalize();
        custom_keys.put_ability("AHhb", binding_collision.clone());
        custom_keys.put_ability("AHds", binding_collision.clone());
        custom_keys.put_ability("AHad", binding_collision);
        custom_keys.put_ability("AHre", binding_next);
        let graph = ConflictGraph::build(&custom_keys);
        let queue = AssignmentQueue::build(graph);
        let fight_groups_at_next = queue
            .groups()
            .iter()
            .filter(|group| {
                group.position() == next_position
                    && group.grid_role() == GridRole::MainCommand
                    && !group.is_gap_pull()
            })
            .count();
        assert!(
            fight_groups_at_next >= 1,
            "cascade chain must emit at least one fight group at (1,0) when losers from (0,0) \
             land on an already-occupied Paladin slot, got {fight_groups_at_next}"
        );
    }

    #[test]
    fn paladin_collision_is_resolved_with_no_orphans() {
        // Four Paladin abilities placed at the same row-0 cell, on top of the
        // default keys (which already put pinned Cmds at every column of row
        // 0).  Phase 1 must produce an unresolved overflow because row 0 has
        // no free cell for Paladin abilities to land in.  Phase 2 spill then
        // rehomes them — possibly cross-row — so that no Paladin ability
        // remains unresolved at the end.  Cross-row movement is acceptable
        // when row 0 is fully occupied by pinned commands; an unresolved
        // collision would be worse.
        let collision_position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
        let collision_binding = AbilityBinding::builder()
            .button_position(collision_position)
            .build();
        let mut custom_keys = CustomKeys::from("").normalize();
        let paladin_abilities = ["AHhb", "AHds", "AHad", "AHre"];
        for ability_id in paladin_abilities {
            custom_keys.put_ability(ability_id, collision_binding.clone());
        }
        let graph = ConflictGraph::build(&custom_keys);
        let queue = AssignmentQueue::build(graph);
        let graph_ref = queue.graph();
        for ability_id in paladin_abilities {
            let node_index = graph_ref
                .find_node(ability_id, GridRole::MainCommand)
                .expect("Paladin ability must exist as a graph node");
            assert!(
                !queue.is_unresolved(node_index),
                "Paladin ability {ability_id} must end up placed (possibly cross-row) — \
                 leaving it unresolved is worse than a row change"
            );
        }
    }

    #[test]
    fn pinned_system_commands_never_move_from_default() {
        // Cmd* slots are pinned: they should always end up at their original
        // database position regardless of any cascade rearrangement.
        let queue = default_queue();
        let graph = queue.graph();
        let mut pinned_command_count = 0;
        for (index, node) in graph.nodes().iter().enumerate() {
            if !matches!(node.slot_id(), GridSlotId::Command(_)) {
                continue;
            }
            pinned_command_count += 1;
            let original = node.current_position();
            let final_position = queue.final_position(index);
            assert_eq!(
                original,
                final_position,
                "pinned command {} moved from ({},{}) to ({},{}) — system commands must stay put",
                node.slot_id().as_str(),
                u8::from(original.column()),
                u8::from(original.row()),
                u8::from(final_position.column()),
                u8::from(final_position.row()),
            );
            assert!(
                !queue.is_unresolved(index),
                "pinned command {} ended up unresolved — system commands must always anchor",
                node.slot_id().as_str(),
            );
        }
        assert!(
            pinned_command_count > 0,
            "default keys must contain at least one Cmd* slot for this test to be meaningful"
        );
    }

    #[test]
    fn pinned_ancient_root_never_moves_from_default() {
        // Aro1 / Aro2 are pinned: the Ancient root/uproot toggle is a structural
        // building command, not a spell, and players rely on its position.
        let queue = default_queue();
        let graph = queue.graph();
        let mut checked_any = false;
        for (index, node) in graph.nodes().iter().enumerate() {
            let slot_id = node.slot_id();
            let ability_str = match slot_id {
                GridSlotId::Ability(id) | GridSlotId::AbilityOff(id) => id.value(),
                GridSlotId::Command(_) => continue,
            };
            if !matches!(ability_str, "Aro1" | "Aro2") {
                continue;
            }
            checked_any = true;
            let original = node.current_position();
            let final_position = queue.final_position(index);
            assert_eq!(
                original,
                final_position,
                "pinned ability {} moved from ({},{}) to ({},{}) — root/uproot must stay put",
                ability_str,
                u8::from(original.column()),
                u8::from(original.row()),
                u8::from(final_position.column()),
                u8::from(final_position.row()),
            );
        }
        assert!(
            checked_any,
            "default keys must contain at least one Aro1/Aro2 node for this test to be meaningful"
        );
    }

    #[test]
    fn unresolved_node_keeps_its_original_row() {
        // For any node the queue cannot place, its final position must still be
        // on its original row.  We don't construct a forced-overflow scenario
        // here (Warcraft data caps abilities per unit), so we exercise the
        // invariant on whatever unresolved nodes the default keys may produce.
        let queue = default_queue();
        let graph = queue.graph();
        for &unresolved_index in queue.unresolved_nodes() {
            let original_row = u8::from(graph.node(unresolved_index).current_position().row());
            let final_row = u8::from(queue.final_position(unresolved_index).row());
            assert_eq!(
                original_row,
                final_row,
                "unresolved node {} ended on row {} but started on row {} — same-row sacred",
                graph.node(unresolved_index).slot_id().as_str(),
                final_row,
                original_row,
            );
        }
    }

    #[test]
    fn gap_pull_does_not_displace_abilities_with_intentional_gaps() {
        // Arav (Raven Form, Druid of the Talon) has default position (3,2).
        // EDOT has Afae at (0,2), Acyc at (1,2), and Arav at (3,2) — column 2 is
        // intentionally empty.  The gap-pull must not pull Arav leftward to (2,2)
        // because no ability was ever displaced from (2,2): the gap is by design.
        let queue = default_queue();
        let graph = queue.graph();
        let Some(arav_index) = graph.find_node("Arav", GridRole::MainCommand) else {
            return;
        };
        let original_column = u8::from(graph.node(arav_index).current_position().column());
        let final_position = queue.final_position(arav_index);
        let final_column = u8::from(final_position.column());
        assert!(
            final_column >= original_column,
            "Arav must not be gap-pulled leftward: started at column {original_column}, \
             ended at column {final_column}"
        );
    }

    #[test]
    fn resolving_rearranged_keys_leaves_no_position_collisions() {
        // After the QWER/ASDF/YXCV rearrange, several toggle abilities (Burrow,
        // Bear/Crow form, Submerge, Robo-Goblin, Web, Call to Arms) have an
        // on-state or off-state sitting on top of a moved command. The conflict
        // graph used to collapse an ability's on-state and off-state into one
        // node tracking a single position, so the cascade only ever resolved one
        // of the two and left the other colliding. Resolving must clear every
        // position collision the collision reports can see, on-state and
        // off-state alike.
        use crate::collision::cross_unit::CrossUnitCollisionReport;
        use crate::collision::unit_report::UnitCollisionReport;
        use crate::grid::layout::GridLayout;

        let mut custom_keys = rearranged_default_keys();
        custom_keys.resolve_conflicts();

        let cross = CrossUnitCollisionReport::compute(&custom_keys);
        assert!(
            cross.is_empty(),
            "cross-unit position collisions remain after resolve: {} group(s)",
            cross.position_groups().len(),
        );

        let layout = GridLayout::qwerty_grid();
        let intra = UnitCollisionReport::compute(&custom_keys, layout);
        let units_with_position_collisions: Vec<&str> = intra
            .entries()
            .iter()
            .filter(|entry| entry.position_cards().iter().any(|card| !card.is_empty()))
            .map(|entry| entry.unit_name())
            .collect();
        assert!(
            units_with_position_collisions.is_empty(),
            "intra-unit position collisions remain after resolve on: {units_with_position_collisions:?}",
        );
    }

    #[test]
    fn pinned_command_overflowing_a_full_row_is_rehomed_not_left_unresolved() {
        // After the QWER/ASDF/YXCV rearrange the bottom row is full of pinned
        // high-carrier commands. On a worker the pinned build command CmdBuild
        // (default 0,2) loses every fight down that row, overflows at 3,2, and
        // used to be left unresolved because phase 2 skipped pinned nodes. It
        // must instead keep searching the grid and land somewhere free.
        let custom_keys = rearranged_default_keys();
        let graph = ConflictGraph::build(&custom_keys);
        let queue = AssignmentQueue::build(graph);
        let graph_ref = queue.graph();
        let build_index = graph_ref
            .find_node("CmdBuild", GridRole::MainCommand)
            .expect("CmdBuild must exist as a node on the main command card");
        let final_position = queue.final_position(build_index);
        assert!(
            !queue.is_unresolved(build_index),
            "CmdBuild overflowed the full bottom row and was left unresolved at ({},{}) — \
             it must be rehomed to a free cell elsewhere in the grid",
            u8::from(final_position.column()),
            u8::from(final_position.row()),
        );
    }
}
