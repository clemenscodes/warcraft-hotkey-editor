#[cfg(test)]
mod parse_and_binding_tests {
    use super::super::*;
    use crate::grid::layout::GridLayout;
    use crate::identity::hotkey_target::HotkeyTarget;
    use crate::identity::hotkey_token::HotkeyToken;
    use crate::model::{ColumnIndex, RowIndex};
    use warcraft_database::WARCRAFT_DATABASE;

    use crate::model::{
        AbilityBinding, AbilityModifier, CommandBinding, GridCoordinate, Hotkey, SystemBinding,
    };

    use warcraft_api::{SystemKeybindClass, SystemKeybindModifier};

    #[test]
    fn parses_single_entry_with_hotkey_and_buttonpos() {
        let input = "[AHhb]\nHotkey=Q\nButtonpos=0,2\n";
        let file = CustomKeys::parse_raw(input);
        let binding = file.binding("AHhb").unwrap();
        let expected_hotkey = Hotkey::Letter('Q');
        assert_eq!(binding.hotkey(), Some(&expected_hotkey));
        let position = binding.button_position().unwrap();
        assert_eq!(position.column(), ColumnIndex::Zero);
        assert_eq!(position.row(), RowIndex::Two);
    }

    #[test]
    fn lookup_uses_canonical_case() {
        let input = "[Hpal]\nHotkey=T\nButtonpos=3,0\n";
        let file = CustomKeys::parse_raw(input);
        assert!(file.binding("Hpal").is_some());
    }

    #[test]
    fn missing_hotkey_returns_none() {
        let input = "[AHbz]\nButtonpos=0,0\n";
        let file = CustomKeys::parse_raw(input);
        assert_eq!(file.binding("AHbz").unwrap().hotkey(), None);
    }

    #[test]
    fn empty_hotkey_value_treated_as_absent() {
        let input = "[AHbz]\nHotkey=\nButtonpos=0,0\n";
        let file = CustomKeys::parse_raw(input);
        assert_eq!(file.binding("AHbz").unwrap().hotkey(), None);
    }

    #[test]
    fn research_fields_parsed() {
        let input = "[AHhb]\nResearchhotkey=T\nResearchbuttonpos=3,1\n";
        let file = CustomKeys::parse_raw(input);
        let binding = file.binding("AHhb").unwrap();
        let expected_hotkey = Hotkey::Letter('T');
        assert_eq!(binding.research_hotkey(), Some(&expected_hotkey));
        let position = binding.research_button_position().unwrap();
        assert_eq!(position.column(), ColumnIndex::Three);
        assert_eq!(position.row(), RowIndex::One);
    }

    #[test]
    fn bindings_in_order_returns_alphabetical_order() {
        let binding_ahhb = AbilityBinding::builder().tip("first").build();
        let binding_ahbz = AbilityBinding::builder().tip("second").build();
        let file = CustomKeys::builder()
            .ability("AHhb", binding_ahhb)
            .ability("AHbz", binding_ahbz)
            .build();
        let ids: Vec<&str> = file
            .bindings_in_order()
            .map(|entry| entry.ability_id().value())
            .collect();
        assert_eq!(ids, ["AHbz", "AHhb"]);
    }

    #[test]
    fn comment_lines_are_skipped() {
        let input = "// This is a comment\n[AHhb]\nHotkey=Q\n; Also a comment\nButtonpos=0,0\n";
        let file = CustomKeys::parse_raw(input);
        let binding = file.binding("AHhb").unwrap();
        let expected_hotkey = Hotkey::Letter('Q');
        assert_eq!(binding.hotkey(), Some(&expected_hotkey));
        assert!(binding.button_position().is_some());
    }

    #[test]
    fn unknown_keys_are_silently_ignored() {
        let input = "[AHhb]\nHotkey=Q\nUnknownField=something\n";
        let file = CustomKeys::parse_raw(input);
        let expected_hotkey = Hotkey::Letter('Q');
        assert_eq!(
            file.binding("AHhb").unwrap().hotkey(),
            Some(&expected_hotkey)
        );
    }

    #[test]
    fn malformed_buttonpos_gives_none() {
        let input = "[AHhb]\nButtonpos=notanumber\n";
        let file = CustomKeys::parse_raw(input);
        assert!(file.binding("AHhb").unwrap().button_position().is_none());
    }

    #[test]
    fn round_trip_preserves_section_id_case() {
        let input = "[AHhb]\nHotkey=Q\nButtonpos=0,0\n\n";
        let file = CustomKeys::parse_raw(input);
        assert!(file.to_string().contains("[AHhb]"));
    }

    #[test]
    fn duplicate_section_uses_first_occurrence() {
        let input = "[AHhb]\nHotkey=Q\n\n[AHhb]\nHotkey=W\n";
        let file = CustomKeys::parse_raw(input);
        let expected_hotkey = Hotkey::Letter('Q');
        assert_eq!(
            file.binding("AHhb").unwrap().hotkey(),
            Some(&expected_hotkey)
        );
    }

    #[test]
    fn untouched_sections_round_trip_byte_identically() {
        let input = "[AHhb]\nHotkey=Q\nButtonpos=0,2\n//inline comment\nIcon=ReplaceableTextures\\CommandButtons\\BTNAvatar.blp\n\n[AHbz]\nHotkey=W\nButtonpos=1,2\n\n";
        let file = CustomKeys::parse_raw(input);
        let output = file.to_string();
        assert!(output.contains("[AHhb]"));
        assert!(output.contains("BTNAvatar.blp"));
        assert!(output.contains("[AHbz]"));
    }

