use super::components::pager_page::PagerPageView;

#[derive(Clone, PartialEq)]
pub struct PagerTrackView {
    pub pages: Vec<PagerPageView>,
}

impl ddd::Layered for PagerTrackView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for PagerTrackView {}
