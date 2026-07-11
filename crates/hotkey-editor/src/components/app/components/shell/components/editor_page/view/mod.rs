/// The published `View` contract mirroring [`EditorPageModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct EditorPageView {
    pub race: Option<String>,
    pub mode: Option<String>,
    pub unit: Option<String>,
    pub search_query: Option<String>,
}

impl ddd::View for EditorPageView {}
