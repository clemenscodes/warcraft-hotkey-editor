use crate::components::app::components::shell::components::collisions_page::presentation::IslandView;

#[derive(Clone, PartialEq)]
pub struct FilledIslandDetailView {
    pub island: IslandView,
}

impl ddd::Layered for FilledIslandDetailView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for FilledIslandDetailView {}