    #[test]
    fn touched_section_uses_formatted_output() {
        let hotkey_q = Hotkey::from('Q');
        let hotkey_w = Hotkey::from('W');
        let position_02 = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Two);
        let position_12 = GridCoordinate::new(ColumnIndex::One, RowIndex::Two);
        let binding_ahhb = AbilityBinding::builder()
            .hotkey(hotkey_q)
            .button_position(position_02)
            .build();
        let binding_ahbz = AbilityBinding::builder()
            .hotkey(hotkey_w)
            .button_position(position_12)
            .build();
        let mut file = CustomKeys::builder()
            .ability("AHhb", binding_ahhb)
            .ability("AHbz", binding_ahbz)
            .build();
        let hotkey_r = Hotkey::from('R');
        file.binding_or_default_mut("AHhb")
            .unwrap()
            .set_hotkey(Some(hotkey_r));
        let output = file.to_string();
        assert!(output.contains("Hotkey=R"), "mutated hotkey must appear");
        assert!(
            output.contains("Hotkey=W"),
            "untouched section hotkey must still be present",
        );
    }

    #[test]
    fn parses_command_section() {
        let input = "[CmdMove]\nHotkey=M\nButtonpos=1,2\nTip=Move\n";
        let file = CustomKeys::parse_raw(input);
        let binding = file.command("CmdMove").expect("CmdMove parsed");
        let expected_hotkey = Hotkey::Letter('M');
        assert_eq!(binding.hotkey(), Some(&expected_hotkey));
        let position = binding.button_position().expect("position parsed");
        assert_eq!(position.column(), ColumnIndex::One);
        assert_eq!(position.row(), RowIndex::Two);
    }

    #[test]
    fn parses_system_section_game_command() {
        let input = "[itm1]\nHotkey=9\nGameCommand=1\n";
        let file = CustomKeys::parse_raw(input);
        let sys = file.system("itm1").expect("system section parsed");
        assert_eq!(sys.hotkey(), &Hotkey::VirtualKey(9));
        assert_eq!(sys.class(), SystemKeybindClass::Game);
        assert!(sys.modifier().is_none());
    }

    #[test]
    fn parses_system_section_ctrl_group_with_modifier() {
        let input = "[Ctr1]\nHotkey=49\nCtrlGroupCommand=1\nModifier=Ctrl\n";
        let file = CustomKeys::parse_raw(input);
        let sys = file.system("Ctr1").expect("parsed");
        assert_eq!(sys.hotkey(), &Hotkey::VirtualKey(49));
        assert_eq!(sys.class(), SystemKeybindClass::ControlGroup);
        assert_eq!(sys.modifier(), Some(SystemKeybindModifier::Ctrl));
    }

    #[test]
    fn system_section_not_returned_by_binding() {
        let input = "[itm1]\nHotkey=9\nGameCommand=1\n";
        let file = CustomKeys::parse_raw(input);
        assert!(file.binding("itm1").is_none());
        assert!(file.system("itm1").is_some());
    }

    #[test]
    fn system_section_round_trips() {
        let input = "[itm1]\nHotkey=9\nGameCommand=1\n\n";
        let file = CustomKeys::parse_raw(input);
        let output = file.to_string();
        assert!(output.contains("[itm1]"));
        assert!(output.contains("Hotkey=9"));
        assert!(output.contains("GameCommand=1"));
    }

    #[test]
    fn all_ability_text_fields_parsed() {
        let input = concat!(
            "[Ahrl]\n",
            "Tip=Cast Holy Light\n",
            "Researchtip=Research something\n",
            "UnTip=Cancel\n",
            "Ubertip=Heals a friendly unit for 200 hit points.\n",
            "Researchubertip=Researches something powerful.\n",
            "Unubertip=Off form description.\n",
        );
        let file = CustomKeys::parse_raw(input);
        let binding = file.binding("Ahrl").expect("Ahrl must be present");
        assert_eq!(binding.tip(), Some("Cast Holy Light"));
        assert_eq!(binding.research_tip(), Some("Research something"));
        assert_eq!(binding.un_tip(), Some("Cancel"));
        assert_eq!(
            binding.ubertip(),
            Some("Heals a friendly unit for 200 hit points.")
        );
        assert_eq!(
            binding.research_ubertip(),
            Some("Researches something powerful.")
        );
        assert_eq!(binding.un_ubertip(), Some("Off form description."));
    }

    #[test]
    fn icon_field_parsed() {
        let input = "[Ahrl]\nIcon=ReplaceableTextures\\CommandButtons\\BTNHolyLight.blp\n";
        let file = CustomKeys::parse_raw(input);
        let binding = file.binding("Ahrl").expect("present");
        assert_eq!(
            binding.icon(),
            Some("ReplaceableTextures\\CommandButtons\\BTNHolyLight.blp"),
        );
    }

    #[test]
    fn art_alias_maps_to_icon_field() {
        let input = "[Ahrl]\nArt=ReplaceableTextures\\CommandButtons\\BTNHolyLight.blp\n";
        let file = CustomKeys::parse_raw(input);
        let binding = file.binding("Ahrl").expect("present");
        assert_eq!(
            binding.icon(),
            Some("ReplaceableTextures\\CommandButtons\\BTNHolyLight.blp"),
        );
    }

    #[test]
    fn unart_alias_maps_to_un_icon_field() {
        let input = "[Ahrl]\nUnArt=ReplaceableTextures\\CommandButtons\\BTNCancel.blp\n";
        let file = CustomKeys::parse_raw(input);
        let binding = file.binding("Ahrl").expect("present");
        assert_eq!(
            binding.un_icon(),
            Some("ReplaceableTextures\\CommandButtons\\BTNCancel.blp"),
        );
    }

    #[test]
    fn modifier_field_parsed_in_ability_binding() {
        let input = "[Ahrl]\nModifier=Alt\n";
        let file = CustomKeys::parse_raw(input);
        let binding = file.binding("Ahrl").expect("present");
        assert_eq!(binding.modifier(), Some(AbilityModifier::Alt));
    }

    #[test]
    fn modifier_field_case_insensitive_in_parsing() {
        let input = "[Ahrl]\nMODIFIER=Ctrl\n";
        let file = CustomKeys::parse_raw(input);
        let binding = file.binding("Ahrl").expect("present");
        assert_eq!(binding.modifier(), Some(AbilityModifier::Ctrl));
    }

    #[test]
    fn empty_file_has_no_entries() {
        let file = CustomKeys::parse_raw("");
        let ability_count = file.bindings_in_order().count();
        let command_count = file.commands_in_order().count();
        assert_eq!(ability_count, 0);
        assert_eq!(command_count, 0);
    }

    #[test]
    fn default_custom_keys_file_is_empty() {
        let file = CustomKeys::default();
        let ability_count = file.bindings_in_order().count();
        assert_eq!(ability_count, 0);
    }

    #[test]
    fn command_is_not_returned_by_binding_accessor() {
        let hotkey = Hotkey::from('M');
        let binding = CommandBinding::builder().hotkey(hotkey).build();
        let file = CustomKeys::builder().command("CmdMove", binding).build();
        assert!(file.binding("CmdMove").is_none());
        assert!(file.command("CmdMove").is_some());
    }

    #[test]
    fn ability_is_not_returned_by_command_accessor() {
        let hotkey = Hotkey::from('Q');
        let binding = AbilityBinding::builder().hotkey(hotkey).build();
        let file = CustomKeys::builder().ability("Ahrl", binding).build();
        assert!(file.command("Ahrl").is_none());
        assert!(file.binding("Ahrl").is_some());
    }

    #[test]
    fn commands_in_order_returns_alphabetical_order() {
        let hotkey_a = Hotkey::from('A');
        let hotkey_m = Hotkey::from('M');
        let hotkey_s = Hotkey::from('S');
        let cmd_attack = CommandBinding::builder().hotkey(hotkey_a).build();
        let cmd_move = CommandBinding::builder().hotkey(hotkey_m).build();
        let cmd_stop = CommandBinding::builder().hotkey(hotkey_s).build();
        let file = CustomKeys::builder()
            .command("CmdAttack", cmd_attack)
            .command("CmdMove", cmd_move)
            .command("CmdStop", cmd_stop)
            .build();
        let names: Vec<&str> = file
            .commands_in_order()
            .map(|entry| entry.name().value())
            .collect();
        assert_eq!(names, ["CmdAttack", "CmdMove", "CmdStop"]);
    }

    #[test]
    fn commands_in_order_excludes_ability_sections() {
        let ability_hotkey = Hotkey::from('Q');
        let command_hotkey = Hotkey::from('A');
        let ability = AbilityBinding::builder().hotkey(ability_hotkey).build();
        let command = CommandBinding::builder().hotkey(command_hotkey).build();
        let file = CustomKeys::builder()
            .ability("Ahrl", ability)
            .command("CmdAttack", command)
            .build();
        let command_count = file.commands_in_order().count();
        assert_eq!(command_count, 1);
    }

    #[test]
    fn bindings_in_order_excludes_command_sections() {
        let command_hotkey = Hotkey::from('A');
        let ability_hotkey = Hotkey::from('Q');
        let command = CommandBinding::builder().hotkey(command_hotkey).build();
        let ability = AbilityBinding::builder().hotkey(ability_hotkey).build();
        let file = CustomKeys::builder()
            .command("CmdAttack", command)
            .ability("Ahrl", ability)
            .build();
        let binding_count = file.bindings_in_order().count();
        assert_eq!(binding_count, 1);
    }

    #[test]
    fn system_observer_command_parsed() {
        let input = "[THer]\nHotkey=120\nObserverCommand=1\n";
        let file = CustomKeys::parse_raw(input);
        let sys = file.system("THer").expect("observer section parsed");
        assert_eq!(sys.hotkey(), &Hotkey::VirtualKey(120));
        assert_eq!(sys.class(), SystemKeybindClass::Observer);
    }

    #[test]
    fn system_replay_command_parsed() {
        let input = "[TRpl]\nHotkey=80\nReplayCommand=1\n";
        let file = CustomKeys::parse_raw(input);
        let sys = file.system("TRpl").expect("replay section parsed");
        assert_eq!(sys.hotkey(), &Hotkey::VirtualKey(80));
        assert_eq!(sys.class(), SystemKeybindClass::Replay);
    }

    #[test]
    fn system_camera_command_parsed() {
        let input = "[ctcr]\nHotkey=65\nCameraCommand=1\n";
        let file = CustomKeys::parse_raw(input);
        let sys = file.system("ctcr").expect("camera section parsed");
        assert_eq!(sys.hotkey(), &Hotkey::VirtualKey(65));
        assert_eq!(sys.class(), SystemKeybindClass::Camera);
    }

    #[test]
    fn system_menu_command_parsed() {
        let input = "[QLog]\nHotkey=27\nMenuCommand=1\n";
        let file = CustomKeys::parse_raw(input);
        let sys = file.system("QLog").expect("menu section parsed");
        assert_eq!(sys.hotkey(), &Hotkey::VirtualKey(27));
        assert_eq!(sys.class(), SystemKeybindClass::Menu);
    }

    #[test]
    fn system_section_all_modifiers_parse() {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        struct ModifierCase {
            modifier_text: &'static str,
            expected_modifier: SystemKeybindModifier,
        }
        let cases = [
            ModifierCase {
                modifier_text: "Alt",
                expected_modifier: SystemKeybindModifier::Alt,
            },
            ModifierCase {
                modifier_text: "Ctrl",
                expected_modifier: SystemKeybindModifier::Ctrl,
            },
            ModifierCase {
                modifier_text: "Ctrl_or_Alt",
                expected_modifier: SystemKeybindModifier::CtrlOrAlt,
            },
            ModifierCase {
                modifier_text: "Shift",
                expected_modifier: SystemKeybindModifier::Shift,
            },
        ];
        for case in &cases {
            let modifier_text = case.modifier_text;
            let input =
                format!("[Ctr1]\nHotkey=49\nCtrlGroupCommand=1\nModifier={modifier_text}\n",);
            let file = CustomKeys::parse_raw(input.as_str());
            let sys = file.system("Ctr1").expect("section parsed");
            let expected_modifier = Some(case.expected_modifier);
            assert_eq!(
                sys.modifier(),
                expected_modifier,
                "Modifier={modifier_text} must parse correctly",
            );
        }
    }

    #[test]
    fn set_system_hotkey_updates_existing_binding() {
        let initial_binding =
            SystemBinding::new(Hotkey::VirtualKey(27), SystemKeybindClass::Game, None);
        let mut file = CustomKeys::builder()
            .system("QLog", initial_binding)
            .build();
        let new_key = KeyCode::try_from(65).expect("valid keycode");
        file.set_system_hotkey("QLog", new_key);
        let expected_hotkey = Hotkey::VirtualKey(65);
        assert_eq!(
            file.system("QLog").map(|binding| *binding.hotkey()),
            Some(expected_hotkey),
        );
    }

    #[test]
    fn set_system_hotkey_is_noop_for_missing_section() {
        let mut file = CustomKeys::default();
        let new_key = KeyCode::try_from(65).expect("valid keycode");
        file.set_system_hotkey("nonexistent", new_key);
        assert!(file.system("nonexistent").is_none());
    }

    #[test]
    fn put_ability_inserts_and_is_accessible() {
        let hotkey = Hotkey::from('Q');
        let binding = AbilityBinding::builder().hotkey(hotkey).build();
        let mut file = CustomKeys::default();
        file.put_ability("Ahrl", binding);
        let expected_hotkey = Hotkey::Letter('Q');
        assert_eq!(
            file.binding("Ahrl").and_then(|binding| binding.hotkey()),
            Some(&expected_hotkey),
        );
    }

    #[test]
    fn put_command_inserts_and_is_accessible() {
        let hotkey = Hotkey::from('A');
        let binding = CommandBinding::builder().hotkey(hotkey).build();
        let mut file = CustomKeys::default();
        file.put_command("CmdAttack", binding);
        let expected_hotkey = Hotkey::Letter('A');
        assert_eq!(
            file.command("CmdAttack")
                .and_then(|binding| binding.hotkey()),
            Some(&expected_hotkey),
        );
    }

    #[test]
    fn put_system_inserts_and_is_accessible() {
        let binding = SystemBinding::new(Hotkey::VirtualKey(9), SystemKeybindClass::Game, None);
        let mut file = CustomKeys::default();
        file.put_system("IsHeroSelect", binding);
        assert_eq!(
            file.system("IsHeroSelect")
                .map(|system_binding| *system_binding.hotkey()),
            Some(Hotkey::VirtualKey(9)),
        );
    }

    #[test]
    fn put_ability_overwrites_existing_entry() {
        let first_hotkey = Hotkey::from('Q');
        let second_hotkey = Hotkey::from('W');
        let first = AbilityBinding::builder().hotkey(first_hotkey).build();
        let second = AbilityBinding::builder().hotkey(second_hotkey).build();
        let mut file = CustomKeys::default();
        file.put_ability("Ahrl", first);
        file.put_ability("Ahrl", second);
        let expected_hotkey = Hotkey::Letter('W');
        assert_eq!(
            file.binding("Ahrl").and_then(|binding| binding.hotkey()),
            Some(&expected_hotkey),
        );
    }

    #[test]
    fn round_trip_of_baseline_preserves_known_sections() {
        let baseline = include_str!("../../templates/CustomKeys.txt");
        let file = CustomKeys::parse_raw(baseline);
        let output = file.to_string();
        let known_sections = [
            "[CmdAttack]",
            "[CmdMove]",
            "[CmdRally]",
            "[CmdCancel]",
            "[CmdBuildHuman]",
            "[Hpal]",
            "[hkee]",
            "[Rhpm]",
            "[AHhb]",
        ];
        for section_marker in known_sections {
            assert!(
                output.contains(section_marker),
                "round-trip output is missing section {section_marker:?}",
            );
        }

        use std::collections::BTreeSet;
        let collect_unique_sections = |text: &str| -> BTreeSet<String> {
            text.lines()
                .filter_map(|line| {
                    let trimmed = line.trim();
                    if trimmed.starts_with('[') && trimmed.ends_with(']') {
                        Some(trimmed.to_ascii_lowercase())
                    } else {
                        None
                    }
                })
                .collect()
        };
        let baseline_unique = collect_unique_sections(baseline);
        let output_unique = collect_unique_sections(&output);
        assert_eq!(
            baseline_unique, output_unique,
            "round-trip preserves the set of unique section headers",
        );
    }

    #[test]
    fn set_hotkey_replicates_to_two_tier_upgrade() {
        let binding_ruba = AbilityBinding::default();
        let mut keys = CustomKeys::builder().ability("Ruba", binding_ruba).build();
        let new_token = HotkeyToken::try_from('F').expect("letter");
        let target = HotkeyTarget::ability("Ruba");
        keys.set_hotkey(target, Some(new_token));
        let binding = keys.binding("Ruba").expect("Ruba exists");
        let hotkey = binding.hotkey().expect("hotkey set");
        assert_eq!(hotkey.level_count(), 2);
    }

    #[test]
    fn set_hotkey_replicates_to_three_tier_upgrade() {
        let binding_rume = AbilityBinding::default();
        let mut keys = CustomKeys::builder().ability("Rume", binding_rume).build();
        let new_token = HotkeyToken::try_from('F').expect("letter");
        let target = HotkeyTarget::ability("Rume");
        keys.set_hotkey(target, Some(new_token));
        let binding = keys.binding("Rume").expect("Rume exists");
        let hotkey = binding.hotkey().expect("hotkey set");
        assert_eq!(hotkey.level_count(), 3);
    }

    #[test]
    fn set_hotkey_keeps_leveled_ability_single_tier() {
        let binding_aeah = AbilityBinding::default();
        let mut keys = CustomKeys::builder().ability("AEah", binding_aeah).build();
        let new_token = HotkeyToken::try_from('F').expect("letter");
        let target = HotkeyTarget::ability("AEah");
        keys.set_hotkey(target, Some(new_token));
        let binding = keys.binding("AEah").expect("AEah exists");
        let hotkey = binding.hotkey().expect("hotkey set");
        assert_eq!(hotkey.level_count(), 1);
    }

    #[test]
    fn set_hotkey_serializes_upgrade_hotkey_per_tier() {
        let binding_rume = AbilityBinding::default();
        let mut keys = CustomKeys::builder().ability("Rume", binding_rume).build();
        let new_token = HotkeyToken::try_from('F').expect("letter");
        let target = HotkeyTarget::ability("Rume");
        keys.set_hotkey(target, Some(new_token));
        let serialized = keys.to_string();
        assert!(
            serialized.contains("Hotkey=F,F,F"),
            "expected three-tier upgrade hotkey, got:\n{serialized}",
        );
    }

    #[test]
    fn apply_grid_preserves_three_tier_upgrade_hotkey() {
        let input = "[Rume]\nHotkey=S,S,S\nButtonpos=0,0\n";
        let mut keys = CustomKeys::parse_raw(input);
        let layout = GridLayout::qwerty_grid();
        keys.apply_grid_to_all_bindings(layout);
        let binding = keys.binding("Rume").expect("Rume exists");
        let hotkey = binding.hotkey().expect("hotkey set");
        let expected = Hotkey::try_from("Q,Q,Q").expect("valid hotkey");
        assert_eq!(hotkey, &expected);
    }

    #[test]
    fn apply_grid_preserves_two_tier_upgrade_hotkey() {
        let input = "[Ruba]\nHotkey=A,A\nButtonpos=1,2\n";
        let mut keys = CustomKeys::parse_raw(input);
        let layout = GridLayout::qwerty_grid();
        keys.apply_grid_to_all_bindings(layout);
        let binding = keys.binding("Ruba").expect("Ruba exists");
        let hotkey = binding.hotkey().expect("hotkey set");
        let expected = Hotkey::try_from("X,X").expect("valid hotkey");
        assert_eq!(hotkey, &expected);
    }

    #[test]
    fn apply_grid_keeps_leveled_ability_single_tier() {
        let input = "[AEah]\nHotkey=D\nButtonpos=2,2\n";
        let mut keys = CustomKeys::parse_raw(input);
        let layout = GridLayout::qwerty_grid();
        keys.apply_grid_to_all_bindings(layout);
        let binding = keys.binding("AEah").expect("AEah exists");
        let hotkey = binding.hotkey().expect("hotkey set");
        assert_eq!(hotkey.level_count(), 1);
    }

    #[test]
    fn normalize_restores_upgrade_hotkey_tiers_after_template_overlay() {
        let mut baseline = CustomKeys::parse_raw(DEFAULT_CUSTOM_KEYS);
        let template = CustomKeys::parse_raw("[Rume]\nHotkey=Q\nButtonpos=0,0\n");
        baseline.extend(template);
        let normalized = baseline.normalize();
        let binding = normalized.binding("Rume").expect("Rume exists");
        let hotkey = binding.hotkey().expect("hotkey set");
        let expected = Hotkey::try_from("Q,Q,Q").expect("valid hotkey");
        assert_eq!(hotkey, &expected);
    }

    #[test]
    fn cascade_preserves_upgrade_hotkey_tiers() {
        let mut baseline = CustomKeys::parse_raw(DEFAULT_CUSTOM_KEYS);
        let template = CustomKeys::parse_raw("[Rume]\nHotkey=Q\nButtonpos=0,0\n");
        baseline.extend(template);
        let mut normalized = baseline.normalize();
        let _plan = normalized.resolve_conflicts();
        let binding = normalized.binding("Rume").expect("Rume exists");
        let hotkey = binding.hotkey().expect("hotkey set");
        assert_eq!(hotkey.level_count(), 3);
    }

    #[test]
    fn apply_grid_over_default_keeps_every_multi_level_upgrade_tiered() {
        let mut keys = CustomKeys::from_text(DEFAULT_CUSTOM_KEYS);
        let layout = GridLayout::qwerty_grid();
        keys.apply_grid_to_all_bindings(layout);
        let mut checked: usize = 0;
        for entry in keys.bindings_in_order() {
            let ability_id = entry.ability_id();
            let object_code = ability_id.value();
            let Some(object) = WARCRAFT_DATABASE.by_id(object_code) else {
                continue;
            };
            let Some(max_level) = object.upgrade_max_level() else {
                continue;
            };
            if max_level < 2 {
                continue;
            }
            let binding = entry.binding();
            if binding.button_position().is_none() {
                continue;
            }
            let Some(hotkey) = binding.hotkey() else {
                continue;
            };
            let level_count = hotkey.level_count();
            assert_eq!(
                level_count, max_level,
                "upgrade {object_code} lost tiers after apply_grid: \
                 `{hotkey}` has {level_count} level(s), expected {max_level}",
            );
            checked += 1;
        }
        assert!(
            checked >= 10,
            "expected to verify many multi-level upgrades, only checked {checked}",
        );
    }

    #[test]
    fn assign_position_replicates_upgrade_hotkey_per_tier() {
        use crate::identity::slot::GridSlotId;
        let input = "[Rume]\nHotkey=S,S,S\nButtonpos=0,0\n";
        let mut keys = CustomKeys::parse_raw(input);
        let layout = GridLayout::qwerty_grid();
        let slot = GridSlotId::ability("Rume");
        keys.assign_position(layout, &slot, 1, 1, false, true);
        let binding = keys.binding("Rume").expect("Rume exists");
        let hotkey = binding.hotkey().expect("hotkey set");
        let expected = Hotkey::try_from("S,S,S").expect("valid hotkey");
        assert_eq!(hotkey, &expected);
    }

    #[test]
    fn move_slot_keeps_hotkey_when_reassignment_disabled() {
        use crate::command::move_request::MoveRequest;
        use crate::identity::slot::GridSlotId;
        let input = "[ACad]\nHotkey=P\nButtonpos=2,2\n";
        let mut keys = CustomKeys::parse_raw(input);
        let layout = GridLayout::qwerty_grid();
        let moving = GridSlotId::ability("ACad");
        let slot_ids = [moving];
        let request = MoveRequest::new(layout, &slot_ids, &moving, 1, 1, false)
            .with_assign_hotkey_on_move(false);
        keys.move_slot(&request);
        let binding = keys.binding("ACad").expect("ACad exists");
        let position = binding.button_position().expect("position set");
        assert_eq!(u8::from(position.column()), 1);
        assert_eq!(u8::from(position.row()), 1);
        let hotkey = binding.hotkey().expect("hotkey set");
        let expected = Hotkey::try_from("P").expect("valid hotkey");
        assert_eq!(hotkey, &expected);
    }

    #[test]
    fn move_slot_reassigns_hotkey_by_default() {
        use crate::command::move_request::MoveRequest;
        use crate::identity::slot::GridSlotId;
        let input = "[ACad]\nHotkey=P\nButtonpos=2,2\n";
        let mut keys = CustomKeys::parse_raw(input);
        let layout = GridLayout::qwerty_grid();
        let moving = GridSlotId::ability("ACad");
        let slot_ids = [moving];
        let request = MoveRequest::new(layout, &slot_ids, &moving, 1, 1, false);
        keys.move_slot(&request);
        let binding = keys.binding("ACad").expect("ACad exists");
        let hotkey = binding.hotkey().expect("hotkey set");
        let expected = Hotkey::try_from("S").expect("valid hotkey");
        assert_eq!(hotkey, &expected);
    }

    #[test]
    fn move_slot_swap_is_reversible_when_layout_consistent() {
        use crate::command::move_request::MoveRequest;
        use crate::identity::slot::GridSlotId;
        let input = "[ACad]\nHotkey=Q\nButtonpos=0,0\n[AHbz]\nHotkey=S\nButtonpos=1,1\n";
        let mut keys = CustomKeys::parse_raw(input);
        let layout = GridLayout::qwerty_grid();
        let acad = GridSlotId::ability("ACad");
        let ahbz = GridSlotId::ability("AHbz");
        let slot_ids = [acad, ahbz];
        let swap = MoveRequest::new(layout, &slot_ids, &acad, 1, 1, false);
        keys.move_slot(&swap);
        let swap_back = MoveRequest::new(layout, &slot_ids, &acad, 0, 0, false);
        keys.move_slot(&swap_back);
        let acad_binding = keys.binding("ACad").expect("ACad exists");
        let acad_hotkey = acad_binding.hotkey().expect("hotkey set");
        assert_eq!(acad_hotkey, &Hotkey::try_from("Q").expect("valid"));
        let ahbz_binding = keys.binding("AHbz").expect("AHbz exists");
        let ahbz_hotkey = ahbz_binding.hotkey().expect("hotkey set");
        assert_eq!(ahbz_hotkey, &Hotkey::try_from("S").expect("valid"));
    }

    #[test]
    fn move_slot_swap_keeps_both_hotkeys_when_reassignment_disabled() {
        use crate::command::move_request::MoveRequest;
        use crate::identity::slot::GridSlotId;
        let input = "[ACad]\nHotkey=P\nButtonpos=0,0\n[AHbz]\nHotkey=K\nButtonpos=1,1\n";
        let mut keys = CustomKeys::parse_raw(input);
        let layout = GridLayout::qwerty_grid();
        let moving = GridSlotId::ability("ACad");
        let displaced = GridSlotId::ability("AHbz");
        let slot_ids = [moving, displaced];
        let request = MoveRequest::new(layout, &slot_ids, &moving, 1, 1, false)
            .with_assign_hotkey_on_move(false);
        keys.move_slot(&request);
        let moving_binding = keys.binding("ACad").expect("ACad exists");
        let moving_position = moving_binding.button_position().expect("position set");
        assert_eq!(u8::from(moving_position.column()), 1);
        assert_eq!(u8::from(moving_position.row()), 1);
        let moving_hotkey = moving_binding.hotkey().expect("hotkey set");
        let expected_moving = Hotkey::try_from("P").expect("valid hotkey");
        assert_eq!(moving_hotkey, &expected_moving);
        let displaced_binding = keys.binding("AHbz").expect("AHbz exists");
        let displaced_position = displaced_binding.button_position().expect("position set");
        assert_eq!(u8::from(displaced_position.column()), 0);
        assert_eq!(u8::from(displaced_position.row()), 0);
        let displaced_hotkey = displaced_binding.hotkey().expect("hotkey set");
        let expected_displaced = Hotkey::try_from("K").expect("valid hotkey");
        assert_eq!(displaced_hotkey, &expected_displaced);
    }

    #[test]
    fn set_hotkey_fans_out_to_tiered_sibling_ability() {
        let hotkey_q_strong = Hotkey::from('Q');
        let binding_abu3 = AbilityBinding::builder().hotkey(hotkey_q_strong).build();
        let hotkey_q_weak = Hotkey::from('Q');
        let binding_abu2 = AbilityBinding::builder().hotkey(hotkey_q_weak).build();
        let mut keys = CustomKeys::builder()
            .ability("Abu3", binding_abu3)
            .ability("Abu2", binding_abu2)
            .build();
        let new_token = HotkeyToken::try_from('Y').expect("letter");
        let target = HotkeyTarget::ability("Abu3");
        keys.set_hotkey(target, Some(new_token));
        let sibling_binding = keys.binding("Abu2").expect("Abu2 exists");
        let sibling_hotkey = sibling_binding.hotkey().expect("Abu2 hotkey set");
        assert_eq!(sibling_hotkey.first_token(), Some(new_token));
    }

    #[test]
    fn set_hotkey_fan_out_is_symmetric_from_weaker_tier() {
        let hotkey_q_strong = Hotkey::from('Q');
        let binding_abu3 = AbilityBinding::builder().hotkey(hotkey_q_strong).build();
        let hotkey_q_weak = Hotkey::from('Q');
        let binding_abu2 = AbilityBinding::builder().hotkey(hotkey_q_weak).build();
        let mut keys = CustomKeys::builder()
            .ability("Abu3", binding_abu3)
            .ability("Abu2", binding_abu2)
            .build();
        let new_token = HotkeyToken::try_from('Z').expect("letter");
        let target = HotkeyTarget::ability("Abu2");
        keys.set_hotkey(target, Some(new_token));
        let sibling_binding = keys.binding("Abu3").expect("Abu3 exists");
        let sibling_hotkey = sibling_binding.hotkey().expect("Abu3 hotkey set");
        assert_eq!(sibling_hotkey.first_token(), Some(new_token));
    }

    #[test]
    fn set_hotkey_off_state_fans_out_to_tiered_sibling() {
        let unhotkey_q_strong = Hotkey::from('Q');
        let binding_abu3 = AbilityBinding::builder()
            .unhotkey(unhotkey_q_strong)
            .build();
        let unhotkey_q_weak = Hotkey::from('Q');
        let binding_abu2 = AbilityBinding::builder().unhotkey(unhotkey_q_weak).build();
        let mut keys = CustomKeys::builder()
            .ability("Abu3", binding_abu3)
            .ability("Abu2", binding_abu2)
            .build();
        let new_token = HotkeyToken::try_from('D').expect("letter");
        let target = HotkeyTarget::ability_off_state("Abu3");
        keys.set_hotkey(target, Some(new_token));
        let sibling_binding = keys.binding("Abu2").expect("Abu2 exists");
        let sibling_unhotkey = sibling_binding.unhotkey().expect("Abu2 unhotkey set");
        assert_eq!(sibling_unhotkey.first_token(), Some(new_token));
    }

    #[test]
    fn set_hotkey_on_ability_without_siblings_does_not_touch_unrelated_binding() {
        let hotkey_q = Hotkey::from('Q');
        let binding_ahbz = AbilityBinding::builder().hotkey(hotkey_q).build();
        let hotkey_w = Hotkey::from('W');
        let binding_ahhb = AbilityBinding::builder().hotkey(hotkey_w).build();
        let mut keys = CustomKeys::builder()
            .ability("AHbz", binding_ahbz)
            .ability("AHhb", binding_ahhb)
            .build();
        let new_token = HotkeyToken::try_from('Y').expect("letter");
        let target = HotkeyTarget::ability("AHbz");
        keys.set_hotkey(target, Some(new_token));
        let unrelated_binding = keys.binding("AHhb").expect("AHhb exists");
        let unrelated_hotkey = unrelated_binding.hotkey().expect("AHhb hotkey set");
        let expected_token = HotkeyToken::try_from('W').expect("letter");
        assert_eq!(unrelated_hotkey.first_token(), Some(expected_token));
    }

    #[test]
    fn move_slot_fans_out_position_to_tiered_sibling() {
        use crate::command::move_request::MoveRequest;
        use crate::grid::layout::GridLayout;
        let position_origin = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
        let hotkey_q_strong = Hotkey::from('Q');
        let binding_abu3 = AbilityBinding::builder()
            .button_position(position_origin)
            .hotkey(hotkey_q_strong)
            .build();
        let hotkey_q_weak = Hotkey::from('Q');
        let binding_abu2 = AbilityBinding::builder()
            .button_position(position_origin)
            .hotkey(hotkey_q_weak)
            .build();
        let mut keys = CustomKeys::builder()
            .ability("Abu3", binding_abu3)
            .ability("Abu2", binding_abu2)
            .build();
        let layout = GridLayout::qwerty_grid();
        let moving = GridSlotId::ability("Abu3");
        let slot_ids = [GridSlotId::ability("Abu3")];
        let request = MoveRequest::new(layout, &slot_ids, &moving, 2, 1, false);
        keys.move_slot(&request);
        let sibling_binding = keys.binding("Abu2").expect("Abu2 exists");
        let sibling_button = sibling_binding
            .button_position()
            .expect("Abu2 Buttonpos set");
        assert_eq!(u8::from(sibling_button.column()), 2);
        assert_eq!(u8::from(sibling_button.row()), 1);
    }
}

