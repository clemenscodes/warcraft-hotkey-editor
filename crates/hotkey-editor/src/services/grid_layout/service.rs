use ddd::ApplicationLayer;
use ddd::ApplicationService;
use ddd::Layered;
use ddd::Service;
use dioxus::prelude::*;
use warcraft_keybinds::GridLayout;

use crate::repository::grid_layout_repository::GridLayoutRepository;

#[derive(Clone, Copy)]
pub struct GridLayoutService {
    layout: Signal<GridLayout>,
}

impl GridLayoutService {
    pub fn new(layout: Signal<GridLayout>) -> Self {
        Self { layout }
    }

    pub fn layout(&self) -> ReadSignal<GridLayout> {
        self.layout.into()
    }

    pub fn select(&self, layout: GridLayout) {
        self.commit(|current| {
            *current = layout;
        });
    }
}

impl Layered for GridLayoutService {
    type Layer = ApplicationLayer;
}

impl ApplicationService for GridLayoutService {}

impl Service<GridLayout> for GridLayoutService {
    type Repository = GridLayoutRepository;

    fn repository(&self) -> Self::Repository {
        GridLayoutRepository
    }

    fn snapshot(&self) -> GridLayout {
        *self.layout.peek()
    }

    fn replace(&self, aggregate: GridLayout) {
        let mut layout_signal = self.layout;
        layout_signal.set(aggregate);
    }
}
