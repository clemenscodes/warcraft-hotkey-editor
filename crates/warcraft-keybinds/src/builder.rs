use crate::{
    AbilityBinding, AbilityModifier, ButtonPosition, CommandBinding, CustomKeysFile, Hotkey,
    SystemBinding,
};

/// Builds an [`AbilityBinding`] by setting individual fields.
///
/// Obtain one via [`AbilityBinding::builder`].
pub struct AbilityBindingBuilder {
    hotkey: Option<Hotkey>,
    unhotkey: Option<Hotkey>,
    button_position: Option<ButtonPosition>,
    unbutton_position: Option<ButtonPosition>,
    research_hotkey: Option<Hotkey>,
    research_button_position: Option<ButtonPosition>,
    tip: Option<String>,
    research_tip: Option<String>,
    un_tip: Option<String>,
    ubertip: Option<String>,
    research_ubertip: Option<String>,
    un_ubertip: Option<String>,
    icon: Option<String>,
    un_icon: Option<String>,
    modifier: Option<AbilityModifier>,
}

impl AbilityBindingBuilder {
    pub(crate) fn new() -> Self {
        Self {
            hotkey: None,
            unhotkey: None,
            button_position: None,
            unbutton_position: None,
            research_hotkey: None,
            research_button_position: None,
            tip: None,
            research_tip: None,
            un_tip: None,
            ubertip: None,
            research_ubertip: None,
            un_ubertip: None,
            icon: None,
            un_icon: None,
            modifier: None,
        }
    }

    pub fn hotkey(mut self, hotkey: Hotkey) -> Self {
        self.hotkey = Some(hotkey);
        self
    }

    pub fn unhotkey(mut self, hotkey: Hotkey) -> Self {
        self.unhotkey = Some(hotkey);
        self
    }

    pub fn button_position(mut self, position: ButtonPosition) -> Self {
        self.button_position = Some(position);
        self
    }

    pub fn unbutton_position(mut self, position: ButtonPosition) -> Self {
        self.unbutton_position = Some(position);
        self
    }

    pub fn research_hotkey(mut self, hotkey: Hotkey) -> Self {
        self.research_hotkey = Some(hotkey);
        self
    }

    pub fn research_button_position(mut self, position: ButtonPosition) -> Self {
        self.research_button_position = Some(position);
        self
    }

    pub fn tip(mut self, text: impl Into<String>) -> Self {
        self.tip = Some(text.into());
        self
    }

    pub fn research_tip(mut self, text: impl Into<String>) -> Self {
        self.research_tip = Some(text.into());
        self
    }

    pub fn un_tip(mut self, text: impl Into<String>) -> Self {
        self.un_tip = Some(text.into());
        self
    }

    pub fn ubertip(mut self, text: impl Into<String>) -> Self {
        self.ubertip = Some(text.into());
        self
    }

    pub fn research_ubertip(mut self, text: impl Into<String>) -> Self {
        self.research_ubertip = Some(text.into());
        self
    }

    pub fn un_ubertip(mut self, text: impl Into<String>) -> Self {
        self.un_ubertip = Some(text.into());
        self
    }

    pub fn icon(mut self, path: impl Into<String>) -> Self {
        self.icon = Some(path.into());
        self
    }

    pub fn un_icon(mut self, path: impl Into<String>) -> Self {
        self.un_icon = Some(path.into());
        self
    }

    pub fn modifier(mut self, ability_modifier: AbilityModifier) -> Self {
        self.modifier = Some(ability_modifier);
        self
    }

