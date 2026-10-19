use crate::components::app::components::shell::components::collisions_page::presentation::IslandView;

#[derive(Clone, PartialEq)]
pub struct IslandPagerCardHostView {
    pub island: IslandView,
}

impl ddd::Layered for IslandPagerCardHostView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for IslandPagerCardHostView {}
