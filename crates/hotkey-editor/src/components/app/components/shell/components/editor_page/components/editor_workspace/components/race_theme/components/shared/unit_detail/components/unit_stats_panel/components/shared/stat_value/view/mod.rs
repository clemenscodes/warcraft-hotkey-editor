use super::super::stat_figure::StatFigure;

/// The published `View` contract mirroring [`StatValueModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct StatValueView<Figure: StatFigure> {
    pub value: Figure,
}

impl<Figure: StatFigure> ddd::View for StatValueView<Figure> {}
