use crate::components::app::components::shell::components::collisions_page::presentation::IslandView;

#[derive(Clone, PartialEq)]
pub struct IslandSidebarView {
    pub islands: Vec<IslandView>,
}

impl ddd::Layered for IslandSidebarView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for IslandSidebarView {}
