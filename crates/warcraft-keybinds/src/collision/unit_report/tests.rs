#[cfg(test)]
mod unit_collision_report_tests {
    use super::super::*;
    use crate::identity::slot::GridSlotId;
    use crate::model::{AbilityBinding, ColumnIndex, GridCoordinate, Hotkey, RowIndex};
    use crate::unit::grids::{GridRole, HotkeyCollisionCardBuilder, PositionCollisionCardBuilder};
    use warcraft_api::WarcraftObjectId;

    #[derive(Clone, Debug, PartialEq, Default)]
    struct UnitCollisionReportBuilder {
        entries: Vec<UnitCollisionEntry>,
    }

    impl UnitCollisionReportBuilder {
        fn new() -> Self {
            Self {
                entries: Vec::new(),
            }
        }

        fn entry(mut self, entry: UnitCollisionEntry) -> Self {
            self.entries.push(entry);
            self
        }

        fn build(self) -> UnitCollisionReport {
            UnitCollisionReport {
                entries: self.entries,
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct UnitCollisionEntryBuilder {
        unit_id: WarcraftObjectId,
        unit_name: &'static str,
        position_cards: [PositionCollisionCard; 2],
        hotkey_cards: [HotkeyCollisionCard; 2],
    }

    impl UnitCollisionEntryBuilder {
        fn new(
            unit_id: &'static str,
            unit_name: &'static str,
            empty_pos: PositionCollisionCard,
            empty_hot: HotkeyCollisionCard,
        ) -> Self {
            Self {
                unit_id: WarcraftObjectId::new(unit_id),
                unit_name,
                position_cards: [empty_pos, empty_pos],
                hotkey_cards: [empty_hot, empty_hot],
            }
        }

        fn main_position_card(mut self, card: PositionCollisionCard) -> Self {
            self.position_cards[0] = card;
            self
        }

        fn main_hotkey_card(mut self, card: HotkeyCollisionCard) -> Self {
            self.hotkey_cards[0] = card;
            self
        }

        fn secondary_hotkey_card(mut self, card: HotkeyCollisionCard) -> Self {
            self.hotkey_cards[1] = card;
            self
        }

        fn build(self) -> UnitCollisionEntry {
            UnitCollisionEntry {
                unit_id: self.unit_id,
                unit_name: self.unit_name,
                position_cards: self.position_cards,
                hotkey_cards: self.hotkey_cards,
            }
        }
    }

    fn paladin_id() -> WarcraftObjectId {
        WarcraftObjectId::new("Hpal")
    }

    #[test]
    fn paladin_has_no_collisions_in_normalized_default() {
        let custom_keys = CustomKeys::from_text("");
        let layout = GridLayout::qwerty_grid();
        let report = UnitCollisionReport::compute(&custom_keys, layout);
        let paladin_filtered = report.for_unit("Hpal");
        assert!(
            paladin_filtered.is_empty(),
            "Paladin must have no collisions in the normalized default on QWERTY",
        );
    }

    #[test]
    fn detects_position_collision_across_all_units() {
        let shared_position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Two);
        let holy_light_binding = AbilityBinding::builder()
            .button_position(shared_position)
            .build();
        let divine_shield_binding = AbilityBinding::builder()
            .button_position(shared_position)
            .build();
        let mut custom_keys = CustomKeys::from_text("");
        custom_keys.put_ability("AHhb", holy_light_binding);
        custom_keys.put_ability("AHds", divine_shield_binding);
        let layout = GridLayout::qwerty_grid();
        let report = UnitCollisionReport::compute(&custom_keys, layout);
        let paladin_entry = report
            .entries()
            .iter()
            .find(|entry| entry.unit_id() == paladin_id());
        assert!(
            paladin_entry.is_some(),
            "Paladin must appear in collision report when two abilities share a position",
        );
        let entry = paladin_entry.unwrap();
        assert!(
            entry.position_cards().iter().any(|card| !card.is_empty()),
            "position collision card must be present for Paladin",
        );
        let has_collision_at_position = entry
            .position_cards()
            .iter()
            .any(|card| card.collision_at(shared_position).is_some());
        assert!(
            has_collision_at_position,
            "collision must be reported at the shared position",
        );
    }

    #[test]
    fn detects_hotkey_collision_across_all_units() {
        let hotkey_q = Hotkey::Letter('Q');
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
        let report = UnitCollisionReport::compute(&custom_keys, layout);
        let paladin_entry = report
            .entries()
            .iter()
            .find(|entry| entry.unit_id() == paladin_id());
        assert!(
            paladin_entry.is_some(),
            "Paladin must appear in collision report when two abilities share a hotkey",
        );
        let entry = paladin_entry.unwrap();
        assert!(
            entry.hotkey_cards().iter().any(|card| !card.is_empty()),
            "hotkey collision card must be present for Paladin",
        );
    }

    #[test]
    fn for_unit_filters_to_matching_unit() {
        let shared_position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Two);
        let holy_light_binding = AbilityBinding::builder()
            .button_position(shared_position)
            .build();
        let divine_shield_binding = AbilityBinding::builder()
            .button_position(shared_position)
            .build();
        let mut custom_keys = CustomKeys::from_text("");
        custom_keys.put_ability("AHhb", holy_light_binding);
        custom_keys.put_ability("AHds", divine_shield_binding);
        let layout = GridLayout::qwerty_grid();
        let report = UnitCollisionReport::compute(&custom_keys, layout);
        let filtered = report.for_unit("Hpal");
        assert!(
            filtered
                .entries()
                .iter()
                .all(|entry| entry.unit_id() == paladin_id()),
            "for_unit must return only entries for the requested unit",
        );
    }

