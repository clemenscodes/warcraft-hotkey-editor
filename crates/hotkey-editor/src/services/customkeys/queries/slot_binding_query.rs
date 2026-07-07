use ddd::ApplicationLayer;
use ddd::Layered;
use ddd::Query;
use std::collections::HashMap;
use warcraft_keybinds::CustomKeys;
use warcraft_keybinds::KeyCode;
use warcraft_keybinds::SystemBindingMap;
use warcraft_keybinds::WarcraftObjectId;

/// The resolved binding and conflict picture for one system keybind section:
/// what key it currently shows, whether that key collides with another binding,
/// which sections it collides with, and the per-key conflict names a picker
/// needs. Everything a system-hotkey slot renders, already decided by the domain.
#[derive(Clone, PartialEq, Debug)]
pub struct SlotBindingView {
    effective_label: String,
    current_code: KeyCode,
    is_conflict: bool,
    colliding_names: Vec<String>,
    picker_conflicts: HashMap<KeyCode, Vec<String>>,
}

impl SlotBindingView {
    /// The view for a section that resolves to nothing (an unknown id): no label,
    /// no conflicts. `KeyCode::Escape` mirrors the domain's own resolve fallback.
    fn empty() -> Self {
        Self {
            effective_label: String::new(),
            current_code: KeyCode::Escape,
            is_conflict: false,
            colliding_names: Vec::new(),
            picker_conflicts: HashMap::new(),
        }
    }

    /// The idle key label (e.g. `Ctrl+Q`), as the domain formats it.
    pub(crate) fn effective_label(&self) -> &str {
        &self.effective_label
    }

    pub(crate) fn current_code(&self) -> KeyCode {
        self.current_code
    }

    pub(crate) fn is_conflict(&self) -> bool {
        self.is_conflict
    }

    /// The section comments of every binding this slot collides with; presentation
    /// composes the "Also used by …" copy from them.
    pub(crate) fn colliding_names(&self) -> &[String] {
        &self.colliding_names
    }

    pub(crate) fn picker_conflicts(&self) -> &HashMap<KeyCode, Vec<String>> {
        &self.picker_conflicts
    }
}

/// Query: the [`SlotBindingView`] for one system keybind section, addressed by its
/// `WarcraftObjectId`. Answered against the live `CustomKeys` aggregate so the
/// renderer never builds the binding map or resolves collisions itself.
pub struct SlotBindingQuery {
    section_id: WarcraftObjectId,
}

impl SlotBindingQuery {
    pub fn new(section_id: WarcraftObjectId) -> Self {
        Self { section_id }
    }

    /// Answer the query against a keys snapshot. Pure — the service's
    /// [`ddd::QueryHandler`] impl wraps this with the reactive signal read.
    pub fn answer(&self, custom_keys: Option<&CustomKeys>) -> SlotBindingView {
        let map = SystemBindingMap::build(custom_keys);
        let Some(resolved) = map.binding_for(self.section_id) else {
            return SlotBindingView::empty();
        };
        let effective = resolved.effective();
        let current_code = effective.key();
        let modifier = effective.modifier();
        let collisions = map.collisions_for(self.section_id, current_code, modifier);
        let is_conflict = !collisions.is_empty();
        let colliding_names: Vec<String> = collisions
            .iter()
            .map(|colliding| colliding.section_comment().to_string())
            .collect();
        let picker_conflicts = map.picker_conflicts(self.section_id, modifier);
        let effective_label = effective.label();
        SlotBindingView {
            effective_label,
            current_code,
            is_conflict,
            colliding_names,
            picker_conflicts,
        }
    }
}

impl Layered for SlotBindingQuery {
    type Layer = ApplicationLayer;
}

impl Query for SlotBindingQuery {
    type Output = SlotBindingView;
}

#[cfg(test)]
mod tests {
    use super::SlotBindingQuery;
    use crate::services::customkeys::queries::assert_query;
    use warcraft_keybinds::SystemBindingMap;

    #[test]
    fn slot_binding_is_a_query() {
        assert_query::<SlotBindingQuery>();
    }

    #[test]
    fn answers_the_default_binding_for_a_known_section() {
        let section_id = SystemBindingMap::resolve_section("itm3")
            .expect("itm3 is a known system keybind section");
        let query = SlotBindingQuery::new(section_id);
        let view = query.answer(None);
        // A known section resolves to its database default, so the label is
        // always populated (whether or not that default happens to collide).
        assert!(!view.effective_label().is_empty());
    }
}
