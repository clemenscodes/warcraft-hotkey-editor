use warcraft_api::ButtonPosition;
use warcraft_keybinds::CustomKeysFile;

use crate::domain::ability_cell::{AbilityCell, BindingHotkey};
use crate::domain::building_traits::BuildingTraits;
use crate::domain::grid_slot::GridSlotId;
use crate::domain::hotkey_token::HotkeyToken;
use crate::domain::icons::IconUrl;
use crate::domain::object_lookup::ObjectLookup;
use crate::domain::positions::Positions;
use crate::text::color_codes::WarcraftColorCodes;

#[derive(Clone, PartialEq)]
pub(crate) struct InspectorDetail {
    display_name: String,
    object_id: String,
    icon_src: Option<String>,
    hotkey_token: Option<HotkeyToken>,
    research_hotkey_token: Option<HotkeyToken>,
    button_position: Option<ButtonPosition>,
    research_button_position: Option<ButtonPosition>,
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
    /// Off-state button position. Reads `Unbuttonpos` from the binding,
    /// falls through to the SLK default `off_button_position`. Same gate
    /// as `alt_hotkey_token`.
    alt_button_position: Option<ButtonPosition>,
    name_levels: Vec<String>,
    icon_levels: Vec<Option<String>>,
    ubertip_levels: Vec<String>,
    is_command: bool,
    is_passive: bool,
}

