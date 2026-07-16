#[derive(Clone, PartialEq)]
pub struct GridCarouselDotsView {
    pub grid_count: usize,
    pub active_grid_index: usize,
}

impl ddd::View for GridCarouselDotsView {}
