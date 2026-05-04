use warcraft_keybinds::CustomKeysFile;

pub(crate) struct UploadOverlay;

impl UploadOverlay {
    pub(crate) fn apply(target_file: &mut CustomKeysFile, uploaded_file: &CustomKeysFile) {
        let uploaded_binding_ids: Vec<String> = uploaded_file
            .bindings_in_order()
            .map(|entry| entry.id().to_string())
            .collect();
        for binding_id in uploaded_binding_ids {
            let Some(uploaded_binding) = uploaded_file.binding(&binding_id) else {
                continue;
            };
            // System entries (inventory slots, hero selection, …) must never be
            // overwritten by an ability overlay — they live in their own section.
            if target_file.system(&binding_id).is_some() {
                continue;
            }
            let Some(target_binding) = target_file.binding_or_default_mut(&binding_id) else {
                continue;
            };
            if let Some(value) = uploaded_binding.hotkey() {
                target_binding.set_hotkey(Some(value.to_string()));
            }
            if let Some(value) = uploaded_binding.button_position() {
                let new_position =
                    warcraft_keybinds::ButtonPosition::new(value.column(), value.row());
                target_binding.set_button_position(Some(new_position));
            }
            if let Some(value) = uploaded_binding.unbutton_position() {
                let new_position =
                    warcraft_keybinds::ButtonPosition::new(value.column(), value.row());
                target_binding.set_unbutton_position(Some(new_position));
            }
            if let Some(value) = uploaded_binding.research_hotkey() {
                target_binding.set_research_hotkey(Some(value.to_string()));
            }
            if let Some(value) = uploaded_binding.research_button_position() {
                let new_position =
                    warcraft_keybinds::ButtonPosition::new(value.column(), value.row());
                target_binding.set_research_button_position(Some(new_position));
            }
            if let Some(value) = uploaded_binding.tip() {
                target_binding.set_tip(Some(value.to_string()));
            }
            if let Some(value) = uploaded_binding.research_tip() {
                target_binding.set_research_tip(Some(value.to_string()));
            }
            if let Some(value) = uploaded_binding.un_tip() {
                target_binding.set_un_tip(Some(value.to_string()));
            }
            if let Some(value) = uploaded_binding.icon() {
                target_binding.set_icon(Some(value.to_string()));
            }
        }
        let uploaded_command_names: Vec<String> = uploaded_file
            .commands_in_order()
            .map(|entry| entry.name().to_string())
            .collect();
        for command_name in uploaded_command_names {
            let Some(uploaded_command) = uploaded_file.command(&command_name) else {
                continue;
            };
            let Some(target_command) = target_file.command_or_default_mut(&command_name) else {
                continue;
            };
            if let Some(value) = uploaded_command.hotkey() {
                target_command.set_hotkey(Some(value.to_string()));
            }
            if let Some(value) = uploaded_command.button_position() {
                let new_position =
                    warcraft_keybinds::ButtonPosition::new(value.column(), value.row());
                target_command.set_button_position(Some(new_position));
            }
            if let Some(value) = uploaded_command.unbutton_position() {
                let new_position =
                    warcraft_keybinds::ButtonPosition::new(value.column(), value.row());
                target_command.set_unbutton_position(Some(new_position));
            }
            if let Some(value) = uploaded_command.tip() {
                target_command.set_tip(Some(value.to_string()));
            }
            if let Some(value) = uploaded_command.un_tip() {
                target_command.set_un_tip(Some(value.to_string()));
            }
        }
    }
}