#[cfg(test)]
mod extend_tests {
    use super::super::*;
    use crate::model::{AbilityBinding, CommandBinding, GridCoordinate, Hotkey, SystemBinding};
    use crate::model::{ColumnIndex, RowIndex};
    use warcraft_api::SystemKeybindClass;

    #[test]
    fn extend_copies_hotkey_from_source_to_target() {
        let target_hotkey = Hotkey::from('Q');
        let uploaded_hotkey = Hotkey::from('W');
        let target_binding = AbilityBinding::builder().hotkey(target_hotkey).build();
        let uploaded_binding = AbilityBinding::builder().hotkey(uploaded_hotkey).build();
        let mut target = CustomKeys::builder()
            .ability("Ahrl", target_binding)
            .build();
        let uploaded = CustomKeys::builder()
            .ability("Ahrl", uploaded_binding)
            .build();
        target.extend(uploaded);
        let expected_hotkey = Hotkey::Letter('W');
        assert_eq!(
            target.binding("Ahrl").and_then(|binding| binding.hotkey()),
            Some(&expected_hotkey),
        );
    }

    #[test]
    fn extend_copies_system_hotkey_from_source_to_target() {
        let default_binding = SystemBinding::new(
            Hotkey::VirtualKey(49),
            SystemKeybindClass::ControlGroup,
            None,
        );
        let imported_binding = SystemBinding::new(
            Hotkey::VirtualKey(186),
            SystemKeybindClass::ControlGroup,
            None,
        );
        let mut target = CustomKeys::builder()
            .system("Ctr1", default_binding)
            .build();
        let uploaded = CustomKeys::builder()
            .system("Ctr1", imported_binding)
            .build();
        target.extend(uploaded);
        let expected_hotkey = Hotkey::VirtualKey(186);
        let actual_hotkey = target.system("Ctr1").map(|binding| *binding.hotkey());
        assert_eq!(
            actual_hotkey,
            Some(expected_hotkey),
            "an imported system hotkey must overwrite the default during extend",
        );
    }

