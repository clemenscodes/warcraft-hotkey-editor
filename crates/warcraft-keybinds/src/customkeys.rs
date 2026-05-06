use crate::CustomKeysFile;
use crate::export::serialize;

const BUNDLED_BASELINE: &str = include_str!("../../hotkey-editor/templates/CustomKeys.txt");

/// Canonical, fully-normalized CustomKeys.txt state.
///
/// This is the only type the frontend may use to represent or mutate
/// CustomKeys.txt state. Constructors run the full normalize pipeline
/// — overlay onto bundled baseline, materialize defaults, cascade
/// resolve — so callers never see un-resolved positions. The text
/// returned by `to_text` is what is written to localStorage verbatim
/// and downloaded as the user's CustomKeys.txt.
///
/// See `docs/ARCHITECTURE.md` for the full contract this type enforces.
pub struct CustomKeys {
    parsed: CustomKeysFile,
    text: String,
}

impl CustomKeys {
    /// Build the default state: the bundled baseline, fully normalized.
    /// Used on first boot when localStorage has no entry yet.
    pub fn from_default() -> Self {
        let empty_overlay = CustomKeysFile::from("");
        Self::normalize(&empty_overlay)
    }

    fn normalize(overlay: &CustomKeysFile) -> Self {
        let normalized_text = serialize(overlay, BUNDLED_BASELINE);
        let parsed = CustomKeysFile::from(normalized_text.as_str());
        Self {
            parsed,
            text: normalized_text,
        }
    }
}

impl From<&str> for CustomKeys {
    fn from(input_text: &str) -> Self {
        let overlay = CustomKeysFile::from(input_text);
        Self::normalize(&overlay)
    }
}

impl From<&CustomKeysFile> for CustomKeys {
    fn from(file: &CustomKeysFile) -> Self {
        Self::normalize(file)
    }
}

impl CustomKeys {
    /// The canonical, normalized text. This is what gets written to
    /// localStorage and downloaded as the user's CustomKeys.txt.
    pub fn to_text(&self) -> &str {
        &self.text
    }

    /// Read-only typed view of the parsed state. Renderers use this to
    /// query bindings; they must never mutate through it.
    pub fn parsed(&self) -> &CustomKeysFile {
        &self.parsed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Hotkey;

    #[test]
    fn from_default_produces_non_empty_text() {
        let custom_keys = CustomKeys::from_default();
        let normalized_text = custom_keys.to_text();
        let normalized_is_empty = normalized_text.is_empty();
        assert!(!normalized_is_empty);
    }

    #[test]
    fn from_default_includes_known_baseline_sections() {
        let custom_keys = CustomKeys::from_default();
        let normalized_text = custom_keys.to_text();
        let has_hpal = normalized_text.contains("[hpal]");
        let has_cmd_attack = normalized_text.contains("[cmdattack]");
        assert!(has_hpal);
        assert!(has_cmd_attack);
    }

    #[test]
    fn from_text_is_idempotent_on_normalized_input() {
        let first_pass = CustomKeys::from_default();
        let first_pass_text = first_pass.to_text();
        let first_text = first_pass_text.to_string();
        let second_pass = CustomKeys::from(first_text.as_str());
        let second_text = second_pass.to_text();
        assert_eq!(first_text, second_text);
    }

    #[test]
    fn parsed_view_returns_consistent_bindings() {
        let custom_keys = CustomKeys::from_default();
        let parsed_view = custom_keys.parsed();
        let hpal_binding = parsed_view.binding("Hpal");
        let hpal_present = hpal_binding.is_some();
        assert!(hpal_present);
    }

    #[test]
    fn from_text_overlays_user_hotkey_on_baseline() {
        let user_input = "[Ahrl]\nHotkey=Z\n\n";
        let custom_keys = CustomKeys::from(user_input);
        let parsed_view = custom_keys.parsed();
        let ahrl_binding = parsed_view.binding("Ahrl");
        let ahrl_hotkey = ahrl_binding.and_then(|binding| binding.hotkey());
        let expected_hotkey = Hotkey::Letter('Z');
        assert_eq!(ahrl_hotkey, Some(&expected_hotkey));
    }

    #[test]
    fn normalized_text_has_concrete_button_position_for_known_ability() {
        // Ahrl (Holy Light) has a database-default Buttonpos. After
        // normalize, that position must be materialized into the text —
        // the renderer must never need to look up defaults itself.
        let custom_keys = CustomKeys::from_default();
        let normalized_text = custom_keys.to_text();
        let ahrl_marker = "[ahrl]";
        let ahrl_marker_length = ahrl_marker.len();
        let ahrl_section_start = normalized_text
            .find(ahrl_marker)
            .expect("baseline must contain [ahrl]");
        let after_ahrl = &normalized_text[ahrl_section_start..];
        let after_ahrl_marker = &after_ahrl[ahrl_marker_length..];
        let after_ahrl_length = after_ahrl.len();
        let relative_next_section = after_ahrl_marker.find('[');
        let next_section_offset = relative_next_section
            .map(|relative_offset| relative_offset + ahrl_marker_length)
            .unwrap_or(after_ahrl_length);
        let ahrl_section = &after_ahrl[..next_section_offset];
        let has_buttonpos = ahrl_section.contains("Buttonpos=");
        assert!(
            has_buttonpos,
            "[Ahrl] section must have a concrete Buttonpos after normalize",
        );
    }
}
