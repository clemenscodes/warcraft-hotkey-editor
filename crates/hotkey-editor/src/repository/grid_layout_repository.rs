use ddd::Adapter;
use ddd::InfrastructureLayer;
use ddd::Layered;
use ddd::Repository;
use warcraft_keybinds::GridLayout;

use crate::persistence::grid_layout_persistence::GridLayoutPersistence;

/// Infrastructure adapter that persists the selected [`GridLayout`] to
/// localStorage as its fixed twelve-character storage string.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GridLayoutRepository;

impl Layered for GridLayoutRepository {
    type Layer = InfrastructureLayer;
}

impl Adapter for GridLayoutRepository {}

impl Repository<GridLayout> for GridLayoutRepository {
    fn load(&self) -> Option<GridLayout> {
        GridLayoutPersistence::load_grid_layout()
    }

    fn save(&self, aggregate: &GridLayout) {
        let layout = *aggregate;
        GridLayoutPersistence::save_grid_layout(layout);
    }
}
