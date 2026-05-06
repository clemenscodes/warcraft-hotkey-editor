use warcraft_api::{WarcraftObjectKind, WarcraftObjectMeta};
use warcraft_database::WARCRAFT_DATABASE;

use crate::{ButtonPosition, CustomKeysFile};

/// Serialize a user's loaded file into a complete CustomKeys.txt, blending
/// it over `baseline` (the stock default hotkeys file content) and
/// materializing missing button positions from the game database.
pub fn serialize(loaded_file: &CustomKeysFile, baseline: &str) -> String {
    let mut export_file = CustomKeysFile::from(baseline);
    export_file.overlay(loaded_file);
    export_file.materialize_default_positions();
    crate::cascade::fully_normalize(&mut export_file);
    export_file.to_file_content()
}

impl CustomKeysFile {
    fn materialize_default_positions(&mut self) {
        for (object_id, warcraft_object) in WARCRAFT_DATABASE.iter() {
            let id_value = object_id.value();
            let default_button = warcraft_object
                .default_button_position()
                .map(|position| ButtonPosition::new(position.column(), position.row()));
            let default_research = warcraft_object
                .default_research_button_position()
                .map(|position| ButtonPosition::new(position.column(), position.row()));

            match warcraft_object.kind() {
                WarcraftObjectKind::Command => continue,
                WarcraftObjectKind::Ability => {
                    if default_button.is_none() && default_research.is_none() {
                        continue;
                    }
                    let Some(binding) = self.binding_or_default_mut(id_value) else {
                        continue;
                    };
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
                    if binding.unbutton_position().is_none()
                        && !Self::is_passive_icon_set(warcraft_object.icons())
                    {
                        let database_off = match warcraft_object.meta() {
                            WarcraftObjectMeta::Ability(ability_meta) => {
                                ability_meta.off_button_position().map(|position| {
                                    ButtonPosition::new(position.column(), position.row())
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
