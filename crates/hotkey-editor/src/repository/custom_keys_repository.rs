use ddd::Adapter;
use ddd::InfrastructureLayer;
use ddd::Layered;
use ddd::Repository;
use warcraft_keybinds::CustomKeys;

use crate::persistence::custom_keys_persistence;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CustomKeysRepository;

impl Layered for CustomKeysRepository {
    type Layer = InfrastructureLayer;
}

impl Adapter for CustomKeysRepository {}

impl Repository<CustomKeys> for CustomKeysRepository {
    fn load(&self) -> Option<CustomKeys> {
        let stored_text = custom_keys_persistence::load_text()?;
        let parsed_keys = CustomKeys::from_text(stored_text.as_str());
        Some(parsed_keys)
    }

    fn save(&self, aggregate: &CustomKeys) {
        let canonical_text = aggregate.to_string();
        custom_keys_persistence::save_text(&canonical_text);
    }
}
