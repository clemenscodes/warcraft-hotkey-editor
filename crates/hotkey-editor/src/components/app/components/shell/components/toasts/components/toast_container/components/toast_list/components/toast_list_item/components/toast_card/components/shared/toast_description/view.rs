/// The published `View` contract mirroring [`ToastDescriptionProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ToastDescriptionView {
    pub description: Option<String>,
}

impl ddd::View for ToastDescriptionView {}
