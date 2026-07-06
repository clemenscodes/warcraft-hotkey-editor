#[cfg(test)]
mod unit_slots_tests {
    use super::super::*;
    use warcraft_database::WARCRAFT_DATABASE;

    #[test]
    fn command_card_for_unknown_unit_is_empty() {
        let unit_id = WarcraftObjectId::new("ZZZUnknown");
        let card = WARCRAFT_DATABASE.command_card(unit_id);
        assert!(card.is_empty());
    }

    #[test]
    fn command_card_for_peasant_is_non_empty() {
        let unit_id = WarcraftObjectId::new("hpea");
        let card = WARCRAFT_DATABASE.command_card(unit_id);
        assert!(!card.is_empty());
    }

    #[test]
    fn command_card_for_peasant_contains_attack() {
        let unit_id = WarcraftObjectId::new("hpea");
        let card = WARCRAFT_DATABASE.command_card(unit_id);
        let has_attack = card
            .filled_slots()
            .any(|slot| slot.id().value().eq_ignore_ascii_case("CmdAttack"));
        assert!(has_attack, "peasant command card must contain CmdAttack");
    }

    #[test]
    fn command_card_for_paladin_is_non_empty() {
        let unit_id = WarcraftObjectId::new("Hpal");
        let card = WARCRAFT_DATABASE.command_card(unit_id);
        assert!(!card.is_empty());
    }

    #[test]
    fn command_card_for_paladin_contains_hero_abilities() {
        let unit_id = WarcraftObjectId::new("Hpal");
        let card = WARCRAFT_DATABASE.command_card(unit_id);
        let ability_count = card
            .filled_slots()
            .filter(|slot| matches!(slot, GridSlotId::Ability(_)))
            .count();
        assert!(
            ability_count > 0,
            "paladin must have at least one ability slot"
        );
    }

    #[test]
    fn build_menu_for_non_worker_returns_none() {
        let unit_id = WarcraftObjectId::new("Hpal");
        let result = WARCRAFT_DATABASE.build_menu(unit_id);
        assert!(result.is_none());
    }

    #[test]
    fn build_menu_for_peasant_returns_some() {
        let unit_id = WarcraftObjectId::new("hpea");
        let result = WARCRAFT_DATABASE.build_menu(unit_id);
        assert!(result.is_some());
    }

    #[test]
    fn build_menu_for_peasant_is_non_empty() {
        let unit_id = WarcraftObjectId::new("hpea");
        let card = WARCRAFT_DATABASE.build_menu(unit_id).unwrap();
        assert!(!card.is_empty());
    }

    #[test]
    fn research_menu_for_non_hero_returns_none() {
        let unit_id = WarcraftObjectId::new("hpea");
        let result = WARCRAFT_DATABASE.research_menu(unit_id);
        assert!(result.is_none());
    }

    #[test]
    fn research_menu_for_paladin_returns_some() {
        let unit_id = WarcraftObjectId::new("Hpal");
        let result = WARCRAFT_DATABASE.research_menu(unit_id);
        assert!(result.is_some());
    }

    #[test]
    fn uprooted_menu_for_non_uprootable_building_returns_none() {
        let unit_id = WarcraftObjectId::new("htow");
        let result = WARCRAFT_DATABASE.uprooted_menu(unit_id);
        assert!(result.is_none());
    }

    #[test]
    fn uprooted_menu_for_tree_of_life_returns_some() {
        let unit_id = WarcraftObjectId::new("etol");
        let result = WARCRAFT_DATABASE.uprooted_menu(unit_id);
        assert!(result.is_some());
    }

    #[test]
    fn uprooted_menu_for_tree_of_life_contains_movement_commands() {
        let unit_id = WarcraftObjectId::new("etol");
        let card = WARCRAFT_DATABASE.uprooted_menu(unit_id).unwrap();
        let has_move = card
            .filled_slots()
            .any(|slot| slot.id().value().eq_ignore_ascii_case("CmdMove"));
        assert!(has_move, "uprooted Tree of Life must have CmdMove");
    }

    #[test]
    fn corrupted_tree_rooted_card_excludes_eat_tree() {
        let unit_id = WarcraftObjectId::new("ncta");
        let rooted_card = WARCRAFT_DATABASE.command_card(unit_id);
        let has_eat_tree = rooted_card
            .filled_slots()
            .any(|slot| slot.id().value().eq_ignore_ascii_case("Aeat"));
        assert!(
            !has_eat_tree,
            "rooted Corrupted Tree of Ages must not contain Eat Tree (it is uprooted-only)",
        );
    }

