use std::collections::{HashMap, HashSet};
use std::fmt;

use crate::conflict_graph::ConflictGraph;
use crate::model::GridCoordinate;
use crate::unit_grids::GridRole;

/// Groups colliding nodes at the same position into one assignment task.
/// The anchor stays; every mover must find a new position.
pub struct PositionAssignmentGroup {
    position: GridCoordinate,
    grid_role: GridRole,
    /// The node with the highest carrier count — it stays put.
    anchor_index: usize,
    /// Nodes that must be relocated, sorted by carrier count ascending
    /// (lowest blast-radius first).
    mover_indices: Vec<usize>,
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
}

/// The ordered work queue for the cascade solver.
///
/// One group corresponds to one **connected component of the collision subgraph
/// at a position**: the set of abilities at a given (position, grid_role) that
/// are mutually reachable through conflict edges.  Two abilities at the same
/// position that share no carrier unit are in different components and belong
/// to independent groups — neither needs to move because of the other.
///
/// Groups are sorted left-to-right, top-to-bottom.  Within a group, movers are
/// ordered by carrier count descending (highest blast-radius first).
///
/// Owns the `ConflictGraph` it was derived from so that `Display` can render
/// full ability names and carrier counts without a separate argument.
pub struct AssignmentQueue {
    graph: ConflictGraph,
    groups: Vec<PositionAssignmentGroup>,
    total_mover_count: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct PositionKey {
    position: GridCoordinate,
    grid_role: GridRole,
}

/// One collision edge stored while building the position's subgraph.
struct PositionEdge {
    first_node_index: usize,
    second_node_index: usize,
}

impl AssignmentQueue {
    pub fn build(graph: ConflictGraph) -> Self {
        // Step 1: group all colliding pairs by their shared position.
        let mut edges_by_position: HashMap<PositionKey, Vec<PositionEdge>> = HashMap::new();
        for pair in graph.colliding_pairs() {
            let first_node = graph.node(pair.first_index());
            let key = PositionKey {
                position: first_node.current_position(),
                grid_role: first_node.grid_role(),
            };
            let edge = PositionEdge {
                first_node_index: pair.first_index(),
                second_node_index: pair.second_index(),
            };
            edges_by_position.entry(key).or_default().push(edge);
        }

        let mut groups: Vec<PositionAssignmentGroup> = Vec::new();

        // Step 2: for each position, decompose into connected components, then
        // recursively split each component into direct-conflict groups.
        //
        // A node is only a mover for the group whose anchor it DIRECTLY conflicts
        // with.  Nodes connected only through chains do not form a single group —
        // they form separate groups at successive recursion levels.
        for (key, edges) in edges_by_position {
            let mut node_set: HashSet<usize> = HashSet::new();
            let mut position_adjacency: HashMap<usize, Vec<usize>> = HashMap::new();
            for edge in &edges {
                let first_index = edge.first_node_index;
                let second_index = edge.second_node_index;
                node_set.insert(first_index);
                node_set.insert(second_index);
                position_adjacency
                    .entry(first_index)
                    .or_default()
                    .push(second_index);
                position_adjacency
                    .entry(second_index)
                    .or_default()
                    .push(first_index);
            }

            let mut all_nodes: Vec<usize> = node_set.into_iter().collect();
            all_nodes.sort();

            let mut visited: HashSet<usize> = HashSet::new();
            for &start_node in &all_nodes {
                if visited.contains(&start_node) {
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
                let component_groups = assignment_groups_for_component(
                    &component,
                    &position_adjacency,
                    &graph,
                    key.position,
                    key.grid_role,
                );
                groups.extend(component_groups);
            }
        }

        // Sort groups left-to-right, top-to-bottom.
        // Within the same position: highest-carrier anchor first, then stable by anchor index.
        groups.sort_by(|left, right| {
            let left_row = u8::from(left.position.row());
            let right_row = u8::from(right.position.row());
            let left_col = u8::from(left.position.column());
            let right_col = u8::from(right.position.column());
            let left_role = grid_role_order(left.grid_role);
            let right_role = grid_role_order(right.grid_role);
            let left_anchor_carriers = graph.node(left.anchor_index).carrier_count();
            let right_anchor_carriers = graph.node(right.anchor_index).carrier_count();
            left_row
                .cmp(&right_row)
                .then_with(|| left_col.cmp(&right_col))
                .then_with(|| left_role.cmp(&right_role))
                .then_with(|| right_anchor_carriers.cmp(&left_anchor_carriers))
                .then_with(|| left.anchor_index.cmp(&right.anchor_index))
        });

        let total_mover_count = groups.iter().map(|group| group.mover_count()).sum();

        Self {
            graph,
            groups,
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
}

/// Recursively splits a connected component at a position into assignment groups.
///
/// Each group contains exactly one anchor (the highest-carrier cross-unit node)
/// and its **direct** conflict neighbours at that position as movers.  Nodes that
/// are connected to the anchor only through a chain — but share no carrier unit
/// with the anchor — are not movers for this group; they are handled in a
/// sub-group produced by the next recursion level.
///
/// This ensures that e.g. Wind Walk is not forced to move just because Abolish
/// Magic is the anchor: Wind Walk conflicts only with Dispel Magic (an
/// intermediate node), so it belongs to a separate Dispel Magic–anchored group.
fn assignment_groups_for_component(
    component: &[usize],
    position_adjacency: &HashMap<usize, Vec<usize>>,
    graph: &ConflictGraph,
    position: GridCoordinate,
    grid_role: GridRole,
) -> Vec<PositionAssignmentGroup> {
    let cross_unit_nodes: Vec<usize> = component
        .iter()
        .copied()
        .filter(|&index| graph.node(index).carrier_count() >= 2)
        .collect();

    if cross_unit_nodes.len() < 2 {
        return Vec::new();
    }

    // Anchor: highest carrier count; stable tiebreak — lower index wins.
    let anchor_index = cross_unit_nodes
        .iter()
        .copied()
        .max_by(|&left, &right| {
            let left_carriers = graph.node(left).carrier_count();
            let right_carriers = graph.node(right).carrier_count();
            left_carriers
                .cmp(&right_carriers)
                .then_with(|| right.cmp(&left))
        })
        .expect("cross_unit_nodes is non-empty");

    let empty_neighbors: Vec<usize> = Vec::new();
    let anchor_position_neighbors: &Vec<usize> = position_adjacency
        .get(&anchor_index)
        .unwrap_or(&empty_neighbors);
    let anchor_neighbor_set: HashSet<usize> = anchor_position_neighbors.iter().copied().collect();

    // Direct movers: cross-unit nodes with a direct conflict edge to the anchor
    // within this position's subgraph.
    let mut direct_mover_indices: Vec<usize> = cross_unit_nodes
        .iter()
        .copied()
        .filter(|&index| index != anchor_index && anchor_neighbor_set.contains(&index))
        .collect();

    if direct_mover_indices.is_empty() {
        // The anchor has no cross-unit direct conflicts at this position.
        // Remove it and let the remaining nodes form their own groups.
        let without_anchor: Vec<usize> = component
            .iter()
            .copied()
            .filter(|&index| index != anchor_index)
            .collect();
        return assignment_groups_for_component(
            &without_anchor,
            position_adjacency,
            graph,
            position,
            grid_role,
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

    // BFS on the remaining subgraph to find independent sub-components.
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
        let sub_groups = assignment_groups_for_component(
            &sub_component,
            &remaining_adjacency,
            graph,
            position,
            grid_role,
        );
        groups.extend(sub_groups);
    }

    groups
}

fn grid_role_order(role: GridRole) -> u8 {
    match role {
        GridRole::MainCommand => 0,
        GridRole::BuildMenu => 1,
        GridRole::UprootedForm => 2,
        GridRole::HeroSkillTree => 3,
    }
}

fn grid_role_label(role: GridRole) -> &'static str {
    match role {
        GridRole::MainCommand => "main command",
        GridRole::BuildMenu => "build menu",
        GridRole::UprootedForm => "uprooted",
        GridRole::HeroSkillTree => "research",
    }
}

impl fmt::Display for AssignmentQueue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.groups.is_empty() {
            return writeln!(
                formatter,
                "Assignment queue: empty — no collisions to resolve."
            );
        }
        writeln!(
            formatter,
            "Assignment queue: {} group(s), {} mover(s) total\n",
            self.groups.len(),
            self.total_mover_count,
        )?;
        for (ordinal, group) in self.groups.iter().enumerate() {
            let column = u8::from(group.position.column());
            let row = u8::from(group.position.row());
            let role = grid_role_label(group.grid_role);
            writeln!(
                formatter,
                "[{}] ({},{}) [{}]  — {} mover(s)",
                ordinal + 1,
                column,
                row,
                role,
                group.mover_count(),
            )?;
            let anchor_node = self.graph.node(group.anchor_index);
            let anchor_name = anchor_node.slot_id().display_name(None, None);
            let anchor_id = anchor_node.slot_id().as_str();
            let anchor_carriers = anchor_node.carrier_count();
            let anchor_carrier_ids = anchor_node
                .carrier_unit_ids()
                .iter()
                .map(|id| id.value())
                .collect::<Vec<_>>()
                .join(", ");
            writeln!(
                formatter,
                "    ANCHOR  {anchor_name} ({anchor_id})  [{anchor_carriers} carriers: {anchor_carrier_ids}]"
            )?;
            for &mover_index in &group.mover_indices {
                let mover_node = self.graph.node(mover_index);
                let mover_name = mover_node.slot_id().display_name(None, None);
                let mover_id = mover_node.slot_id().as_str();
                let mover_carriers = mover_node.carrier_count();
                let mover_carrier_ids = mover_node
                    .carrier_unit_ids()
                    .iter()
                    .map(|id| id.value())
                    .collect::<Vec<_>>()
                    .join(", ");
                writeln!(
                    formatter,
                    "    MOVE    {mover_name} ({mover_id})  [{mover_carriers} carriers: {mover_carrier_ids}]"
                )?;
            }
            writeln!(formatter)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod cascade_queue_tests {
    use super::*;
    use crate::conflict_graph::ConflictGraph;
    use crate::custom_keys::CustomKeys;
    use crate::model::{AbilityBinding, ColumnIndex, GridCoordinate, RowIndex};

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
    fn every_mover_has_a_conflict_edge_within_its_group() {
        // Every mover must conflict with at least one other node in its group
        // (the anchor or another mover).  A mover with no in-group edges would
        // mean it was pulled in from a different connected component — the core
        // bug this decomposition fixes.
        let custom_keys = CustomKeys::from("").normalize();
        let graph = ConflictGraph::build(&custom_keys);
        let queue = AssignmentQueue::build(graph);
        for group in queue.groups() {
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
    fn groups_are_sorted_left_to_right_top_to_bottom() {
        let queue = default_queue();
        let positions: Vec<(u8, u8)> = queue
            .groups()
            .iter()
            .map(|group| {
                let col = u8::from(group.position().column());
                let row = u8::from(group.position().row());
                (row, col)
            })
            .collect();
        let mut sorted = positions.clone();
        sorted.sort();
        assert_eq!(
            positions, sorted,
            "groups must be sorted row-then-column ascending"
        );
    }

    #[test]
    fn anchor_has_highest_carrier_count_in_group() {
        let queue = default_queue();
        for group in queue.groups() {
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
    fn movers_are_sorted_most_carriers_first_within_group() {
        let queue = default_queue();
        for group in queue.groups() {
            let mover_counts: Vec<usize> = group
                .mover_indices()
                .iter()
                .map(|&index| queue.graph().node(index).carrier_count())
                .collect();
            let mut sorted = mover_counts.clone();
            sorted.sort_by(|left, right| right.cmp(left));
            assert_eq!(
                mover_counts, sorted,
                "movers within a group must be sorted by carrier count descending"
            );
        }
    }

    #[test]
    fn every_mover_is_at_the_group_position() {
        let queue = default_queue();
        for group in queue.groups() {
            let expected_position = group.position();
            let expected_role = group.grid_role();
            for &mover_index in group.mover_indices() {
                let mover_node = queue.graph().node(mover_index);
                assert_eq!(
                    mover_node.current_position(),
                    expected_position,
                    "mover must be at the group's collision position"
                );
                assert_eq!(
                    mover_node.grid_role(),
                    expected_role,
                    "mover must be on the group's grid role"
                );
            }
        }
    }

    #[test]
    fn three_way_collision_produces_at_least_two_movers() {
        // Place three Paladin abilities at the same position.
        // The Paladin carries AHhb, AHds, and AHad — all three share a unit,
        // so all three have conflict edges with each other.
        // Result: 1 anchor + at least 2 movers at that position.
        let position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
        let binding = AbilityBinding::builder().button_position(position).build();
        let mut custom_keys = CustomKeys::from("").normalize();
        custom_keys.put_ability("AHhb", binding.clone());
        custom_keys.put_ability("AHds", binding.clone());
        custom_keys.put_ability("AHad", binding);
        let graph = ConflictGraph::build(&custom_keys);
        let queue = AssignmentQueue::build(graph);
        let group = queue
            .groups()
            .iter()
            .find(|group| {
                group.position() == position && group.grid_role() == GridRole::MainCommand
            })
            .expect("must have a group at (0,0) main command");
        assert!(
            group.mover_count() >= 2,
            "three Paladin abilities at the same position must produce at least 2 movers, \
             got {}",
            group.mover_count()
        );
    }

    #[test]
    fn all_single_carrier_group_is_excluded_from_queue() {
        // If every ability at a colliding position has exactly 1 carrier, the
        // collision is purely intra-unit and must not appear in the queue.
        // We simulate this by placing two abilities that share only one unit
        // and verifying no group survives the filter.
        // AHhb (Holy Light) is only on Paladin variants — all have carrier_count >= 1.
        // To guarantee a 1-carrier-only collision we need two abilities whose only
        // shared carrier is a single unit. We use two abilities that are each
        // exclusively on the Paladin (carrier_count == small number but > 1 due to variants).
        // Instead, verify via the default keys that every surviving group has at
        // least one node with carrier_count >= 2.
        let queue = default_queue();
        for group in queue.groups() {
            let anchor_carriers = queue.graph().node(group.anchor_index()).carrier_count();
            let any_mover_shared = group
                .mover_indices()
                .iter()
                .any(|&index| queue.graph().node(index).carrier_count() >= 2);
            assert!(
                anchor_carriers >= 2 || any_mover_shared,
                "every queue group must have at least one node with carrier_count >= 2"
            );
        }
    }

    #[test]
    fn four_way_collision_produces_at_least_three_movers() {
        // The Paladin carries AHhb, AHds, AHad, and AHre — all four share edges.
        // Placing them all at the same position must produce at least 3 movers.
        let position = GridCoordinate::new(ColumnIndex::One, RowIndex::One);
        let binding = AbilityBinding::builder().button_position(position).build();
        let mut custom_keys = CustomKeys::from("").normalize();
        custom_keys.put_ability("AHhb", binding.clone());
        custom_keys.put_ability("AHds", binding.clone());
        custom_keys.put_ability("AHad", binding.clone());
        custom_keys.put_ability("AHre", binding);
        let graph = ConflictGraph::build(&custom_keys);
        let queue = AssignmentQueue::build(graph);
        let group = queue
            .groups()
            .iter()
            .find(|group| {
                group.position() == position && group.grid_role() == GridRole::MainCommand
            })
            .expect("must have a group at (1,1) main command");
        assert!(
            group.mover_count() >= 3,
            "four Paladin abilities at the same position must produce at least 3 movers, \
             got {}",
            group.mover_count()
        );
    }
}
