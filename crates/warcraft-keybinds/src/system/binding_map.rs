use std::collections::HashMap;

use warcraft_api::{ContextSet, KeyCode, SystemKeybindModifier};
use warcraft_database::WARCRAFT_SYSTEM_KEYBINDS;

use crate::{CustomKeys, Hotkey};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct EffectiveBinding {
    hotkey_code: u32,
    modifier: SystemKeybindModifier,
}

impl EffectiveBinding {
    pub fn resolve_from_file(
        custom_keys: Option<&CustomKeys>,
        section_id: &str,
        default_hotkey: u32,
        default_modifier: SystemKeybindModifier,
    ) -> Self {
        let custom_hotkey = custom_keys
            .and_then(|file| file.system(section_id))
            .and_then(|binding| match binding.hotkey() {
                Hotkey::VirtualKey(code) => Some(*code),
                _ => None,
            });
        let hotkey_code = custom_hotkey.unwrap_or(default_hotkey);
        // Warcraft III hardcodes the modifier per system hotkey — any
        // `Modifier=` line in CustomKeys.txt is written for transparency but
        // discarded at load time. The editor mirrors that: the effective
        // modifier is always the system default, regardless of the file.
        Self {
            hotkey_code,
            modifier: default_modifier,
        }
    }

    pub fn hotkey_code(&self) -> u32 {
        self.hotkey_code
    }

    pub fn modifier(&self) -> SystemKeybindModifier {
        self.modifier
    }

    pub fn label(&self) -> String {
        let code = KeyCode::from(self.hotkey_code);
        format!("{}{code}", self.modifier)
    }
}

#[derive(Clone, PartialEq)]
pub struct ResolvedSystemBinding {
    section_id: String,
    section_comment: String,
    binding: EffectiveBinding,
    context_set: ContextSet,
}

impl ResolvedSystemBinding {
    pub fn section_comment(&self) -> &str {
        &self.section_comment
    }
}

#[derive(Clone, PartialEq)]
pub struct SystemBindingMap {
    bindings_by_section: HashMap<String, ResolvedSystemBinding>,
}

impl SystemBindingMap {
    pub fn build(custom_keys: Option<&CustomKeys>) -> Self {
        let mut bindings_by_section: HashMap<String, ResolvedSystemBinding> =
            HashMap::with_capacity(WARCRAFT_SYSTEM_KEYBINDS.len());
        for entry in WARCRAFT_SYSTEM_KEYBINDS.iter() {
            let section_id = entry.section_id().to_string();
            let section_comment = entry.comment().to_string();
            let binding = EffectiveBinding::resolve_from_file(
                custom_keys,
                &section_id,
                entry.default_hotkey(),
                entry.default_modifier(),
            );
            let context_set = entry.context_set();
            let resolved = ResolvedSystemBinding {
                section_id: section_id.clone(),
                section_comment,
                binding,
                context_set,
            };
            bindings_by_section.insert(section_id, resolved);
        }
        Self {
            bindings_by_section,
        }
    }

    pub fn collisions_for(
        &self,
        excluded_section_id: &str,
        code: u32,
        modifier: SystemKeybindModifier,
    ) -> Vec<&ResolvedSystemBinding> {
        let own_context = self
            .bindings_by_section
            .get(excluded_section_id)
            .map(|resolved| resolved.context_set)
            .unwrap_or(ContextSet::ALWAYS);
        let mut matches: Vec<&ResolvedSystemBinding> = self
            .bindings_by_section
            .values()
            .filter(|resolved| resolved.section_id != excluded_section_id)
            .filter(|resolved| {
                resolved.binding.hotkey_code == code && resolved.binding.modifier == modifier
            })
            .filter(|resolved| own_context.overlaps(resolved.context_set))
            .collect();
        matches.sort_by(|left, right| left.section_id.cmp(&right.section_id));
        matches
    }

    pub fn picker_conflicts(
        &self,
        own_section_id: &str,
        own_modifier: SystemKeybindModifier,
    ) -> HashMap<u32, Vec<String>> {
        let own_context = self
            .bindings_by_section
            .get(own_section_id)
            .map(|resolved| resolved.context_set)
            .unwrap_or(ContextSet::ALWAYS);
        let mut conflicts: HashMap<u32, Vec<String>> = HashMap::new();
        for resolved in self.bindings_by_section.values() {
            if resolved.section_id == own_section_id {
                continue;
            }
            if resolved.binding.modifier != own_modifier {
                continue;
            }
            if !own_context.overlaps(resolved.context_set) {
                continue;
            }
            let code = resolved.binding.hotkey_code;
            let names = conflicts.entry(code).or_default();
            names.push(resolved.section_comment.clone());
        }
        for names in conflicts.values_mut() {
            names.sort();
        }
        conflicts
    }
}
