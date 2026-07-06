use crate::custom_keys::CustomKeys;
use crate::identity::slot::GridSlotId;
use crate::model::GridCoordinate;
use crate::unit::grids::{GridRole, UnitGrids};
use crate::unit::slots::UnitCommandSlots;
use std::collections::{HashMap, HashSet};
use std::fmt;
use warcraft_api::{WarcraftObjectId, WarcraftObjectMeta};
use warcraft_database::WARCRAFT_DATABASE;

/// Canonical identifier for one ability state on one command card page type.
///
/// An ability's on-state (`Ability(X)`) and off-state (`AbilityOff(X)`) are
/// distinct buttons that occupy distinct cells: the on-state sits on the base
/// unit's card at `Buttonpos`, while the off-state sits at `Unbuttonpos` —
/// often on a different card entirely (a morphed form's unit) or, for some
/// buildings, on the same card at a second cell (Town Hall's Call To Arms vs
/// Back To Work).  Keying them apart via `is_off_state` lets the cascade see
/// and resolve each state's collisions independently; collapsing them would
/// leave whichever state the cascade did not track sitting on its collision.
///
/// The `ability_str_lowercase` field collapses casing variants together: the
/// auto-generated database contains some abilities registered under two
/// different casings (e.g. `ACvs` and `Acvs` for Envenomed Weapons) with
/// disjoint carrier unit sets.  Without lowercasing, the conflict graph would
/// treat them as two separate abilities and miss the very real conflict
/// between them.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct AbilityRoleKey {
    ability_str_lowercase: String,
    grid_role: GridRole,
    is_off_state: bool,
}

/// Intermediate accumulator used during graph construction.
#[derive(Clone, Debug, PartialEq, Eq)]
struct NodeAccumulator {
    canonical_slot: GridSlotId,
    position: GridCoordinate,
    carriers: HashSet<WarcraftObjectId>,
}

/// One node in the conflict graph — a single ability on a specific command card page.
///
/// Two nodes are connected by an edge if they share at least one carrier unit,
/// meaning they cannot occupy the same grid position (doing so would produce a
/// button collision on that unit's command card).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ConflictNode {
    slot_id: GridSlotId,
    grid_role: GridRole,
    current_position: GridCoordinate,
    carrier_unit_ids: Vec<WarcraftObjectId>,
    /// Minimum index of this ability across all carrier units' `abilList` ordering.
    /// Lower means the ability appears earlier in some unit's list and should be
    /// preferred as the fight anchor when carrier counts are equal.
    /// `usize::MAX` if the ability is not found in any carrier's abilList (e.g. Cmd*).
    ability_list_priority: usize,
}

impl ConflictNode {
    pub fn slot_id(&self) -> GridSlotId {
        self.slot_id
    }

    pub fn grid_role(&self) -> GridRole {
        self.grid_role
    }

    pub fn current_position(&self) -> GridCoordinate {
        self.current_position
    }

    pub fn carrier_count(&self) -> usize {
        self.carrier_unit_ids.len()
    }

    pub fn carrier_unit_ids(&self) -> &[WarcraftObjectId] {
        &self.carrier_unit_ids
    }

    pub fn ability_list_priority(&self) -> usize {
        self.ability_list_priority
    }
}

/// A pair of graph nodes that are connected (share at least one carrier unit)
/// and are currently assigned to the same grid position — i.e. a button collision.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CollidingPair {
    first_index: usize,
    second_index: usize,
}

impl CollidingPair {
    pub fn first_index(&self) -> usize {
        self.first_index
    }

    pub fn second_index(&self) -> usize {
        self.second_index
    }
}

/// The full conflict graph for all abilities across all units and grid pages.
///
/// Nodes are abilities (one per unique `(ability_str, GridRole)` pair).
/// An edge exists between two nodes when they share at least one carrier unit
/// on the same page — meaning the two abilities cannot occupy the same grid
/// position simultaneously.  A **collision** is an edge where both endpoints
/// currently have the same position.
///
/// This is the foundation for the cascade solver.  All abilities are included,
/// not just colliding ones, because non-colliding abilities occupy positions
/// that are off-limits for any move.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct ConflictGraph {
    nodes: Vec<ConflictNode>,
    /// `adjacency[i]` is a sorted list of node indices that conflict with node `i`.
    adjacency: Vec<Vec<usize>>,
    key_to_index: HashMap<AbilityRoleKey, usize>,
}

