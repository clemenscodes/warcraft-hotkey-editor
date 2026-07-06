use super::{AssignmentScope, GroupKind, PositionAssignmentGroup};
use crate::cascade::conflict_graph::ConflictGraph;
use crate::model::GridCoordinate;
use crate::unit::grids::GridRole;
use std::collections::{HashMap, HashSet};

impl PositionAssignmentGroup {
    /// Decomposes the conflict subgraph among current residents at one cell
    /// into one or more anchor+movers groups.  The cell's position and grid
    /// role are carried into each emitted group.
    pub(super) fn fight_groups_at_cell(
        residents: &[usize],
        position: GridCoordinate,
        grid_role: GridRole,
        graph: &ConflictGraph,
        scope: AssignmentScope,
    ) -> Vec<Self> {
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
        let mut groups: Vec<Self> = Vec::new();
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
    ) -> Vec<Self> {
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
        let first_group = Self {
            position,
            grid_role,
            anchor_index,
            mover_indices: direct_mover_indices,
            kind: GroupKind::Fight,
        };
        let mut groups: Vec<Self> = vec![first_group];
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
