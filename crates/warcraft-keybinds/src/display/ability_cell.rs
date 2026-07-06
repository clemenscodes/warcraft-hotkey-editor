use crate::identity::ability_id::AbilityId;
use crate::identity::hotkey_token::HotkeyToken;
use crate::model::{AbilityBinding, CommandBinding, Hotkey};
use crate::text::command_label::CommandLabel;
use crate::text::tip::Tip;
use warcraft_api::WarcraftObjectId;
use warcraft_database::ObjectLookup;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AbilityIconPath {
    Database(&'static str),
    Binding(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct AbilityCell {
    object_id: WarcraftObjectId,
    display_name: String,
    icon_path: Option<AbilityIconPath>,
    binding_hotkey: Option<HotkeyToken>,
    binding_research_hotkey: Option<HotkeyToken>,
}

impl AbilityCell {
    pub fn for_ability(ability_id: AbilityId, binding: Option<&AbilityBinding>) -> Self {
        let id_str = ability_id.value();
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
        let database_icon_path: Option<AbilityIconPath> = database_object
            .and_then(|warcraft_object| warcraft_object.icons().first().copied())
            .map(AbilityIconPath::Database);
        let binding_icon_path: Option<AbilityIconPath> = binding
            .and_then(|ability_binding| ability_binding.icon())
            .map(|raw_path| {
                let owned = raw_path.to_owned();
                AbilityIconPath::Binding(owned)
            });
        let icon_path = database_icon_path.or(binding_icon_path);
        let binding_hotkey = binding
            .and_then(|ability_binding| ability_binding.hotkey())
            .and_then(Hotkey::first_token);
        let binding_research_hotkey = binding
            .and_then(|ability_binding| ability_binding.research_hotkey())
            .and_then(Hotkey::first_token);
        let object_id = ability_id.object_id();
        Self {
            object_id,
            display_name,
            icon_path,
            binding_hotkey,
            binding_research_hotkey,
        }
    }

    /// Off-state half of a toggle ability ("Stop Defend", "Unburrow",
    /// unmorph). Used by the off-state position picker; pulls the
    /// alternate name (`un_tip` from the database, falling back to the
    /// on-state name) and the `unhotkey` from the binding.
    pub fn for_ability_off(ability_id: AbilityId, binding: Option<&AbilityBinding>) -> Self {
        let id_str = ability_id.value();
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
        let database_off_icon_path: Option<AbilityIconPath> =
            ObjectLookup::off_icon(id_str).map(AbilityIconPath::Database);
        let database_icon_path: Option<AbilityIconPath> = database_object
            .and_then(|warcraft_object| warcraft_object.icons().first().copied())
            .map(AbilityIconPath::Database);
        let un_icon_path: Option<AbilityIconPath> = binding
            .and_then(|ability_binding| ability_binding.un_icon())
            .map(|raw_path| {
                let owned = raw_path.to_owned();
                AbilityIconPath::Binding(owned)
            });
        let icon_path = un_icon_path
            .or(database_off_icon_path)
            .or(database_icon_path);
        let binding_hotkey = binding
            .and_then(|ability_binding| ability_binding.unhotkey())
            .and_then(Hotkey::first_token);
        let object_id = ability_id.object_id();
        Self {
            object_id,
            display_name,
            icon_path,
            binding_hotkey,
            binding_research_hotkey: None,
        }
    }

    pub fn for_command(command_name: WarcraftObjectId, binding: Option<&CommandBinding>) -> Self {
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
        let icon_path: Option<AbilityIconPath> = database_object
            .and_then(|warcraft_object| warcraft_object.icons().first().copied())
            .map(AbilityIconPath::Database);
        let binding_hotkey = binding
            .and_then(|command_binding| command_binding.hotkey())
            .and_then(Hotkey::first_token);
        Self {
            object_id: command_name,
            display_name,
            icon_path,
            binding_hotkey,
            binding_research_hotkey: None,
        }
    }

    pub fn object_id(&self) -> WarcraftObjectId {
        self.object_id
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn binding_hotkey(&self) -> Option<HotkeyToken> {
        self.binding_hotkey
    }

    pub fn binding_research_hotkey(&self) -> Option<HotkeyToken> {
        self.binding_research_hotkey
    }

    pub fn icon_path(&self) -> Option<&AbilityIconPath> {
        self.icon_path.as_ref()
    }
}

impl ddd::ReadModel for AbilityCell {}