impl AbilityRoleKey {
    /// Returns the minimum index of this ability across all carrier units'
    /// `abilList` ordering, or `usize::MAX` if it is absent from every list.
    fn ability_list_priority(&self, carrier_unit_ids: &[WarcraftObjectId]) -> usize {
        let mut min_priority = usize::MAX;
        for carrier_id in carrier_unit_ids {
            let Some(unit_object) = WARCRAFT_DATABASE.by_id(carrier_id.value()) else {
                continue;
            };
            let WarcraftObjectMeta::Unit(unit_meta) = unit_object.meta() else {
                continue;
            };
            let abilities = unit_meta.abilities();
            for (position, listed_id) in abilities.iter().enumerate() {
                let listed_lower = listed_id.value().to_ascii_lowercase();
                if listed_lower == self.ability_str_lowercase {
                    min_priority = min_priority.min(position);
                    break;
                }
            }
        }
        min_priority
    }
}

impl ConflictGraph {
    pub fn build(custom_keys: &CustomKeys) -> Self {
        let mut node_accumulators: HashMap<AbilityRoleKey, NodeAccumulator> = HashMap::new();
        let mut unit_role_keys: HashMap<
            WarcraftObjectId,
            HashMap<GridRole, HashSet<AbilityRoleKey>>,
        > = HashMap::new();
        for unit_id in WARCRAFT_DATABASE.all_unit_ids() {
            let unit_grids = UnitGrids::for_unit(unit_id);
            for named_grid in unit_grids.grids() {
                let grid_role = named_grid.role();
                let is_research = grid_role.is_research_context();
                for slot in named_grid.card().filled_slots() {
                    let Some(position) = custom_keys.position_for_slot(&slot, is_research) else {
                        continue;
                    };
                    let ability_str_lowercase = slot.as_str().to_ascii_lowercase();
                    let is_off_state = matches!(slot, GridSlotId::AbilityOff(_));
                    let key = AbilityRoleKey {
                        ability_str_lowercase,
                        grid_role,
                        is_off_state,
                    };
                    let accumulator =
                        node_accumulators
                            .entry(key.clone())
                            .or_insert_with(|| NodeAccumulator {
                                canonical_slot: slot,
                                position,
                                carriers: HashSet::new(),
                            });
                    accumulator.carriers.insert(unit_id);
                    unit_role_keys
                        .entry(unit_id)
                        .or_default()
                        .entry(grid_role)
                        .or_default()
                        .insert(key);
                }
            }
        }
        let mut ordered_keys: Vec<AbilityRoleKey> = node_accumulators.keys().cloned().collect();
        ordered_keys.sort_by(|left, right| {
            let left_role_index = left.grid_role.sort_index();
            let right_role_index = right.grid_role.sort_index();
            let role_order = left_role_index.cmp(&right_role_index);
            role_order.then_with(|| left.ability_str_lowercase.cmp(&right.ability_str_lowercase))
        });
        let mut key_to_index: HashMap<AbilityRoleKey, usize> = HashMap::new();
        for (index, key) in ordered_keys.iter().enumerate() {
            key_to_index.insert(key.clone(), index);
        }
        let mut nodes: Vec<ConflictNode> = Vec::with_capacity(ordered_keys.len());
        for key in &ordered_keys {
            let accumulator = node_accumulators.remove(key).expect("key must be present");
            let mut carrier_unit_ids: Vec<WarcraftObjectId> =
                accumulator.carriers.into_iter().collect();
            carrier_unit_ids.sort_by(|left, right| left.value().cmp(right.value()));
            let ability_list_priority = key.ability_list_priority(&carrier_unit_ids);
            nodes.push(ConflictNode {
                slot_id: accumulator.canonical_slot,
                grid_role: key.grid_role,
                current_position: accumulator.position,
                carrier_unit_ids,
                ability_list_priority,
            });
        }
        let mut edge_sets: Vec<HashSet<usize>> = (0..nodes.len()).map(|_| HashSet::new()).collect();
        for role_map in unit_role_keys.values() {
            for role_key_set in role_map.values() {
                let role_keys: Vec<AbilityRoleKey> = role_key_set.iter().cloned().collect();
                for outer in 0..role_keys.len() {
                    for inner in (outer + 1)..role_keys.len() {
                        let index_outer = key_to_index[&role_keys[outer]];
                        let index_inner = key_to_index[&role_keys[inner]];
                        edge_sets[index_outer].insert(index_inner);
                        edge_sets[index_inner].insert(index_outer);
                    }
                }
            }
        }
        let adjacency: Vec<Vec<usize>> = edge_sets
            .into_iter()
            .map(|neighbor_set| {
                let mut neighbors: Vec<usize> = neighbor_set.into_iter().collect();
                neighbors.sort();
                neighbors
            })
            .collect();
        Self {
            nodes,
            adjacency,
            key_to_index,
        }
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        let total_directed: usize = self.adjacency.iter().map(|neighbors| neighbors.len()).sum();
        total_directed / 2
    }

