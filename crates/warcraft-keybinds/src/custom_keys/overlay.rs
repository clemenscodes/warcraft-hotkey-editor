//! The import-overlay merge: `Extend` layers an imported CustomKeys.txt onto
//! the bundled baseline field-by-field before normalization (the domain home of
//! the "import replaces, then normalize" rule, R7).

use super::CustomKeys;
use crate::model::WarcraftKeybinding;
use warcraft_api::WarcraftObjectId;

impl Extend<(WarcraftObjectId, WarcraftKeybinding)> for CustomKeys {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = (WarcraftObjectId, WarcraftKeybinding)>,
    {
        for (object_id, binding) in iter {
            let raw_key = object_id.value();
            match binding {
                WarcraftKeybinding::Ability(source_binding) => {
                    if self.system(raw_key).is_some() {
                        continue;
                    }
                    let Some(target_binding) = self.binding_or_default_mut(object_id) else {
                        continue;
                    };
                    if let Some(hotkey) = source_binding.hotkey() {
                        let hotkey_clone = *hotkey;
                        target_binding.set_hotkey(Some(hotkey_clone));
                    }
                    if let Some(hotkey) = source_binding.unhotkey() {
                        let hotkey_clone = *hotkey;
                        target_binding.set_unhotkey(Some(hotkey_clone));
                    }
                    if let Some(position) = source_binding.button_position().copied() {
                        target_binding.set_button_position(Some(position));
                    }
                    if let Some(position) = source_binding.unbutton_position().copied() {
                        target_binding.set_unbutton_position(Some(position));
                    }
                    if let Some(hotkey) = source_binding.research_hotkey() {
                        let hotkey_clone = *hotkey;
                        target_binding.set_research_hotkey(Some(hotkey_clone));
                    }
                    if let Some(position) = source_binding.research_button_position().copied() {
                        target_binding.set_research_button_position(Some(position));
                    }
                    if let Some(tip) = source_binding.tip() {
                        let tip_string = tip.to_string();
                        target_binding.set_tip(Some(tip_string));
                    }
                    if let Some(tip) = source_binding.research_tip() {
                        let tip_string = tip.to_string();
                        target_binding.set_research_tip(Some(tip_string));
                    }
                    if let Some(tip) = source_binding.un_tip() {
                        let tip_string = tip.to_string();
                        target_binding.set_un_tip(Some(tip_string));
                    }
                    if let Some(icon) = source_binding.icon() {
                        let icon_string = icon.to_string();
                        target_binding.set_icon(Some(icon_string));
                    }
                }
                WarcraftKeybinding::Command(source_binding) => {
                    let Some(target_binding) = self.command_or_default_mut(object_id) else {
                        continue;
                    };
                    if let Some(hotkey) = source_binding.hotkey() {
                        let hotkey_clone = *hotkey;
                        target_binding.set_hotkey(Some(hotkey_clone));
                    }
                    if let Some(position) = source_binding.button_position().copied() {
                        target_binding.set_button_position(Some(position));
                    }
                    if let Some(position) = source_binding.unbutton_position().copied() {
                        target_binding.set_unbutton_position(Some(position));
                    }
                    if let Some(tip) = source_binding.tip() {
                        let tip_string = tip.to_string();
                        target_binding.set_tip(Some(tip_string));
                    }
                    if let Some(tip) = source_binding.un_tip() {
                        let tip_string = tip.to_string();
                        target_binding.set_un_tip(Some(tip_string));
                    }
                }
                WarcraftKeybinding::System(source_binding) => {
                    self.put_system(object_id, source_binding);
                }
            }
        }
    }
}
