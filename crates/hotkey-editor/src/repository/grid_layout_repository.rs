use ddd::Adapter;
use ddd::InfrastructureLayer;
use ddd::Layered;
use ddd::Repository;
use warcraft_keybinds::GridLayout;

use crate::persistence::grid_layout_persistence;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GridLayoutRepository;

impl Layered for GridLayoutRepository {
    type Layer = InfrastructureLayer;
}

impl Adapter for GridLayoutRepository {}

impl Repository<GridLayout> for GridLayoutRepository {
    fn load(&self) -> Option<GridLayout> {
        grid_layout_persistence::load_grid_layout()
    }

    fn save(&self, aggregate: &GridLayout) {
        let layout = *aggregate;
        grid_layout_persistence::save_grid_layout(layout);
    }
}
