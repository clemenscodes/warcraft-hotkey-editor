use warcraft_keybinds::{AbilityBinding, CommandBinding};

use crate::domain::hotkey_token::{CustomKeysValue, HotkeyToken};
use crate::domain::icons::IconUrl;
use crate::domain::object_lookup::ObjectLookup;
use crate::text::command_label::CommandLabel;
use crate::text::tip::Tip;

#[derive(Clone, PartialEq)]
pub(crate) struct AbilityCell {
    object_id: String,
    display_name: String,
    icon_src: Option<String>,
    binding_hotkey: Option<HotkeyToken>,
    binding_research_hotkey: Option<HotkeyToken>,
}

impl AbilityCell {
    pub(crate) fn for_ability(object_id: &str, binding: Option<&AbilityBinding>) -> Self {
        let database_object = ObjectLookup::by_id(object_id);
        let database_name = database_object
            .and_then(|warcraft_object| warcraft_object.names().first().copied())
            .map(String::from);
        let tip_name = binding
            .and_then(|ability_binding| ability_binding.tip())
            .map(Tip::shortened);
        let display_name = database_name
            .or(tip_name)
            .unwrap_or_else(|| String::from("(unknown)"));
        let database_icon = database_object
            .and_then(|warcraft_object| warcraft_object.icons().first().copied())
            .map(IconUrl::from_database_path);
        let binding_icon = binding
            .and_then(|ability_binding| ability_binding.icon())
            .map(IconUrl::from_binding_path);
        let icon_src = database_icon.or(binding_icon);
        let binding_hotkey = binding
            .and_then(|ability_binding| ability_binding.hotkey())
            .and_then(BindingHotkey::first_token);
        let binding_research_hotkey = binding
            .and_then(|ability_binding| ability_binding.research_hotkey())
            .and_then(BindingHotkey::first_token);
        Self {
            object_id: object_id.to_string(),
            display_name,
            icon_src,
            binding_hotkey,
            binding_research_hotkey,
        }
    }

    pub(crate) fn for_command(command_name: &str, binding: Option<&CommandBinding>) -> Self {
        let database_object = ObjectLookup::by_id(command_name);
        let database_name = database_object
            .and_then(|warcraft_object| warcraft_object.names().first().copied())
            .map(String::from);
        let display_name = binding
            .and_then(|command_binding| command_binding.tip())
            .map(Tip::shortened)
            .or(database_name)
            .unwrap_or_else(|| CommandLabel::pretty(command_name));
        let icon_src = database_object
            .and_then(|warcraft_object| warcraft_object.icons().first().copied())
            .map(IconUrl::from_database_path);
        let binding_hotkey = binding
            .and_then(|command_binding| command_binding.hotkey())
            .and_then(BindingHotkey::first_token);
        Self {
            object_id: command_name.to_string(),
            display_name,
            icon_src,
            binding_hotkey,
            binding_research_hotkey: None,
        }
    }

    pub(crate) fn object_id(&self) -> &str {
        &self.object_id
    }

    pub(crate) fn display_name(&self) -> &str {
        &self.display_name
    }

    pub(crate) fn binding_hotkey(&self) -> Option<HotkeyToken> {
        self.binding_hotkey
    }

    pub(crate) fn binding_research_hotkey(&self) -> Option<HotkeyToken> {
        self.binding_research_hotkey
    }

    pub(crate) fn cloned_object_id(&self) -> String {
        self.object_id.clone()
    }

    pub(crate) fn cloned_display_name(&self) -> String {
        self.display_name.clone()
    }

    pub(crate) fn cloned_icon_src(&self) -> Option<String> {
        self.icon_src.clone()
    }
}

pub(crate) struct BindingHotkey;

impl BindingHotkey {
    pub(crate) fn first_token(raw_value: &str) -> Option<HotkeyToken> {
        let first_segment = raw_value.split(',').next()?;
        HotkeyToken::try_from(first_segment).ok()
    }

    pub(crate) fn comma_segment_count(raw_value: &str) -> usize {
        let trimmed = raw_value.trim();
        if trimmed.is_empty() {
            return 0;
        }
        trimmed.split(',').count()
    }

    pub(crate) fn replicated_token(token: HotkeyToken, level_count: usize) -> String {
        let count = level_count.max(1);
        let serialized_value = CustomKeysValue::from(token);
        let segment = serialized_value.as_str();
        std::iter::repeat_n(segment, count)
            .collect::<Vec<_>>()
            .join(",")
    }

    /// Whether the grid-layout apply pass is allowed to overwrite this raw
    /// hotkey value with a positional letter. Special tokens (Escape /
    /// Mouse4 / Mouse5) have no grid position by design — the user picked
    /// them deliberately, so an "Apply grid to all hotkeys" pass should
    /// leave them alone. Empty / unparseable values return true so a fresh
    /// import gets populated normally.
    pub(crate) fn accepts_grid_letter(raw_value: Option<&str>) -> bool {
        let Some(value) = raw_value else {
            return true;
        };
        let Some(token) = Self::first_token(value) else {
            return true;
        };
        char::try_from(token).is_ok()
    }
}