impl InspectorDetail {
    pub(crate) fn build(
        slot: &GridSlotId,
        custom_keys: &Option<CustomKeysFile>,
        host_unit_id: &str,
        from_uprooted: bool,
    ) -> Self {
        let custom_keys_ref = custom_keys.as_ref();
        match slot {
            GridSlotId::Ability(ability_id) => {
                let binding = custom_keys_ref.and_then(|file| file.binding(ability_id));
                let cell = AbilityCell::for_ability(ability_id, binding);
                let position = Positions::current_for(slot, custom_keys_ref, false);
                let research_position = custom_keys_ref
                    .and_then(|file| file.binding(ability_id))
                    .and_then(|ability_binding| ability_binding.research_button_position())
                    .map(|raw_position| {
                        ButtonPosition::new(raw_position.column(), raw_position.row())
                    });
                let hotkey_token = binding
                    .and_then(|ability_binding| {
                        ability_binding
                            .hotkey()
                            .or_else(|| ability_binding.research_hotkey())
                    })
                    .and_then(BindingHotkey::first_token);
                let research_hotkey_token = binding
                    .and_then(|ability_binding| ability_binding.research_hotkey())
                    .and_then(BindingHotkey::first_token);
                let tip = binding.and_then(|ability_binding| {
                    ability_binding.tip().map(WarcraftColorCodes::stripped)
                });
                let research_tip = binding.and_then(|ability_binding| {
                    ability_binding
                        .research_tip()
                        .map(WarcraftColorCodes::stripped)
                });
                let database_object = ObjectLookup::by_id(ability_id);
                let is_passive = database_object
                    .and_then(|warcraft_object| warcraft_object.icons().first().copied())
                    .map(|icon_path| {
                        icon_path
                            .to_ascii_lowercase()
                            .starts_with("passivebuttons/")
                    })
                    .unwrap_or(false);
                let object_has_alt_state = database_object
                    .map(|warcraft_object| {
                        warcraft_object.un_ubertip().is_some() || warcraft_object.un_tip().is_some()
                    })
                    .unwrap_or(false);
                let host_starts_in_alt =
                    BuildingTraits::unit_starts_in_toggle_alt_state(host_unit_id);
                let prefer_un_state = !from_uprooted && host_starts_in_alt && object_has_alt_state;
                let primary_ubertip = if prefer_un_state {
                    database_object.and_then(|warcraft_object| warcraft_object.un_ubertip())
                } else {
                    database_object.and_then(|warcraft_object| warcraft_object.ubertip())
                };
                let ubertip = primary_ubertip.map(WarcraftColorCodes::stripped);
                // Surface the *other* state too. When the inspector is already
                // showing the off state (burrowed crypt fiend → "Unburrow"),
                // there's nothing extra to add. When it's showing the on state
                // (a footman's "Defend"), pull `un_tip`/`un_ubertip` so the
                // player can see the "Stop Defend" name and tooltip without
                // having to hunt for the toggle.
                let ability_is_morph = ObjectLookup::morph_target_unit(ability_id).is_some();
                let (alt_display_name, alt_ubertip, alt_hotkey_token, alt_button_position) =
                    if object_has_alt_state && !prefer_un_state && !ability_is_morph {
                        let alt_name = database_object
                            .and_then(|warcraft_object| warcraft_object.un_tip())
                            .map(WarcraftColorCodes::stripped);
                        let alt_long = database_object
                            .and_then(|warcraft_object| warcraft_object.un_ubertip())
                            .map(WarcraftColorCodes::stripped);
                        let alt_hotkey = binding
                            .and_then(|ability_binding| ability_binding.unhotkey())
                            .and_then(BindingHotkey::first_token);
                        let alt_position =
                            Positions::current_for_ability_off(ability_id, custom_keys_ref);
                        (alt_name, alt_long, alt_hotkey, alt_position)
                    } else {
                        (None, None, None, None)
                    };
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
                let icon_levels: Vec<Option<String>> = database_object
                    .map(|warcraft_object| {
                        warcraft_object
                            .icons()
                            .iter()
                            .map(|raw_icon| {
                                let trimmed_icon = raw_icon.trim();
                                if trimmed_icon.is_empty() {
                                    None
                                } else {
                                    Some(IconUrl::from_database_path(trimmed_icon))
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
                let icon_src = cell.cloned_icon_src();
                let object_id = cell.cloned_object_id();
                Self {
                    display_name: resolved_display_name,
                    object_id,
                    icon_src,
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
                    alt_button_position,
                    name_levels,
                    icon_levels,
                    ubertip_levels,
                    is_command: false,
                    is_passive,
                }
            }
            GridSlotId::AbilityOff(ability_id) => {
                // Off-state of a toggle ability — only encountered inside
                // the off-state position picker dialog, where clicking a
                // cell that hosts this slot pops up an inspector preview.
                // Pull `un_tip` / `un_ubertip` for the text and the
                // `unhotkey` from the binding; no research / no level
                // tiering applies to the off state.
                let binding = custom_keys_ref.and_then(|file| file.binding(ability_id));
                let cell = AbilityCell::for_ability_off(ability_id, binding);
                let position = Positions::current_for(slot, custom_keys_ref, false);
                let hotkey_token = binding
                    .and_then(|ability_binding| ability_binding.unhotkey())
                    .and_then(BindingHotkey::first_token);
                let database_object = ObjectLookup::by_id(ability_id);
                let display_name = database_object
                    .and_then(|warcraft_object| warcraft_object.un_tip())
                    .map(WarcraftColorCodes::stripped)
                    .unwrap_or_else(|| cell.display_name().to_string());
                let ubertip = database_object
                    .and_then(|warcraft_object| warcraft_object.un_ubertip())
                    .map(WarcraftColorCodes::stripped);
                let icon_src = cell.cloned_icon_src();
                let object_id = cell.cloned_object_id();
                Self {
                    display_name,
                    object_id,
                    icon_src,
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
                    alt_button_position: None,
                    name_levels: Vec::new(),
                    icon_levels: Vec::new(),
                    ubertip_levels: Vec::new(),
                    is_command: false,
                    is_passive: false,
                }
            }
            GridSlotId::Command(command_name) => {
                let binding = custom_keys_ref.and_then(|file| file.command(command_name));
                let cell = AbilityCell::for_command(command_name, binding);
                let position = Positions::current_for(slot, custom_keys_ref, false);
                let hotkey_token = binding
                    .and_then(|command_binding| command_binding.hotkey())
                    .and_then(BindingHotkey::first_token);
                let database_object = ObjectLookup::by_id(command_name);
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
                let icon_src = cell.cloned_icon_src();
                let display_name = cell.cloned_display_name();
                let object_id = cell.cloned_object_id();
                Self {
                    display_name,
                    object_id,
                    icon_src,
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
                    alt_button_position: None,
                    name_levels: Vec::new(),
                    icon_levels: Vec::new(),
                    ubertip_levels: Vec::new(),
                    is_command: true,
                    is_passive: false,
                }
            }
        }
    }

    pub(crate) fn display_name(&self) -> &str {
        &self.display_name
    }

    pub(crate) fn object_id(&self) -> &str {
        &self.object_id
    }

    pub(crate) fn hotkey_token(&self) -> Option<HotkeyToken> {
        self.hotkey_token
    }

    pub(crate) fn research_hotkey_token(&self) -> Option<HotkeyToken> {
        self.research_hotkey_token
    }

    pub(crate) fn button_position(&self) -> Option<ButtonPosition> {
        self.button_position
    }

    pub(crate) fn research_button_position(&self) -> Option<ButtonPosition> {
        self.research_button_position
    }

    pub(crate) fn tip(&self) -> Option<&str> {
        self.tip.as_deref()
    }

    pub(crate) fn research_tip(&self) -> Option<&str> {
        self.research_tip.as_deref()
    }

    pub(crate) fn ubertip(&self) -> Option<&str> {
        self.ubertip.as_deref()
    }

    pub(crate) fn research_ubertip(&self) -> Option<&str> {
        self.research_ubertip.as_deref()
    }

    pub(crate) fn alt_display_name(&self) -> Option<&str> {
        self.alt_display_name.as_deref()
    }

    pub(crate) fn alt_ubertip(&self) -> Option<&str> {
        self.alt_ubertip.as_deref()
    }

    pub(crate) fn name_levels(&self) -> &[String] {
        &self.name_levels
    }

    pub(crate) fn icon_levels_len(&self) -> usize {
        self.icon_levels.len()
    }

    pub(crate) fn ubertip_levels(&self) -> &[String] {
        &self.ubertip_levels
    }

    pub(crate) fn alt_hotkey_token(&self) -> Option<HotkeyToken> {
        self.alt_hotkey_token
    }

    /// Off-state button position, surfaced for the picker dialog. The
    /// override card itself no longer exposes the coordinate as text;
    /// kept on `InspectorDetail` so future surfaces (status line,
    /// tooltip preview) can pull it without re-querying.
    #[allow(dead_code)]
    pub(crate) fn alt_button_position(&self) -> Option<ButtonPosition> {
        self.alt_button_position
    }

    pub(crate) fn is_command(&self) -> bool {
        self.is_command
    }

    pub(crate) fn is_passive(&self) -> bool {
        self.is_passive
    }
}
