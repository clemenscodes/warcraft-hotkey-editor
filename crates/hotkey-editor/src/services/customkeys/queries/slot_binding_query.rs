use ddd::ApplicationLayer;
use ddd::Layered;
use ddd::Query;
use std::collections::HashMap;
use warcraft_keybinds::CustomKeys;
use warcraft_keybinds::KeyCode;
use warcraft_keybinds::SystemBindingMap;
use warcraft_keybinds::WarcraftObjectId;

#[derive(Clone, PartialEq, Debug)]
pub struct SlotBindingView {
    effective_label: String,
    current_code: KeyCode,
    is_conflict: bool,
    colliding_names: Vec<String>,
    picker_conflicts: HashMap<KeyCode, Vec<String>>,
}

impl SlotBindingView {
    fn empty() -> Self {
        Self {
            effective_label: String::new(),
            current_code: KeyCode::Escape,
            is_conflict: false,
            colliding_names: Vec::new(),
            picker_conflicts: HashMap::new(),
        }
    }

    pub(crate) fn effective_label(&self) -> &str {
        &self.effective_label
    }

    pub(crate) fn current_code(&self) -> KeyCode {
        self.current_code
    }

    pub(crate) fn is_conflict(&self) -> bool {
        self.is_conflict
    }

    pub(crate) fn colliding_names(&self) -> &[String] {
        &self.colliding_names
    }

    pub(crate) fn picker_conflicts(&self) -> &HashMap<KeyCode, Vec<String>> {
        &self.picker_conflicts
    }
}

pub struct SlotBindingQuery {
    section_id: WarcraftObjectId,
}

impl SlotBindingQuery {
    pub fn new(section_id: WarcraftObjectId) -> Self {
        Self { section_id }
    }

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
        assert!(!view.effective_label().is_empty());
    }
}
