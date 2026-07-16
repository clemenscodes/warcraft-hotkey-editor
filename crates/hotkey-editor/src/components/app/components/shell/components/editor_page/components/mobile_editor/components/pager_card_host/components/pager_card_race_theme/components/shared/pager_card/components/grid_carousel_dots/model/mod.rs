use super::view::GridCarouselDotsView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct GridCarouselDotsModel {
    pub grid_count: usize,
    pub active_grid_index: usize,
}

impl From<&GridCarouselDotsView> for GridCarouselDotsModel {
    fn from(view: &GridCarouselDotsView) -> Self {
        let GridCarouselDotsView {
            grid_count,
            active_grid_index,
        } = view.clone();
        Self {
            grid_count,
            active_grid_index,
        }
    }
}

impl ddd::Model for GridCarouselDotsModel {
    type View = GridCarouselDotsView;
}