    /// Consume the builder and produce a clean (not-dirty) [`AbilityBinding`].
    pub fn build(self) -> AbilityBinding {
        let AbilityBindingBuilder {
            hotkey,
            unhotkey,
            button_position,
            unbutton_position,
            research_hotkey,
            research_button_position,
            tip,
            research_tip,
            un_tip,
            ubertip,
            research_ubertip,
            un_ubertip,
            icon,
            un_icon,
            modifier,
        } = self;
        let mut binding = AbilityBinding::default();
        binding.set_hotkey(hotkey);
        binding.set_unhotkey(unhotkey);
        binding.set_button_position(button_position);
        binding.set_unbutton_position(unbutton_position);
        binding.set_research_hotkey(research_hotkey);
        binding.set_research_button_position(research_button_position);
        binding.set_tip(tip);
        binding.set_research_tip(research_tip);
        binding.set_un_tip(un_tip);
        binding.set_ubertip(ubertip);
        binding.set_research_ubertip(research_ubertip);
        binding.set_un_ubertip(un_ubertip);
        binding.set_icon(icon);
        binding.set_un_icon(un_icon);
        binding.set_modifier(modifier);
        binding
    }
}

/// Builds a [`CommandBinding`] by setting individual fields.
///
/// Obtain one via [`CommandBinding::builder`].
pub struct CommandBindingBuilder {
    hotkey: Option<Hotkey>,
    button_position: Option<ButtonPosition>,
    unbutton_position: Option<ButtonPosition>,
    tip: Option<String>,
    un_tip: Option<String>,
}

impl CommandBindingBuilder {
    pub(crate) fn new() -> Self {
        Self {
            hotkey: None,
            button_position: None,
            unbutton_position: None,
            tip: None,
            un_tip: None,
        }
    }

    pub fn hotkey(mut self, hotkey: Hotkey) -> Self {
        self.hotkey = Some(hotkey);
        self
    }

    pub fn button_position(mut self, position: ButtonPosition) -> Self {
        self.button_position = Some(position);
        self
    }

    pub fn unbutton_position(mut self, position: ButtonPosition) -> Self {
        self.unbutton_position = Some(position);
        self
    }

    pub fn tip(mut self, text: impl Into<String>) -> Self {
        self.tip = Some(text.into());
        self
    }

    pub fn un_tip(mut self, text: impl Into<String>) -> Self {
        self.un_tip = Some(text.into());
        self
    }

    /// Consume the builder and produce a clean (not-dirty) [`CommandBinding`].
    pub fn build(self) -> CommandBinding {
        let CommandBindingBuilder {
            hotkey,
            button_position,
            unbutton_position,
            tip,
            un_tip,
        } = self;
        let mut binding = CommandBinding::default();
        binding.set_hotkey(hotkey);
        binding.set_button_position(button_position);
        binding.set_unbutton_position(unbutton_position);
        binding.set_tip(tip);
        binding.set_un_tip(un_tip);
        binding
    }
}

/// A single entry to be inserted into a [`CustomKeysFile`].
enum PendingEntry {
    Ability {
        id: String,
        binding: AbilityBinding,
    },
    Command {
        name: String,
        binding: CommandBinding,
    },
    System {
        id: String,
        binding: SystemBinding,
    },
}

/// Builds a [`CustomKeysFile`] from typed bindings without parsing raw text.
///
/// Obtain one via [`CustomKeysFile::builder`].
pub struct CustomKeysFileBuilder {
    pending_entries: Vec<PendingEntry>,
}

impl CustomKeysFileBuilder {
    pub(crate) fn new() -> Self {
        Self {
            pending_entries: Vec::new(),
        }
    }

    pub fn ability(mut self, id: impl Into<String>, binding: AbilityBinding) -> Self {
        let id_string = id.into();
        let entry = PendingEntry::Ability {
            id: id_string,
            binding,
        };
        self.pending_entries.push(entry);
        self
    }

    pub fn command(mut self, name: impl Into<String>, binding: CommandBinding) -> Self {
        let name_string = name.into();
        let entry = PendingEntry::Command {
            name: name_string,
            binding,
        };
        self.pending_entries.push(entry);
        self
    }

    pub fn system(mut self, id: impl Into<String>, binding: SystemBinding) -> Self {
        let id_string = id.into();
        let entry = PendingEntry::System {
            id: id_string,
            binding,
        };
        self.pending_entries.push(entry);
        self
    }

