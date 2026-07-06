use crate::{CustomKeys, Hotkey, KeyCode};
use std::collections::HashMap;
use warcraft_api::{ContextSet, SystemKeybindModifier, WarcraftObjectId};
use warcraft_database::WARCRAFT_SYSTEM_KEYBINDS;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EffectiveBinding {
    key: KeyCode,
    modifier: SystemKeybindModifier,
}

impl EffectiveBinding {
    pub fn resolve_from_file(
        custom_keys: Option<&CustomKeys>,
        section_id: WarcraftObjectId,
        default_hotkey: u32,
        default_modifier: SystemKeybindModifier,
    ) -> Self {
        let section_key = section_id.value();
        let custom_key = custom_keys
            .and_then(|file| file.system(section_key))
            .and_then(|binding| match binding.hotkey() {
                Hotkey::VirtualKey(code) => KeyCode::try_from(*code).ok(),
                _ => None,
            });
        let fallback_key = KeyCode::Escape;
        let default_key = KeyCode::try_from(default_hotkey).unwrap_or(fallback_key);
        let key = custom_key.unwrap_or(default_key);
        Self {
            key,
            modifier: default_modifier,
        }
    }

    pub fn key(&self) -> KeyCode {
        self.key
    }

    pub fn modifier(&self) -> SystemKeybindModifier {
        self.modifier
    }

    pub fn label(&self) -> String {
        format!("{}{}", self.modifier, self.key)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ResolvedSystemBinding {
    section_id: WarcraftObjectId,
    section_comment: String,
    binding: EffectiveBinding,
    context_set: ContextSet,
}

impl ResolvedSystemBinding {
    pub fn section_id(&self) -> WarcraftObjectId {
        self.section_id
    }

    pub fn section_comment(&self) -> &str {
        &self.section_comment
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct SystemBindingMap {
    bindings_by_section: HashMap<WarcraftObjectId, ResolvedSystemBinding>,
}

impl SystemBindingMap {
    pub fn build(custom_keys: Option<&CustomKeys>) -> Self {
        let mut bindings_by_section: HashMap<WarcraftObjectId, ResolvedSystemBinding> =
            HashMap::with_capacity(WARCRAFT_SYSTEM_KEYBINDS.len());
        for entry in WARCRAFT_SYSTEM_KEYBINDS.iter() {
            let section_id = WarcraftObjectId::from(entry.section_id());
            let section_comment = entry.comment().to_string();
            let binding = EffectiveBinding::resolve_from_file(
                custom_keys,
                section_id,
                entry.default_hotkey(),
                entry.default_modifier(),
            );
            let context_set = entry.context_set();
            let resolved = ResolvedSystemBinding {
                section_id,
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
        excluded_section_id: WarcraftObjectId,
        key: KeyCode,
        modifier: SystemKeybindModifier,
    ) -> Vec<&ResolvedSystemBinding> {
        let own_context = self
            .bindings_by_section
            .get(&excluded_section_id)
            .map(|resolved| resolved.context_set)
            .unwrap_or(ContextSet::ALWAYS);
        let mut matches: Vec<&ResolvedSystemBinding> = self
            .bindings_by_section
            .values()
            .filter(|resolved| resolved.section_id != excluded_section_id)
            .filter(|resolved| resolved.binding.key == key && resolved.binding.modifier == modifier)
            .filter(|resolved| own_context.overlaps(resolved.context_set))
            .collect();
        matches.sort_by_key(|resolved| resolved.section_id);
        matches
    }

    pub fn picker_conflicts(
        &self,
        own_section_id: WarcraftObjectId,
        own_modifier: SystemKeybindModifier,
    ) -> HashMap<KeyCode, Vec<String>> {
        let own_context = self
            .bindings_by_section
            .get(&own_section_id)
            .map(|resolved| resolved.context_set)
            .unwrap_or(ContextSet::ALWAYS);
        let mut conflicts: HashMap<KeyCode, Vec<String>> = HashMap::new();
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
            let key = resolved.binding.key;
            let names = conflicts.entry(key).or_default();
            names.push(resolved.section_comment.clone());
        }
        for names in conflicts.values_mut() {
            names.sort();
        }
        conflicts
    }
}