    #[test]
    fn tree_of_life_rooted_card_contains_entangle_gold_mine() {
        let unit_id = WarcraftObjectId::new("etol");
        let card = WARCRAFT_DATABASE.command_card(unit_id);
        let has_entangle = card
            .filled_slots()
            .any(|slot| slot.id().value().eq_ignore_ascii_case("Aent"));
        assert!(
            has_entangle,
            "rooted Tree of Life must contain Entangle Gold Mine (Aent)",
        );
    }

    #[test]
    fn tree_of_life_uprooted_menu_excludes_entangle_gold_mine() {
        let unit_id = WarcraftObjectId::new("etol");
        let card = WARCRAFT_DATABASE
            .uprooted_menu(unit_id)
            .expect("Tree of Life must have an uprooted form");
        let has_entangle = card
            .filled_slots()
            .any(|slot| slot.id().value().eq_ignore_ascii_case("Aent"));
        assert!(
            !has_entangle,
            "uprooted Tree of Life must not contain Entangle Gold Mine (it is rooted-only)",
        );
    }

    #[test]
    fn corrupted_tree_uprooted_menu_contains_eat_tree() {
        let unit_id = WarcraftObjectId::new("ncta");
        let uprooted_card = WARCRAFT_DATABASE
            .uprooted_menu(unit_id)
            .expect("Corrupted Tree of Ages must have an uprooted form");
        let has_eat_tree = uprooted_card
            .filled_slots()
            .any(|slot| slot.id().value().eq_ignore_ascii_case("Aeat"));
        assert!(
            has_eat_tree,
            "uprooted Corrupted Tree of Ages must contain Eat Tree"
        );
    }

    #[test]
    fn all_unit_ids_is_non_empty() {
        let count = WARCRAFT_DATABASE.all_unit_ids().count();
        assert!(count > 0);
    }

    #[test]
    fn all_unit_ids_contains_peasant() {
        let has_peasant = WARCRAFT_DATABASE
            .all_unit_ids()
            .any(|id| id.value().eq_ignore_ascii_case("hpea"));
        assert!(has_peasant);
    }

    #[test]
    fn goblin_lab_command_card_shows_all_three_sell_units() {
        let unit_id = WarcraftObjectId::new("ngad");
        let card = WARCRAFT_DATABASE.command_card(unit_id);
        let has_sapper = card
            .filled_slots()
            .any(|slot| slot.id().value().eq_ignore_ascii_case("ngsp"));
        let has_zeppelin = card
            .filled_slots()
            .any(|slot| slot.id().value().eq_ignore_ascii_case("nzep"));
        let has_shredder = card
            .filled_slots()
            .any(|slot| slot.id().value().eq_ignore_ascii_case("ngir"));
        assert!(
            has_sapper,
            "Goblin Lab command card must contain Goblin Sapper (ngsp)"
        );
        assert!(
            has_zeppelin,
            "Goblin Lab command card must contain Goblin Zeppelin (nzep)",
        );
        assert!(
            has_shredder,
            "Goblin Lab command card must contain Goblin Shredder (ngir)",
        );
    }

    #[test]
    fn goblin_merchant_command_card_shows_all_eleven_sell_items() {
        let unit_id = WarcraftObjectId::new("ngme");
        let card = WARCRAFT_DATABASE.command_card(unit_id);
        let sell_item_ids = [
            "stwp", "bspd", "dust", "tret", "prvt", "cnob", "stel", "pnvl", "shea", "spro", "pinv",
        ];
        for sell_item_id in sell_item_ids {
            let present = card
                .filled_slots()
                .any(|slot| slot.id().value().eq_ignore_ascii_case(sell_item_id));
            assert!(
                present,
                "Goblin Merchant command card must contain sell item {sell_item_id}",
            );
        }
    }

    #[test]
    fn gargoyle_command_card_contains_prioritize() {
        let unit_id = WarcraftObjectId::new("ugar");
        let card = WARCRAFT_DATABASE.command_card(unit_id);
        let has_prioritize = card
            .filled_slots()
            .any(|slot| slot.id().value().eq_ignore_ascii_case("Aatp"));
        assert!(
            has_prioritize,
            "Gargoyle command card must contain Prioritize (Aatp)"
        );
    }