    /// Consume the builder and produce a [`CustomKeysFile`].
    ///
    /// Entries are stored with lowercase keys and iterate in alphabetical
    /// order. Duplicate section IDs are silently overwritten by the last
    /// entry with that ID.
    pub fn build(self) -> CustomKeysFile {
        let pending_entries = self.pending_entries;
        let mut file = CustomKeysFile::default();
        for entry in pending_entries {
            match entry {
                PendingEntry::Ability { id, binding } => {
                    file.put_ability(&id, binding);
                }
                PendingEntry::Command { name, binding } => {
                    file.put_command(&name, binding);
                }
                PendingEntry::System { id, binding } => {
                    file.put_system(&id, binding);
                }
            }
        }
        file
    }
}

#[cfg(test)]
mod builder_tests {
    use super::*;
    use warcraft_api::{SystemKeybindClass, SystemKeybindModifier};

    #[test]
    fn hotkey_letter_is_uppercased() {
        let hotkey = Hotkey::from('q');
        assert_eq!(hotkey, Hotkey::Letter('Q'));
    }

    #[test]
    fn hotkey_uppercase_letter_is_unchanged() {
        let hotkey = Hotkey::from('Q');
        assert_eq!(hotkey, Hotkey::Letter('Q'));
    }

    #[test]
    fn hotkey_function_key_single_digit() {
        let hotkey = Hotkey::FunctionKey(1);
        assert_eq!(hotkey.to_string(), "F1");
    }

    #[test]
    fn hotkey_function_key_double_digit() {
        let hotkey = Hotkey::FunctionKey(12);
        assert_eq!(hotkey.to_string(), "F12");
    }

    #[test]
    fn hotkey_virtual_key_passes_value_through() {
        let hotkey = Hotkey::VirtualKey(512);
        assert_eq!(hotkey.to_string(), "512");
    }

    #[test]
    fn hotkey_display_matches_letter() {
        let hotkey = Hotkey::from('W');
        let displayed = hotkey.to_string();
        assert_eq!(displayed, "W");
    }

    #[test]
    fn hotkey_into_string() {
        let hotkey = Hotkey::from('E');
        let converted: String = hotkey.into();
        assert_eq!(converted, "E");
    }

    #[test]
    fn hotkey_parses_letter_from_str() {
        let hotkey = Hotkey::try_from("Q").unwrap();
        assert_eq!(hotkey, Hotkey::Letter('Q'));
    }

    #[test]
    fn hotkey_parses_function_key_from_str() {
        let hotkey = Hotkey::try_from("F5").unwrap();
        assert_eq!(hotkey, Hotkey::FunctionKey(5));
    }

    #[test]
    fn hotkey_parses_function_key_case_insensitive() {
        let hotkey = Hotkey::try_from("f5").unwrap();
        assert_eq!(hotkey, Hotkey::FunctionKey(5));
    }

    #[test]
    fn hotkey_empty_string_returns_err() {
        assert!(Hotkey::try_from("").is_err());
    }

    #[test]
    fn hotkey_numeric_string_becomes_virtual_key() {
        let hotkey = Hotkey::try_from("512").unwrap();
        assert_eq!(hotkey, Hotkey::VirtualKey(512));
    }

    #[test]
    fn hotkey_multi_level_parses_from_comma_separated() {
        let hotkey = Hotkey::try_from("Q,Q,Q").unwrap();
        let expected = Hotkey::MultiLevel(vec![
            Hotkey::Letter('Q'),
            Hotkey::Letter('Q'),
            Hotkey::Letter('Q'),
        ]);
        assert_eq!(hotkey, expected);
    }

    #[test]
    fn hotkey_multi_level_displays_with_commas() {
        let hotkey = Hotkey::MultiLevel(vec![Hotkey::Letter('Q'), Hotkey::Letter('W')]);
        assert_eq!(hotkey.to_string(), "Q,W");
    }

