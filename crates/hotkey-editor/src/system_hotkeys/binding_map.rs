use std::collections::HashMap;

use warcraft_database::WARCRAFT_SYSTEM_KEYBINDS;
use warcraft_keybinds::CustomKeysFile;

use crate::components::system_hotkeys::key_cell::EffectiveBinding;

#[derive(Clone)]
pub(crate) struct ResolvedSystemBinding {
    section_id: String,
    section_comment: String,
    binding: EffectiveBinding,
}

impl ResolvedSystemBinding {
    pub(crate) fn section_comment(&self) -> &str {
        &self.section_comment
    }
}

#[derive(Clone)]
pub(crate) struct SystemBindingMap {
    bindings_by_section: HashMap<String, ResolvedSystemBinding>,
}

impl SystemBindingMap {
    pub(crate) fn build(custom_keys: Option<&CustomKeysFile>) -> Self {
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
            let resolved = ResolvedSystemBinding {
                section_id: section_id.clone(),
                section_comment,
                binding,
            };
            bindings_by_section.insert(section_id, resolved);
        }
        Self {
            bindings_by_section,
        }
    }

    /// All sections — other than `excluded_section_id` — whose effective
    /// binding equals the given `(code, modifier)` pair.
    pub(crate) fn collisions_for(
        &self,
        excluded_section_id: &str,
        code: u32,
        modifier: Option<&str>,
    ) -> Vec<&ResolvedSystemBinding> {
        let mut matches: Vec<&ResolvedSystemBinding> = self
            .bindings_by_section
            .values()
            .filter(|resolved| resolved.section_id != excluded_section_id)
            .filter(|resolved| {
                let other_binding = resolved.binding;
                other_binding.hotkey_code == code && other_binding.modifier == modifier
            })
            .collect();
        matches.sort_by(|left, right| left.section_id.cmp(&right.section_id));
        matches
    }

    /// For the picker dialog: every keycode currently bound by another
    /// section under the given modifier maps to the comments of those sections
    /// (joined for tooltip rendering by the caller).
    pub(crate) fn picker_conflicts(
        &self,
        own_section_id: &str,
        own_modifier: Option<&str>,
    ) -> HashMap<u32, Vec<String>> {
        let mut conflicts: HashMap<u32, Vec<String>> = HashMap::new();
        for resolved in self.bindings_by_section.values() {
            if resolved.section_id == own_section_id {
                continue;
            }
            if resolved.binding.modifier != own_modifier {
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
