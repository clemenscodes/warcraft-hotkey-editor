use ddd::Adapter;
use ddd::InfrastructureLayer;
use ddd::Layered;
use ddd::SynchronousRepository;
use std::convert::Infallible;
use warcraft_keybinds::CustomKeys;
use warcraft_keybinds::CustomKeysIdentity;

use crate::persistence::custom_keys_persistence::CustomKeysPersistence;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CustomKeysRepository;

impl Layered for CustomKeysRepository {
    type Layer = InfrastructureLayer;
}

impl Adapter for CustomKeysRepository {}

impl SynchronousRepository<CustomKeys> for CustomKeysRepository {
    type Failure = Infallible;

    fn find_by_id(&self, _identity: &CustomKeysIdentity) -> Result<Option<CustomKeys>, Infallible> {
        let Some(stored_text) = CustomKeysPersistence::load() else {
            return Ok(None);
        };
        let parsed_keys = CustomKeys::from_text(stored_text.as_str());
        Ok(Some(parsed_keys))
    }

    fn find_many(&self, identities: &[CustomKeysIdentity]) -> Result<Vec<CustomKeys>, Infallible> {
        let mut found_keys = Vec::new();
        for identity in identities {
            let Ok(stored_keys) = self.find_by_id(identity);
            found_keys.extend(stored_keys);
        }
        Ok(found_keys)
    }

    fn save(&self, aggregate: &CustomKeys) -> Result<(), Infallible> {
        let canonical_text = aggregate.to_string();
        CustomKeysPersistence::save(&canonical_text);
        Ok(())
    }

    fn delete(&self, identity: &CustomKeysIdentity) -> Result<Option<CustomKeys>, Infallible> {
        let Ok(stored_keys) = self.find_by_id(identity);
        CustomKeysPersistence::remove();
        Ok(stored_keys)
    }
}