    #[test]
    fn modifier_alt_displays_correctly() {
        assert_eq!(AbilityModifier::Alt.to_string(), "Alt");
    }

    #[test]
    fn modifier_ctrl_displays_correctly() {
        assert_eq!(AbilityModifier::Ctrl.to_string(), "Ctrl");
    }

    #[test]
    fn modifier_ctrl_or_alt_displays_correctly() {
        assert_eq!(AbilityModifier::CtrlOrAlt.to_string(), "Ctrl_or_Alt");
    }

    #[test]
    fn modifier_shift_displays_correctly() {
        assert_eq!(AbilityModifier::Shift.to_string(), "Shift");
    }

    #[test]
    fn modifier_into_string() {
        let converted: String = AbilityModifier::Alt.into();
        assert_eq!(converted, "Alt");
    }

    #[test]
    fn modifier_parses_from_str() {
        assert_eq!(
            AbilityModifier::try_from("Alt").unwrap(),
            AbilityModifier::Alt
        );
        assert_eq!(
            AbilityModifier::try_from("Ctrl").unwrap(),
            AbilityModifier::Ctrl
        );
        assert_eq!(
            AbilityModifier::try_from("Ctrl_or_Alt").unwrap(),
            AbilityModifier::CtrlOrAlt
        );
        assert_eq!(
            AbilityModifier::try_from("Shift").unwrap(),
            AbilityModifier::Shift
        );
    }

    #[test]
    fn modifier_parse_is_case_insensitive() {
        assert_eq!(
            AbilityModifier::try_from("ALT").unwrap(),
            AbilityModifier::Alt
        );
        assert_eq!(
            AbilityModifier::try_from("ctrl").unwrap(),
            AbilityModifier::Ctrl
        );
    }

    #[test]
    fn modifier_unknown_value_returns_err() {
        assert!(AbilityModifier::try_from("Meta").is_err());
    }

    #[test]
    fn ability_builder_empty_produces_all_none_binding() {
        let binding = AbilityBinding::builder().build();
        assert!(binding.hotkey().is_none());
        assert!(binding.unhotkey().is_none());
        assert!(binding.button_position().is_none());
        assert!(binding.unbutton_position().is_none());
        assert!(binding.research_hotkey().is_none());
        assert!(binding.research_button_position().is_none());
        assert!(binding.tip().is_none());
        assert!(binding.research_tip().is_none());
        assert!(binding.un_tip().is_none());
        assert!(binding.ubertip().is_none());
        assert!(binding.research_ubertip().is_none());
        assert!(binding.un_ubertip().is_none());
        assert!(binding.icon().is_none());
        assert!(binding.un_icon().is_none());
        assert!(binding.modifier().is_none());
    }

    #[test]
    fn ability_builder_sets_hotkey() {
        let hotkey = Hotkey::from('Q');
        let expected = Hotkey::from('Q');
        let binding = AbilityBinding::builder().hotkey(hotkey).build();
        assert_eq!(binding.hotkey(), Some(&expected));
    }

    #[test]
    fn ability_builder_sets_unhotkey() {
        let hotkey = Hotkey::from('W');
        let expected = Hotkey::from('W');
        let binding = AbilityBinding::builder().unhotkey(hotkey).build();
        assert_eq!(binding.unhotkey(), Some(&expected));
    }

    #[test]
    fn ability_builder_sets_button_position() {
        let position = ButtonPosition::new(2, 1);
        let binding = AbilityBinding::builder().button_position(position).build();
        assert_eq!(
            binding.button_position().copied(),
            Some(ButtonPosition::new(2, 1))
        );
    }

    #[test]
    fn ability_builder_sets_unbutton_position() {
        let position = ButtonPosition::new(3, 2);
        let binding = AbilityBinding::builder()
            .unbutton_position(position)
            .build();
        assert_eq!(
            binding.unbutton_position().copied(),
            Some(ButtonPosition::new(3, 2))
        );
    }