    #[test]
    fn imported_system_hotkeys_survive_normalize() {
        let imported_text = concat!(
            "[Ctr1]\nCtrlGroupCommand=1,1\nHotkey=186\n\n",
            "[itm1]\nGameCommand=1,1\nHotkey=222\n\n",
        );
        let uploaded = CustomKeys::parse_raw(imported_text);
        let mut baseline = CustomKeys::parse_raw(DEFAULT_CUSTOM_KEYS);
        baseline.extend(uploaded);
        let normalized = baseline.normalize();
        let control_group_hotkey = normalized.system("Ctr1").map(|binding| *binding.hotkey());
        let inventory_hotkey = normalized.system("itm1").map(|binding| *binding.hotkey());
        assert_eq!(
            control_group_hotkey,
            Some(Hotkey::VirtualKey(186)),
            "customized control-group hotkey must persist through the import normalize step",
        );
        assert_eq!(
            inventory_hotkey,
            Some(Hotkey::VirtualKey(222)),
            "customized inventory hotkey must persist through the import normalize step",
        );
    }

    #[test]
    fn extend_copies_unhotkey_from_source_to_target() {
        let target_unhotkey = Hotkey::from('W');
        let uploaded_unhotkey = Hotkey::from('C');
        let target_binding = AbilityBinding::builder().unhotkey(target_unhotkey).build();
        let uploaded_binding = AbilityBinding::builder()
            .unhotkey(uploaded_unhotkey)
            .build();
        let mut target = CustomKeys::builder()
            .ability("Amil", target_binding)
            .build();
        let uploaded = CustomKeys::builder()
            .ability("Amil", uploaded_binding)
            .build();
        target.extend(uploaded);
        let expected_unhotkey = Hotkey::Letter('C');
        assert_eq!(
            target
                .binding("Amil")
                .and_then(|binding| binding.unhotkey()),
            Some(&expected_unhotkey),
        );
    }

    #[test]
    fn extend_copies_button_position() {
        let target_position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
        let uploaded_position = GridCoordinate::new(ColumnIndex::Two, RowIndex::One);
        let target_binding = AbilityBinding::builder()
            .button_position(target_position)
            .build();
        let uploaded_binding = AbilityBinding::builder()
            .button_position(uploaded_position)
            .build();
        let mut target = CustomKeys::builder()
            .ability("Ahrl", target_binding)
            .build();
        let uploaded = CustomKeys::builder()
            .ability("Ahrl", uploaded_binding)
            .build();
        target.extend(uploaded);
        let position = target
            .binding("Ahrl")
            .and_then(|binding| binding.button_position())
            .copied();
        assert_eq!(
            position,
            Some(GridCoordinate::new(ColumnIndex::Two, RowIndex::One))
        );
    }

    #[test]
    fn extend_does_not_overwrite_system_entries() {
        let system_binding =
            SystemBinding::new(Hotkey::VirtualKey(27), SystemKeybindClass::Game, None);
        let mut target = CustomKeys::builder().system("IsS1", system_binding).build();
        let uploaded_hotkey = Hotkey::from('Q');
        let uploaded_binding = AbilityBinding::builder().hotkey(uploaded_hotkey).build();
        let uploaded = CustomKeys::builder()
            .ability("IsS1", uploaded_binding)
            .build();
        target.extend(uploaded);
        assert!(target.system("IsS1").is_some());
    }

