use super::components::active_stat_gain::ActiveStatGainProps;
use super::components::muted_stat_gain::MutedStatGainProps;
use super::props::StatGainProps;
use super::stat_figure::StatFigure;

impl<Figure: StatFigure> From<&StatGainProps<Figure>> for ActiveStatGainProps {
    fn from(props: &StatGainProps<Figure>) -> Self {
        let text = props.value.display();
        Self { text }
    }
}

impl<Figure: StatFigure> From<&StatGainProps<Figure>> for MutedStatGainProps {
    fn from(props: &StatGainProps<Figure>) -> Self {
        let text = props.value.display();
        Self { text }
    }
}
