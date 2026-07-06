#[cfg(test)]
mod unit_grids_tests {
    use super::super::*;
    use crate::custom_keys::CustomKeys;
    use crate::grid::layout::GridLayout;
    use crate::identity::ability_id::AbilityId;
    use crate::identity::keycode::Letter;
    use crate::model::{AbilityBinding, ColumnIndex, GridCoordinate, Hotkey, RowIndex};

    fn paladin_id() -> WarcraftObjectId {
        WarcraftObjectId::new("Hpal")
    }

    fn peasant_id() -> WarcraftObjectId {
        WarcraftObjectId::new("hpea")
    }

    fn footman_id() -> WarcraftObjectId {
        WarcraftObjectId::new("hfoo")
    }

    fn tree_of_life_id() -> WarcraftObjectId {
        WarcraftObjectId::new("etol")
    }

    fn firelord_id() -> WarcraftObjectId {
        WarcraftObjectId::new("Nfir")
    }

    /// Sections matching the Firelord (`Nfir`) abilities and Patrol command as
    /// they appear in a real imported `CustomKeys.txt`.  The decisive detail is
    /// that `ANic` (the autocast Incinerate companion ability) carries only a
    /// `Researchbuttonpos` and a leftover `Hotkey=C` — it has no command-card
    /// `Buttonpos`, so it is never drawn on the main command card.  The
    /// command-card Incinerate the player sees is `ANia` at `E`.
    fn firelord_incinerate_sections() -> &'static str {
        concat!(
            "[ANso]\nButtonpos=0,0\nHotkey=Q\nResearchbuttonpos=0,0\nResearchhotkey=Q\n\n",
            "[ANlm]\nButtonpos=1,0\nHotkey=W\nResearchbuttonpos=1,0\nResearchhotkey=W\n\n",
            "[ANia]\nButtonpos=2,0\nHotkey=E\nResearchhotkey=C\nUnbuttonpos=2,0\nUnhotkey=E\n\n",
            "[ANic]\nHotkey=C\nResearchbuttonpos=2,0\nResearchhotkey=E\n\n",
            "[ANvc]\nButtonpos=3,0\nHotkey=R\nResearchbuttonpos=3,0\nResearchhotkey=R\n\n",
            "[CmdPatrol]\nButtonpos=2,2\nHotkey=C\nUnbuttonpos=2,2\n",
        )
    }

    #[test]
    fn ability_without_command_position_has_no_main_command_token() {
        let custom_keys = CustomKeys::from_text(firelord_incinerate_sections());
        let layout = GridLayout::qwerty_grid();
        let incinerate_companion = GridSlotId::Ability(AbilityId::new("ANic"));
        let main_command_token =
            custom_keys.effective_hotkey_token(&incinerate_companion, layout, false);
        assert_eq!(
            main_command_token, None,
            "ANic has no command-card Buttonpos, so it occupies no main-command \
             cell and must not contribute a hotkey token there",
        );
    }

    #[test]
    fn ability_with_research_position_keeps_research_token() {
        let custom_keys = CustomKeys::from_text(firelord_incinerate_sections());
        let layout = GridLayout::qwerty_grid();
        let incinerate_companion = GridSlotId::Ability(AbilityId::new("ANic"));
        let research_token =
            custom_keys.effective_hotkey_token(&incinerate_companion, layout, true);
        let expected = Some(HotkeyToken::Letter(Letter::E));
        assert_eq!(
            research_token, expected,
            "ANic has a Researchbuttonpos at E and Researchhotkey=E, so it must \
             still report E in the research context",
        );
    }

    #[test]
    fn firelord_has_no_phantom_incinerate_patrol_collision() {
        let custom_keys = CustomKeys::from_text(firelord_incinerate_sections());
        let unit_grids = UnitGrids::for_unit(firelord_id());
        let layout = GridLayout::qwerty_grid();
        let cards = unit_grids.hotkey_collisions(&custom_keys, layout);
        let main_command_card = cards
            .iter()
            .find(|card| card.role() == GridRole::MainCommand)
            .expect("Firelord has a main command card");
        let patrol_cell = GridCoordinate::new(ColumnIndex::Two, RowIndex::Two);
        assert!(
            main_command_card.collision_at(patrol_cell).is_none(),
            "CmdPatrol (C) shares no main-command cell with any drawn ability; \
             ANic is not on the command card and must not collide with it",
        );
    }

    #[test]
    fn regular_unit_has_one_grid() {
        let unit_grids = UnitGrids::for_unit(footman_id());
        assert_eq!(unit_grids.grid_count(), 1);
    }

    #[test]
    fn regular_unit_grid_role_is_main_command() {
        let unit_grids = UnitGrids::for_unit(footman_id());
        let first_grid = &unit_grids.grids()[0];
        assert_eq!(first_grid.role(), GridRole::MainCommand);
    }

    #[test]
    fn hero_has_two_grids() {
        let unit_grids = UnitGrids::for_unit(paladin_id());
        assert_eq!(unit_grids.grid_count(), 2);
    }

    #[test]
    fn hero_second_grid_role_is_hero_skill_tree() {
        let unit_grids = UnitGrids::for_unit(paladin_id());
        let second_grid = &unit_grids.grids()[1];
        assert_eq!(second_grid.role(), GridRole::HeroSkillTree);
    }

    #[test]
    fn hero_skill_tree_is_research_context() {
        assert!(GridRole::HeroSkillTree.is_research_context());
    }

    #[test]
    fn main_command_is_not_research_context() {
        assert!(!GridRole::MainCommand.is_research_context());
    }

    #[test]
    fn build_menu_is_not_research_context() {
        assert!(!GridRole::BuildMenu.is_research_context());
    }

    #[test]
    fn uprooted_form_is_not_research_context() {
        assert!(!GridRole::UprootedForm.is_research_context());
    }

    #[test]
    fn worker_has_two_grids() {
        let unit_grids = UnitGrids::for_unit(peasant_id());
        assert_eq!(unit_grids.grid_count(), 2);
    }

    #[test]
    fn worker_second_grid_role_is_build_menu() {
        let unit_grids = UnitGrids::for_unit(peasant_id());
        let second_grid = &unit_grids.grids()[1];
        assert_eq!(second_grid.role(), GridRole::BuildMenu);
    }

    #[test]
    fn uprootable_building_has_two_grids() {
        let unit_grids = UnitGrids::for_unit(tree_of_life_id());
        assert_eq!(unit_grids.grid_count(), 2);
    }

    #[test]
    fn uprootable_building_second_grid_role_is_uprooted_form() {
        let unit_grids = UnitGrids::for_unit(tree_of_life_id());
        let second_grid = &unit_grids.grids()[1];
        assert_eq!(second_grid.role(), GridRole::UprootedForm);
    }

    #[test]
    fn hero_skill_tree_is_non_empty() {
        let unit_grids = UnitGrids::for_unit(paladin_id());
        let skill_tree = &unit_grids.grids()[1];
        assert!(!skill_tree.card().is_empty());
    }

    #[test]
    fn worker_build_menu_is_non_empty() {
        let unit_grids = UnitGrids::for_unit(peasant_id());
        let build_menu = &unit_grids.grids()[1];
        assert!(!build_menu.card().is_empty());
    }

    #[test]
    fn unit_grids_exposes_correct_unit_id() {
        let unit_id = paladin_id();
        let unit_grids = UnitGrids::for_unit(unit_id);
        assert_eq!(unit_grids.unit_id(), unit_id);
    }

    #[test]
    fn position_collisions_empty_for_normalized_default() {
        let custom_keys = CustomKeys::from_text("");
        let unit_grids = UnitGrids::for_unit(paladin_id());
        let cards = unit_grids.position_collisions(&custom_keys);
        assert!(
            cards.iter().all(|card| card.is_empty()),
            "normalized default state must have no position collisions for Paladin",
        );
    }

    #[test]
    fn position_collisions_detects_two_abilities_at_same_slot() {
        let collision_position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
        let holy_light_binding = AbilityBinding::builder()
            .button_position(collision_position)
            .build();
        let divine_shield_binding = AbilityBinding::builder()
            .button_position(collision_position)
            .build();
        let mut custom_keys = CustomKeys::from_text("");
        custom_keys.put_ability("AHhb", holy_light_binding);
        custom_keys.put_ability("AHds", divine_shield_binding);
        let unit_grids = UnitGrids::for_unit(paladin_id());
        let cards = unit_grids.position_collisions(&custom_keys);
        let has_collision = cards
            .iter()
            .any(|card| card.collision_at(collision_position).is_some());
        assert!(
            has_collision,
            "placing two Paladin abilities at (0,0) must produce a position collision",
        );
    }

    #[test]
    fn position_collision_reports_both_slots() {
        let shared_position = GridCoordinate::new(ColumnIndex::One, RowIndex::Zero);
        let holy_light_binding = AbilityBinding::builder()
            .button_position(shared_position)
            .build();
        let divine_shield_binding = AbilityBinding::builder()
            .button_position(shared_position)
            .build();
        let mut custom_keys = CustomKeys::from_text("");
        custom_keys.put_ability("AHhb", holy_light_binding);
        custom_keys.put_ability("AHds", divine_shield_binding);
        let unit_grids = UnitGrids::for_unit(paladin_id());
        let cards = unit_grids.position_collisions(&custom_keys);
        let collision = cards
            .iter()
            .find_map(|card| card.collision_at(shared_position))
            .expect("collision at (1,0) must be found");
        let slot_ids: Vec<&str> = collision.iter().map(|slot| slot.as_str()).collect();
        assert!(slot_ids.contains(&"AHhb"), "collision must include AHhb");
        assert!(slot_ids.contains(&"AHds"), "collision must include AHds");
    }

    #[test]
    fn hotkey_collisions_empty_for_normalized_default() {
        let custom_keys = CustomKeys::from_text("");
        let layout = GridLayout::qwerty_grid();
        let unit_grids = UnitGrids::for_unit(paladin_id());
        let cards = unit_grids.hotkey_collisions(&custom_keys, layout);
        assert!(
            cards.iter().all(|card| card.is_empty()),
            "normalized default state must have no hotkey collisions for Paladin",
        );
    }

    #[test]
    fn hotkey_collisions_detects_two_abilities_with_same_hotkey() {
        let hotkey_q = Hotkey::from('Q');
        let first_cell = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
        let second_cell = GridCoordinate::new(ColumnIndex::One, RowIndex::Zero);
        let holy_light_binding = AbilityBinding::builder()
            .button_position(first_cell)
            .hotkey(hotkey_q)
            .build();
        let divine_shield_binding = AbilityBinding::builder()
            .button_position(second_cell)
            .hotkey(hotkey_q)
            .build();
        let mut custom_keys = CustomKeys::from_text("");
        custom_keys.put_ability("AHhb", holy_light_binding);
        custom_keys.put_ability("AHds", divine_shield_binding);
        let layout = GridLayout::qwerty_grid();
        let unit_grids = UnitGrids::for_unit(paladin_id());
        let cards = unit_grids.hotkey_collisions(&custom_keys, layout);
        assert!(
            cards.iter().any(|card| !card.is_empty()),
            "two Paladin abilities with hotkey Q must produce a hotkey collision",
        );
    }

    #[test]
    fn hotkey_collision_reports_colliding_token() {
        let hotkey_w = Hotkey::from('W');
        let first_cell = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
        let second_cell = GridCoordinate::new(ColumnIndex::Two, RowIndex::Zero);
        let holy_light_binding = AbilityBinding::builder()
            .button_position(first_cell)
            .hotkey(hotkey_w)
            .build();
        let divine_shield_binding = AbilityBinding::builder()
            .button_position(second_cell)
            .hotkey(hotkey_w)
            .build();
        let mut custom_keys = CustomKeys::from_text("");
        custom_keys.put_ability("AHhb", holy_light_binding);
        custom_keys.put_ability("AHds", divine_shield_binding);
        let layout = GridLayout::qwerty_grid();
        let unit_grids = UnitGrids::for_unit(paladin_id());
        let cards = unit_grids.hotkey_collisions(&custom_keys, layout);
        let w_position = GridCoordinate::new(ColumnIndex::One, RowIndex::Zero);
        let w_entry = cards
            .iter()
            .find_map(|card| card.collision_at(w_position))
            .expect("collision at W position must be found");
        assert_eq!(w_entry.slots().len(), 2);
        assert!(
            matches!(w_entry.token(), HotkeyToken::Letter(Letter::W)),
            "collision token must be W",
        );
    }

    #[test]
    fn hotkey_collisions_are_per_grid_not_cross_grid() {
        let hotkey_q = Hotkey::from('Q');
        let main_grid_cell = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
        let research_cell = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
        let holy_light_binding = AbilityBinding::builder()
            .button_position(main_grid_cell)
            .hotkey(hotkey_q)
            .build();
        let divine_shield_research = AbilityBinding::builder()
            .research_button_position(research_cell)
            .research_hotkey(hotkey_q)
            .build();
        let mut custom_keys = CustomKeys::from_text("");
        custom_keys.put_ability("AHhb", holy_light_binding);
        custom_keys.put_ability("AHds", divine_shield_research);
        let layout = GridLayout::qwerty_grid();
        let unit_grids = UnitGrids::for_unit(paladin_id());
        let cards = unit_grids.hotkey_collisions(&custom_keys, layout);
        let q_position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
        let cross_grid_collision = cards.iter().any(|card| {
            card.collision_at(q_position).is_some_and(|entry| {
                let slot_ids: Vec<&str> = entry.slots().iter().map(|slot| slot.as_str()).collect();
                slot_ids.contains(&"AHhb") && slot_ids.contains(&"AHds")
            })
        });
        assert!(
            !cross_grid_collision,
            "same hotkey in main grid and skill tree must not be reported as a collision",
        );
    }
}

#[cfg(test)]
mod cache_tests {
    use super::super::UnitGrids;
    use warcraft_database::WARCRAFT_DATABASE;

    fn first_unit_id() -> warcraft_api::WarcraftObjectId {
        // any real unit id from the DB; take the first command-card-bearing object
        *WARCRAFT_DATABASE
            .into_iter()
            .map(|(object_id, _object)| object_id)
            .next()
            .expect("database is non-empty")
    }

    #[test]
    fn for_unit_is_stable_across_calls() {
        let unit_id = first_unit_id();
        let first = UnitGrids::for_unit(unit_id);
        let second = UnitGrids::for_unit(unit_id);
        assert_eq!(first.grid_count(), second.grid_count());
        assert_eq!(first.unit_id(), second.unit_id());
    }

    #[test]
    fn cached_grids_match_a_fresh_build() {
        let unit_id = first_unit_id();
        let cached = UnitGrids::for_unit(unit_id);
        let fresh = UnitGrids::build_for_unit(unit_id); // uncached builder (added below)
        assert_eq!(cached.grid_count(), fresh.grid_count());
        assert_eq!(cached.unit_id(), fresh.unit_id());
        assert_eq!(cached, fresh);
    }
}
