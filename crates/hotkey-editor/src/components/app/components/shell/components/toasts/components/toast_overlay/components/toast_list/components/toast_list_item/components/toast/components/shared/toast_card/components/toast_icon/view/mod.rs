/// The published `View` contract mirroring [`ToastIconModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ToastIconView {
    pub icon: &'static str,
}

impl ddd::View for ToastIconView {}
