#[cfg(test)]
mod cascade_queue_tests {
    use super::super::*;
    use crate::cascade::conflict_graph::ConflictGraph;
    use crate::custom_keys::CustomKeys;
    use crate::identity::slot::GridSlotId;
    use crate::model::{AbilityBinding, ColumnIndex, CommandBinding, GridCoordinate, RowIndex};
    use std::collections::HashSet;

    /// Replays the global-command half of the QWER/ASDF/YXCV drag-drop
    /// rearrange on top of the default keys: the four bottom-row cells fill up
    /// with pinned, high-carrier system commands.
    fn rearranged_default_keys() -> CustomKeys {
        let mut custom_keys = CustomKeys::from_text("");
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
        let custom_keys = CustomKeys::from_text("");
        let graph = ConflictGraph::build(&custom_keys);
        AssignmentQueue::build(graph)
    }

    #[test]
    fn queue_is_nonempty_for_default_keys() {
        let queue = default_queue();
        assert!(
            !queue.is_empty(),
            "default keys have known collisions so the queue must be non-empty",
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
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
        struct RowColumn {
            row: u8,
            column: u8,
        }
        let queue = default_queue();
        let positions: Vec<RowColumn> = queue
            .groups()
            .iter()
            .filter(|group| !group.is_spill())
            .map(|group| {
                let column = u8::from(group.position().column());
                let row = u8::from(group.position().row());
                RowColumn { row, column }
            })
            .collect();
        let mut sorted = positions.clone();
        sorted.sort();
        assert_eq!(
            positions, sorted,
            "phase-1 groups must be sorted row-then-column ascending",
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
                    "anchor must have at least as many carriers as any mover in the group",
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
                "movers within a fight group must be sorted by carrier count descending",
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
                "anchor of every group must end up at the group's position",
            );
        }
    }

    #[test]
    fn every_fight_group_has_at_least_two_members() {
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
        let mut custom_keys = CustomKeys::from_text("");
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
             groups at (0,0) main command, got {combined_movers}",
        );
    }

    #[test]
    fn four_way_collision_produces_at_least_three_movers() {
        let position = GridCoordinate::new(ColumnIndex::One, RowIndex::One);
        let binding = AbilityBinding::builder().button_position(position).build();
        let mut custom_keys = CustomKeys::from_text("");
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
             groups at (1,1) main command, got {combined_movers}",
        );
    }

    #[test]
    fn every_fight_mover_never_moves_to_an_earlier_row() {
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
            let group_row = u8::from(group.position().row());
            for &mover_index in group.mover_indices() {
                if spilled_anchors.contains(&mover_index) {
                    continue;
                }
                let final_position = queue.final_position(mover_index);
                let mover_row = u8::from(final_position.row());
                assert!(
                    mover_row >= group_row,
                    "mover {} ended on row {} but its fight group was on row {} — \
                     cascade may wrap forward to a later row, never backward",
                    queue.graph().node(mover_index).slot_id().as_str(),
                    mover_row,
                    group_row,
                );
            }
        }
    }

    #[test]
    fn no_post_queue_collisions_for_resolved_cross_unit_nodes() {
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
        let collision_position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
        let next_position = GridCoordinate::new(ColumnIndex::One, RowIndex::Zero);
        let binding_collision = AbilityBinding::builder()
            .button_position(collision_position)
            .build();
        let binding_next = AbilityBinding::builder()
            .button_position(next_position)
            .build();
        let mut custom_keys = CustomKeys::from_text("");
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
             land on an already-occupied Paladin slot, got {fight_groups_at_next}",
        );
    }

    #[test]
    fn paladin_collision_is_resolved_with_no_orphans() {
        let collision_position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
        let collision_binding = AbilityBinding::builder()
            .button_position(collision_position)
            .build();
        let mut custom_keys = CustomKeys::from_text("");
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
                 leaving it unresolved is worse than a row change",
            );
        }
    }

    #[test]
    fn pinned_system_commands_never_move_from_default() {
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
            "default keys must contain at least one Cmd* slot for this test to be meaningful",
        );
    }

    #[test]
    fn pinned_ancient_root_never_moves_from_default() {
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
            "default keys must contain at least one Aro1/Aro2 node for this test to be meaningful",
        );
    }

    #[test]
    fn unresolved_node_keeps_its_original_row() {
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
             ended at column {final_column}",
        );
    }

    #[test]
    fn resolving_rearranged_keys_leaves_no_position_collisions() {
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
