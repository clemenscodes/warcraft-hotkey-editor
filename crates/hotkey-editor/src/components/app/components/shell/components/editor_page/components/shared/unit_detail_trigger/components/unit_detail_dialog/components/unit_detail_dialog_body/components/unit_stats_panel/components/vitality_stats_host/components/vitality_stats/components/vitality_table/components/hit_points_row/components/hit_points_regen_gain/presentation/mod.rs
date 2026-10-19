use crate::components::app::components::shell::components::editor_page::components::shared::unit_detail_trigger::components::unit_detail_dialog::components::unit_detail_dialog_body::components::unit_stats_panel::components::shared::stat_figure::StatFigure;
use crate::components::app::components::shell::components::editor_page::components::shared::unit_detail_trigger::components::unit_detail_dialog::components::unit_detail_dialog_body::components::unit_stats_panel::components::vitality_stats_host::components::vitality_stats::components::vitality_table::components::shared::regen_mark::RegenMark;
use warcraft_api::HitPointsRegen;

/// The regeneration as drawn: the figure, marked with an icon when it only
/// applies at night or on blight, and that condition spelled out for the
/// tooltip.
pub(super) struct HitPointsRegenGainPresentation {
    pub(super) text: String,
    pub(super) mark: Option<&'static str>,
    pub(super) condition: Option<&'static str>,
}

impl From<HitPointsRegen> for HitPointsRegenGainPresentation {
    fn from(value: HitPointsRegen) -> Self {
        let figure = value.display();
        let regen_type = value.regen_type();
        let mark = RegenMark::for_regen_type(regen_type);
        let Some(mark) = mark else {
            return Self {
                text: figure,
                mark: None,
                condition: None,
            };
        };
        Self {
            text: figure,
            mark: Some(mark.icon()),
            condition: Some(mark.meaning()),
        }
    }
}