    #[test]
    fn phoenix_command_card_hides_phoenix_fire() {
        let unit_id = WarcraftObjectId::new("hphx");
        let card = WARCRAFT_DATABASE.command_card(unit_id);
        let has_phoenix_fire = card
            .filled_slots()
            .any(|slot| slot.id().value().eq_ignore_ascii_case("Apxf"));
        assert!(
            !has_phoenix_fire,
            "Phoenix (hphx) command card must hide Phoenix Fire (Apxf); the in-game client never shows it",
        );
        let has_phoenix_summon = card
            .filled_slots()
            .any(|slot| slot.id().value().eq_ignore_ascii_case("Ahpe"));
        assert!(
            has_phoenix_summon,
            "Phoenix (hphx) command card must still contain its remaining ability Ahpe",
        );
    }

    #[test]
    fn entangled_gold_mine_command_card_hides_load() {
        let unit_id = WarcraftObjectId::new("egol");
        let card = WARCRAFT_DATABASE.command_card(unit_id);
        let has_load = card
            .filled_slots()
            .any(|slot| slot.id().value().eq_ignore_ascii_case("Aenc"));
        assert!(
            !has_load,
            "Entangled Gold Mine (egol) command card must hide Load (Aenc); the in-game client only shows Unload All when a Wisp is inside",
        );
        let has_unload_all = card
            .filled_slots()
            .any(|slot| slot.id().value().eq_ignore_ascii_case("Adri"));
        assert!(
            has_unload_all,
            "Entangled Gold Mine (egol) command card must still contain Unload All (Adri)",
        );
    }

    #[test]
    fn forest_troll_high_priest_command_card_contains_exactly_one_abolish_magic() {
        let unit_id = WarcraftObjectId::new("nfsh");
        let card = WARCRAFT_DATABASE.command_card(unit_id);
        let abolish_count = card
            .filled_slots()
            .filter(|slot| {
                slot.id().value().eq_ignore_ascii_case("ACdm")
                    || slot.id().value().eq_ignore_ascii_case("ACd2")
            })
            .count();
        assert_eq!(
            abolish_count, 1,
            "Forest Troll High Priest (nfsh) must have exactly one Abolish Magic ability, not both ACdm and ACd2",
        );
    }

    #[test]
    fn forest_troll_high_priest_command_card_uses_competitive_abolish_magic() {
        let unit_id = WarcraftObjectId::new("nfsh");
        let card = WARCRAFT_DATABASE.command_card(unit_id);
        let has_acd2 = card
            .filled_slots()
            .any(|slot| slot.id().value().eq_ignore_ascii_case("ACd2"));
        let has_acdm = card
            .filled_slots()
            .any(|slot| slot.id().value().eq_ignore_ascii_case("ACdm"));
        assert!(
            has_acd2,
            "Forest Troll High Priest (nfsh) must have ACd2 (competitive balance Abolish Magic)",
        );
        assert!(
            !has_acdm,
            "Forest Troll High Priest (nfsh) must not have ACdm (alternative mode variant)",
        );
    }

    #[test]
    fn ice_troll_high_priest_command_card_contains_exactly_one_abolish_magic() {
        let unit_id = WarcraftObjectId::new("nith");
        let card = WARCRAFT_DATABASE.command_card(unit_id);
        let abolish_count = card
            .filled_slots()
            .filter(|slot| {
                slot.id().value().eq_ignore_ascii_case("ACdm")
                    || slot.id().value().eq_ignore_ascii_case("ACd2")
            })
            .count();
        assert_eq!(
            abolish_count, 1,
            "Ice Troll High Priest (nith) must have exactly one Abolish Magic ability, not both ACdm and ACd2",
        );
    }

    #[test]
    fn ice_troll_high_priest_command_card_contains_exactly_one_frost_armor() {
        let unit_id = WarcraftObjectId::new("nith");
        let card = WARCRAFT_DATABASE.command_card(unit_id);
        let frost_armor_count = card
            .filled_slots()
            .filter(|slot| {
                slot.id().value().eq_ignore_ascii_case("ACfu")
                    || slot.id().value().eq_ignore_ascii_case("ACf2")
            })
            .count();
        assert_eq!(
            frost_armor_count, 1,
            "Ice Troll High Priest (nith) must have exactly one Frost Armor ability, not both ACfu and ACf2",
        );
    }

