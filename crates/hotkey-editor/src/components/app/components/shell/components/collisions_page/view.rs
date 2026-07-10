/// The published `View` contract mirroring [`CollisionsPageProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct CollisionsPageView {
    pub kind: Option<String>,
    pub entry: Option<String>,
}

impl ddd::View for CollisionsPageView {}
