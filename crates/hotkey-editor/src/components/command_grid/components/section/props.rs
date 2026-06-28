use dioxus::prelude::*;

use crate::components::command_grid::CommandGridSectionProps;
use warcraft_keybinds::GridBehavior;

/// The shared section body's input: the common section props, parameterized by the
/// behavior type the body resolves and mutates tiles with. The behavior is a
/// zero-sized marker; the three public wrappers each instantiate it with their own
/// concrete type.
#[derive(Props, Clone, PartialEq)]
pub(crate) struct GridSectionProps<B: GridBehavior> {
    pub(crate) behavior: B,
    pub(crate) section: CommandGridSectionProps,
}

impl<B: GridBehavior> GridSectionProps<B> {
    pub(crate) fn new(section: CommandGridSectionProps) -> Self {
        let behavior = B::default();
        Self { behavior, section }
    }
}
