use crate::components::app::components::shell::components::collisions_page::presentation::IslandView;

#[derive(Clone, PartialEq)]
pub struct IslandPagerCardView {
    pub island: IslandView,
}

impl ddd::View for IslandPagerCardView {}