    #[test]
    fn extend_skips_absent_fields() {
        let target_hotkey = Hotkey::from('Q');
        let uploaded_position = GridCoordinate::new(ColumnIndex::One, RowIndex::Zero);
        let target_binding = AbilityBinding::builder().hotkey(target_hotkey).build();
        let uploaded_binding = AbilityBinding::builder()
            .button_position(uploaded_position)
            .build();
        let mut target = CustomKeys::builder()
            .ability("Ahrl", target_binding)
            .build();
        let uploaded = CustomKeys::builder()
            .ability("Ahrl", uploaded_binding)
            .build();
        target.extend(uploaded);
        let expected_hotkey = Hotkey::Letter('Q');
        assert_eq!(
            target.binding("Ahrl").and_then(|binding| binding.hotkey()),
            Some(&expected_hotkey),
        );
        let position = target
            .binding("Ahrl")
            .and_then(|binding| binding.button_position())
            .copied();
        assert_eq!(
            position,
            Some(GridCoordinate::new(ColumnIndex::One, RowIndex::Zero)),
        );
    }

    #[test]
    fn extend_copies_command_hotkey() {
        let target_hotkey = Hotkey::from('A');
        let uploaded_hotkey = Hotkey::from('G');
        let target_binding = CommandBinding::builder().hotkey(target_hotkey).build();
        let uploaded_binding = CommandBinding::builder().hotkey(uploaded_hotkey).build();
        let mut target = CustomKeys::builder()
            .command("CmdAttack", target_binding)
            .build();
        let uploaded = CustomKeys::builder()
            .command("CmdAttack", uploaded_binding)
            .build();
        target.extend(uploaded);
        let expected_hotkey = Hotkey::Letter('G');
        assert_eq!(
            target
                .command("CmdAttack")
                .and_then(|binding| binding.hotkey()),
            Some(&expected_hotkey),
        );
    }

    #[test]
    fn extend_merges_by_canonical_id() {
        let target_hotkey = Hotkey::from('Q');
        let uploaded_hotkey = Hotkey::from('E');
        let target_binding = AbilityBinding::builder().hotkey(target_hotkey).build();
        let uploaded_binding = AbilityBinding::builder().hotkey(uploaded_hotkey).build();
        let mut target = CustomKeys::builder()
            .ability("Ahrl", target_binding)
            .build();
        let uploaded = CustomKeys::builder()
            .ability("Ahrl", uploaded_binding)
            .build();
        target.extend(uploaded);
        let expected_hotkey = Hotkey::Letter('E');
        assert_eq!(
            target.binding("Ahrl").and_then(|binding| binding.hotkey()),
            Some(&expected_hotkey),
        );
    }
}

#[cfg(test)]
mod export_tests {
    use crate::CustomKeys;

    #[test]
    fn empty_overlay_on_minimal_baseline_round_trips() {
        let baseline = "[Ahrl]\nHotkey=Q\nButtonpos=0,0\n\n";
        let loaded = CustomKeys::parse_raw("");
        let output = loaded.serialize(baseline);
        assert!(
            output.contains("[Ahrl]"),
            "baseline section should be present in output",
        );
        assert!(output.contains("Hotkey=Q"));
    }

    #[test]
    fn overlay_values_appear_in_export() {
        let baseline = "[Ahrl]\nHotkey=Q\n\n";
        let loaded = CustomKeys::parse_raw("[Ahrl]\nHotkey=W\n\n");
        let output = loaded.serialize(baseline);
        assert!(output.contains("Hotkey=W"), "user hotkey override must win");
    }

    #[test]
    fn export_with_real_baseline_contains_known_sections() {
        let baseline = include_str!("../../templates/CustomKeys.txt");
        let loaded = CustomKeys::parse_raw("");
        let output = loaded.serialize(baseline);
        for section in &["[Hpal]", "[CmdAttack]", "[CmdMove]"] {
            assert!(output.contains(section), "export should contain {section}");
        }
    }

    #[test]
    fn export_materializes_default_button_positions() {
        let baseline = include_str!("../../templates/CustomKeys.txt");
        let loaded = CustomKeys::parse_raw("");
        let output = loaded.serialize(baseline);
        let after_ahrl = output
            .split("[Ahrl]")
            .nth(1)
            .expect("[Ahrl] must be in output");
        let next_section = after_ahrl.split('[').next().unwrap_or(after_ahrl);
        assert!(
            next_section.contains("Buttonpos="),
            "[Ahrl] section must have a Buttonpos after materialization",
        );
    }

    #[test]
    fn export_assigns_positions_to_goblin_merchant_shop_items_without_db_positions() {
        let baseline = include_str!("../../templates/CustomKeys.txt");
        let loaded = CustomKeys::parse_raw("");
        let output = loaded.serialize(baseline);
        for item_id in &["bspd", "spro", "pinv"] {
            let section_marker = format!("[{item_id}]");
            let after_section = output
                .to_ascii_lowercase()
                .split(section_marker.as_str())
                .nth(1)
                .unwrap_or("")
                .to_string();
            let before_next_section = after_section.split('[').next().unwrap_or("").to_string();
            assert!(
                before_next_section.contains("buttonpos="),
                "[{item_id}] must have a Buttonpos assigned by shop item materialization",
            );
        }
    }

    #[test]
    fn export_assigns_position_to_goblin_shredder_sell_unit_without_db_position() {
        let baseline = include_str!("../../templates/CustomKeys.txt");
        let loaded = CustomKeys::parse_raw("");
        let output = loaded.serialize(baseline);
        let lowercase_output = output.to_ascii_lowercase();
        let after_ngir = lowercase_output
            .split("[ngir]")
            .nth(1)
            .expect("[ngir] must be in output after sell_unit materialization");
        let before_next_section = after_ngir.split('[').next().unwrap_or(after_ngir);
        assert!(
            before_next_section.contains("buttonpos="),
            "[ngir] must have a Buttonpos assigned by sell_unit materialization",
        );
    }
}

#[cfg(test)]
mod normalize_tests {
    use crate::CustomKeys;
    use crate::model::{ColumnIndex, GridCoordinate, Hotkey, RowIndex};

    #[test]
    fn normalize_produces_non_empty_text() {
        let normalized = CustomKeys::from_text("");
        let normalized_text = normalized.to_string();
        assert!(!normalized_text.is_empty());
    }

    #[test]
    fn normalize_syncs_single_button_toggle_offstate_to_onstate() {
        use crate::identity::slot::GridSlotId;
        use crate::model::AbilityBinding;
        let on_position = GridCoordinate::new(ColumnIndex::Two, RowIndex::Two);
        let off_position = GridCoordinate::new(ColumnIndex::One, RowIndex::Two);
        let binding = AbilityBinding::builder()
            .button_position(on_position)
            .unbutton_position(off_position)
            .build();
        let mut overlay = CustomKeys::parse_raw("");
        overlay.put_ability("ACf2", binding);
        let normalized = overlay.normalize();
        let resolved_on = normalized.position_for_slot(&GridSlotId::ability("ACf2"), false);
        let resolved_off = normalized.position_for_slot(&GridSlotId::ability_off("ACf2"), false);
        assert_eq!(resolved_on, Some(on_position));
        assert_eq!(
            resolved_off, resolved_on,
            "autocast Frost Armor off-state must mirror its on-state after normalize",
        );
    }

    #[test]
    fn normalize_mirrors_morph_ability_onto_produced_unit_command() {
        use crate::model::AbilityBinding;
        let morph_position = GridCoordinate::new(ColumnIndex::Two, RowIndex::One);
        let aave_binding = AbilityBinding::builder()
            .hotkey(Hotkey::Letter('R'))
            .button_position(morph_position)
            .build();
        let stale_position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
        let ubsp_binding = AbilityBinding::builder()
            .hotkey(Hotkey::Letter('T'))
            .button_position(stale_position)
            .build();
        let mut overlay = CustomKeys::parse_raw("");
        overlay.put_ability("Aave", aave_binding);
        overlay.put_ability("ubsp", ubsp_binding);
        let normalized = overlay.normalize();
        let produced_unit = normalized.binding("ubsp").expect("ubsp section exists");
        let expected_hotkey = Hotkey::Letter('R');
        assert_eq!(produced_unit.hotkey(), Some(&expected_hotkey));
        assert_eq!(produced_unit.button_position(), Some(&morph_position));
    }

