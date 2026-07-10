use super::super::stat_figure::StatFigure;

/// The published `View` contract mirroring [`StatGainProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct StatGainView<Figure: StatFigure> {
    pub value: Figure,
}

impl<Figure: StatFigure> ddd::View for StatGainView<Figure> {}