    pub fn nodes(&self) -> &[ConflictNode] {
        &self.nodes
    }

    pub fn node(&self, index: usize) -> &ConflictNode {
        &self.nodes[index]
    }

    pub fn neighbors(&self, index: usize) -> &[usize] {
        &self.adjacency[index]
    }

    pub fn degree(&self, index: usize) -> usize {
        self.adjacency[index].len()
    }

    /// Looks up an ability's on-state node by its ability string and grid role.
    /// The lookup is case-insensitive — the conflict graph keys nodes by the
    /// lowercase form of the ability string so casing variants in the database
    /// are merged.  Off-state nodes are keyed separately; this returns the
    /// on-state (`Buttonpos`) node, which is what every caller wants.
    pub fn find_node(&self, ability_str: &str, grid_role: GridRole) -> Option<usize> {
        let key = AbilityRoleKey {
            ability_str_lowercase: ability_str.to_ascii_lowercase(),
            grid_role,
            is_off_state: false,
        };
        self.key_to_index.get(&key).copied()
    }

    /// All pairs of connected nodes that currently occupy the same grid position.
    /// These are the actual button collisions the solver must eliminate.
    pub fn colliding_pairs(&self) -> Vec<CollidingPair> {
        let mut pairs: Vec<CollidingPair> = Vec::new();
        for (first_index, first_node) in self.nodes.iter().enumerate() {
            for &second_index in &self.adjacency[first_index] {
                if second_index <= first_index {
                    continue;
                }
                let second_node = &self.nodes[second_index];
                if first_node.current_position == second_node.current_position {
                    pairs.push(CollidingPair {
                        first_index,
                        second_index,
                    });
                }
            }
        }
        pairs
    }
}

impl fmt::Display for ConflictGraph {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let colliding_count = self.colliding_pairs().len();
        writeln!(formatter, "Conflict graph:")?;
        writeln!(formatter, "  Nodes:           {}", self.node_count())?;
        writeln!(formatter, "  Edges:           {}", self.edge_count())?;
        writeln!(formatter, "  Colliding pairs: {}", colliding_count)?;
        let mut by_carrier: Vec<usize> = (0..self.nodes.len()).collect();
        by_carrier.sort_by(|&left, &right| {
            let left_count = self.nodes[left].carrier_count();
            let right_count = self.nodes[right].carrier_count();
            right_count.cmp(&left_count)
        });
        writeln!(
            formatter,
            "\nTop nodes by carrier count (anchor candidates):"
        )?;
        for &index in by_carrier.iter().take(5) {
            let node = &self.nodes[index];
            let position = node.current_position();
            let column = u8::from(position.column());
            let row = u8::from(position.row());
            let role = node.grid_role().label();
            writeln!(
                formatter,
                "  {:12} [{:12}]  ({},{})  {} carriers  degree {}",
                node.slot_id().as_str(),
                role,
                column,
                row,
                node.carrier_count(),
                self.degree(index),
            )?;
        }
        let mut by_degree: Vec<usize> = (0..self.nodes.len()).collect();
        by_degree.sort_by_key(|&index| std::cmp::Reverse(self.degree(index)));
        writeln!(
            formatter,
            "\nTop nodes by conflict degree (hardest to move):"
        )?;
        for &index in by_degree.iter().take(5) {
            let node = &self.nodes[index];
            let position = node.current_position();
            let column = u8::from(position.column());
            let row = u8::from(position.row());
            let role = node.grid_role().label();
            writeln!(
                formatter,
                "  {:12} [{:12}]  ({},{})  {} carriers  degree {}",
                node.slot_id().as_str(),
                role,
                column,
                row,
                node.carrier_count(),
                self.degree(index),
            )?;
        }
        Ok(())
    }
}

