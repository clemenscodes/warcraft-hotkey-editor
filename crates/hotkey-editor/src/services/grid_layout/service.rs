use ddd::ApplicationLayer;
use ddd::ApplicationService;
use ddd::Layered;
use ddd::Service;
use dioxus::prelude::*;
use warcraft_keybinds::GridLayout;

use crate::repository::grid_layout_repository::GridLayoutRepository;

/// The application-layer service that owns the live selected [`GridLayout`] and is
/// the only sanctioned way for the renderer to change it. `select` runs through
/// [`Service::commit`], write-throughs to the repository, then updates the live
/// signal, so localStorage never trails the selection.
#[derive(Clone, Copy)]
pub struct GridLayoutService {
    layout: Signal<GridLayout>,
}

impl GridLayoutService {
    pub fn new(layout: Signal<GridLayout>) -> Self {
        Self { layout }
    }

    /// A read-only, reactive view of the selected layout for the renderer.
    pub fn layout(&self) -> ReadSignal<GridLayout> {
        self.layout.into()
    }

    /// The sanctioned mutation command: replace the selected layout and persist.
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
        *self.layout.read()
    }

    fn replace(&self, aggregate: GridLayout) {
        let mut layout_signal = self.layout;
        layout_signal.set(aggregate);
    }
}