    #[test]
    fn for_unit_returns_empty_for_unknown_unit() {
        let custom_keys = CustomKeys::from_text("");
        let layout = GridLayout::qwerty_grid();
        let report = UnitCollisionReport::compute(&custom_keys, layout);
        let filtered = report.for_unit("ZZZZ");
        assert!(
            filtered.is_empty(),
            "unknown unit id must yield empty report"
        );
    }

    #[test]
    fn entries_are_sorted_by_unit_name() {
        let custom_keys = CustomKeys::from_text("");
        let layout = GridLayout::qwerty_grid();
        let report = UnitCollisionReport::compute(&custom_keys, layout);
        let names: Vec<&str> = report
            .entries()
            .iter()
            .map(|entry| entry.unit_name())
            .collect();
        let mut sorted_names = names.clone();
        sorted_names.sort();
        assert_eq!(names, sorted_names, "entries must be sorted by unit name");
    }

    #[test]
    fn default_customkeys_collision_matches_expected() {
        let join_handle = std::thread::Builder::new()
            .stack_size(32 * 1024 * 1024)
            .spawn(run_default_collision_check)
            .unwrap();
        join_handle.join().unwrap();
    }

    #[test]
    #[ignore = "code generator — run with --ignored to refresh the snapshot in run_default_collision_check after a db.rs regeneration"]
    fn dump_default_collision_report_as_builder_code() {
        let join_handle = std::thread::Builder::new()
            .stack_size(32 * 1024 * 1024)
            .spawn(dump_default_collision_report)
            .unwrap();
        join_handle.join().unwrap();
    }

