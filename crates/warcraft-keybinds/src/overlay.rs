use crate::CustomKeysFile;

impl CustomKeysFile {
    pub fn overlay(&mut self, source: &CustomKeysFile) {
        let uploaded_binding_ids: Vec<String> = source
            .bindings_in_order()
            .map(|entry| entry.id().to_string())
            .collect();
        for binding_id in uploaded_binding_ids {
            let Some(uploaded_binding) = source.binding(&binding_id) else {
                continue;
            };
            if self.system(&binding_id).is_some() {
                continue;
            }
            let Some(target_binding) = self.binding_or_default_mut(&binding_id) else {
                continue;
            };
            if let Some(hotkey) = uploaded_binding.hotkey() {
                let hotkey_clone = hotkey.clone();
                target_binding.set_hotkey(Some(hotkey_clone));
            }
            if let Some(button_position) = uploaded_binding.button_position().copied() {
                target_binding.set_button_position(Some(button_position));
            }
            if let Some(unbutton_position) = uploaded_binding.unbutton_position().copied() {
                target_binding.set_unbutton_position(Some(unbutton_position));
            }
            if let Some(hotkey) = uploaded_binding.research_hotkey() {
                let hotkey_clone = hotkey.clone();
                target_binding.set_research_hotkey(Some(hotkey_clone));
            }
            if let Some(research_button_position) =
                uploaded_binding.research_button_position().copied()
            {
                target_binding.set_research_button_position(Some(research_button_position));
            }
            if let Some(tip_str) = uploaded_binding.tip() {
                let tip_string = tip_str.to_string();
                target_binding.set_tip(Some(tip_string));
            }
            if let Some(tip_str) = uploaded_binding.research_tip() {
                let tip_string = tip_str.to_string();
                target_binding.set_research_tip(Some(tip_string));
            }
            if let Some(tip_str) = uploaded_binding.un_tip() {
                let tip_string = tip_str.to_string();
                target_binding.set_un_tip(Some(tip_string));
            }
            if let Some(icon_str) = uploaded_binding.icon() {
                let icon_string = icon_str.to_string();
                target_binding.set_icon(Some(icon_string));
            }
        }

        let uploaded_command_names: Vec<String> = source
            .commands_in_order()
            .map(|entry| entry.name().to_string())
            .collect();
        for command_name in uploaded_command_names {
            let Some(uploaded_command) = source.command(&command_name) else {
                continue;
            };
            let Some(target_command) = self.command_or_default_mut(&command_name) else {
                continue;
            };
            if let Some(hotkey) = uploaded_command.hotkey() {
                let hotkey_clone = hotkey.clone();
                target_command.set_hotkey(Some(hotkey_clone));
            }
            if let Some(button_position) = uploaded_command.button_position().copied() {
                target_command.set_button_position(Some(button_position));
            }
            if let Some(unbutton_position) = uploaded_command.unbutton_position().copied() {
                target_command.set_unbutton_position(Some(unbutton_position));
            }
            if let Some(tip_str) = uploaded_command.tip() {
                let tip_string = tip_str.to_string();
                target_command.set_tip(Some(tip_string));
            }
            if let Some(tip_str) = uploaded_command.un_tip() {
                let tip_string = tip_str.to_string();
                target_command.set_un_tip(Some(tip_string));
            }
        }
    }
}