    #[test]
    fn ability_builder_sets_research_hotkey() {
        let hotkey = Hotkey::from('R');
        let expected = Hotkey::from('R');
        let binding = AbilityBinding::builder().research_hotkey(hotkey).build();
        assert_eq!(binding.research_hotkey(), Some(&expected));
    }

    #[test]
    fn ability_builder_sets_research_button_position() {
        let position = ButtonPosition::new(1, 0);
        let binding = AbilityBinding::builder()
            .research_button_position(position)
            .build();
        assert_eq!(
            binding.research_button_position().copied(),
            Some(ButtonPosition::new(1, 0))
        );
    }

    #[test]
    fn ability_builder_sets_tip() {
        let binding = AbilityBinding::builder().tip("Cast Holy Light").build();
        assert_eq!(binding.tip(), Some("Cast Holy Light"));
    }

    #[test]
    fn ability_builder_sets_research_tip() {
        let binding = AbilityBinding::builder()
            .research_tip("Research Paladin")
            .build();
        assert_eq!(binding.research_tip(), Some("Research Paladin"));
    }

    #[test]
    fn ability_builder_sets_un_tip() {
        let binding = AbilityBinding::builder().un_tip("Cancel").build();
        assert_eq!(binding.un_tip(), Some("Cancel"));
    }

    #[test]
    fn ability_builder_sets_ubertip() {
        let binding = AbilityBinding::builder()
            .ubertip("Heals a friendly unit.")
            .build();
        assert_eq!(binding.ubertip(), Some("Heals a friendly unit."));
    }

    #[test]
    fn ability_builder_sets_research_ubertip() {
        let binding = AbilityBinding::builder()
            .research_ubertip("Researches something.")
            .build();
        assert_eq!(binding.research_ubertip(), Some("Researches something."));
    }

    #[test]
    fn ability_builder_sets_un_ubertip() {
        let binding = AbilityBinding::builder()
            .un_ubertip("Off form description.")
            .build();
        assert_eq!(binding.un_ubertip(), Some("Off form description."));
    }

    #[test]
    fn ability_builder_sets_icon() {
        let binding = AbilityBinding::builder()
            .icon("ReplaceableTextures\\CommandButtons\\BTNHolyLight.blp")
            .build();
        assert_eq!(
            binding.icon(),
            Some("ReplaceableTextures\\CommandButtons\\BTNHolyLight.blp"),
        );
    }

    #[test]
    fn ability_builder_sets_un_icon() {
        let binding = AbilityBinding::builder()
            .un_icon("ReplaceableTextures\\CommandButtons\\BTNCancel.blp")
            .build();
        assert_eq!(
            binding.un_icon(),
            Some("ReplaceableTextures\\CommandButtons\\BTNCancel.blp"),
        );
    }

    #[test]
    fn ability_builder_sets_modifier() {
        let binding = AbilityBinding::builder()
            .modifier(AbilityModifier::Alt)
            .build();
        assert_eq!(binding.modifier(), Some(AbilityModifier::Alt));
    }

    #[test]
    fn ability_builder_sets_modifier_ctrl_or_alt() {
        let binding = AbilityBinding::builder()
            .modifier(AbilityModifier::CtrlOrAlt)
            .build();
        assert_eq!(binding.modifier(), Some(AbilityModifier::CtrlOrAlt));
    }

