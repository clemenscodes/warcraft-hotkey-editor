use warcraft_api::{GridCoordinate, WarcraftObjectId, WarcraftObjectMeta};
use warcraft_database::{BuildingTraits, ObjectLookup};

use crate::custom_keys::CustomKeys;
use crate::display::ability_cell::{AbilityCell, AbilityIconPath};
use crate::identity::hotkey_token::HotkeyToken;
use crate::identity::slot::GridSlotId;
use crate::text::color_codes::WarcraftColorCodes;

#[derive(Clone, PartialEq)]
pub struct InspectorDetail {
    display_name: String,
    object_id: WarcraftObjectId,
    icon_path: Option<AbilityIconPath>,
    hotkey_token: Option<HotkeyToken>,
    research_hotkey_token: Option<HotkeyToken>,
    button_position: Option<GridCoordinate>,
    research_button_position: Option<GridCoordinate>,
    tip: Option<String>,
    research_tip: Option<String>,
    ubertip: Option<String>,
    research_ubertip: Option<String>,
    /// Display name of the off / alt state (e.g. "Stop Defend"). Populated for
    /// toggleable abilities whose `un_tip` differs from the on-state name; the
    /// inspector renders both so the player can see what the alternate button
    /// becomes when the ability is active. `None` for non-toggle abilities and
    /// for hosts where the inspector is already flipped to the alt state via
    /// `prefer_un_state`.
    alt_display_name: Option<String>,
    /// Off-state long description ("Deactivate to move at normal speed…").
    /// Same conditions as `alt_display_name`.
    alt_ubertip: Option<String>,
    /// Off-state hotkey on a toggle ability (Defend → Stop Defend, Burrow →
    /// Unburrow, …). Drives a dedicated edit field on the override card so a
    /// player can bind D for Defend and X for Stop Defend independently. Set
    /// only on `Ability` slots whose object carries `un_tip`/`un_ubertip`;
    /// `None` everywhere else.
    alt_hotkey_token: Option<HotkeyToken>,
    name_levels: Vec<String>,
    icon_levels: Vec<Option<&'static str>>,
    ubertip_levels: Vec<String>,
    is_command: bool,
    is_passive: bool,
    /// Passive racial ability shown in the research panel for informational
    /// purposes only (e.g. Shadow Meld Item, Ultravision Item). Not bindable.
    info_only: bool,
    /// Upgraded-form unit ID for train-slot pairs that share a button position
    /// (e.g. base Siege Engine `hmtt` → upgraded `hrtt`). Populated only on
    /// the base train slot; `None` everywhere else.
    upgrade_unit_id: Option<WarcraftObjectId>,
    /// Display name of the upgraded form (e.g. "Siege Engine").
    upgrade_display_name: Option<String>,
    /// Hotkey currently assigned to the upgraded form's binding, if any.
    upgrade_hotkey_token: Option<HotkeyToken>,
    /// True when this detail was built from an `AbilityOff` slot — the hotkey
    /// field holds `Unhotkey` and the override button must write `Unhotkey`.
    is_off_state: bool,
}