    fn dump_default_collision_report() {
        use crate::identity::hotkey_token::HotkeyToken;
        use crate::unit::grids::PositionCollisionCard;
        let default_text = crate::DEFAULT_CUSTOM_KEYS;
        let custom_keys = CustomKeys::from_text(default_text);
        let layout = GridLayout::qwerty_grid();
        let report = UnitCollisionReport::compute(&custom_keys, layout);
        for entry in report.entries() {
            let unit_id = entry.unit_id().value();
            let unit_name = entry.unit_name();
            let position_cards = entry.position_cards();
            let hotkey_cards = entry.hotkey_cards();
            let main_position = position_cards[0];
            let secondary_position = position_cards[1];
            let main_hotkey = hotkey_cards[0];
            let secondary_hotkey = hotkey_cards[1];
            println!("// {unit_id} ({unit_name})");
            println!("let entry = {{");
            if !main_position.is_empty() {
                emit_position_card_builder("main_pos", &main_position);
            }
            if !secondary_position.is_empty() {
                emit_position_card_builder("secondary_pos", &secondary_position);
            }
            if !main_hotkey.is_empty() {
                emit_hotkey_card_builder("main_hot", &main_hotkey);
            }
            if !secondary_hotkey.is_empty() {
                emit_hotkey_card_builder("secondary_hot", &secondary_hotkey);
            }
            println!(
                "    let eb = UnitCollisionEntryBuilder::new(\"{unit_id}\", \"{unit_name}\", empty_pos, empty_hot);",
            );
            if !main_position.is_empty() {
                println!("    let eb = eb.main_position_card(main_pos);");
            }
            if !main_hotkey.is_empty() {
                println!("    let eb = eb.main_hotkey_card(main_hot);");
            }
            if !secondary_hotkey.is_empty() {
                println!("    let eb = eb.secondary_hotkey_card(secondary_hot);");
            }
            println!("    eb.build()");
            println!("}};");
            println!("builder = builder.entry(entry);");
            println!();
        }

        fn role_expr(role: crate::unit::grids::GridRole) -> &'static str {
            match role {
                crate::unit::grids::GridRole::MainCommand => "GridRole::MainCommand",
                crate::unit::grids::GridRole::HeroSkillTree => "GridRole::HeroSkillTree",
                crate::unit::grids::GridRole::BuildMenu => "GridRole::BuildMenu",
                crate::unit::grids::GridRole::UprootedForm => "GridRole::UprootedForm",
            }
        }

        fn emit_position_card_builder(name: &str, card: &PositionCollisionCard) {
            let role_text = role_expr(card.role());
            println!("    let {name} = PositionCollisionCardBuilder::new({role_text})");
            for (position, slots) in card.into_iter() {
                let column_u8 = u8::from(position.column());
                let row_u8 = u8::from(position.row());
                let slot_idents: Vec<String> = slots
                    .iter()
                    .map(|slot| format!("GridSlotId::ability(\"{}\")", slot.as_str()))
                    .collect();
                let slots_array = slot_idents.join(", ");
                println!("        .collision_at({column_u8}, {row_u8}, &[{slots_array}])",);
            }
            println!("        .build();");
        }

        fn emit_hotkey_card_builder(name: &str, card: &crate::unit::grids::HotkeyCollisionCard) {
            let role_text = role_expr(card.role());
            println!("    let {name} = HotkeyCollisionCardBuilder::new({role_text}, layout)",);
            for (_position, cell) in card.into_iter() {
                let token = cell.token();
                let letter_char = match token {
                    HotkeyToken::Letter(letter) => letter.character(),
                    _ => continue,
                };
                let slot_idents: Vec<String> = cell
                    .slots()
                    .iter()
                    .map(|slot| format!("GridSlotId::ability(\"{}\")", slot.as_str()))
                    .collect();
                let slots_array = slot_idents.join(", ");
                println!("        .collision('{letter_char}', &[{slots_array}])");
            }
            println!("        .build();");
        }
    }

