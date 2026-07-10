/// The published `View` contract mirroring [`LayoutIntroLineProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct LayoutIntroLineView {
    pub line: String,
}

impl ddd::View for LayoutIntroLineView {}