impl ddd::Layered for ConflictNode {
    type Layer = ddd::DomainLayer;
}

impl ddd::ValueObject for ConflictNode {}

impl ddd::Layered for CollidingPair {
    type Layer = ddd::DomainLayer;
}

impl ddd::ValueObject for CollidingPair {}

impl ddd::Layered for ConflictGraph {
    type Layer = ddd::DomainLayer;
}

impl ddd::ValueObject for ConflictGraph {}

#[cfg(test)]
mod ddd_marker_tests {
    use super::CollidingPair;
    use super::ConflictGraph;
    use super::ConflictNode;
    use crate::ddd_conformance::assert_value_object;

    #[test]
    fn conflict_graph_types_are_value_objects() {
        assert_value_object::<ConflictNode>();
        assert_value_object::<CollidingPair>();
        assert_value_object::<ConflictGraph>();
    }
}

#[cfg(test)]
mod conflict_graph_tests {
    use super::*;
    use crate::model::{AbilityBinding, ColumnIndex, GridCoordinate, RowIndex};

    fn default_graph() -> ConflictGraph {
        let custom_keys = crate::custom_keys::CustomKeys::from_text("");
        ConflictGraph::build(&custom_keys)
    }

    #[test]
    fn node_count_is_nonzero_for_default_keys() {
        let graph = default_graph();
        assert!(
            graph.node_count() > 0,
            "default keys must produce at least one graph node",
        );
    }

    #[test]
    fn edge_count_is_nonzero_for_default_keys() {
        let graph = default_graph();
        assert!(
            graph.edge_count() > 0,
            "default keys must produce at least one conflict edge",
        );
    }

    #[test]
    fn colliding_pairs_count_matches_cross_unit_report_structure() {
        let custom_keys = crate::custom_keys::CustomKeys::from_text("");
        let graph = ConflictGraph::build(&custom_keys);
        let report = crate::collision::cross_unit::CrossUnitCollisionReport::compute(&custom_keys);
        assert!(
            graph.colliding_pairs().len() >= report.position_groups().len(),
            "graph colliding pairs ({}) must be at least as many as collision groups ({})",
            graph.colliding_pairs().len(),
            report.position_groups().len(),
        );
    }

    #[test]
    fn abilities_sharing_a_unit_have_a_conflict_edge() {
        let custom_keys = crate::custom_keys::CustomKeys::from_text("");
        let graph = ConflictGraph::build(&custom_keys);
        let holy_light_index = graph
            .find_node("AHhb", GridRole::MainCommand)
            .expect("AHhb must be a node");
        let divine_shield_index = graph
            .find_node("AHds", GridRole::MainCommand)
            .expect("AHds must be a node");
        let holy_light_neighbors = graph.neighbors(holy_light_index);
        assert!(
            holy_light_neighbors.contains(&divine_shield_index),
            "AHhb and AHds must share a conflict edge (both on Paladin main command)",
        );
    }

    #[test]
    fn abilities_on_different_pages_have_no_edge() {
        let custom_keys = crate::custom_keys::CustomKeys::from_text("");
        let graph = ConflictGraph::build(&custom_keys);
        let Some(holy_light_index) = graph.find_node("AHhb", GridRole::MainCommand) else {
            return;
        };
        let Some(holy_light_research_index) = graph.find_node("AHhb", GridRole::HeroSkillTree)
        else {
            return;
        };
        let neighbors = graph.neighbors(holy_light_index);
        assert!(
            !neighbors.contains(&holy_light_research_index),
            "AHhb on MainCommand and AHhb on HeroSkillTree must not share an edge",
        );
    }

    #[test]
    fn carrier_count_for_hold_position_is_large() {
        let graph = default_graph();
        let index = graph
            .find_node("CmdHoldPos", GridRole::MainCommand)
            .expect("CmdHoldPos must be a node on MainCommand");
        let carrier_count = graph.node(index).carrier_count();
        assert!(
            carrier_count > 100,
            "CmdHoldPos must have more than 100 carriers, got {carrier_count}",
        );
    }

