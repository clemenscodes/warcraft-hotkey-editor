/// The published `View` contract mirroring [`FooterHeartProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct FooterHeartView {
    pub svg: &'static str,
}

impl ddd::View for FooterHeartView {}
