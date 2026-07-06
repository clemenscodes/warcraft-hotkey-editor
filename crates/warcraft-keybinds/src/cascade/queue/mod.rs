mod build_state;
mod fight;

use crate::cascade::conflict_graph::ConflictGraph;
use crate::model::GridCoordinate;
use crate::unit::grids::GridRole;
use build_state::QueueBuildState;
use std::fmt;

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
#[derive(Clone, Debug, PartialEq, Eq)]
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
/// **Phase 1 — Raster sweep over every grid cell** (`row` asc, `column` asc,
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
#[derive(Clone, Debug, PartialEq, Eq, Default)]
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

impl AssignmentQueue {
    pub fn build(graph: ConflictGraph) -> Self {
        Self::build_with_scope(graph, AssignmentScope::CrossUnitOnly)
    }

    pub fn build_with_scope(graph: ConflictGraph, scope: AssignmentScope) -> Self {
        QueueBuildState::build(graph, scope)
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

impl fmt::Display for AssignmentQueue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.groups.is_empty() && self.unresolved.is_empty() {
            return writeln!(
                formatter,
                "Assignment queue: empty — no collisions or gaps to resolve.",
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
                 {anchor_carrier_ids}]",
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
                     {mover_carrier_ids}]",
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
                    "  {name} ({id})  [{role}]  stuck at ({column},{row})",
                )?;
            }
        }
        Ok(())
    }
}

impl ddd::Layered for GroupKind {
    type Layer = ddd::DomainLayer;
}

impl ddd::ValueObject for GroupKind {}

impl ddd::Layered for PositionAssignmentGroup {
    type Layer = ddd::DomainLayer;
}

impl ddd::ValueObject for PositionAssignmentGroup {}

impl ddd::Layered for AssignmentScope {
    type Layer = ddd::DomainLayer;
}

impl ddd::ValueObject for AssignmentScope {}

impl ddd::Layered for AssignmentQueue {
    type Layer = ddd::DomainLayer;
}

impl ddd::DomainService for AssignmentQueue {}

#[cfg(test)]
mod ddd_marker_tests {
    use super::AssignmentQueue;
    use super::AssignmentScope;
    use super::GroupKind;
    use super::PositionAssignmentGroup;
    use crate::ddd_conformance::assert_domain_service;
    use crate::ddd_conformance::assert_value_object;

    #[test]
    fn cascade_queue_types_carry_their_ddd_roles() {
        assert_value_object::<GroupKind>();
        assert_value_object::<PositionAssignmentGroup>();
        assert_value_object::<AssignmentScope>();
        assert_domain_service::<AssignmentQueue>();
    }
}

#[cfg(test)]
mod tests;