impl InspectorDetail {
    pub fn build(
        slot: &GridSlotId,
        custom_keys: &Option<CustomKeys>,
        host_unit_id: &str,
        from_uprooted: bool,
        from_research: bool,
        upgrade_unit_id: Option<WarcraftObjectId>,
    ) -> Self {
        let custom_keys_ref = custom_keys.as_ref();
        match slot {
            GridSlotId::Ability(ability_id) => {
                let ability_id_str = ability_id.value();
                let binding = custom_keys_ref.and_then(|file| file.binding(ability_id_str));
                let cell = AbilityCell::for_ability(*ability_id, binding);
                let position = custom_keys_ref.and_then(|file| file.position_for_slot(slot, false));
                let research_position = custom_keys_ref
                    .and_then(|file| file.binding(ability_id_str))
                    .and_then(|ability_binding| ability_binding.research_button_position())
                    .copied();
                let hotkey_token = binding
                    .and_then(|ability_binding| {
                        ability_binding
                            .hotkey()
                            .or_else(|| ability_binding.research_hotkey())
                    })
                    .and_then(|hotkey| hotkey.first_token());
                let research_hotkey_token = binding
                    .and_then(|ability_binding| ability_binding.research_hotkey())
                    .and_then(|hotkey| hotkey.first_token());
                let tip = binding.and_then(|ability_binding| {
                    ability_binding.tip().map(WarcraftColorCodes::stripped)
                });
                let research_tip = binding.and_then(|ability_binding| {
                    ability_binding
                        .research_tip()
                        .map(WarcraftColorCodes::stripped)
                });
                let database_object = ObjectLookup::by_id(ability_id_str);
                let is_passive = database_object
                    .and_then(|warcraft_object| warcraft_object.icons().first().copied())
                    .map(|icon_path| {
                        icon_path
                            .to_ascii_lowercase()
                            .starts_with("passivebuttons/")
                    })
                    .unwrap_or(false);
                let info_only = from_research
                    && database_object
                        .map(|object| matches!(object.meta(), WarcraftObjectMeta::Ability(meta) if meta.max_level() == 1 && !meta.is_ultimate()))
                        .unwrap_or(false);
                let object_has_alt_state = database_object
                    .map(|warcraft_object| {
                        warcraft_object.un_ubertip().is_some() || warcraft_object.un_tip().is_some()
                    })
                    .unwrap_or(false);
                let host_starts_in_alt =
                    BuildingTraits::unit_starts_in_toggle_alt_state(host_unit_id);
                let is_morph_targeting_host = !host_unit_id.is_empty()
                    && ObjectLookup::morph_target_unit(ability_id_str)
                        .is_some_and(|target| target.eq_ignore_ascii_case(host_unit_id));
                let prefer_un_state = !from_uprooted
                    && (host_starts_in_alt || is_morph_targeting_host)
                    && object_has_alt_state;
                let primary_ubertip = if prefer_un_state {
                    database_object.and_then(|warcraft_object| warcraft_object.un_ubertip())
                } else {
                    database_object.and_then(|warcraft_object| warcraft_object.ubertip())
                };
                let ubertip = primary_ubertip.map(WarcraftColorCodes::stripped);
                let ability_is_morph = ObjectLookup::morph_target_unit(ability_id_str).is_some();
                let ability_off_on_alt_unit = !ability_is_morph
                    && BuildingTraits::ability_is_on_alt_state_unit(ability_id_str);
                let should_show_alt_state = object_has_alt_state
                    && !prefer_un_state
                    && !ability_is_morph
                    && !ability_off_on_alt_unit
                    && !from_uprooted;
                let alt_display_name = should_show_alt_state
                    .then(|| {
                        database_object
                            .and_then(|warcraft_object| warcraft_object.un_tip())
                            .map(WarcraftColorCodes::stripped)
                    })
                    .flatten();
                let alt_ubertip = should_show_alt_state
                    .then(|| {
                        database_object
                            .and_then(|warcraft_object| warcraft_object.un_ubertip())
                            .map(WarcraftColorCodes::stripped)
                    })
                    .flatten();
                let alt_hotkey_token = should_show_alt_state
                    .then(|| {
                        binding
                            .and_then(|ability_binding| ability_binding.unhotkey())
                            .and_then(|hotkey| hotkey.first_token())
                    })
                    .flatten();
                let research_ubertip = database_object
                    .and_then(|warcraft_object| warcraft_object.research_ubertip())
                    .map(WarcraftColorCodes::stripped);
                let resolved_display_name = if prefer_un_state {
                    database_object
                        .and_then(|warcraft_object| warcraft_object.un_tip())
                        .map(WarcraftColorCodes::stripped)
                        .unwrap_or_else(|| cell.display_name().to_string())
                } else {
                    cell.display_name().to_string()
                };
                let name_levels: Vec<String> = database_object
                    .map(|warcraft_object| {
                        warcraft_object
                            .names()
                            .iter()
                            .map(|raw| (*raw).to_string())
                            .collect()
                    })
                    .unwrap_or_default();
                let icon_levels: Vec<Option<&'static str>> = database_object
                    .map(|warcraft_object| {
                        warcraft_object
                            .icons()
                            .iter()
                            .copied()
                            .map(|raw_icon| {
                                let trimmed_icon = raw_icon.trim();
                                if trimmed_icon.is_empty() {
                                    None
                                } else {
                                    Some(trimmed_icon)
                                }
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                let ubertip_levels: Vec<String> = database_object
                    .map(|warcraft_object| {
                        warcraft_object
                            .ubertip_levels()
                            .iter()
                            .map(|raw_text| WarcraftColorCodes::stripped(raw_text))
                            .collect()
                    })
                    .unwrap_or_default();
                let icon_path = if prefer_un_state {
                    let off_cell = AbilityCell::for_ability_off(*ability_id, binding);
                    off_cell.icon_path().cloned()
                } else {
                    cell.icon_path().cloned()
                };
                let object_id = cell.object_id();
                let upgrade_unit_id_field = upgrade_unit_id;
                let upgrade_display_name = upgrade_unit_id
                    .and_then(|upgrade_id| ObjectLookup::by_id(upgrade_id.value()))
                    .and_then(|obj| obj.names().first().copied())
                    .map(String::from);
                let upgrade_hotkey_token = upgrade_unit_id
                    .and_then(|upgrade_id| {
                        let upgrade_id_str = upgrade_id.value();
                        custom_keys_ref.and_then(|file| file.binding(upgrade_id_str))
                    })
                    .and_then(|upgrade_binding| upgrade_binding.hotkey())
                    .and_then(|hotkey| hotkey.first_token());
                Self {
                    display_name: resolved_display_name,
                    object_id,
                    icon_path,
                    hotkey_token,
                    research_hotkey_token,
                    button_position: position,
                    research_button_position: research_position,
                    tip,
                    research_tip,
                    ubertip,
                    research_ubertip,
                    alt_display_name,
                    alt_ubertip,
                    alt_hotkey_token,
                    name_levels,
                    icon_levels,
                    ubertip_levels,
                    is_command: false,
                    is_passive,
                    info_only,
                    upgrade_unit_id: upgrade_unit_id_field,
                    upgrade_display_name,
                    upgrade_hotkey_token,
                    is_off_state: false,
                }
            }
            GridSlotId::AbilityOff(ability_id) => {
                let ability_id_str = ability_id.value();
                let binding = custom_keys_ref.and_then(|file| file.binding(ability_id_str));
                let cell = AbilityCell::for_ability_off(*ability_id, binding);
                let position = custom_keys_ref.and_then(|file| file.position_for_slot(slot, false));
                let hotkey_token = binding
                    .and_then(|ability_binding| ability_binding.unhotkey())
                    .and_then(|hotkey| hotkey.first_token());
                let database_object = ObjectLookup::by_id(ability_id_str);
                let display_name = database_object
                    .and_then(|warcraft_object| warcraft_object.un_tip())
                    .map(WarcraftColorCodes::stripped)
                    .unwrap_or_else(|| cell.display_name().to_string());
                let ubertip = database_object
                    .and_then(|warcraft_object| warcraft_object.un_ubertip())
                    .map(WarcraftColorCodes::stripped);
                let icon_path = cell.icon_path().cloned();
                let object_id = cell.object_id();
                Self {
                    display_name,
                    object_id,
                    icon_path,
                    hotkey_token,
                    research_hotkey_token: None,
                    button_position: position,
                    research_button_position: None,
                    tip: None,
                    research_tip: None,
                    ubertip,
                    research_ubertip: None,
                    alt_display_name: None,
                    alt_ubertip: None,
                    alt_hotkey_token: None,
                    name_levels: Vec::new(),
                    icon_levels: Vec::new(),
                    ubertip_levels: Vec::new(),
                    is_command: false,
                    is_passive: false,
                    info_only: false,
                    upgrade_unit_id: None,
                    upgrade_display_name: None,
                    upgrade_hotkey_token: None,
                    is_off_state: true,
                }
            }
            GridSlotId::Command(command_name) => {
                let command_name_str = command_name.value();
                let binding = custom_keys_ref.and_then(|file| file.command(command_name_str));
                let cell = AbilityCell::for_command(*command_name, binding);
                let position = custom_keys_ref.and_then(|file| file.position_for_slot(slot, false));
                let hotkey_token = binding
                    .and_then(|command_binding| command_binding.hotkey())
                    .and_then(|hotkey| hotkey.first_token());
                let database_object = ObjectLookup::by_id(command_name_str);
                let tip = database_object
                    .and_then(|warcraft_object| warcraft_object.tip())
                    .map(WarcraftColorCodes::stripped)
                    .or_else(|| {
                        binding.and_then(|command_binding| {
                            command_binding.tip().map(WarcraftColorCodes::stripped)
                        })
                    });
                let ubertip = database_object
                    .and_then(|warcraft_object| warcraft_object.ubertip())
                    .map(WarcraftColorCodes::stripped);
                let icon_path = cell.icon_path().cloned();
                let display_name = cell.display_name().to_string();
                let object_id = cell.object_id();
                Self {
                    display_name,
                    object_id,
                    icon_path,
                    hotkey_token,
                    research_hotkey_token: None,
                    button_position: position,
                    research_button_position: None,
                    tip,
                    research_tip: None,
                    ubertip,
                    research_ubertip: None,
                    alt_display_name: None,
                    alt_ubertip: None,
                    alt_hotkey_token: None,
                    name_levels: Vec::new(),
                    icon_levels: Vec::new(),
                    ubertip_levels: Vec::new(),
                    is_command: true,
                    is_passive: false,
                    info_only: false,
                    upgrade_unit_id: None,
                    upgrade_display_name: None,
                    upgrade_hotkey_token: None,
                    is_off_state: false,
                }
            }
        }
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn object_id(&self) -> WarcraftObjectId {
        self.object_id
    }

    pub fn icon_path(&self) -> Option<&AbilityIconPath> {
        self.icon_path.as_ref()
    }

    pub fn hotkey_token(&self) -> Option<HotkeyToken> {
        self.hotkey_token
    }

    pub fn research_hotkey_token(&self) -> Option<HotkeyToken> {
        self.research_hotkey_token
    }

    pub fn button_position(&self) -> Option<GridCoordinate> {
        self.button_position
    }

    pub fn research_button_position(&self) -> Option<GridCoordinate> {
        self.research_button_position
    }

    pub fn tip(&self) -> Option<&str> {
        self.tip.as_deref()
    }

    pub fn research_tip(&self) -> Option<&str> {
        self.research_tip.as_deref()
    }

    pub fn ubertip(&self) -> Option<&str> {
        self.ubertip.as_deref()
    }

    pub fn research_ubertip(&self) -> Option<&str> {
        self.research_ubertip.as_deref()
    }

    pub fn alt_display_name(&self) -> Option<&str> {
        self.alt_display_name.as_deref()
    }

    pub fn alt_ubertip(&self) -> Option<&str> {
        self.alt_ubertip.as_deref()
    }

    pub fn name_levels(&self) -> &[String] {
        &self.name_levels
    }

    pub fn icon_levels_len(&self) -> usize {
        self.icon_levels.len()
    }

    pub fn ubertip_levels(&self) -> &[String] {
        &self.ubertip_levels
    }

    pub fn alt_hotkey_token(&self) -> Option<HotkeyToken> {
        self.alt_hotkey_token
    }

    pub fn is_command(&self) -> bool {
        self.is_command
    }

    pub fn is_passive(&self) -> bool {
        self.is_passive
    }

    pub fn info_only(&self) -> bool {
        self.info_only
    }

    pub fn upgrade_unit_id(&self) -> Option<WarcraftObjectId> {
        self.upgrade_unit_id
    }

    pub fn upgrade_display_name(&self) -> Option<&str> {
        self.upgrade_display_name.as_deref()
    }

    pub fn upgrade_hotkey_token(&self) -> Option<HotkeyToken> {
        self.upgrade_hotkey_token
    }

    pub fn is_off_state(&self) -> bool {
        self.is_off_state
    }
}
