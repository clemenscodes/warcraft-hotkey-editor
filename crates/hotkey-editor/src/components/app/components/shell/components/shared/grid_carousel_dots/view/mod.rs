#[derive(Clone, PartialEq)]
pub struct GridCarouselDotsView {
    pub grid_count: usize,
    pub active_grid_index: usize,
}

impl ddd::Layered for GridCarouselDotsView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for GridCarouselDotsView {}
