pub use warcraft_api::{ColumnIndex, GridCoordinate, RowIndex};

pub mod builders;
pub mod hotkey;
pub(crate) mod section;

mod ability_binding;
mod binding_entry;
mod command_binding;
mod command_entry;
mod system_binding;
mod warcraft_keybinding;

pub use ability_binding::AbilityBinding;
pub use binding_entry::BindingEntry;
pub use builders::{AbilityBindingBuilder, CommandBindingBuilder, CustomKeysBuilder};
pub use command_binding::CommandBinding;
pub use command_entry::CommandEntry;
pub use hotkey::{AbilityModifier, Hotkey, ParseAbilityModifierError, ParseHotkeyError};
pub(crate) use section::{SectionAccumulator, SectionResolution};
pub use system_binding::SystemBinding;
pub use warcraft_keybinding::WarcraftKeybinding;

#[cfg(test)]
mod model_tests {
    use super::section::SectionKind;
    use super::*;
    use crate::identity::keycode::Letter;

    #[test]
    fn hotkey_letter_is_normalized_to_uppercase() {
        let hotkey = Hotkey::from('q');
        assert_eq!(hotkey, Hotkey::Letter('Q'));
    }

    #[test]
    fn hotkey_try_from_empty_string_returns_err() {
        assert!(Hotkey::try_from("").is_err());
    }

    #[test]
    fn hotkey_try_from_letter_returns_uppercased() {
        let hotkey = Hotkey::try_from("w").unwrap();
        assert_eq!(hotkey, Hotkey::Letter('W'));
    }

    #[test]
    fn hotkey_try_from_function_key_case_insensitive() {
        let hotkey_lower = Hotkey::try_from("f3").unwrap();
        let hotkey_upper = Hotkey::try_from("F3").unwrap();
        assert_eq!(hotkey_lower, Hotkey::FunctionKey(3));
        assert_eq!(hotkey_upper, Hotkey::FunctionKey(3));
    }

    #[test]
    fn hotkey_try_from_virtual_key_numeric_string() {
        let hotkey = Hotkey::try_from("27").unwrap();
        assert_eq!(hotkey, Hotkey::VirtualKey(27));
    }

    #[test]
    fn hotkey_try_from_multi_level_comma_separated() {
        use crate::identity::hotkey_token::HotkeyToken;
        let hotkey = Hotkey::try_from("Q,W,E").unwrap();
        let expected = Hotkey::MultiLevel {
            tokens: [
                Some(HotkeyToken::Letter(Letter::Q)),
                Some(HotkeyToken::Letter(Letter::W)),
                Some(HotkeyToken::Letter(Letter::E)),
                None,
            ],
        };
        assert_eq!(hotkey, expected);
    }

    #[test]
    fn hotkey_display_letter() {
        let hotkey = Hotkey::Letter('A');
        assert_eq!(hotkey.to_string(), "A");
    }

    #[test]
    fn hotkey_display_function_key() {
        let hotkey = Hotkey::FunctionKey(7);
        assert_eq!(hotkey.to_string(), "F7");
    }

    #[test]
    fn hotkey_display_virtual_key() {
        let hotkey = Hotkey::VirtualKey(9);
        assert_eq!(hotkey.to_string(), "9");
    }

    #[test]
    fn hotkey_display_multi_level() {
        use crate::identity::hotkey_token::HotkeyToken;
        let hotkey = Hotkey::MultiLevel {
            tokens: [
                Some(HotkeyToken::Letter(Letter::Q)),
                Some(HotkeyToken::Letter(Letter::Q)),
                None,
                None,
            ],
        };
        assert_eq!(hotkey.to_string(), "Q,Q");
    }

    #[test]
    fn hotkey_from_string_roundtrip() {
        let original = Hotkey::FunctionKey(12);
        let string_form: String = original.into();
        let reparsed = Hotkey::try_from(string_form.as_str()).unwrap();
        assert_eq!(original, reparsed);
    }

    #[test]
    fn button_position_try_from_valid_string() {
        let position = GridCoordinate::try_from("2,1").unwrap();
        assert_eq!(position.column(), ColumnIndex::Two);
        assert_eq!(position.row(), RowIndex::One);
    }

    #[test]
    fn button_position_try_from_invalid_returns_err() {
        assert!(GridCoordinate::try_from("notanumber").is_err());
        assert!(GridCoordinate::try_from("1").is_err());
        assert!(GridCoordinate::try_from("").is_err());
    }

    #[test]
    fn button_position_display_roundtrip() {
        let position = GridCoordinate::new(ColumnIndex::Three, RowIndex::Two);
        let displayed = position.to_string();
        let reparsed = GridCoordinate::try_from(displayed.as_str()).unwrap();
        assert_eq!(position, reparsed);
    }

    #[test]
    fn ability_modifier_display_variants() {
        assert_eq!(AbilityModifier::Alt.to_string(), "Alt");
        assert_eq!(AbilityModifier::Ctrl.to_string(), "Ctrl");
        assert_eq!(AbilityModifier::CtrlOrAlt.to_string(), "Ctrl_or_Alt");
        assert_eq!(AbilityModifier::Shift.to_string(), "Shift");
    }

    #[test]
    fn ability_modifier_try_from_case_insensitive() {
        assert_eq!(
            AbilityModifier::try_from("ALT").unwrap(),
            AbilityModifier::Alt
        );
        assert_eq!(
            AbilityModifier::try_from("shift").unwrap(),
            AbilityModifier::Shift
        );
    }

    #[test]
    fn section_resolution_resolves_known_ability() {
        let resolution = SectionResolution::from_section_id("Hpal").unwrap();
        assert!(matches!(resolution.kind(), SectionKind::Ability));
    }

    #[test]
    fn section_resolution_resolves_known_command() {
        let resolution = SectionResolution::from_section_id("CmdAttack").unwrap();
        assert!(matches!(resolution.kind(), SectionKind::Command));
    }

    #[test]
    fn section_resolution_returns_none_for_unknown_id() {
        let result = SectionResolution::from_section_id("ZZZUnknown");
        assert!(result.is_none());
    }

    #[test]
    fn section_resolution_is_case_insensitive() {
        let lower = SectionResolution::from_section_id("hpal");
        let upper = SectionResolution::from_section_id("HPAL");
        assert!(lower.is_some());
        assert!(upper.is_some());
        let lower_unwrapped = lower.unwrap();
        let upper_unwrapped = upper.unwrap();
        assert_eq!(
            lower_unwrapped.canonical_id(),
            upper_unwrapped.canonical_id()
        );
    }
}
