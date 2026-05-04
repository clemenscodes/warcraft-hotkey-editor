use warcraft_api::{WarcraftObjectKind, WarcraftObjectMeta};
use warcraft_database::WARCRAFT_DATABASE;
use warcraft_keybinds::CustomKeysFile;

use crate::customkeys::baseline::BASELINE_CUSTOM_KEYS;
use crate::customkeys::upload_overlay::UploadOverlay;

pub(crate) struct ExplicitExport;

impl ExplicitExport {
    pub(crate) fn serialize(loaded_file: &CustomKeysFile) -> String {
        let mut export_file = CustomKeysFile::from(BASELINE_CUSTOM_KEYS);
        UploadOverlay::apply(&mut export_file, loaded_file);
        Self::materialize_default_positions(&mut export_file);
        export_file.to_file_content()
    }

    fn materialize_default_positions(file: &mut CustomKeysFile) {
        for (object_id, warcraft_object) in WARCRAFT_DATABASE.iter() {
            let id_value = object_id.value();
            let default_button = warcraft_object.default_button_position().map(|position| {
                warcraft_keybinds::ButtonPosition::new(position.column(), position.row())
            });
            let default_research =
                warcraft_object
                    .default_research_button_position()
                    .map(|position| {
                        warcraft_keybinds::ButtonPosition::new(position.column(), position.row())
                    });

            match warcraft_object.kind() {
                WarcraftObjectKind::Command => continue,
                WarcraftObjectKind::Ability => {
                    if default_button.is_none() && default_research.is_none() {
                        continue;
                    }
                    let binding = file.binding_or_default_mut(id_value);
                    if binding.button_position().is_none()
                        && let Some(position_value) = default_button
                    {
                        binding.set_button_position(Some(position_value));
                    }
                    if binding.research_button_position().is_none()
                        && let Some(position_value) = default_research
                    {
                        binding.set_research_button_position(Some(position_value));
                    }
                    // Off-state position priority: SLK default
                    // (`AbilityMeta::off_button_position`, parsed from
                    // `Unbuttonpos` in `abilityfunc.txt`) wins over mirroring
                    // the on-state position. The mirror is the legacy
                    // fallback for non-passive abilities that have no
                    // explicit off position in the data — preserves the
                    // game-default placement so existing exports keep
                    // working.
                    if binding.unbutton_position().is_none()
                        && !Self::is_passive_icon_set(warcraft_object.icons())
                    {
                        let database_off = match warcraft_object.meta() {
                            WarcraftObjectMeta::Ability(ability_meta) => {
                                ability_meta.off_button_position().map(|position| {
                                    warcraft_keybinds::ButtonPosition::new(
                                        position.column(),
                                        position.row(),
                                    )
                                })
                            }
                            _ => None,
                        };
                        if let Some(off_position) = database_off {
                            binding.set_unbutton_position(Some(off_position));
                        } else if let Some(button_position) = binding.button_position() {
                            binding.set_unbutton_position(Some(*button_position));
                        }
                    }
                }
                _ => continue,
            }
        }
    }

    fn is_passive_icon_set(icons: &[&str]) -> bool {
        icons
            .first()
            .map(|icon_path| {
                icon_path
                    .trim()
                    .to_ascii_lowercase()
                    .starts_with("passivebuttons/")
            })
            .unwrap_or(false)
    }
}