    #[test]
    fn human_main_hall_tiers_show_both_militia_buttons() {
        for hall_id in ["htow", "hkee", "hcas"] {
            let unit_id = WarcraftObjectId::new(hall_id);
            let card = WARCRAFT_DATABASE.command_card(unit_id);
            let has_call_to_arms = card
                .filled_slots()
                .any(|slot| matches!(slot, GridSlotId::Ability(id) if id.value() == "Amic"));
            let has_back_to_work = card
                .filled_slots()
                .any(|slot| matches!(slot, GridSlotId::AbilityOff(id) if id.value() == "Amic"));
            assert!(
                has_call_to_arms,
                "{hall_id} must show Call To Arms (on-state Amic)",
            );
            assert!(
                has_back_to_work,
                "{hall_id} must show Back To Work (off-state Amic)",
            );
        }
    }

    #[test]
    fn peasant_shows_single_militia_button() {
        let unit_id = WarcraftObjectId::new("hpea");
        let card = WARCRAFT_DATABASE.command_card(unit_id);
        let militia_slot_count = card
            .filled_slots()
            .filter(|slot| slot.id().value() == "Amil")
            .count();
        assert_eq!(
            militia_slot_count, 1,
            "Peasant must show exactly one militia button (no off-state second button)",
        );
        let has_off_state = card
            .filled_slots()
            .any(|slot| matches!(slot, GridSlotId::AbilityOff(id) if id.value() == "Amil"));
        assert!(
            !has_off_state,
            "Peasant must not carry an off-state militia button"
        );
    }

    #[test]
    fn orc_barracks_command_card_shows_demolisher() {
        let unit_id = WarcraftObjectId::new("obar");
        let card = WARCRAFT_DATABASE.command_card(unit_id);
        let has_demolisher = card.filled_slots().any(|slot| slot.id().value() == "ocat");
        assert!(
            has_demolisher,
            "Orc Barracks (obar) command card must contain the Demolisher (ocat)",
        );
        let has_berserker = card.filled_slots().any(|slot| slot.id().value() == "otbk");
        assert!(
            !has_berserker,
            "Orc Barracks (obar) must keep the Berserker (otbk) collapsed behind the Headhunter",
        );
    }

    #[test]
    fn orc_barracks_train_upgrades_exclude_demolisher() {
        let unit_id = WarcraftObjectId::new("obar");
        let upgrades = WARCRAFT_DATABASE.train_unit_upgrades(unit_id);
        let headhunter = WarcraftObjectId::new("ohun");
        let grunt = WarcraftObjectId::new("ogru");
        assert_eq!(
            upgrades.get(&headhunter).map(|id| id.value()),
            Some("otbk"),
            "Headhunter (ohun) must be recorded as upgrading to the Berserker (otbk)",
        );
        assert!(
            !upgrades.contains_key(&grunt),
            "Grunt (ogru) must not be modelled as upgrading to the Demolisher",
        );
    }

    #[test]
    fn necropolis_upgraded_tiers_show_backpack_research() {
        for hall_id in ["unp1", "unp2"] {
            let unit_id = WarcraftObjectId::new(hall_id);
            let card = WARCRAFT_DATABASE.command_card(unit_id);
            let has_backpack = card.filled_slots().any(|slot| slot.id().value() == "Rupm");
            assert!(
                has_backpack,
                "{hall_id} command card must contain the Backpack research (Rupm)",
            );
        }
    }

    #[test]
    fn ice_troll_high_priest_command_card_uses_competitive_abilities() {
        let unit_id = WarcraftObjectId::new("nith");
        let card = WARCRAFT_DATABASE.command_card(unit_id);
        let has_acd2 = card
            .filled_slots()
            .any(|slot| slot.id().value().eq_ignore_ascii_case("ACd2"));
        let has_acf2 = card
            .filled_slots()
            .any(|slot| slot.id().value().eq_ignore_ascii_case("ACf2"));
        let has_acdm = card
            .filled_slots()
            .any(|slot| slot.id().value().eq_ignore_ascii_case("ACdm"));
        let has_acfu = card
            .filled_slots()
            .any(|slot| slot.id().value().eq_ignore_ascii_case("ACfu"));
        assert!(
            has_acd2,
            "Ice Troll High Priest (nith) must have ACd2 (competitive balance Abolish Magic)",
        );
        assert!(
            has_acf2,
            "Ice Troll High Priest (nith) must have ACf2 (competitive balance Frost Armor)",
        );
        assert!(
            !has_acdm,
            "Ice Troll High Priest (nith) must not have ACdm (alternative mode variant)",
        );
        assert!(
            !has_acfu,
            "Ice Troll High Priest (nith) must not have ACfu (alternative mode variant)",
        );
    }
}
