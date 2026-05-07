use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::{AbilityBinding, CommandBinding, Hotkey};

use crate::domain::hotkey_token::HotkeyToken;
use crate::domain::icons::IconUrl;
use crate::domain::object_lookup::ObjectLookup;
use crate::text::command_label::CommandLabel;
use crate::text::tip::Tip;

#[derive(Clone, PartialEq)]
pub(crate) struct AbilityCell {
    object_id: WarcraftObjectId,
    display_name: String,
    icon_src: Option<String>,
    binding_hotkey: Option<HotkeyToken>,
    binding_research_hotkey: Option<HotkeyToken>,
}

impl AbilityCell {
    pub(crate) fn for_ability(
        object_id: WarcraftObjectId,
        binding: Option<&AbilityBinding>,
    ) -> Self {
        let id_str = object_id.value();
        let database_object = ObjectLookup::by_id(id_str);
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
            object_id,
            display_name,
            icon_src,
            binding_hotkey,
            binding_research_hotkey,
        }
    }

    /// Off-state half of a toggle ability ("Stop Defend", "Unburrow",
    /// unmorph). Used by the off-state position picker; pulls the
    /// alternate name (`un_tip` from the database, falling back to the
    /// on-state name) and the `unhotkey` from the binding.
    pub(crate) fn for_ability_off(
        object_id: WarcraftObjectId,
        binding: Option<&AbilityBinding>,
    ) -> Self {
        let id_str = object_id.value();
        let database_object = ObjectLookup::by_id(id_str);
        let alt_name = database_object
            .and_then(|warcraft_object| warcraft_object.un_tip())
            .map(String::from);
        let database_name = database_object
            .and_then(|warcraft_object| warcraft_object.names().first().copied())
            .map(String::from);
        let tip_name = binding
            .and_then(|ability_binding| ability_binding.tip())
            .map(Tip::shortened);
        let display_name = alt_name
            .or(database_name)
            .or(tip_name)
            .unwrap_or_else(|| String::from("(unknown)"));
        // Off icon priority: binding un_icon override → database UnArt
        // (AbilityMeta::off_icon, parsed from UnArt= in abilityfunc.txt) →
        // on-state icon as last resort.
        let database_off_icon = ObjectLookup::off_icon(id_str).map(IconUrl::from_database_path);
        let database_icon = database_object
            .and_then(|warcraft_object| warcraft_object.icons().first().copied())
            .map(IconUrl::from_database_path);
        let un_icon = binding
            .and_then(|ability_binding| ability_binding.un_icon())
            .map(IconUrl::from_binding_path);
        let icon_src = un_icon.or(database_off_icon).or(database_icon);
        let binding_hotkey = binding
            .and_then(|ability_binding| ability_binding.unhotkey())
            .and_then(BindingHotkey::first_token);
        Self {
            object_id,
            display_name,
            icon_src,
            binding_hotkey,
            binding_research_hotkey: None,
        }
    }

    pub(crate) fn for_command(
        command_name: WarcraftObjectId,
        binding: Option<&CommandBinding>,
    ) -> Self {
        let id_str = command_name.value();
        let database_object = ObjectLookup::by_id(id_str);
        let database_name = database_object
            .and_then(|warcraft_object| warcraft_object.names().first().copied())
            .map(String::from);
        let display_name = binding
            .and_then(|command_binding| command_binding.tip())
            .map(Tip::shortened)
            .or(database_name)
            .unwrap_or_else(|| CommandLabel::pretty(id_str));
        let icon_src = database_object
            .and_then(|warcraft_object| warcraft_object.icons().first().copied())
            .map(IconUrl::from_database_path);
        let binding_hotkey = binding
            .and_then(|command_binding| command_binding.hotkey())
            .and_then(BindingHotkey::first_token);
        Self {
            object_id: command_name,
            display_name,
            icon_src,
            binding_hotkey,
            binding_research_hotkey: None,
        }
    }

    pub(crate) fn object_id(&self) -> WarcraftObjectId {
        self.object_id
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

    pub(crate) fn cloned_display_name(&self) -> String {
        self.display_name.clone()
    }

    pub(crate) fn cloned_icon_src(&self) -> Option<String> {
        self.icon_src.clone()
    }
}

pub(crate) struct BindingHotkey;

impl BindingHotkey {
    pub(crate) fn first_token(hotkey: &Hotkey) -> Option<HotkeyToken> {
        let single = match hotkey {
            Hotkey::MultiLevel(levels) => levels.first()?,
            other => other,
        };
        HotkeyToken::try_from(single).ok()
    }

    pub(crate) fn comma_segment_count(hotkey: &Hotkey) -> usize {
        match hotkey {
            Hotkey::MultiLevel(levels) => levels.len(),
            _ => 1,
        }
    }

    pub(crate) fn replicated_token(token: HotkeyToken, level_count: usize) -> Hotkey {
        let count = level_count.max(1);
        let single = Hotkey::from(token);
        if count == 1 {
            single
        } else {
            Hotkey::MultiLevel(vec![single; count])
        }
    }

    /// Whether the grid-layout apply pass is allowed to overwrite this hotkey
    /// with a positional letter. Special tokens (Escape / Mouse4 / Mouse5)
    /// have no grid position by design — leave them alone. Empty / unknown
    /// values return true so a fresh import gets populated normally.
    pub(crate) fn accepts_grid_letter(hotkey: Option<&Hotkey>) -> bool {
        let Some(hotkey) = hotkey else {
            return true;
        };
        let Some(token) = Self::first_token(hotkey) else {
            return true;
        };
        char::try_from(token).is_ok()
    }
}
