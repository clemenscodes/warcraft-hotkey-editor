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

    /// The effective key + modifier this section resolves to, materialized when
    /// the map was built (custom override if present, else the database default).
    pub fn effective(&self) -> EffectiveBinding {
        self.binding
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

    /// The resolved binding for one section, or `None` when the string names no
    /// known system keybind section. Reads the entry the build already
    /// materialized — the query layer uses this instead of re-resolving the
    /// effective binding (and re-supplying the database defaults) at render time.
    pub fn binding_for(&self, section_id: WarcraftObjectId) -> Option<&ResolvedSystemBinding> {
        self.bindings_by_section.get(&section_id)
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

    /// Resolve a raw section-id string — for example one read back from a DOM
    /// `data-*` attribute during an inventory drag — to its canonical
    /// [`WarcraftObjectId`]. Returns `None` when the string names no known system
    /// keybind section, so a stray drop target is simply ignored. This is the
    /// domain's answer to "which known section is this string?"; the renderer must
    /// never invent a `WarcraftObjectId` of its own.
    pub fn resolve_section(section_key: &str) -> Option<WarcraftObjectId> {
        let matching_entry = WARCRAFT_SYSTEM_KEYBINDS
            .iter()
            .find(|entry| entry.section_id() == section_key)?;
        let canonical_key = matching_entry.section_id();
        let section_id = WarcraftObjectId::from(canonical_key);
        Some(section_id)
    }
}

#[cfg(test)]
mod tests {
    use super::SystemBindingMap;

    #[test]
    fn resolves_a_known_inventory_section() {
        let resolved = SystemBindingMap::resolve_section("itm3");
        let section_id = resolved.expect("itm3 is a known system keybind section");
        assert_eq!(section_id.value(), "itm3");
    }

    #[test]
    fn rejects_an_unknown_section() {
        let resolved = SystemBindingMap::resolve_section("not-a-real-section");
        assert!(resolved.is_none());
    }

    #[test]
    fn exposes_the_materialized_binding_for_a_known_section() {
        let section_id = SystemBindingMap::resolve_section("itm3")
            .expect("itm3 is a known system keybind section");
        let map = SystemBindingMap::build(None);
        let resolved = map
            .binding_for(section_id)
            .expect("a built map holds an entry for every known section");
        assert_eq!(resolved.section_id(), section_id);
        let effective = resolved.effective();
        assert!(!effective.label().is_empty());
    }

    #[test]
    fn has_no_binding_for_an_unknown_section() {
        let stray_id = warcraft_api::WarcraftObjectId::from("not-a-real-section");
        let map = SystemBindingMap::build(None);
        assert!(map.binding_for(stray_id).is_none());
    }
}