    fn run_default_collision_check() {
        let default_text = crate::DEFAULT_CUSTOM_KEYS;
        let custom_keys = CustomKeys::from_text(default_text);
        let layout = GridLayout::qwerty_grid();
        let report = UnitCollisionReport::compute(&custom_keys, layout);
        let empty_pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
        let empty_pos = empty_pos_builder.build();
        let empty_hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
        let empty_hot = empty_hot_builder.build();
        let mut builder = UnitCollisionReportBuilder::new();
        // nahy (Ancient Hydra)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    1,
                    2,
                    &[GridSlotId::ability("Awrh"), GridSlotId::ability("Aspo")],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'X',
                    &[GridSlotId::ability("Awrh"), GridSlotId::ability("Aspo")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nahy", "Ancient Hydra", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nsqa (Ancient Sasquatch)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    1,
                    2,
                    &[GridSlotId::ability("ACfr"), GridSlotId::ability("ACtc")],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'X',
                    &[GridSlotId::ability("ACfr"), GridSlotId::ability("ACtc")],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("nsqa", "Ancient Sasquatch", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // eden (Ancient of Wonders)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    3,
                    2,
                    &[GridSlotId::ability("Anei"), GridSlotId::ability("Aro1")],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("eden", "Ancient of Wonders", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // espv (Avatar of Vengeance)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'V',
                    &[GridSlotId::ability("Avng"), GridSlotId::ability("ACrk")],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("espv", "Avatar of Vengeance", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // obar (Barracks)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    0,
                    &[GridSlotId::ability("ogru"), GridSlotId::ability("ocat")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("obar", "Barracks", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // Orex (Beastmaster)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'S',
                    &[GridSlotId::ability("CmdStop"), GridSlotId::ability("Aamk")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Orex", "Beastmaster", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // unp2 (Black Citadel)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    3,
                    0,
                    &[
                        GridSlotId::ability("Rupm"),
                        GridSlotId::ability("CmdAttack"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("unp2", "Black Citadel", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nadr (Blue Dragon)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("Afrc"), GridSlotId::ability("ACdv")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nadr", "Blue Dragon", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nbdo (Blue Dragonspawn Overseer)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    2,
                    2,
                    &[GridSlotId::ability("ACav"), GridSlotId::ability("ACev")],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[GridSlotId::ability("ACav"), GridSlotId::ability("ACev")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new(
                "nbdo",
                "Blue Dragonspawn Overseer",
                empty_pos,
                empty_hot,
            );
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Nsjs (Brewmaster)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'S',
                    &[GridSlotId::ability("CmdStop"), GridSlotId::ability("Aamk")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Nsjs", "Brewmaster", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nsbm (Brood Mother)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("ACen"), GridSlotId::ability("ACvs")],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'Z',
                    &[GridSlotId::ability("ACen"), GridSlotId::ability("ACvs")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nsbm", "Brood Mother", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // ncks (Centaur Sorcerer)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("ACdm"), GridSlotId::ability("ACbl")],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("ncks", "Centaur Sorcerer", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nchp (Chaplain)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("Adsm"), GridSlotId::ability("Anh2")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nchp", "Chaplain", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // ndth (Dark Troll High Priest)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("ACdm"), GridSlotId::ability("Anh2")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new(
                "ndth",
                "Dark Troll High Priest",
                empty_pos,
                empty_hot,
            );
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nwzd (Dark Wizard)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    2,
                    2,
                    &[GridSlotId::ability("ACpy"), GridSlotId::ability("ACba")],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[GridSlotId::ability("ACpy"), GridSlotId::ability("ACba")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nwzd", "Dark Wizard", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Udea (Death Knight)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[GridSlotId::ability("AUdc"), GridSlotId::ability("AUau")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Udea", "Death Knight", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Uear (Death Knight)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[GridSlotId::ability("AUdc"), GridSlotId::ability("AUau")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Uear", "Death Knight", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nrvd (Death Revenant)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("ACdc"), GridSlotId::ability("ACrd")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nrvd", "Death Revenant", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // Ecen (Demigod)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'S',
                    &[GridSlotId::ability("CmdStop"), GridSlotId::ability("SCc1")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Ecen", "Demigod", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Eevi (Demon Hunter)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    2,
                    0,
                    &[
                        GridSlotId::ability("CmdHoldPos"),
                        GridSlotId::ability("ANcl"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Eevi", "Demon Hunter", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // Eevm (Demon Hunter)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    2,
                    0,
                    &[
                        GridSlotId::ability("CmdHoldPos"),
                        GridSlotId::ability("ANcl"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Eevm", "Demon Hunter", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // ubsp (Destroyer)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("Aabs"), GridSlotId::ability("Advm")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("ubsp", "Destroyer", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nba2 (Doom Guard)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'F',
                    &[GridSlotId::ability("ACsk"), GridSlotId::ability("ACrf")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nba2", "Doom Guard", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nbal (Doom Guard)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'F',
                    &[GridSlotId::ability("ACsk"), GridSlotId::ability("ACrf")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nbal", "Doom Guard", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // ndh3 (Draenei Barracks)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    2,
                    0,
                    &[GridSlotId::ability("ndrt"), GridSlotId::ability("ndrn")],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'E',
                    &[GridSlotId::ability("ndrt"), GridSlotId::ability("ndrn")],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("ndh3", "Draenei Barracks", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // ndrs (Draenei Seer)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("AChv"), GridSlotId::ability("ACsw")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("ndrs", "Draenei Seer", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // Ubal (Dreadlord)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'E',
                    &[GridSlotId::ability("AUsl"), GridSlotId::ability("AOeq")],
                )
                .build();
            let secondary_hot = HotkeyCollisionCardBuilder::new(GridRole::HeroSkillTree, layout)
                .collision(
                    'E',
                    &[GridSlotId::ability("AUsl"), GridSlotId::ability("AOeq")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Ubal", "Dreadlord", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            let eb = eb.secondary_hotkey_card(secondary_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Udre (Dreadlord)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[GridSlotId::ability("AUcs"), GridSlotId::ability("AUav")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Udre", "Dreadlord", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Udth (Dreadlord)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'D',
                    &[GridSlotId::ability("AEsh"), GridSlotId::ability("AUdd")],
                )
                .build();
            let secondary_hot = HotkeyCollisionCardBuilder::new(GridRole::HeroSkillTree, layout)
                .collision(
                    'D',
                    &[GridSlotId::ability("AEsh"), GridSlotId::ability("AUdd")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Udth", "Dreadlord", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            let eb = eb.secondary_hotkey_card(secondary_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Umal (Dreadlord)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[GridSlotId::ability("AUcs"), GridSlotId::ability("ANdc")],
                )
                .build();
            let secondary_hot = HotkeyCollisionCardBuilder::new(GridRole::HeroSkillTree, layout)
                .collision(
                    'E',
                    &[GridSlotId::ability("AUsl"), GridSlotId::ability("ANdc")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Umal", "Dreadlord", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            let eb = eb.secondary_hotkey_card(secondary_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Utic (Dreadlord)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    1,
                    2,
                    &[GridSlotId::ability("ANrc"), GridSlotId::ability("AUsl")],
                )
                .collision_at(
                    3,
                    2,
                    &[GridSlotId::ability("AUin"), GridSlotId::ability("ANfd")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Utic", "Dreadlord", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // Uvng (Dreadlord)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[GridSlotId::ability("AUcs"), GridSlotId::ability("AUav")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Uvng", "Dreadlord", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // npn3 (Earth)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("ANta"), GridSlotId::ability("ACpv")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("npn3", "Earth", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // npn6 (Earth)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("ANta"), GridSlotId::ability("ACpv")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("npn6", "Earth", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nvde (Elder Voidwalker)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("ACde"), GridSlotId::ability("ACfl")],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'Z',
                    &[GridSlotId::ability("ACde"), GridSlotId::ability("ACfl")],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("nvde", "Elder Voidwalker", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nerd (Eredar Diabolist)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("ANfb"), GridSlotId::ability("ACpa")],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'Z',
                    &[GridSlotId::ability("ANfb"), GridSlotId::ability("ACpa")],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("nerd", "Eredar Diabolist", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nfot (Faceless One Terror)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    1,
                    2,
                    &[GridSlotId::ability("ACmf"), GridSlotId::ability("ACsl")],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'X',
                    &[GridSlotId::ability("ACmf"), GridSlotId::ability("ACsl")],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("nfot", "Faceless One Terror", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nfor (Faceless One Trickster)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("ACpu"), GridSlotId::ability("ACcs")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new(
                "nfor",
                "Faceless One Trickster",
                empty_pos,
                empty_hot,
            );
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // npfm (Fel Ravager)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("ACde"), GridSlotId::ability("ACbk")],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'Z',
                    &[GridSlotId::ability("ACde"), GridSlotId::ability("ACbk")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("npfm", "Fel Ravager", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nfsp (Forest Troll Shadow Priest)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("ACdm"), GridSlotId::ability("Anh1")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new(
                "nfsp",
                "Forest Troll Shadow Priest",
                empty_pos,
                empty_hot,
            );
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nfgo (Forgotten One)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("ACtn"), GridSlotId::ability("ACfb")],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'Z',
                    &[GridSlotId::ability("ACtn"), GridSlotId::ability("ACfb")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nfgo", "Forgotten One", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // ugar (Gargoyle)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    0,
                    &[GridSlotId::ability("CmdMove"), GridSlotId::ability("Aatp")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("ugar", "Gargoyle", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nsgt (Giant Spider)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("ACen"), GridSlotId::ability("ACvs")],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'Z',
                    &[GridSlotId::ability("ACen"), GridSlotId::ability("ACvs")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nsgt", "Giant Spider", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // ngow (Gnoll Warlord)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("ACro"), GridSlotId::ability("ACbl")],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'Z',
                    &[GridSlotId::ability("ACro"), GridSlotId::ability("ACbl")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("ngow", "Gnoll Warlord", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // ngad (Goblin Laboratory)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    0,
                    &[
                        GridSlotId::ability("ngsp"),
                        GridSlotId::ability("nzep"),
                        GridSlotId::ability("Andt"),
                    ],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("ngad", "Goblin Laboratory", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // ngme (Goblin Merchant)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'E',
                    &[GridSlotId::ability("bspd"), GridSlotId::ability("stel")],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("ngme", "Goblin Merchant", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // unp1 (Halls of the Dead)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    3,
                    0,
                    &[
                        GridSlotId::ability("Rupm"),
                        GridSlotId::ability("CmdAttack"),
                    ],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("unp1", "Halls of the Dead", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nhhr (Heretic)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("ACca"), GridSlotId::ability("ACrd")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nhhr", "Heretic", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nheb (High Elven Barracks)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'D',
                    &[GridSlotId::ability("nws1"), GridSlotId::ability("Rhde")],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("nheb", "High Elven Barracks", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nith (Ice Troll High Priest)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    1,
                    2,
                    &[GridSlotId::ability("ACd2"), GridSlotId::ability("ACf2")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new(
                "nith",
                "Ice Troll High Priest",
                empty_pos,
                empty_hot,
            );
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nkog (Kobold Geomancer)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("ACdm"), GridSlotId::ability("ACsw")],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("nkog", "Kobold Geomancer", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nrvl (Lightning Revenant)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("ACcl"), GridSlotId::ability("ACpu")],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("nrvl", "Lightning Revenant", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // umtw (Meat Wagon)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[GridSlotId::ability("Amel"), GridSlotId::ability("Apts")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("umtw", "Meat Wagon", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nmr4 (Mercenary Camp)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    0,
                    &[GridSlotId::ability("ncea"), GridSlotId::ability("ncen")],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'W',
                    &[GridSlotId::ability("nhrw"), GridSlotId::ability("nqbh")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nmr4", "Mercenary Camp", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nmrd (Mercenary Camp)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    0,
                    &[
                        GridSlotId::ability("ntkh"),
                        GridSlotId::ability("nbdw"),
                        GridSlotId::ability("nubw"),
                    ],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nmrd", "Mercenary Camp", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // Hmbr (Mountain King)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[GridSlotId::ability("AHtc"), GridSlotId::ability("AHbh")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Hmbr", "Mountain King", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Hmkg (Mountain King)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[GridSlotId::ability("AHtc"), GridSlotId::ability("AHbh")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Hmkg", "Mountain King", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nmsn (Mur'gul Snarecaster)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("ACdm"), GridSlotId::ability("ACsw")],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("nmsn", "Mur'gul Snarecaster", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nnrg (Naga Royal Guard)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("ACcb"), GridSlotId::ability("ACcv")],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'Z',
                    &[GridSlotId::ability("ACcb"), GridSlotId::ability("ACcv")],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("nnrg", "Naga Royal Guard", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nnwq (Nerubian Queen)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("ACrd"), GridSlotId::ability("ACca")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nnwq", "Nerubian Queen", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nnwr (Nerubian Seer)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("ACdm"), GridSlotId::ability("ACrd")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nnwr", "Nerubian Seer", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nnwl (Nerubian Webspinner)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("ACrd"), GridSlotId::ability("ACwb")],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("nnwl", "Nerubian Webspinner", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nndr (Nether Dragon)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    2,
                    2,
                    &[GridSlotId::ability("ACcr"), GridSlotId::ability("ACmi")],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[GridSlotId::ability("ACcr"), GridSlotId::ability("ACmi")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nndr", "Nether Dragon", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Nman (Pit Lord)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[GridSlotId::ability("AHtc"), GridSlotId::ability("ANrn")],
                )
                .build();
            let secondary_hot = HotkeyCollisionCardBuilder::new(GridRole::HeroSkillTree, layout)
                .collision(
                    'E',
                    &[GridSlotId::ability("ANrn"), GridSlotId::ability("AOeq")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Nman", "Pit Lord", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            let eb = eb.secondary_hotkey_card(secondary_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Npld (Pit Lord)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[GridSlotId::ability("AHtc"), GridSlotId::ability("ANrn")],
                )
                .build();
            let secondary_hot = HotkeyCollisionCardBuilder::new(GridRole::HeroSkillTree, layout)
                .collision(
                    'E',
                    &[GridSlotId::ability("ANrn"), GridSlotId::ability("AOeq")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Npld", "Pit Lord", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            let eb = eb.secondary_hotkey_card(secondary_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nenp (Poison Treant)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("ACvs"), GridSlotId::ability("Aenr")],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'Z',
                    &[GridSlotId::ability("ACvs"), GridSlotId::ability("Aenr")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nenp", "Poison Treant", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nfpe (Polar Furbolg Elder Shaman)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("AChv"), GridSlotId::ability("ACfn")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new(
                "nfpe",
                "Polar Furbolg Elder Shaman",
                empty_pos,
                empty_hot,
            );
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // Emoo (Priestess of the Moon)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[GridSlotId::ability("AEst"), GridSlotId::ability("AEar")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new(
                "Emoo",
                "Priestess of the Moon",
                empty_pos,
                empty_hot,
            );
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Etyr (Priestess of the Moon)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[GridSlotId::ability("AEst"), GridSlotId::ability("AEar")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new(
                "Etyr",
                "Priestess of the Moon",
                empty_pos,
                empty_hot,
            );
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Hvwd (Ranger)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[GridSlotId::ability("AEst"), GridSlotId::ability("AEar")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Hvwd", "Ranger", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nslv (Salamander Vizier)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[
                        GridSlotId::ability("Ambd"),
                        GridSlotId::ability("ACdm"),
                        GridSlotId::ability("ACbl"),
                    ],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'Z',
                    &[GridSlotId::ability("Ambd"), GridSlotId::ability("ACbl")],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("nslv", "Salamander Vizier", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nstl (Satyr Soulstealer)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("ACrd"), GridSlotId::ability("Ambd")],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("nstl", "Satyr Soulstealer", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nsgb (Sea Giant Behemoth)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("ACtb"), GridSlotId::ability("ACpv")],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("nsgb", "Sea Giant Behemoth", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nsgh (Sea Giant Hunter)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("ACen"), GridSlotId::ability("ACpv")],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'Z',
                    &[GridSlotId::ability("ACen"), GridSlotId::ability("ACpv")],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("nsgh", "Sea Giant Hunter", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // ndh4 (Seer's Den)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    0,
                    &[GridSlotId::ability("ndrs"), GridSlotId::ability("ndrh")],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'Q',
                    &[GridSlotId::ability("ndrs"), GridSlotId::ability("ndrh")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("ndh4", "Seer's Den", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Orkn (Shadow Hunter)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'S',
                    &[GridSlotId::ability("CmdStop"), GridSlotId::ability("Aamk")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Orkn", "Shadow Hunter", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nnsa (Shrine of Azshara)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    0,
                    &[GridSlotId::ability("nnsw"), GridSlotId::ability("nwgs")],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("nnsa", "Shrine of Azshara", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nsoc (Skeletal Orc Champion)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    2,
                    2,
                    &[GridSlotId::ability("ACvp"), GridSlotId::ability("ACcr")],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[GridSlotId::ability("ACvp"), GridSlotId::ability("ACcr")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new(
                "nsoc",
                "Skeletal Orc Champion",
                empty_pos,
                empty_hot,
            );
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // ngos (Snarlmane the Bloodgorger)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("ACac"), GridSlotId::ability("ACbl")],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'Z',
                    &[GridSlotId::ability("ACac"), GridSlotId::ability("ACbl")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new(
                "ngos",
                "Snarlmane the Bloodgorger",
                empty_pos,
                empty_hot,
            );
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nnsg (Spawning Grounds)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    0,
                    &[
                        GridSlotId::ability("nmyr"),
                        GridSlotId::ability("nsnp"),
                        GridSlotId::ability("nhyc"),
                    ],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("nnsg", "Spawning Grounds", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // hspt (Spellbreaker)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[GridSlotId::ability("Acmg"), GridSlotId::ability("Amim")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("hspt", "Spellbreaker", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // ospm (Spirit Walker)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'F',
                    &[GridSlotId::ability("ACsk"), GridSlotId::ability("Acpf")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("ospm", "Spirit Walker", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // noga (Stonemaul Warchief)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    2,
                    2,
                    &[GridSlotId::ability("ACbh"), GridSlotId::ability("SCae")],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[GridSlotId::ability("ACbh"), GridSlotId::ability("SCae")],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("noga", "Stonemaul Warchief", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // npn2 (Storm)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("ANwk"), GridSlotId::ability("Adsm")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("npn2", "Storm", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // npn5 (Storm)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("ANwk"), GridSlotId::ability("Adsm")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("npn5", "Storm", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nstw (Storm Wyrm)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("ACdv"), GridSlotId::ability("ACcl")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nstw", "Storm Wyrm", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // nsrn (Stormreaver Necrolyte)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("ACcl"), GridSlotId::ability("ACbl")],
                )
                .build();
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'Z',
                    &[GridSlotId::ability("ACcl"), GridSlotId::ability("ACbl")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new(
                "nsrn",
                "Stormreaver Necrolyte",
                empty_pos,
                empty_hot,
            );
            let eb = eb.main_position_card(main_pos);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Ocb2 (Tauren Chieftain)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'S',
                    &[GridSlotId::ability("CmdStop"), GridSlotId::ability("Aamk")],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("Ocb2", "Tauren Chieftain", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nntt (Temple of Tides)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    0,
                    &[GridSlotId::ability("nmpe"), GridSlotId::ability("nnmg")],
                )
                .build();
            let eb =
                UnitCollisionEntryBuilder::new("nntt", "Temple of Tides", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // Nrob (Tinker)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'S',
                    &[GridSlotId::ability("CmdStop"), GridSlotId::ability("ANde")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Nrob", "Tinker", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // ntkh (Tuskarr Healer)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("Anh1"), GridSlotId::ability("ACdm")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("ntkh", "Tuskarr Healer", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // Ewar (Warden)
        let entry = {
            let secondary_hot = HotkeyCollisionCardBuilder::new(GridRole::HeroSkillTree, layout)
                .collision(
                    'D',
                    &[GridSlotId::ability("AEsh"), GridSlotId::ability("AIhm")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Ewar", "Warden", empty_pos, empty_hot);
            let eb = eb.secondary_hotkey_card(secondary_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Ewrd (Warden)
        let entry = {
            let secondary_hot = HotkeyCollisionCardBuilder::new(GridRole::HeroSkillTree, layout)
                .collision(
                    'D',
                    &[GridSlotId::ability("AEsh"), GridSlotId::ability("AIhm")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Ewrd", "Warden", empty_pos, empty_hot);
            let eb = eb.secondary_hotkey_card(secondary_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Uwar (Warlock)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'S',
                    &[GridSlotId::ability("CmdStop"), GridSlotId::ability("ACm2")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("Uwar", "Warlock", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // nsns (Watery Minion)
        let entry = {
            let main_pos = PositionCollisionCardBuilder::new(GridRole::MainCommand)
                .collision_at(
                    0,
                    2,
                    &[GridSlotId::ability("ACdm"), GridSlotId::ability("ACsw")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("nsns", "Watery Minion", empty_pos, empty_hot);
            let eb = eb.main_position_card(main_pos);
            eb.build()
        };
        builder = builder.entry(entry);

        // ngh2 (Wraith)
        let entry = {
            let main_hot = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout)
                .collision(
                    'C',
                    &[GridSlotId::ability("ACcs"), GridSlotId::ability("ACps")],
                )
                .build();
            let eb = UnitCollisionEntryBuilder::new("ngh2", "Wraith", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            eb.build()
        };
        builder = builder.entry(entry);
        let expected = builder.build();
        assert_eq!(
            report, expected,
            "default CustomKeys.txt collision report changed — update the expected entries if intentional",
        );
    }
}