    #[test]
    fn normalize_keeps_independent_offstate_separate() {
        use crate::identity::slot::GridSlotId;
        use crate::model::AbilityBinding;
        let on_position = GridCoordinate::new(ColumnIndex::Two, RowIndex::Two);
        let off_position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::One);
        let binding = AbilityBinding::builder()
            .button_position(on_position)
            .unbutton_position(off_position)
            .build();
        let mut overlay = CustomKeys::parse_raw("");
        overlay.put_ability("Abur", binding);
        let normalized = overlay.normalize();
        let resolved_off = normalized.position_for_slot(&GridSlotId::ability_off("Abur"), false);
        assert_eq!(
            resolved_off,
            Some(off_position),
            "morph Burrow off-state is a separate button and must keep its own position",
        );
    }

    #[test]
    fn normalize_includes_known_baseline_sections() {
        let normalized = CustomKeys::from_text("");
        let normalized_text = normalized.to_string();
        assert!(normalized_text.contains("[Hpal]"));
        assert!(normalized_text.contains("[CmdAttack]"));
    }

    #[test]
    fn normalize_prunes_non_button_shop_mechanics() {
        let normalized = CustomKeys::from_text("");
        for phantom_id in ["Aall", "Aneu", "Ane2", "Adt1"] {
            assert!(
                normalized.binding(phantom_id).is_none(),
                "normalize must prune non-button mechanic {phantom_id}",
            );
        }
        assert!(
            normalized.binding("Anei").is_some(),
            "normalize must keep Select User (Anei), a real shop button",
        );
    }

    #[test]
    fn normalize_prunes_phantom_from_uploaded_file() {
        let uploaded = "[Aall]\nHotkey=Q\nButtonpos=0,0\n";
        let normalized = CustomKeys::from_text(uploaded);
        assert!(
            normalized.binding("Aall").is_none(),
            "normalize must strip a phantom [Aall] carried in over an upload",
        );
    }

    #[test]
    fn normalize_is_idempotent() {
        let first_text = CustomKeys::from_text("").to_string();
        let second_text = CustomKeys::from_text(first_text.as_str()).to_string();
        assert_eq!(first_text, second_text);
    }

    #[test]
    fn normalize_includes_known_ability() {
        let normalized = CustomKeys::from_text("");
        let hpal_present = normalized.binding("Hpal").is_some();
        assert!(hpal_present);
    }

    #[test]
    fn normalize_overlays_user_hotkey_on_baseline() {
        let user_input = "[Ahrl]\nHotkey=Z\n\n";
        let normalized = CustomKeys::from_text(user_input);
        let ahrl_binding = normalized.binding("Ahrl");
        let ahrl_hotkey = ahrl_binding.and_then(|binding| binding.hotkey());
        let expected_hotkey = Hotkey::Letter('Z');
        assert_eq!(ahrl_hotkey, Some(&expected_hotkey));
    }

    #[test]
    fn normalize_materializes_button_position_for_known_ability() {
        let normalized = CustomKeys::from_text("");
        let normalized_text = normalized.to_string();
        let ahrl_marker = "[Ahrl]";
        let ahrl_section_start = normalized_text
            .find(ahrl_marker)
            .expect("baseline must contain [Ahrl]");
        let after_ahrl = &normalized_text[ahrl_section_start + ahrl_marker.len()..];
        let next_section_length = after_ahrl.find('[').unwrap_or(after_ahrl.len());
        let ahrl_section = &after_ahrl[..next_section_length];
        assert!(
            ahrl_section.contains("Buttonpos="),
            "[Ahrl] section must have a concrete Buttonpos after normalize",
        );
    }

    #[test]
    fn normalize_assigns_positions_to_goblin_merchant_sell_items_without_template_positions() {
        let normalized = CustomKeys::from_text("");
        for item_id in &["bspd", "spro", "pinv"] {
            let binding = normalized.binding(*item_id);
            let button_position = binding.and_then(|binding| binding.button_position());
            assert!(
                button_position.is_some(),
                "[{item_id}] must have a button_position in the normalized output",
            );
        }
    }

    #[test]
    fn normalize_mirrors_build_command_position_and_hotkey_to_build_ability() {
        let uploaded = "[CmdBuildHuman]\nHotkey=Q\nButtonpos=3,1\n";
        let normalized = CustomKeys::from_text(uploaded);
        let ability_binding = normalized
            .binding("AHbu")
            .expect("build ability AHbu must exist after normalize");
        let mirrored_position = ability_binding.button_position();
        let expected_position = GridCoordinate::new(ColumnIndex::Three, RowIndex::One);
        assert_eq!(
            mirrored_position,
            Some(&expected_position),
            "AHbu must mirror the build command's Buttonpos",
        );
        let mirrored_hotkey = ability_binding.hotkey();
        let expected_hotkey = Hotkey::Letter('Q');
        assert_eq!(
            mirrored_hotkey,
            Some(&expected_hotkey),
            "AHbu must mirror the build command's Hotkey",
        );
    }

    #[test]
    fn normalize_mirrors_build_command_to_ability_for_every_race() {
        let uploaded = "[CmdBuildOrc]\nButtonpos=2,1\n\n[CmdBuildUndead]\nButtonpos=2,1\n\n[CmdBuildNightElf]\nButtonpos=2,1\n";
        let normalized = CustomKeys::from_text(uploaded);
        let expected_position = GridCoordinate::new(ColumnIndex::Two, RowIndex::One);
        for ability_id in &["AObu", "AUbu", "AEbu"] {
            let ability_binding = normalized
                .binding(*ability_id)
                .unwrap_or_else(|| panic!("build ability {ability_id} must exist after normalize"));
            let mirrored_position = ability_binding.button_position();
            assert_eq!(
                mirrored_position,
                Some(&expected_position),
                "{ability_id} must mirror its build command's Buttonpos",
            );
        }
    }

    #[test]
    fn build_ability_section_survives_parse_round_trip() {
        let uploaded = "[CmdBuildHuman]\nHotkey=Q\nButtonpos=3,1\n";
        let canonical_once = CustomKeys::from_text(uploaded).to_string();
        let canonical_twice = CustomKeys::parse_raw(canonical_once.as_str()).to_string();
        assert!(
            canonical_once.contains("[AHbu]"),
            "normalized output must contain the mirrored [AHbu] section",
        );
        assert_eq!(
            canonical_once, canonical_twice,
            "the mirrored build ability must survive a parse/serialize round trip",
        );
    }

    #[test]
    fn normalize_assigns_position_to_goblin_shredder_sell_unit() {
        let normalized = CustomKeys::from_text("");
        let binding = normalized.binding("ngir");
        let button_position = binding.and_then(|binding| binding.button_position());
        assert!(
            button_position.is_some(),
            "[ngir] (Goblin Shredder) must have a button_position in the normalized output",
        );
    }

    #[test]
    fn normalize_defaults_button_position_to_origin_when_database_has_no_position() {
        let normalized = CustomKeys::from_text("");
        let binding = normalized
            .binding("Aatp")
            .expect("Aatp must have a binding after normalize");
        let button_position = binding
            .button_position()
            .expect("Aatp must have a fallback button_position");
        let origin = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
        assert_eq!(*button_position, origin);
    }

    #[test]
    fn normalize_does_not_invent_off_state_for_one_shot_ability() {
        let normalized = CustomKeys::from_text("");
        let healing_wave_off = normalized
            .binding("AChv")
            .and_then(|binding| binding.unbutton_position());
        assert!(
            healing_wave_off.is_none(),
            "AChv has no off-state — normalize must not invent an unbutton_position",
        );
    }

    #[test]
    fn move_slot_co_moves_colocated_offstate_when_slot_ids_lack_abilityoff_variant() {
        use crate::command::move_request::MoveRequest;
        use crate::grid::layout::GridLayout;
        use crate::identity::slot::GridSlotId;
        let input = "[ACsw]\nButtonpos=0,0\nHotkey=Q\nUnbuttonpos=0,0\nUnhotkey=Q\n";
        let mut keys = CustomKeys::parse_raw(input);
        let layout = GridLayout::qwerty_grid();
        let moving = GridSlotId::ability("ACsw");
        let slot_ids = [GridSlotId::ability("ACsw")];
        let request = MoveRequest::new(layout, &slot_ids, &moving, 1, 0, false);
        keys.move_slot(&request);
        let binding = keys.binding("ACsw").expect("ACsw must exist");
        let button_position = binding.button_position().expect("Buttonpos set");
        let unbutton_position = binding
            .unbutton_position()
            .expect("Unbuttonpos must follow");
        assert_eq!(
            u8::from(button_position.column()),
            1,
            "ability must move to column 1",
        );
        assert_eq!(
            u8::from(button_position.row()),
            0,
            "ability must move to row 0"
        );
        assert_eq!(
            unbutton_position, button_position,
            "Unbuttonpos must co-move with Buttonpos",
        );
    }

    #[test]
    fn move_slot_swaps_both_colocated_offstates_when_two_toggle_abilities_are_swapped() {
        use crate::command::move_request::MoveRequest;
        use crate::grid::layout::GridLayout;
        use crate::identity::slot::GridSlotId;
        let input = concat!(
            "[ACsw]\nButtonpos=0,0\nHotkey=Q\nUnbuttonpos=0,0\nUnhotkey=Q\n",
            "[ACdm]\nButtonpos=1,0\nHotkey=W\nUnbuttonpos=1,0\nUnhotkey=W\n",
        );
        let mut keys = CustomKeys::parse_raw(input);
        let layout = GridLayout::qwerty_grid();
        let moving = GridSlotId::ability("ACsw");
        let slot_ids = [GridSlotId::ability("ACsw"), GridSlotId::ability("ACdm")];
        let request = MoveRequest::new(layout, &slot_ids, &moving, 1, 0, false);
        keys.move_slot(&request);
        let acsw = keys.binding("ACsw").expect("ACsw must exist");
        let acsw_button = acsw.button_position().expect("ACsw Buttonpos set");
        let acsw_unbutton = acsw
            .unbutton_position()
            .expect("ACsw Unbuttonpos must follow");
        assert_eq!(
            u8::from(acsw_button.column()),
            1,
            "ACsw must move to column 1"
        );
        assert_eq!(
            acsw_unbutton, acsw_button,
            "ACsw Unbuttonpos must co-move with Buttonpos",
        );
        let acdm = keys.binding("ACdm").expect("ACdm must exist");
        let acdm_button = acdm.button_position().expect("ACdm Buttonpos set");
        let acdm_unbutton = acdm
            .unbutton_position()
            .expect("ACdm Unbuttonpos must follow");
        assert_eq!(
            u8::from(acdm_button.column()),
            0,
            "ACdm must be displaced to column 0",
        );
        assert_eq!(
            acdm_unbutton, acdm_button,
            "ACdm Unbuttonpos must co-move with Buttonpos",
        );
    }

    #[test]
    fn move_slot_does_not_co_move_offstate_when_not_colocated() {
        use crate::command::move_request::MoveRequest;
        use crate::grid::layout::GridLayout;
        use crate::identity::slot::GridSlotId;
        let input = concat!(
            "[ACsw]\nButtonpos=0,0\nHotkey=Q\nUnbuttonpos=2,0\nUnhotkey=E\n",
            "[ACdm]\nButtonpos=1,0\nHotkey=W\n",
        );
        let mut keys = CustomKeys::parse_raw(input);
        let layout = GridLayout::qwerty_grid();
        let moving = GridSlotId::ability("ACsw");
        let slot_ids = [GridSlotId::ability("ACsw"), GridSlotId::ability("ACdm")];
        let request = MoveRequest::new(layout, &slot_ids, &moving, 1, 0, false);
        keys.move_slot(&request);
        let acsw = keys.binding("ACsw").expect("ACsw must exist");
        let acsw_unbutton = acsw
            .unbutton_position()
            .expect("Unbuttonpos must be preserved");
        assert_eq!(
            u8::from(acsw_unbutton.column()),
            2,
            "non-colocated Unbuttonpos must stay at column 2",
        );
        assert_eq!(
            u8::from(acsw_unbutton.row()),
            0,
            "non-colocated Unbuttonpos must stay at row 0",
        );
    }

    #[test]
    fn resolve_conflicts_co_moves_off_state_with_ability() {
        use crate::model::{ColumnIndex, GridCoordinate, RowIndex};
        let mut keys = CustomKeys::from_text("");
        let normalized_position = keys
            .binding("ACsw")
            .and_then(|binding| binding.button_position())
            .copied();
        let default_slow_position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Two);
        assert_eq!(
            normalized_position,
            Some(default_slow_position),
            "ACsw must start at (0,2) after normalize",
        );
        let _plan = keys.resolve_conflicts();
        let binding = keys
            .binding("ACsw")
            .expect("ACsw must remain after resolve");
        let button_position = binding.button_position().copied();
        let unbutton_position = binding.unbutton_position().copied();
        assert_ne!(
            button_position,
            Some(default_slow_position),
            "ACsw must have been moved by the cascade",
        );
        assert_eq!(
            unbutton_position, button_position,
            "ACsw off-state must be co-located with on-state after resolve_conflicts",
        );
    }

    #[test]
    fn resolve_conflicts_produces_at_least_one_move_on_default_keys() {
        let mut normalized = CustomKeys::from_text("");
        let plan = normalized.resolve_conflicts();
        assert!(
            plan.move_count() > 0,
            "default keys have known collisions so resolve_conflicts must produce moves",
        );
    }

    #[test]
    fn resolve_conflicts_is_idempotent_on_default_keys() {
        let mut keys = CustomKeys::from_text("");
        let first_plan = keys.resolve_conflicts();
        assert!(first_plan.move_count() > 0, "first call must make moves");
        let second_plan = keys.resolve_conflicts();
        if second_plan.move_count() != 0 {
            let mut lines: Vec<String> = Vec::new();
            for planned_move in second_plan.moves() {
                let line = format!(
                    "  {} {:?} ({},{}) -> ({},{})",
                    planned_move.slot_id().as_str(),
                    planned_move.grid_role(),
                    u8::from(planned_move.old_position().column()),
                    u8::from(planned_move.old_position().row()),
                    u8::from(planned_move.new_position().column()),
                    u8::from(planned_move.new_position().row()),
                );
                lines.push(line);
            }
            panic!(
                "second resolve_conflicts call produced {} moves:\n{}",
                second_plan.move_count(),
                lines.join("\n"),
            );
        }
    }

    #[test]
    fn resolve_conflicts_writes_new_positions_into_bindings() {
        use crate::identity::slot::GridSlotId;
        let mut keys = CustomKeys::from_text("");
        let plan = keys.resolve_conflicts();
        for planned_move in plan.moves() {
            let slot = planned_move.slot_id();
            let expected = planned_move.new_position();
            let stored = match slot {
                GridSlotId::Ability(ability_id) => keys
                    .binding(ability_id)
                    .and_then(|binding| {
                        if planned_move.grid_role().is_research_context() {
                            binding.research_button_position()
                        } else {
                            binding.button_position()
                        }
                    })
                    .copied(),
                GridSlotId::AbilityOff(ability_id) => keys
                    .binding(ability_id)
                    .and_then(|binding| binding.unbutton_position())
                    .copied(),
                GridSlotId::Command(command_id) => keys
                    .command(command_id.value())
                    .and_then(|binding| binding.button_position())
                    .copied(),
            };
            assert_eq!(
                stored,
                Some(expected),
                "{} must have its new position written back to the binding",
                slot.as_str(),
            );
        }
    }

    #[test]
    fn preview_resolve_does_not_mutate_self() {
        let keys = CustomKeys::from_text("");
        let before_text = keys.to_string();
        let plan = keys.preview_resolve();
        let after_text = keys.to_string();
        assert!(
            plan.move_count() > 0,
            "default keys must produce moves for this test to be meaningful",
        );
        assert_eq!(
            before_text, after_text,
            "preview_resolve must not modify the receiver — serialized text changed",
        );
    }

    #[test]
    fn preview_resolve_matches_resolve_conflicts_plan_byte_for_byte() {
        let mut keys_for_apply = CustomKeys::from_text("");
        let keys_for_preview = keys_for_apply.clone();
        let preview_plan = keys_for_preview.preview_resolve();
        let applied_plan = keys_for_apply.resolve_conflicts();
        let preview_text = preview_plan.to_string();
        let applied_text = applied_plan.to_string();
        assert_eq!(
            preview_text, applied_text,
            "preview_resolve and resolve_conflicts must produce identical plans",
        );
    }

    #[test]
    fn resolve_conflicts_final_state_matches_preview_apply_endpoint() {
        let mut keys = CustomKeys::from_text("");
        let preview_plan = keys.preview_resolve();
        assert!(
            preview_plan.move_count() > 0,
            "default keys must produce moves for this test to be meaningful",
        );
        let applied_plan = keys.resolve_conflicts();
        assert_eq!(
            preview_plan.move_count(),
            applied_plan.move_count(),
            "preview move count must match the applied plan move count",
        );
        let second_preview_plan = keys.preview_resolve();
        assert_eq!(
            second_preview_plan.move_count(),
            0,
            "preview after resolve_conflicts must produce zero further moves",
        );
    }

    #[test]
    fn resolve_conflicts_eliminates_intra_unit_collisions_too() {
        use crate::cascade::conflict_graph::ConflictGraph;
        use crate::cascade::planner::CascadePlan;
        use crate::cascade::queue::{AssignmentQueue, AssignmentScope};
        let mut keys = CustomKeys::from_text("");
        let _plan = keys.resolve_conflicts();
        let graph = ConflictGraph::build(&keys);
        let queue = AssignmentQueue::build_with_scope(graph, AssignmentScope::IncludingIntraUnit);
        let plan = CascadePlan::from(&queue);
        let unresolved: std::collections::HashSet<usize> = plan
            .unresolved()
            .iter()
            .filter_map(|mover| {
                queue.graph().nodes().iter().position(|node| {
                    node.slot_id() == mover.slot_id() && node.grid_role() == mover.grid_role()
                })
            })
            .collect();
        let graph_ref = queue.graph();
        for (first_index, first_node) in graph_ref.nodes().iter().enumerate() {
            if unresolved.contains(&first_index) {
                continue;
            }
            for &second_index in graph_ref.neighbors(first_index) {
                if second_index <= first_index {
                    continue;
                }
                if unresolved.contains(&second_index) {
                    continue;
                }
                let second_node = graph_ref.node(second_index);
                let first_position = queue.final_position(first_index);
                let second_position = queue.final_position(second_index);
                let same_role = first_node.grid_role() == second_node.grid_role();
                assert!(
                    first_position != second_position || !same_role,
                    "intra/cross-unit collision survives resolve_conflicts: {} and {} at ({},{})",
                    first_node.slot_id().as_str(),
                    second_node.slot_id().as_str(),
                    u8::from(first_position.column()),
                    u8::from(first_position.row()),
                );
            }
        }
    }

    #[test]
    fn destroyer_intra_unit_collision_produces_minimal_displacement() {
        let mut keys = CustomKeys::from_text("");
        let _plan = keys.resolve_conflicts();

        use crate::cascade::conflict_graph::ConflictGraph;
        use crate::unit::grids::GridRole;
        let graph = ConflictGraph::build(&keys);
        let check = |ability: &str, expected_column: u8, expected_row: u8| {
            let index = graph
                .find_node(ability, GridRole::MainCommand)
                .unwrap_or_else(|| panic!("{ability} not found in conflict graph"));
            let position = graph.node(index).current_position();
            let column = u8::from(position.column());
            let row = u8::from(position.row());
            assert_eq!(
                column, expected_column,
                "{ability} expected column {expected_column}, got {column}",
            );
            assert_eq!(
                row, expected_row,
                "{ability} expected row {expected_row}, got {row}",
            );
        };
        check("Advm", 0, 2);
        check("Afak", 1, 2);
        check("Aabs", 3, 2);
    }

    #[test]
    fn resolve_conflicts_cascades_origin_default_to_leftmost_free_cell() {
        let mut keys = CustomKeys::from_text("");
        let _plan = keys.resolve_conflicts();
        let binding = keys.binding("Aatp").expect("Aatp must have a binding");
        let position = binding
            .button_position()
            .expect("Aatp must have a button_position after resolve");
        let column = u8::from(position.column());
        let row = u8::from(position.row());
        assert_eq!(
            (column, row),
            (1, 1),
            "Aatp expected to cascade to (1,1), got ({column},{row})",
        );
    }

    #[test]
    fn resolved_default_customkeys_matches_snapshot() {
        let mut keys = CustomKeys::from_text("");
        let _plan = keys.resolve_conflicts();
        let actual = keys.to_string();
        let expected = include_str!("../../fixtures/resolved_default_customkeys.txt");
        if actual != expected {
            let actual_bytes = actual.len();
            let expected_bytes = expected.len();
            let mut first_difference_offset: Option<usize> = None;
            for (offset, (actual_char, expected_char)) in
                actual.chars().zip(expected.chars()).enumerate()
            {
                if actual_char != expected_char {
                    first_difference_offset = Some(offset);
                    break;
                }
            }
            panic!(
                "resolved default CustomKeys drifted from snapshot \
                 (actual={actual_bytes}B, expected={expected_bytes}B, \
                 first diff at char {first_difference_offset:?}). \
                 To accept the new output, regenerate the snapshot via the CLI — \
                 see the test source for the exact command.",
            );
        }
    }

    #[test]
    fn canonical_text_round_trips_through_parser() {
        let canonical = include_str!("../../fixtures/resolved_default_customkeys.txt");
        let reparsed = CustomKeys::parse_raw(canonical);
        let serialized = reparsed.to_string();
        if serialized != canonical {
            let serialized_bytes = serialized.len();
            let canonical_bytes = canonical.len();
            let mut first_difference_offset: Option<usize> = None;
            for (offset, (serialized_char, canonical_char)) in
                serialized.chars().zip(canonical.chars()).enumerate()
            {
                if serialized_char != canonical_char {
                    first_difference_offset = Some(offset);
                    break;
                }
            }
            let difference_offset = first_difference_offset.unwrap_or(0);
            let window_start = difference_offset.saturating_sub(60);
            let serialized_window: String =
                serialized.chars().skip(window_start).take(160).collect();
            let canonical_window: String = canonical.chars().skip(window_start).take(160).collect();
            panic!(
                "canonical CustomKeys did not survive a parse/serialize round trip \
                 (serialized={serialized_bytes}B, canonical={canonical_bytes}B, \
                 first diff at char {first_difference_offset:?}).\n\
                 canonical near diff:\n{canonical_window}\n\
                 serialized near diff:\n{serialized_window}",
            );
        }
    }

    #[test]
    fn canonical_form_is_idempotent() {
        let edited_overlay = "[acad]\nHotkey=Q\nButtonpos=0,0\n";
        let overlay_keys = CustomKeys::from_text(edited_overlay);
        let mut resolved_keys = overlay_keys;
        let _plan = resolved_keys.resolve_conflicts();
        let canonical_once = resolved_keys.to_string();
        let reparsed_keys = CustomKeys::parse_raw(canonical_once.as_str());
        let canonical_twice = reparsed_keys.to_string();
        assert_eq!(
            canonical_once, canonical_twice,
            "canonical form is not a fixed point: re-parsing canonical output \
             and re-serializing produced different bytes",
        );
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod template_generation_tests {
    use super::super::CustomKeys;
    use crate::grid::layout::GridLayout;
    use warcraft_api::WarcraftObjectMeta;
    use warcraft_database::ObjectLookup;
    use warcraft_database::{WARCRAFT_DATABASE, WARCRAFT_SYSTEM_KEYBINDS};

    fn join_levels(levels: &[&str]) -> Option<String> {
        if levels.is_empty() {
            None
        } else {
            Some(levels.join(","))
        }
    }

    fn build_text(layout: &GridLayout) -> String {
        let tmpl = CustomKeys::parse_raw(super::super::DEFAULT_CUSTOM_KEYS);
        let mut out = String::new();
        for (object_id, warcraft_object) in WARCRAFT_DATABASE.iter() {
            let id = object_id.value();
            let WarcraftObjectMeta::Command(cmd_meta) = warcraft_object.meta() else {
                continue;
            };
            let Some(default_position) = cmd_meta.default_button_position() else {
                continue;
            };
            let traditional = tmpl.command(id);
            let section_header = format!("[{id}]\n");
            out.push_str(&section_header);
            if let Some(hotkey_string) = traditional
                .and_then(|command| command.hotkey())
                .map(|hotkey_display| hotkey_display.to_string())
            {
                let hotkey_line = format!("Hotkey={hotkey_string}\n");
                out.push_str(&hotkey_line);
            }
            let default_column = u8::from(default_position.column());
            let default_row = u8::from(default_position.row());
            let buttonpos_line = format!("Buttonpos={default_column},{default_row}\n");
            out.push_str(&buttonpos_line);
            if let Some(tip) = traditional
                .and_then(|command| command.tip())
                .map(str::to_owned)
                .or_else(|| join_levels(warcraft_object.tip_levels()))
            {
                let tip_line = format!("Tip={tip}\n");
                out.push_str(&tip_line);
            }
            if let Some(ubertip) = warcraft_object.ubertip() {
                let ubertip_line = format!("Ubertip={ubertip}\n");
                out.push_str(&ubertip_line);
            }
            out.push('\n');
        }
        for (object_id, warcraft_object) in WARCRAFT_DATABASE.iter() {
            let id = object_id.value();
            let WarcraftObjectMeta::Ability(ability_meta) = warcraft_object.meta() else {
                continue;
            };
            let default_button_position = warcraft_object.default_button_position();
            let default_research_position = warcraft_object.default_research_button_position();
            let off_button_position = ability_meta.off_button_position();
            if default_button_position.is_none()
                && default_research_position.is_none()
                && off_button_position.is_none()
            {
                continue;
            }
            let is_passive = ObjectLookup::is_passive_ability(id);
            let existing_binding = tmpl.binding(id);
            let section_header = format!("[{id}]\n");
            out.push_str(&section_header);
            if let Some(button_position) = default_button_position {
                if !is_passive {
                    let hotkey = existing_binding
                        .and_then(|binding| binding.hotkey())
                        .map(|hotkey_display| hotkey_display.to_string())
                        .or_else(|| {
                            layout
                                .letter_at(button_position.column(), button_position.row())
                                .map(|letter| letter.to_string())
                        });
                    if let Some(hotkey_string) = hotkey {
                        let hotkey_line = format!("Hotkey={hotkey_string}\n");
                        out.push_str(&hotkey_line);
                    }
                }
                let btn_column = u8::from(button_position.column());
                let btn_row = u8::from(button_position.row());
                let buttonpos_line = format!("Buttonpos={btn_column},{btn_row}\n");
                out.push_str(&buttonpos_line);
            }
            if let Some(research_position) = default_research_position {
                let research_hotkey = existing_binding
                    .and_then(|binding| binding.research_hotkey())
                    .map(|hotkey_display| hotkey_display.to_string())
                    .or_else(|| {
                        layout
                            .letter_at(research_position.column(), research_position.row())
                            .map(|letter| letter.to_string())
                    });
                if let Some(research_hotkey_string) = research_hotkey {
                    let research_hotkey_line =
                        format!("ResearchHotkey={research_hotkey_string}\n",);
                    out.push_str(&research_hotkey_line);
                }
                let res_column = u8::from(research_position.column());
                let res_row = u8::from(research_position.row());
                let research_buttonpos_line =
                    format!("ResearchButtonpos={res_column},{res_row}\n",);
                out.push_str(&research_buttonpos_line);
            }
            if let Some(off_position) = off_button_position {
                let un_hotkey = existing_binding
                    .and_then(|binding| binding.unhotkey())
                    .map(|hotkey_display| hotkey_display.to_string())
                    .or_else(|| {
                        layout
                            .letter_at(off_position.column(), off_position.row())
                            .map(|letter| letter.to_string())
                    });
                if let Some(unhotkey_string) = un_hotkey {
                    let unhotkey_line = format!("Unhotkey={unhotkey_string}\n");
                    out.push_str(&unhotkey_line);
                }
                let off_column = u8::from(off_position.column());
                let off_row = u8::from(off_position.row());
                let unbuttonpos_line = format!("Unbuttonpos={off_column},{off_row}\n");
                out.push_str(&unbuttonpos_line);
            }
            if let Some(tip) = existing_binding
                .and_then(|binding| binding.tip())
                .map(str::to_owned)
                .or_else(|| join_levels(warcraft_object.tip_levels()))
            {
                let tip_line = format!("Tip={tip}\n");
                out.push_str(&tip_line);
            }
            if let Some(un_tip) = existing_binding
                .and_then(|binding| binding.un_tip())
                .map(str::to_owned)
                .or_else(|| warcraft_object.un_tip().map(str::to_owned))
            {
                let untip_line = format!("Untip={un_tip}\n");
                out.push_str(&untip_line);
            }
            if let Some(ubertip) = existing_binding
                .and_then(|binding| binding.ubertip())
                .map(str::to_owned)
                .or_else(|| join_levels(warcraft_object.ubertip_levels()))
            {
                let ubertip_line = format!("Ubertip={ubertip}\n");
                out.push_str(&ubertip_line);
            }
            if let Some(un_ubertip) = existing_binding
                .and_then(|binding| binding.un_ubertip())
                .map(str::to_owned)
                .or_else(|| warcraft_object.un_ubertip().map(str::to_owned))
            {
                let un_ubertip_line = format!("Unubertip={un_ubertip}\n");
                out.push_str(&un_ubertip_line);
            }
            if let Some(research_ubertip) = existing_binding
                .and_then(|binding| binding.research_ubertip())
                .map(str::to_owned)
                .or_else(|| warcraft_object.research_ubertip().map(str::to_owned))
            {
                let research_ubertip_line = format!("Researchubertip={research_ubertip}\n",);
                out.push_str(&research_ubertip_line);
            }
            out.push('\n');
        }
        for (object_id, warcraft_object) in WARCRAFT_DATABASE.iter() {
            let id = object_id.value();
            let WarcraftObjectMeta::Unit(_) = warcraft_object.meta() else {
                continue;
            };
            let Some(default_position) = warcraft_object.default_button_position() else {
                continue;
            };
            let existing_binding = tmpl.binding(id);
            let section_header = format!("[{id}]\n");
            out.push_str(&section_header);
            if let Some(hotkey_string) = existing_binding
                .and_then(|binding| binding.hotkey())
                .map(|hotkey_display| hotkey_display.to_string())
            {
                let hotkey_line = format!("Hotkey={hotkey_string}\n");
                out.push_str(&hotkey_line);
            }
            let cmd_column = u8::from(default_position.column());
            let cmd_row = u8::from(default_position.row());
            let buttonpos_line = format!("Buttonpos={cmd_column},{cmd_row}\n");
            out.push_str(&buttonpos_line);
            if let Some(tip) = existing_binding
                .and_then(|binding| binding.tip())
                .map(str::to_owned)
                .or_else(|| join_levels(warcraft_object.tip_levels()))
            {
                let tip_line = format!("Tip={tip}\n");
                out.push_str(&tip_line);
            }
            if let Some(ubertip) = existing_binding
                .and_then(|binding| binding.ubertip())
                .map(str::to_owned)
                .or_else(|| join_levels(warcraft_object.ubertip_levels()))
            {
                let ubertip_line = format!("Ubertip={ubertip}\n");
                out.push_str(&ubertip_line);
            }
            out.push('\n');
        }
        for (object_id, warcraft_object) in WARCRAFT_DATABASE.iter() {
            let id = object_id.value();
            if !matches!(
                warcraft_object.meta(),
                WarcraftObjectMeta::Upgrade(_) | WarcraftObjectMeta::Item(_)
            ) {
                continue;
            }
            let Some(default_position) = warcraft_object.default_button_position() else {
                continue;
            };
            let research_position = warcraft_object.default_research_button_position();
            let existing_binding = tmpl.binding(id);
            let section_header = format!("[{id}]\n");
            out.push_str(&section_header);
            let hotkey = existing_binding
                .and_then(|binding| binding.hotkey())
                .map(|hotkey_display| hotkey_display.to_string())
                .or_else(|| {
                    layout
                        .letter_at(default_position.column(), default_position.row())
                        .map(|letter| letter.to_string())
                });
            if let Some(hotkey_string) = hotkey {
                let hotkey_line = format!("Hotkey={hotkey_string}\n");
                out.push_str(&hotkey_line);
            }
            let upg_column = u8::from(default_position.column());
            let upg_row = u8::from(default_position.row());
            let buttonpos_line = format!("Buttonpos={upg_column},{upg_row}\n");
            out.push_str(&buttonpos_line);
            if let Some(research_button_position) = research_position {
                let research_hotkey_string = existing_binding
                    .and_then(|binding| binding.research_hotkey())
                    .map(|hotkey_display| hotkey_display.to_string())
                    .or_else(|| {
                        layout
                            .letter_at(
                                research_button_position.column(),
                                research_button_position.row(),
                            )
                            .map(|letter| letter.to_string())
                    });
                if let Some(research_hotkey_line) = research_hotkey_string {
                    let research_hotkey_line = format!("ResearchHotkey={research_hotkey_line}\n",);
                    out.push_str(&research_hotkey_line);
                }
                let res_btn_column = u8::from(research_button_position.column());
                let res_btn_row = u8::from(research_button_position.row());
                let research_buttonpos_line =
                    format!("ResearchButtonpos={res_btn_column},{res_btn_row}\n",);
                out.push_str(&research_buttonpos_line);
            }
            if let Some(tip) = existing_binding
                .and_then(|binding| binding.tip())
                .map(str::to_owned)
                .or_else(|| join_levels(warcraft_object.tip_levels()))
            {
                let tip_line = format!("Tip={tip}\n");
                out.push_str(&tip_line);
            }
            if let Some(ubertip) = existing_binding
                .and_then(|binding| binding.ubertip())
                .map(str::to_owned)
                .or_else(|| join_levels(warcraft_object.ubertip_levels()))
            {
                let ubertip_line = format!("Ubertip={ubertip}\n");
                out.push_str(&ubertip_line);
            }
            out.push('\n');
        }
        for entry in WARCRAFT_SYSTEM_KEYBINDS.iter() {
            let id = entry.section_id();
            let hotkey_code = tmpl
                .system(id)
                .map(|binding| binding.hotkey().to_string())
                .unwrap_or_else(|| entry.default_hotkey().to_string());
            let section_header = format!("[{id}]\n");
            out.push_str(&section_header);
            let hotkey_line = format!("Hotkey={hotkey_code}\n");
            out.push_str(&hotkey_line);
            out.push_str(entry.class().ini_field());
            out.push('\n');
            if let Some(modifier_text) = entry.default_modifier().ini_str() {
                let modifier_line = format!("Modifier={modifier_text}\n");
                out.push_str(&modifier_line);
            }
            out.push('\n');
        }
        out
    }

    /// Regenerates CustomKeys.txt from the database. Run this whenever
    /// warcraft-database changes to keep the default template in sync.
    /// Ignored in CI: this is a regeneration tool, not a spec.
    /// After running, inspect the diff before committing the new template.
    #[test]
    #[ignore]
    fn regenerate_default_template() {
        let content = build_text(&GridLayout::qwerty_grid());
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/templates/CustomKeys.txt");
        std::fs::write(path, &content).expect("failed to write default template");
        println!("wrote {} bytes to {path}", content.len());
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod system_hotkey_command_tests {
    use super::super::CustomKeys;
    use crate::KeyCode;
    use crate::model::{Hotkey, SystemBinding};
    use warcraft_api::SystemKeybindClass;

    #[test]
    fn set_system_hotkey_replaces_the_binding_key() {
        let initial = SystemBinding::new(Hotkey::VirtualKey(9), SystemKeybindClass::Game, None);
        let mut file = CustomKeys::builder()
            .system("IsHeroSelect", initial)
            .build();
        let replacement = KeyCode::try_from(49).expect("49 is a valid key code");
        file.set_system_hotkey("IsHeroSelect", replacement);
        let retrieved = file.system("IsHeroSelect").expect("system entry present");
        let expected_code = u32::from(replacement);
        assert_eq!(retrieved.hotkey(), &Hotkey::VirtualKey(expected_code));
    }

    #[test]
    fn swap_system_bindings_exchanges_the_two_hotkeys() {
        let first = SystemBinding::new(Hotkey::VirtualKey(9), SystemKeybindClass::Game, None);
        let second = SystemBinding::new(Hotkey::VirtualKey(49), SystemKeybindClass::Game, None);
        let mut file = CustomKeys::builder()
            .system("Ctr1", first)
            .system("Ctr2", second)
            .build();
        file.swap_system_bindings("Ctr1", "Ctr2");
        let source = file.system("Ctr1").expect("source present");
        let target = file.system("Ctr2").expect("target present");
        assert_eq!(source.hotkey(), &Hotkey::VirtualKey(49));
        assert_eq!(target.hotkey(), &Hotkey::VirtualKey(9));
    }
}
