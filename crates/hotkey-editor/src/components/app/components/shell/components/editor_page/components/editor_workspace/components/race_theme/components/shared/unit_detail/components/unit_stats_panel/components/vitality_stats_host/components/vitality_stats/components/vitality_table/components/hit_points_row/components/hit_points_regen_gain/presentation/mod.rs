use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::components::shared::stat_figure::StatFigure;
use warcraft_api::HitPointsRegen;
use warcraft_api::RegenType;

const NIGHT_MARK: RegenMark = RegenMark {
    symbol: "\u{263E}",
    meaning: "Regenerates hit points only at night",
};
const BLIGHT_MARK: RegenMark = RegenMark {
    symbol: "\u{2620}",
    meaning: "Regenerates hit points only on blight",
};

/// The symbol a conditional regeneration is marked with, and what it means.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
struct RegenMark {
    symbol: &'static str,
    meaning: &'static str,
}

/// The regeneration as drawn: the figure, marked with a symbol when it only
/// applies at night or on blight, and that condition spelled out for the
/// tooltip.
pub(super) struct HitPointsRegenGainPresentation {
    pub(super) text: String,
    pub(super) condition: Option<&'static str>,
}

impl From<HitPointsRegen> for HitPointsRegenGainPresentation {
    fn from(value: HitPointsRegen) -> Self {
        let figure = value.display();
        let regen_type = value.regen_type();
        let mark = match regen_type {
            RegenType::Night => Some(NIGHT_MARK),
            RegenType::Blight => Some(BLIGHT_MARK),
            RegenType::Always | RegenType::None => None,
        };
        let Some(mark) = mark else {
            return Self {
                text: figure,
                condition: None,
            };
        };
        let symbol = mark.symbol;
        let text = format!("{figure} {symbol}");
        Self {
            text,
            condition: Some(mark.meaning),
        }
    }
}