    #[test]
    fn ability_builder_all_fields_survive_serialization_round_trip() {
        let hotkey = Hotkey::from('Q');
        let unhotkey = Hotkey::from('W');
        let research_hotkey = Hotkey::from('E');
        let button_position = ButtonPosition::new(0, 2);
        let unbutton_position = ButtonPosition::new(1, 2);
        let research_button_position = ButtonPosition::new(3, 0);
        let binding = AbilityBinding::builder()
            .hotkey(hotkey)
            .unhotkey(unhotkey)
            .button_position(button_position)
            .unbutton_position(unbutton_position)
            .research_hotkey(research_hotkey)
            .research_button_position(research_button_position)
            .tip("My Tip")
            .research_tip("Research Tip")
            .un_tip("Un Tip")
            .ubertip("Uber Tip")
            .research_ubertip("Research Uber")
            .un_ubertip("Un Uber")
            .icon("buttons\\BTNFoo.blp")
            .un_icon("buttons\\BTNBar.blp")
            .modifier(AbilityModifier::Shift)
            .build();
        let file = CustomKeysFile::builder().ability("Ahrl", binding).build();
        let serialized = file.to_file_content();
        let reparsed = CustomKeysFile::from(serialized.as_str());
        let reparsed_binding = reparsed
            .binding("Ahrl")
            .expect("Ahrl must survive round-trip");
        assert_eq!(reparsed_binding.hotkey(), Some(&Hotkey::Letter('Q')));
        assert_eq!(reparsed_binding.unhotkey(), Some(&Hotkey::Letter('W')));
        assert_eq!(
            reparsed_binding.button_position().copied(),
            Some(ButtonPosition::new(0, 2))
        );
        assert_eq!(
            reparsed_binding.unbutton_position().copied(),
            Some(ButtonPosition::new(1, 2))
        );
        assert_eq!(
            reparsed_binding.research_hotkey(),
            Some(&Hotkey::Letter('E'))
        );
        assert_eq!(
            reparsed_binding.research_button_position().copied(),
            Some(ButtonPosition::new(3, 0)),
        );
        assert_eq!(reparsed_binding.tip(), Some("My Tip"));
        assert_eq!(reparsed_binding.research_tip(), Some("Research Tip"));
        assert_eq!(reparsed_binding.un_tip(), Some("Un Tip"));
        assert_eq!(reparsed_binding.ubertip(), Some("Uber Tip"));
        assert_eq!(reparsed_binding.research_ubertip(), Some("Research Uber"));
        assert_eq!(reparsed_binding.un_ubertip(), Some("Un Uber"));
        assert_eq!(reparsed_binding.icon(), Some("buttons\\BTNFoo.blp"));
        assert_eq!(reparsed_binding.modifier(), Some(AbilityModifier::Shift));
    }

    #[test]
    fn ability_builder_function_key_hotkey_round_trips() {
        let hotkey = Hotkey::FunctionKey(5);
        let binding = AbilityBinding::builder().hotkey(hotkey).build();
        let file = CustomKeysFile::builder().ability("Ahrl", binding).build();
        let serialized = file.to_file_content();
        let reparsed = CustomKeysFile::from(serialized.as_str());
        let hotkey_value = reparsed
            .binding("Ahrl")
            .and_then(|binding| binding.hotkey());
        assert_eq!(hotkey_value, Some(&Hotkey::FunctionKey(5)));
    }

    #[test]
    fn command_builder_empty_produces_all_none_binding() {
        let binding = CommandBinding::builder().build();
        assert!(binding.hotkey().is_none());
        assert!(binding.button_position().is_none());
        assert!(binding.unbutton_position().is_none());
        assert!(binding.tip().is_none());
        assert!(binding.un_tip().is_none());
    }

    #[test]
    fn command_builder_sets_hotkey() {
        let hotkey = Hotkey::from('M');
        let expected = Hotkey::from('M');
        let binding = CommandBinding::builder().hotkey(hotkey).build();
        assert_eq!(binding.hotkey(), Some(&expected));
    }

    #[test]
    fn command_builder_sets_button_position() {
        let position = ButtonPosition::new(1, 2);
        let binding = CommandBinding::builder().button_position(position).build();
        assert_eq!(
            binding.button_position().copied(),
            Some(ButtonPosition::new(1, 2))
        );
    }

    #[test]
    fn command_builder_sets_unbutton_position() {
        let position = ButtonPosition::new(0, 1);
        let binding = CommandBinding::builder()
            .unbutton_position(position)
            .build();
        assert_eq!(
            binding.unbutton_position().copied(),
            Some(ButtonPosition::new(0, 1))
        );
    }

