use ddd::Adapter;
use ddd::InfrastructureLayer;
use ddd::Layered;
use ddd::SynchronousRepository;
use std::convert::Infallible;
use warcraft_keybinds::GridLayout;
use warcraft_keybinds::GridLayoutIdentity;

use crate::persistence::grid_layout_persistence::GridLayoutPersistence;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GridLayoutRepository;

impl Layered for GridLayoutRepository {
    type Layer = InfrastructureLayer;
}

impl Adapter for GridLayoutRepository {}

impl SynchronousRepository<GridLayout> for GridLayoutRepository {
    type Failure = Infallible;

    fn find_by_id(&self, _identity: &GridLayoutIdentity) -> Result<Option<GridLayout>, Infallible> {
        let stored_layout = GridLayoutPersistence::load();
        Ok(stored_layout)
    }

    fn find_many(&self, identities: &[GridLayoutIdentity]) -> Result<Vec<GridLayout>, Infallible> {
        let mut found_layouts = Vec::new();
        for identity in identities {
            let Ok(stored_layout) = self.find_by_id(identity);
            found_layouts.extend(stored_layout);
        }
        Ok(found_layouts)
    }

    fn save(&self, aggregate: &GridLayout) -> Result<(), Infallible> {
        let layout = *aggregate;
        GridLayoutPersistence::save(layout);
        Ok(())
    }

    fn delete(&self, identity: &GridLayoutIdentity) -> Result<Option<GridLayout>, Infallible> {
        let Ok(stored_layout) = self.find_by_id(identity);
        GridLayoutPersistence::remove();
        Ok(stored_layout)
    }
}