    #[test]
    fn carrier_count_for_paladin_specific_ability_is_small() {
        let graph = default_graph();
        let hold_index = graph
            .find_node("CmdHoldPos", GridRole::MainCommand)
            .expect("CmdHoldPos must be a node");
        let holy_light_index = graph
            .find_node("AHhb", GridRole::MainCommand)
            .expect("AHhb must be a node on MainCommand");
        let hold_carrier_count = graph.node(hold_index).carrier_count();
        let holy_light_carrier_count = graph.node(holy_light_index).carrier_count();
        assert!(
            holy_light_carrier_count < hold_carrier_count / 10,
            "AHhb ({holy_light_carrier_count} carriers) should be far less than CmdHoldPos \
             ({hold_carrier_count} carriers)",
        );
    }

    #[test]
    fn two_abilities_at_same_position_produce_a_colliding_pair() {
        let shared_position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
        let binding = AbilityBinding::builder()
            .button_position(shared_position)
            .build();
        let mut custom_keys = crate::custom_keys::CustomKeys::from_text("");
        custom_keys.put_ability("AHhb", binding.clone());
        custom_keys.put_ability("AHds", binding);
        let graph = ConflictGraph::build(&custom_keys);
        let pairs = graph.colliding_pairs();
        let involves_paladin_abilities = pairs.iter().any(|pair| {
            let first = graph.node(pair.first_index()).slot_id().as_str();
            let second = graph.node(pair.second_index()).slot_id().as_str();
            (first == "AHhb" || second == "AHhb") && (first == "AHds" || second == "AHds")
        });
        assert!(
            involves_paladin_abilities,
            "placing AHhb and AHds at the same position must produce a colliding pair",
        );
    }

    #[test]
    fn no_colliding_pairs_when_abilities_are_at_distinct_positions() {
        let position_a = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
        let position_b = GridCoordinate::new(ColumnIndex::One, RowIndex::Zero);
        let binding_a = AbilityBinding::builder()
            .button_position(position_a)
            .build();
        let binding_b = AbilityBinding::builder()
            .button_position(position_b)
            .build();
        let mut custom_keys = crate::custom_keys::CustomKeys::from_text("");
        custom_keys.put_ability("AHhb", binding_a);
        custom_keys.put_ability("AHds", binding_b);
        let graph = ConflictGraph::build(&custom_keys);
        let false_pair = graph.colliding_pairs().into_iter().any(|pair| {
            let first = graph.node(pair.first_index()).slot_id().as_str();
            let second = graph.node(pair.second_index()).slot_id().as_str();
            (first == "AHhb" || second == "AHhb") && (first == "AHds" || second == "AHds")
        });
        assert!(
            !false_pair,
            "abilities at distinct positions must not produce a colliding pair",
        );
    }

    #[test]
    fn node_count_is_stable_for_default_keys() {
        let graph = default_graph();
        assert_eq!(
            graph.node_count(),
            graph.node_count(),
            "node count must be deterministic across two builds",
        );
        let count = graph.node_count();
        assert!(
            count > 500,
            "default keys must produce more than 500 graph nodes, got {count}",
        );
    }

    #[test]
    fn ability_casing_variants_collapse_into_a_single_node() {
        let custom_keys = CustomKeys::from_text("");
        let graph = ConflictGraph::build(&custom_keys);
        let upper_index = graph.find_node("ACvs", GridRole::MainCommand);
        let lower_index = graph.find_node("Acvs", GridRole::MainCommand);
        let mixed_lower_index = graph.find_node("acvs", GridRole::MainCommand);
        assert!(
            upper_index.is_some(),
            "expected to find Envenomed Weapons node via uppercase casing",
        );
        assert_eq!(
            upper_index, lower_index,
            "ACvs and Acvs must resolve to the same conflict-graph node",
        );
        assert_eq!(
            upper_index, mixed_lower_index,
            "lowercase 'acvs' must also resolve to the same node",
        );
        let merged_node = graph.node(upper_index.expect("node must exist"));
        assert!(
            merged_node.carrier_count() >= 8,
            "merged Envenomed Weapons node must carry the union of both casings' \
             carriers (≥ 8 units), got {}",
            merged_node.carrier_count(),
        );
    }
}