    #[test]
    fn command_builder_sets_tip() {
        let binding = CommandBinding::builder().tip("Move").build();
        assert_eq!(binding.tip(), Some("Move"));
    }

    #[test]
    fn command_builder_sets_un_tip() {
        let binding = CommandBinding::builder().un_tip("Cancel Move").build();
        assert_eq!(binding.un_tip(), Some("Cancel Move"));
    }

    #[test]
    fn command_builder_all_fields_survive_serialization_round_trip() {
        let hotkey = Hotkey::from('M');
        let button_position = ButtonPosition::new(1, 2);
        let unbutton_position = ButtonPosition::new(2, 2);
        let binding = CommandBinding::builder()
            .hotkey(hotkey)
            .button_position(button_position)
            .unbutton_position(unbutton_position)
            .tip("Move Unit")
            .un_tip("Cancel Move")
            .build();
        let file = CustomKeysFile::builder()
            .command("CmdMove", binding)
            .build();
        let serialized = file.to_file_content();
        let reparsed = CustomKeysFile::from(serialized.as_str());
        let reparsed_binding = reparsed
            .command("CmdMove")
            .expect("CmdMove must survive round-trip");
        assert_eq!(reparsed_binding.hotkey(), Some(&Hotkey::Letter('M')));
        assert_eq!(
            reparsed_binding.button_position().copied(),
            Some(ButtonPosition::new(1, 2))
        );
        assert_eq!(
            reparsed_binding.unbutton_position().copied(),
            Some(ButtonPosition::new(2, 2)),
        );
        assert_eq!(reparsed_binding.tip(), Some("Move Unit"));
        assert_eq!(reparsed_binding.un_tip(), Some("Cancel Move"));
    }

    #[test]
    fn file_builder_single_ability_entry_is_accessible() {
        let hotkey = Hotkey::from('Q');
        let expected = Hotkey::from('Q');
        let position = ButtonPosition::new(0, 0);
        let binding = AbilityBinding::builder()
            .hotkey(hotkey)
            .button_position(position)
            .build();
        let file = CustomKeysFile::builder().ability("Ahrl", binding).build();
        let retrieved = file.binding("Ahrl").expect("Ahrl must be present");
        assert_eq!(retrieved.hotkey(), Some(&expected));
    }

    #[test]
    fn file_builder_lookup_is_case_insensitive() {
        let hotkey = Hotkey::from('T');
        let binding = AbilityBinding::builder().hotkey(hotkey).build();
        let file = CustomKeysFile::builder().ability("Hpal", binding).build();
        assert!(file.binding("Hpal").is_some());
        assert!(file.binding("hpal").is_some());
        assert!(file.binding("HPAL").is_some());
    }

    #[test]
    fn file_builder_multiple_entries_iterate_in_alphabetical_order() {
        let binding_ahrl = AbilityBinding::builder().tip("First").build();
        let binding_ahbz = AbilityBinding::builder().tip("Second").build();
        let binding_ahhb = AbilityBinding::builder().tip("Third").build();
        let file = CustomKeysFile::builder()
            .ability("Ahrl", binding_ahrl)
            .ability("AHbz", binding_ahbz)
            .ability("AHhb", binding_ahhb)
            .build();
        let ids: Vec<&str> = file.bindings_in_order().map(|entry| entry.id()).collect();
        assert_eq!(ids, ["ahbz", "ahhb", "ahrl"]);
    }

    #[test]
    fn file_builder_command_entry_is_accessible() {
        let hotkey = Hotkey::from('A');
        let expected = Hotkey::from('A');
        let binding = CommandBinding::builder().hotkey(hotkey).build();
        let file = CustomKeysFile::builder()
            .command("CmdAttack", binding)
            .build();
        let retrieved = file
            .command("CmdAttack")
            .expect("CmdAttack must be present");
        assert_eq!(retrieved.hotkey(), Some(&expected));
    }

