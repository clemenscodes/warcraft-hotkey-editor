/// The published `View` contract mirroring [`ResolvePageProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ResolvePageView {
    pub entry: Option<String>,
}

impl ddd::View for ResolvePageView {}
