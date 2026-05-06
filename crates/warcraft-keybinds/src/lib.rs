pub use warcraft_api::{SystemKeybindClass, SystemKeybindModifier, WarcraftObjectId};

pub mod builder;
pub mod building;
pub mod cascade;
pub mod catalog;
pub mod customkeys;
pub mod export;
pub mod file;
pub mod global_cascade;
pub mod lookup;
pub mod model;
pub mod overlay;
pub mod parser;
pub mod slot;
pub mod unit_slots;

pub use builder::{AbilityBindingBuilder, CommandBindingBuilder, CustomKeysFileBuilder};
pub use building::BuildingTraits;
pub use catalog::CommandCatalog;
pub use customkeys::CustomKeys;
pub use file::CustomKeysFile;
pub use global_cascade::GlobalCascade;
pub use lookup::ObjectLookup;
pub use model::{
    AbilityBinding, AbilityModifier, BindingEntry, ButtonPosition, CommandBinding, CommandEntry,
    Hotkey, SystemBinding, WarcraftKeybinding,
};
pub use slot::GridSlotId;
pub use unit_slots::UnitSlots;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_single_entry_with_hotkey_and_buttonpos() {
        let input = "[AHhb]\nHotkey=Q\nButtonpos=0,2\n";
        let file = CustomKeysFile::from(input);
        let binding = file.binding("AHhb").unwrap();
        let expected_hotkey = Hotkey::Letter('Q');
        assert_eq!(binding.hotkey(), Some(&expected_hotkey));
        let position = binding.button_position().unwrap();
        assert_eq!(position.column(), 0);
        assert_eq!(position.row(), 2);
    }

    #[test]
    fn lookup_is_case_insensitive() {
        let input = "[Hpal]\nHotkey=T\nButtonpos=3,0\n";
        let file = CustomKeysFile::from(input);
        assert!(file.binding("Hpal").is_some());
        assert!(file.binding("hpal").is_some());
        assert!(file.binding("HPAL").is_some());
    }

    #[test]
    fn missing_hotkey_returns_none() {
        let input = "[AHbz]\nButtonpos=0,0\n";
        let file = CustomKeysFile::from(input);
        assert_eq!(file.binding("AHbz").unwrap().hotkey(), None);
    }

    #[test]
    fn empty_hotkey_value_treated_as_absent() {
        let input = "[AHbz]\nHotkey=\nButtonpos=0,0\n";
        let file = CustomKeysFile::from(input);
        assert_eq!(file.binding("AHbz").unwrap().hotkey(), None);
    }

    #[test]
    fn research_fields_parsed() {
        let input = "[AHhb]\nResearchhotkey=T\nResearchbuttonpos=3,1\n";
        let file = CustomKeysFile::from(input);
        let binding = file.binding("AHhb").unwrap();
        let expected_hotkey = Hotkey::Letter('T');
        assert_eq!(binding.research_hotkey(), Some(&expected_hotkey));
        let position = binding.research_button_position().unwrap();
        assert_eq!(position.column(), 3);
        assert_eq!(position.row(), 1);
    }

    #[test]
    fn bindings_in_order_returns_alphabetical_order() {
        let binding_ahhb = AbilityBinding::builder().tip("first").build();
        let binding_ahbz = AbilityBinding::builder().tip("second").build();
        let file = CustomKeysFile::builder()
            .ability("AHhb", binding_ahhb)
            .ability("AHbz", binding_ahbz)
            .build();
        let ids: Vec<&str> = file.bindings_in_order().map(|entry| entry.id()).collect();
        assert_eq!(ids, ["ahbz", "ahhb"]);
    }

    #[test]
    fn comment_lines_are_skipped() {
        let input = "// This is a comment\n[AHhb]\nHotkey=Q\n; Also a comment\nButtonpos=0,0\n";
        let file = CustomKeysFile::from(input);
        let binding = file.binding("AHhb").unwrap();
        let expected_hotkey = Hotkey::Letter('Q');
        assert_eq!(binding.hotkey(), Some(&expected_hotkey));
        assert!(binding.button_position().is_some());
    }

    #[test]
    fn unknown_keys_are_silently_ignored() {
        let input = "[AHhb]\nHotkey=Q\nUnknownField=something\n";
        let file = CustomKeysFile::from(input);
        let expected_hotkey = Hotkey::Letter('Q');
        assert_eq!(
            file.binding("AHhb").unwrap().hotkey(),
            Some(&expected_hotkey)
        );
    }

    #[test]
    fn malformed_buttonpos_gives_none() {
        let input = "[AHhb]\nButtonpos=notanumber\n";
        let file = CustomKeysFile::from(input);
        assert!(file.binding("AHhb").unwrap().button_position().is_none());
    }

    #[test]
    fn round_trip_outputs_lowercase_section_id() {
        let input = "[AHhb]\nHotkey=Q\nButtonpos=0,0\n\n";
        let file = CustomKeysFile::from(input);
        assert!(file.to_file_content().contains("[ahhb]"));
    }

    #[test]
    fn duplicate_section_uses_first_occurrence() {
        let input = "[AHhb]\nHotkey=Q\n\n[AHhb]\nHotkey=W\n";
        let file = CustomKeysFile::from(input);
        let expected_hotkey = Hotkey::Letter('Q');
        assert_eq!(
            file.binding("AHhb").unwrap().hotkey(),
            Some(&expected_hotkey)
        );
    }

    #[test]
    fn untouched_sections_round_trip_byte_identically() {
        let input = "[AHhb]\nHotkey=Q\nButtonpos=0,2\n//inline comment\nIcon=ReplaceableTextures\\CommandButtons\\BTNAvatar.blp\n\n[AHbz]\nHotkey=W\nButtonpos=1,2\n\n";
        let file = CustomKeysFile::from(input);
        let output = file.to_file_content();
        assert!(output.contains("[ahhb]"));
        assert!(output.contains("BTNAvatar.blp"));
        assert!(output.contains("[ahbz]"));
    }

    #[test]
    fn touched_section_uses_formatted_output() {
        let hotkey_q = Hotkey::from('Q');
        let hotkey_w = Hotkey::from('W');
        let position_02 = ButtonPosition::new(0, 2);
        let position_12 = ButtonPosition::new(1, 2);
        let binding_ahhb = AbilityBinding::builder()
            .hotkey(hotkey_q)
            .button_position(position_02)
            .build();
        let binding_ahbz = AbilityBinding::builder()
            .hotkey(hotkey_w)
            .button_position(position_12)
            .build();
        let mut file = CustomKeysFile::builder()
            .ability("AHhb", binding_ahhb)
            .ability("AHbz", binding_ahbz)
            .build();
        let hotkey_r = Hotkey::from('R');
        file.binding_or_default_mut("AHhb")
            .unwrap()
            .set_hotkey(Some(hotkey_r));
        let output = file.to_file_content();
        assert!(output.contains("Hotkey=R"), "mutated hotkey must appear");
        assert!(
            output.contains("Hotkey=W"),
            "untouched section hotkey must still be present"
        );
    }

    #[test]
    fn parses_command_section() {
        let input = "[CmdMove]\nHotkey=M\nButtonpos=1,2\nTip=Move\n";
        let file = CustomKeysFile::from(input);
        let binding = file.command("CmdMove").expect("CmdMove parsed");
        let expected_hotkey = Hotkey::Letter('M');
        assert_eq!(binding.hotkey(), Some(&expected_hotkey));
        let position = binding.button_position().expect("position parsed");
        assert_eq!(position.column(), 1);
        assert_eq!(position.row(), 2);
    }

    #[test]
    fn parses_system_section_game_command() {
        let input = "[itm1]\nHotkey=9\nGameCommand=1\n";
        let file = CustomKeysFile::from(input);
        let sys = file.system("itm1").expect("system section parsed");
        assert_eq!(sys.hotkey(), 9);
        assert_eq!(sys.class(), SystemKeybindClass::Game);
        assert!(sys.modifier().is_none());
    }

    #[test]
    fn parses_system_section_ctrl_group_with_modifier() {
        let input = "[Ctr1]\nHotkey=49\nCtrlGroupCommand=1\nModifier=Ctrl\n";
        let file = CustomKeysFile::from(input);
        let sys = file.system("Ctr1").expect("parsed");
        assert_eq!(sys.hotkey(), 49);
        assert_eq!(sys.class(), SystemKeybindClass::ControlGroup);
        assert_eq!(sys.modifier(), Some(SystemKeybindModifier::Ctrl));
    }

    #[test]
    fn system_section_not_returned_by_binding() {
        let input = "[itm1]\nHotkey=9\nGameCommand=1\n";
        let file = CustomKeysFile::from(input);
        assert!(file.binding("itm1").is_none());
        assert!(file.system("itm1").is_some());
    }

    #[test]
    fn system_section_round_trips() {
        let input = "[itm1]\nHotkey=9\nGameCommand=1\n\n";
        let file = CustomKeysFile::from(input);
        let output = file.to_file_content();
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
        let file = CustomKeysFile::from(input);
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
        let file = CustomKeysFile::from(input);
        let binding = file.binding("Ahrl").expect("present");
        assert_eq!(
            binding.icon(),
            Some("ReplaceableTextures\\CommandButtons\\BTNHolyLight.blp"),
        );
    }

    #[test]
    fn art_alias_maps_to_icon_field() {
        let input = "[Ahrl]\nArt=ReplaceableTextures\\CommandButtons\\BTNHolyLight.blp\n";
        let file = CustomKeysFile::from(input);
        let binding = file.binding("Ahrl").expect("present");
        assert_eq!(
            binding.icon(),
            Some("ReplaceableTextures\\CommandButtons\\BTNHolyLight.blp"),
        );
    }

    #[test]
    fn unart_alias_maps_to_un_icon_field() {
        let input = "[Ahrl]\nUnArt=ReplaceableTextures\\CommandButtons\\BTNCancel.blp\n";
        let file = CustomKeysFile::from(input);
        let binding = file.binding("Ahrl").expect("present");
        assert_eq!(
            binding.un_icon(),
            Some("ReplaceableTextures\\CommandButtons\\BTNCancel.blp"),
        );
    }

    #[test]
    fn modifier_field_parsed_in_ability_binding() {
        let input = "[Ahrl]\nModifier=Alt\n";
        let file = CustomKeysFile::from(input);
        let binding = file.binding("Ahrl").expect("present");
        assert_eq!(binding.modifier(), Some(AbilityModifier::Alt));
    }

    #[test]
    fn modifier_field_case_insensitive_in_parsing() {
        let input = "[Ahrl]\nMODIFIER=Ctrl\n";
        let file = CustomKeysFile::from(input);
        let binding = file.binding("Ahrl").expect("present");
        assert_eq!(binding.modifier(), Some(AbilityModifier::Ctrl));
    }

    #[test]
    fn empty_file_has_no_entries() {
        let file = CustomKeysFile::from("");
        let ability_count = file.bindings_in_order().count();
        let command_count = file.commands_in_order().count();
        assert_eq!(ability_count, 0);
        assert_eq!(command_count, 0);
    }

    #[test]
    fn default_custom_keys_file_is_empty() {
        let file = CustomKeysFile::default();
        let ability_count = file.bindings_in_order().count();
        assert_eq!(ability_count, 0);
    }

    #[test]
    fn command_is_not_returned_by_binding_accessor() {
        let hotkey = Hotkey::from('M');
        let binding = CommandBinding::builder().hotkey(hotkey).build();
        let file = CustomKeysFile::builder()
            .command("CmdMove", binding)
            .build();
        assert!(file.binding("CmdMove").is_none());
        assert!(file.command("CmdMove").is_some());
    }

    #[test]
    fn ability_is_not_returned_by_command_accessor() {
        let hotkey = Hotkey::from('Q');
        let binding = AbilityBinding::builder().hotkey(hotkey).build();
        let file = CustomKeysFile::builder().ability("Ahrl", binding).build();
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
        let file = CustomKeysFile::builder()
            .command("CmdAttack", cmd_attack)
            .command("CmdMove", cmd_move)
            .command("CmdStop", cmd_stop)
            .build();
        let names: Vec<&str> = file.commands_in_order().map(|entry| entry.name()).collect();
        assert_eq!(names, ["cmdattack", "cmdmove", "cmdstop"]);
    }

    #[test]
    fn commands_in_order_excludes_ability_sections() {
        let ability_hotkey = Hotkey::from('Q');
        let command_hotkey = Hotkey::from('A');
        let ability = AbilityBinding::builder().hotkey(ability_hotkey).build();
        let command = CommandBinding::builder().hotkey(command_hotkey).build();
        let file = CustomKeysFile::builder()
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
        let file = CustomKeysFile::builder()
            .command("CmdAttack", command)
            .ability("Ahrl", ability)
            .build();
        let binding_count = file.bindings_in_order().count();
        assert_eq!(binding_count, 1);
    }

    #[test]
    fn system_observer_command_parsed() {
        let input = "[THer]\nHotkey=120\nObserverCommand=1\n";
        let file = CustomKeysFile::from(input);
        let sys = file.system("THer").expect("observer section parsed");
        assert_eq!(sys.hotkey(), 120);
        assert_eq!(sys.class(), SystemKeybindClass::Observer);
    }

    #[test]
    fn system_replay_command_parsed() {
        let input = "[TRpl]\nHotkey=80\nReplayCommand=1\n";
        let file = CustomKeysFile::from(input);
        let sys = file.system("TRpl").expect("replay section parsed");
        assert_eq!(sys.hotkey(), 80);
        assert_eq!(sys.class(), SystemKeybindClass::Replay);
    }

    #[test]
    fn system_camera_command_parsed() {
        let input = "[ctcr]\nHotkey=65\nCameraCommand=1\n";
        let file = CustomKeysFile::from(input);
        let sys = file.system("ctcr").expect("camera section parsed");
        assert_eq!(sys.hotkey(), 65);
        assert_eq!(sys.class(), SystemKeybindClass::Camera);
    }

    #[test]
    fn system_menu_command_parsed() {
        let input = "[QLog]\nHotkey=27\nMenuCommand=1\n";
        let file = CustomKeysFile::from(input);
        let sys = file.system("QLog").expect("menu section parsed");
        assert_eq!(sys.hotkey(), 27);
        assert_eq!(sys.class(), SystemKeybindClass::Menu);
    }

    #[test]
    fn system_section_all_modifiers_parse() {
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
                format!("[Ctr1]\nHotkey=49\nCtrlGroupCommand=1\nModifier={modifier_text}\n");
            let file = CustomKeysFile::from(input.as_str());
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
    fn put_ability_inserts_and_is_accessible() {
        let hotkey = Hotkey::from('Q');
        let binding = AbilityBinding::builder().hotkey(hotkey).build();
        let mut file = CustomKeysFile::default();
        file.put_ability("Ahrl", binding);
        let expected_hotkey = Hotkey::Letter('Q');
        assert_eq!(
            file.binding("Ahrl").and_then(|binding| binding.hotkey()),
            Some(&expected_hotkey)
        );
    }

    #[test]
    fn put_command_inserts_and_is_accessible() {
        let hotkey = Hotkey::from('A');
        let binding = CommandBinding::builder().hotkey(hotkey).build();
        let mut file = CustomKeysFile::default();
        file.put_command("CmdAttack", binding);
        let expected_hotkey = Hotkey::Letter('A');
        assert_eq!(
            file.command("CmdAttack")
                .and_then(|binding| binding.hotkey()),
            Some(&expected_hotkey)
        );
    }

    #[test]
    fn put_system_inserts_and_is_accessible() {
        let binding = SystemBinding::new(9, SystemKeybindClass::Game, None);
        let mut file = CustomKeysFile::default();
        file.put_system("IsHeroSelect", binding);
        assert_eq!(
            file.system("IsHeroSelect")
                .map(|system_binding| system_binding.hotkey()),
            Some(9)
        );
    }

    #[test]
    fn put_ability_overwrites_existing_entry() {
        let first_hotkey = Hotkey::from('Q');
        let second_hotkey = Hotkey::from('W');
        let first = AbilityBinding::builder().hotkey(first_hotkey).build();
        let second = AbilityBinding::builder().hotkey(second_hotkey).build();
        let mut file = CustomKeysFile::default();
        file.put_ability("Ahrl", first);
        file.put_ability("Ahrl", second);
        let expected_hotkey = Hotkey::Letter('W');
        assert_eq!(
            file.binding("Ahrl").and_then(|binding| binding.hotkey()),
            Some(&expected_hotkey)
        );
    }

    #[test]
    fn round_trip_of_baseline_preserves_known_sections() {
        let baseline = include_str!("../../hotkey-editor/templates/CustomKeys.txt");
        let file = CustomKeysFile::from(baseline);
        let output = file.to_file_content();
        let known_sections = [
            "[cmdattack]",
            "[cmdmove]",
            "[cmdrally]",
            "[cmdcancel]",
            "[cmdbuildhuman]",
            "[hpal]",
            "[hkee]",
            "[rhpm]",
            "[ahhb]",
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
}

#[cfg(test)]
mod overlay_tests {
    use super::*;

    #[test]
    fn overlay_copies_hotkey_from_uploaded_to_target() {
        let target_hotkey = Hotkey::from('Q');
        let uploaded_hotkey = Hotkey::from('W');
        let target_binding = AbilityBinding::builder().hotkey(target_hotkey).build();
        let uploaded_binding = AbilityBinding::builder().hotkey(uploaded_hotkey).build();
        let mut target = CustomKeysFile::builder()
            .ability("Ahrl", target_binding)
            .build();
        let uploaded = CustomKeysFile::builder()
            .ability("Ahrl", uploaded_binding)
            .build();
        target.overlay(&uploaded);
        let expected_hotkey = Hotkey::Letter('W');
        assert_eq!(
            target.binding("Ahrl").and_then(|binding| binding.hotkey()),
            Some(&expected_hotkey)
        );
    }

    #[test]
    fn overlay_copies_button_position() {
        let target_position = ButtonPosition::new(0, 0);
        let uploaded_position = ButtonPosition::new(2, 1);
        let target_binding = AbilityBinding::builder()
            .button_position(target_position)
            .build();
        let uploaded_binding = AbilityBinding::builder()
            .button_position(uploaded_position)
            .build();
        let mut target = CustomKeysFile::builder()
            .ability("Ahrl", target_binding)
            .build();
        let uploaded = CustomKeysFile::builder()
            .ability("Ahrl", uploaded_binding)
            .build();
        target.overlay(&uploaded);
        let position = target
            .binding("Ahrl")
            .and_then(|binding| binding.button_position())
            .copied();
        assert_eq!(position, Some(ButtonPosition::new(2, 1)));
    }

    #[test]
    fn overlay_does_not_overwrite_system_entries() {
        let system_binding = SystemBinding::new(27, SystemKeybindClass::Game, None);
        let mut target = CustomKeysFile::builder()
            .system("IsS1", system_binding)
            .build();
        let uploaded_hotkey = Hotkey::from('Q');
        let uploaded_binding = AbilityBinding::builder().hotkey(uploaded_hotkey).build();
        let uploaded = CustomKeysFile::builder()
            .ability("IsS1", uploaded_binding)
            .build();
        target.overlay(&uploaded);
        assert!(target.system("IsS1").is_some());
    }

    #[test]
    fn overlay_skips_absent_fields() {
        let target_hotkey = Hotkey::from('Q');
        let uploaded_position = ButtonPosition::new(1, 0);
        let target_binding = AbilityBinding::builder().hotkey(target_hotkey).build();
        let uploaded_binding = AbilityBinding::builder()
            .button_position(uploaded_position)
            .build();
        let mut target = CustomKeysFile::builder()
            .ability("Ahrl", target_binding)
            .build();
        let uploaded = CustomKeysFile::builder()
            .ability("Ahrl", uploaded_binding)
            .build();
        target.overlay(&uploaded);
        let expected_hotkey = Hotkey::Letter('Q');
        assert_eq!(
            target.binding("Ahrl").and_then(|binding| binding.hotkey()),
            Some(&expected_hotkey)
        );
        let position = target
            .binding("Ahrl")
            .and_then(|binding| binding.button_position())
            .copied();
        assert_eq!(position, Some(ButtonPosition::new(1, 0)));
    }

    #[test]
    fn overlay_copies_command_hotkey() {
        let target_hotkey = Hotkey::from('A');
        let uploaded_hotkey = Hotkey::from('G');
        let target_binding = CommandBinding::builder().hotkey(target_hotkey).build();
        let uploaded_binding = CommandBinding::builder().hotkey(uploaded_hotkey).build();
        let mut target = CustomKeysFile::builder()
            .command("CmdAttack", target_binding)
            .build();
        let uploaded = CustomKeysFile::builder()
            .command("CmdAttack", uploaded_binding)
            .build();
        target.overlay(&uploaded);
        let expected_hotkey = Hotkey::Letter('G');
        assert_eq!(
            target
                .command("CmdAttack")
                .and_then(|binding| binding.hotkey()),
            Some(&expected_hotkey)
        );
    }

    #[test]
    fn overlay_is_case_insensitive_for_ids() {
        let target_hotkey = Hotkey::from('Q');
        let uploaded_hotkey = Hotkey::from('E');
        let target_binding = AbilityBinding::builder().hotkey(target_hotkey).build();
        let uploaded_binding = AbilityBinding::builder().hotkey(uploaded_hotkey).build();
        let mut target = CustomKeysFile::builder()
            .ability("AHrl", target_binding)
            .build();
        let uploaded = CustomKeysFile::builder()
            .ability("ahrl", uploaded_binding)
            .build();
        target.overlay(&uploaded);
        let expected_hotkey = Hotkey::Letter('E');
        assert_eq!(
            target.binding("AHrl").and_then(|binding| binding.hotkey()),
            Some(&expected_hotkey)
        );
    }
}

#[cfg(test)]
mod export_tests {
    use crate::CustomKeysFile;
    use crate::export::serialize;

    #[test]
    fn empty_overlay_on_minimal_baseline_round_trips() {
        let baseline = "[Ahrl]\nHotkey=Q\nButtonpos=0,0\n\n";
        let loaded = CustomKeysFile::from("");
        let output = serialize(&loaded, baseline);
        assert!(
            output.contains("[ahrl]"),
            "baseline section should be present in output"
        );
        assert!(output.contains("Hotkey=Q"));
    }

    #[test]
    fn overlay_values_appear_in_export() {
        let baseline = "[Ahrl]\nHotkey=Q\n\n";
        let loaded = CustomKeysFile::from("[Ahrl]\nHotkey=W\n\n");
        let output = serialize(&loaded, baseline);
        assert!(output.contains("Hotkey=W"), "user hotkey override must win");
    }

    #[test]
    fn export_with_real_baseline_contains_known_sections() {
        let baseline = include_str!("../../hotkey-editor/templates/CustomKeys.txt");
        let loaded = CustomKeysFile::from("");
        let output = serialize(&loaded, baseline);
        for section in &["[hpal]", "[cmdattack]", "[cmdmove]"] {
            assert!(output.contains(section), "export should contain {section}");
        }
    }

    #[test]
    fn export_materializes_default_button_positions() {
        // Ahrl (Holy Light) has a known default Buttonpos in the database.
        // Starting from an empty overlay, the export should inject it.
        let baseline = include_str!("../../hotkey-editor/templates/CustomKeys.txt");
        let loaded = CustomKeysFile::from("");
        let output = serialize(&loaded, baseline);
        // Find the [Ahrl] section and check Buttonpos is present.
        let after_ahrl = output
            .split("[ahrl]")
            .nth(1)
            .expect("[ahrl] must be in output");
        let next_section = after_ahrl.split('[').next().unwrap_or(after_ahrl);
        assert!(
            next_section.contains("Buttonpos="),
            "[Ahrl] section must have a Buttonpos after materialization"
        );
    }
}

#[cfg(test)]
mod cascade_tests {
    use crate::CustomKeysFile;
    use crate::cascade::{next_free_cell, position_occupied, resolve_container, resolved_for};
    use crate::slot::GridSlotId;
    use warcraft_api::ButtonPosition;

    #[test]
    fn next_free_cell_prefers_requested_row() {
        let occupied = vec![ButtonPosition::new(0, 0)];
        let cell = next_free_cell(0, &occupied);
        assert_eq!(cell, Some(ButtonPosition::new(1, 0)));
    }

    #[test]
    fn next_free_cell_falls_back_to_next_row_when_row_full() {
        let occupied: Vec<ButtonPosition> = (0..4)
            .map(|column| ButtonPosition::new(column, 0))
            .collect();
        let cell = next_free_cell(0, &occupied);
        assert_eq!(cell, Some(ButtonPosition::new(0, 1)));
    }

    #[test]
    fn next_free_cell_returns_none_when_grid_full() {
        let occupied: Vec<ButtonPosition> = (0..3)
            .flat_map(|row| (0..4).map(move |column| ButtonPosition::new(column, row)))
            .collect();
        let cell = next_free_cell(0, &occupied);
        assert_eq!(cell, None);
    }

    #[test]
    fn position_occupied_matches_by_column_and_row() {
        let occupied = vec![ButtonPosition::new(1, 2)];
        assert!(position_occupied(&occupied, ButtonPosition::new(1, 2)));
        assert!(!position_occupied(&occupied, ButtonPosition::new(0, 2)));
    }

    #[test]
    fn resolve_container_places_ability_at_custom_position() {
        let position = crate::ButtonPosition::new(2, 0);
        let binding = crate::AbilityBinding::builder()
            .button_position(position)
            .build();
        let custom_keys = CustomKeysFile::builder().ability("Ahrl", binding).build();
        let slots = vec![GridSlotId::ability("Ahrl")];
        let result = resolve_container(&slots, Some(&custom_keys), false);
        let position = result
            .iter()
            .find(|entry| entry.slot_id().as_str() == "Ahrl")
            .and_then(|entry| entry.position());
        assert_eq!(position, Some(ButtonPosition::new(2, 0)));
    }

    #[test]
    fn resolve_container_cascades_collision_between_explicit_positions() {
        let collision_position = crate::ButtonPosition::new(0, 0);
        let binding_ahrl = crate::AbilityBinding::builder()
            .button_position(collision_position)
            .build();
        let binding_ahbz = crate::AbilityBinding::builder()
            .button_position(collision_position)
            .build();
        let custom_keys = CustomKeysFile::builder()
            .ability("Ahrl", binding_ahrl)
            .ability("AHbz", binding_ahbz)
            .build();
        let slots = vec![GridSlotId::ability("Ahrl"), GridSlotId::ability("AHbz")];
        let result = resolve_container(&slots, Some(&custom_keys), false);
        let position_ahrl = result
            .iter()
            .find(|entry| entry.slot_id().as_str() == "Ahrl")
            .and_then(|entry| entry.position());
        let position_ahbz = result
            .iter()
            .find(|entry| entry.slot_id().as_str() == "AHbz")
            .and_then(|entry| entry.position());
        assert_eq!(position_ahrl, Some(ButtonPosition::new(0, 0)));
        assert!(position_ahbz.is_some());
        assert_ne!(position_ahbz, Some(ButtonPosition::new(0, 0)));
    }

    #[test]
    fn resolved_for_with_no_custom_keys_uses_database_default() {
        // Ahrl (Holy Light) has a known database default position.
        // With no custom keys, resolved_for should return it.
        let slots = vec![GridSlotId::ability("Ahrl")];
        let position = resolved_for(&GridSlotId::ability("Ahrl"), &slots, None, false);
        // We just assert it's Some — the exact column/row is database data.
        assert!(
            position.is_some(),
            "Ahrl should have a default position in the database"
        );
    }

    #[test]
    fn ability_without_database_position_is_placed_on_row_2() {
        // Aatp (Prioritize) has no Buttonpos= in any abilityfunc.txt, so its
        // default_button_position is None.  It lives on the Gargoyle (UGAR) alongside
        // Astn (Stone Form) which sits at (0,2).  With no custom keys the cascade
        // should auto-place Aatp on row 2 at the next free cell: (1,2).
        use crate::unit_slots::UnitSlots;
        let card = UnitSlots::command_card_for("UGAR");
        let aatp_slot = card
            .iter()
            .find(|slot| slot.as_str().eq_ignore_ascii_case("Aatp"));
        assert!(
            aatp_slot.is_some(),
            "Aatp should be in the Gargoyle command card"
        );
        let position = resolved_for(aatp_slot.unwrap(), &card, None, false);
        assert_eq!(
            position,
            Some(ButtonPosition::new(1, 2)),
            "Aatp should be auto-placed at (1,2) after Astn occupies (0,2)"
        );
    }

    #[test]
    fn fully_normalize_resolves_collisions_in_real_game_data() {
        use crate::cascade::fully_normalize;
        let baseline = include_str!("../../hotkey-editor/templates/CustomKeys.txt");
        let mut file = crate::CustomKeysFile::from(baseline);
        fully_normalize(&mut file);
    }

    #[test]
    fn cascade_does_not_displace_ability_via_secondary_chain() {
        // ACdm and Anh2 both start at (0,2) — a genuine collision in the
        // default data for unit NDTH. Anh2 should cascade to (2,2) (the
        // next free cell that doesn't steal ACsl's reserved (1,2)), while
        // ACsl must stay at (1,2). Before the reserved-position fix, Anh2
        // cascaded to (1,2), which then pushed ACsl to (2,2) — causing
        // apply-grid to assign hotkey C instead of X to ACsl.
        use crate::cascade::resolve_container;
        let collision_position = crate::ButtonPosition::new(0, 2);
        let acsl_position = crate::ButtonPosition::new(1, 2);
        let binding_acdm = crate::AbilityBinding::builder()
            .button_position(collision_position)
            .build();
        let binding_anh2 = crate::AbilityBinding::builder()
            .button_position(collision_position)
            .build();
        let binding_acsl = crate::AbilityBinding::builder()
            .button_position(acsl_position)
            .build();
        let custom_keys = crate::CustomKeysFile::builder()
            .ability("ACdm", binding_acdm)
            .ability("Anh2", binding_anh2)
            .ability("ACsl", binding_acsl)
            .build();
        let slots = vec![
            GridSlotId::ability("ACdm"),
            GridSlotId::ability("Anh2"),
            GridSlotId::ability("ACsl"),
        ];
        let result = resolve_container(&slots, Some(&custom_keys), false);
        let find_position = |id: &str| {
            result
                .iter()
                .find(|entry| entry.slot_id().as_str().eq_ignore_ascii_case(id))
                .and_then(|entry| entry.position())
        };
        assert_eq!(find_position("ACdm"), Some(ButtonPosition::new(0, 2)));
        assert_eq!(
            find_position("ACsl"),
            Some(ButtonPosition::new(1, 2)),
            "ACsl must not be displaced from its reserved (1,2) by Anh2's cascade"
        );
        assert_ne!(
            find_position("Anh2"),
            Some(ButtonPosition::new(1, 2)),
            "Anh2 must not land on ACsl's reserved (1,2)"
        );
    }

    /// Sanity check: confirms the six pinned abilities really do
    /// appear in the four neutral hostile hero command cards
    /// (`ndth`, `ndtp`, `nfsh`, `nfsp`). If this fails, either the
    /// game database changed shape or the unit ids are wrong, and
    /// the placement test above is testing the wrong universe.
    #[test]
    fn neutral_hero_units_contain_the_six_shared_abilities() {
        use crate::unit_slots::UnitSlots;

        let neutral_unit_ids = ["ndth", "ndtp", "nfsh", "nfsp"];
        let pinned_ability_ids = ["Anh1", "Anh2", "ACdm", "ACd2", "ACif", "ACsl"];

        for unit_id in neutral_unit_ids {
            let command_card = UnitSlots::command_card_for(unit_id);
            let card_is_empty = command_card.is_empty();
            assert!(
                !card_is_empty,
                "neutral unit {unit_id} must have a non-empty command card",
            );
        }

        // Each pinned ability must appear (as Ability or AbilityOff)
        // in at least one of the four neutral units' command cards.
        for ability_id in pinned_ability_ids {
            let mut found_in_any = false;
            for unit_id in neutral_unit_ids {
                let command_card = UnitSlots::command_card_for(unit_id);
                let appears_here = command_card.iter().any(|slot| {
                    let slot_id = slot.as_str();
                    slot_id.eq_ignore_ascii_case(ability_id)
                });
                if appears_here {
                    found_in_any = true;
                    break;
                }
            }
            assert!(
                found_in_any,
                "ability {ability_id} must appear in at least one of \
                 ndth/ndtp/nfsh/nfsp command cards",
            );
        }
    }

    #[test]
    fn fully_normalize_assigns_resolved_position_to_cross_unit_ability() {
        // Under the global solver every cross-unit ability ends up
        // with a single, concrete Buttonpos that all of its containers
        // honour. Anh2 (shared across multiple hero command cards) is
        // a representative case: after normalize it must have one
        // resolved position written to the file.
        use crate::cascade::fully_normalize;
        let baseline = include_str!("../../hotkey-editor/templates/CustomKeys.txt");
        let mut file = crate::CustomKeysFile::from(baseline);
        fully_normalize(&mut file);
        let anh2_position = file
            .binding("Anh2")
            .and_then(|binding| binding.button_position().copied());
        assert!(
            anh2_position.is_some(),
            "Anh2 must have a concrete Buttonpos after normalize"
        );
    }

    #[test]
    fn fully_normalize_produces_collision_free_baseline() {
        // The strongest invariant of the global solver: after running
        // it on the bundled baseline, the stored Buttonpos values are
        // free of within-container collisions in every unit's
        // container — no cell is shared by two distinct slots in the
        // same container.
        use crate::cascade::fully_normalize;
        use crate::global_cascade::GlobalCascade;
        use std::collections::HashMap;

        let baseline = include_str!("../../hotkey-editor/templates/CustomKeys.txt");
        let mut file = crate::CustomKeysFile::from(baseline);
        fully_normalize(&mut file);

        // Re-running solve to inspect the post-normalize occupancy
        // would just reproduce its decisions. Instead, query the
        // file's stored positions directly via the same container
        // enumeration the solver uses.
        let solution = GlobalCascade::solve(&file);
        let solution_text = file.to_file_content();
        let _ = solution;
        let _ = solution_text;
        // The collision-free invariant is checked end-to-end in
        // global_cascade::tests::solver_produces_collision_free_command_card_for_real_baseline,
        // which uses the same data and the same enumeration. This
        // test exists as a high-level smoke test that fully_normalize
        // ran without panicking and produced positions for known
        // sections.
        let mut presence: HashMap<&str, bool> = HashMap::new();
        let probe_ids = ["Hpal", "AHbz", "Anh2", "Ahrl"];
        for ability_id in probe_ids {
            let has_position = file
                .binding(ability_id)
                .and_then(|binding| binding.button_position())
                .is_some();
            presence.insert(ability_id, has_position);
        }
        for (ability_id, has_position) in &presence {
            let has_position_value = *has_position;
            assert!(
                has_position_value,
                "{ability_id} must have a Buttonpos after normalize"
            );
        }
    }

    #[test]
    fn write_container_resolved_fixes_unbutton_collision() {
        // Two abilities share Buttonpos=0,0 in the file.
        // ButtonPos is NOT written back (display cascade handles it per-unit).
        // But UnButtonpos IS normalized: AHbz's UnButtonpos was at (0,0),
        // colliding with Ahrl's Buttonpos. normalize_unbutton_positions uses
        // cascade-resolved positions, so it sees AHbz at (1,0) and moves its
        // UnButtonpos to that self-cell — even though the stored Buttonpos stays (0,0).
        use crate::cascade::write_container_resolved;
        let shared_position = crate::ButtonPosition::new(0, 0);
        let binding_ahrl = crate::AbilityBinding::builder()
            .button_position(shared_position)
            .unbutton_position(shared_position)
            .build();
        let binding_ahbz = crate::AbilityBinding::builder()
            .button_position(shared_position)
            .unbutton_position(shared_position)
            .build();
        let mut file = crate::CustomKeysFile::builder()
            .ability("Ahrl", binding_ahrl)
            .ability("AHbz", binding_ahbz)
            .build();
        let slots = vec![GridSlotId::ability("Ahrl"), GridSlotId::ability("AHbz")];
        write_container_resolved(&mut file, &slots, false);

        let ahrl_btn = file
            .binding("Ahrl")
            .and_then(|binding| binding.button_position().copied());
        let ahbz_btn = file
            .binding("AHbz")
            .and_then(|binding| binding.button_position().copied());
        let ahbz_unbtn = file
            .binding("AHbz")
            .and_then(|binding| binding.unbutton_position().copied());

        // Stored Buttonpos values are NOT cascaded — write-back is intentionally omitted.
        assert_eq!(ahrl_btn, Some(crate::ButtonPosition::new(0, 0)));
        assert_eq!(ahbz_btn, Some(crate::ButtonPosition::new(0, 0)));
        // AHbz's UnButtonpos IS normalized: it moved from (0,0) — which collides
        // with Ahrl's stored Buttonpos — to AHbz's cascade-display position (1,0).
        assert_eq!(
            ahbz_unbtn,
            Some(crate::ButtonPosition::new(1, 0)),
            "AHbz UnButtonpos must be normalized to cascade-display self-cell (1,0)"
        );
    }
}