    #[test]
    fn file_builder_system_entry_is_accessible() {
        let binding = SystemBinding::new(9, SystemKeybindClass::Game, None);
        let file = CustomKeysFile::builder()
            .system("IsHeroSelect", binding)
            .build();
        let retrieved = file
            .system("IsHeroSelect")
            .expect("IsHeroSelect must be present");
        assert_eq!(retrieved.hotkey(), 9);
        assert_eq!(retrieved.class(), SystemKeybindClass::Game);
    }

    #[test]
    fn file_builder_mixed_entry_types_coexist() {
        let ability_hotkey = Hotkey::from('Q');
        let ability = AbilityBinding::builder().hotkey(ability_hotkey).build();
        let command_hotkey = Hotkey::from('A');
        let command = CommandBinding::builder().hotkey(command_hotkey).build();
        let system = SystemBinding::new(9, SystemKeybindClass::Game, None);
        let file = CustomKeysFile::builder()
            .ability("Ahrl", ability)
            .command("CmdAttack", command)
            .system("IsHeroSelect", system)
            .build();
        assert!(file.binding("Ahrl").is_some());
        assert!(file.command("CmdAttack").is_some());
        assert!(file.system("IsHeroSelect").is_some());
    }

    #[test]
    fn file_builder_ability_is_not_returned_as_command() {
        let hotkey = Hotkey::from('Q');
        let binding = AbilityBinding::builder().hotkey(hotkey).build();
        let file = CustomKeysFile::builder().ability("Ahrl", binding).build();
        assert!(file.command("Ahrl").is_none());
        assert!(file.system("Ahrl").is_none());
    }

    #[test]
    fn file_builder_serializes_ability_section_header() {
        let binding = AbilityBinding::builder().tip("test").build();
        let file = CustomKeysFile::builder().ability("AHhb", binding).build();
        let serialized = file.to_file_content();
        assert!(
            serialized.contains("[ahhb]"),
            "section header must appear in output"
        );
    }

    #[test]
    fn file_builder_serializes_command_section_header() {
        let binding = CommandBinding::builder().tip("Move").build();
        let file = CustomKeysFile::builder()
            .command("CmdMove", binding)
            .build();
        let serialized = file.to_file_content();
        assert!(
            serialized.contains("[cmdmove]"),
            "command section header must appear in output"
        );
    }

    #[test]
    fn file_builder_round_trips_through_parse() {
        let hotkey = Hotkey::from('Q');
        let position = ButtonPosition::new(0, 2);
        let binding = AbilityBinding::builder()
            .hotkey(hotkey)
            .button_position(position)
            .tip("Holy Light")
            .build();
        let original_file = CustomKeysFile::builder().ability("Ahrl", binding).build();
        let serialized = original_file.to_file_content();
        let reparsed_file = CustomKeysFile::from(serialized.as_str());
        let original_binding = original_file.binding("Ahrl").expect("present in original");
        let reparsed_binding = reparsed_file
            .binding("Ahrl")
            .expect("present after round-trip");
        assert_eq!(original_binding.hotkey(), reparsed_binding.hotkey());
        assert_eq!(
            original_binding.button_position(),
            reparsed_binding.button_position()
        );
        assert_eq!(original_binding.tip(), reparsed_binding.tip());
    }

    #[test]
    fn file_builder_system_entry_survives_serialization() {
        let binding = SystemBinding::new(
            49,
            SystemKeybindClass::ControlGroup,
            Some(SystemKeybindModifier::Ctrl),
        );
        let file = CustomKeysFile::builder().system("Ctr1", binding).build();
        let serialized = file.to_file_content();
        let reparsed = CustomKeysFile::from(serialized.as_str());
        let retrieved = reparsed.system("Ctr1").expect("must survive round-trip");
        assert_eq!(retrieved.hotkey(), 49);
        assert_eq!(retrieved.class(), SystemKeybindClass::ControlGroup);
        assert_eq!(retrieved.modifier(), Some(SystemKeybindModifier::Ctrl));
    }
}
