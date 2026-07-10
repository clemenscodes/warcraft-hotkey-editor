use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::components::shared::stat_figure::StatFigure;
use warcraft_api::RegenType;
use warcraft_keybinds::HitPointsRegen;

const AT_NIGHT: &str = "at night";
const ON_BLIGHT: &str = "on blight";

/// The hit-points regeneration row's shaped presentation: its optional italic
/// qualifier and the formatted gain, resolved out of the component body. The qualifier
/// names the condition the regeneration depends on; it is `None` when regeneration
/// applies unconditionally.
pub(super) struct HitPointsRegenPresentation {
    pub(super) qualifier: Option<&'static str>,
    pub(super) gain_text: String,
    pub(super) gain_muted: bool,
}

impl From<HitPointsRegen> for HitPointsRegenPresentation {
    fn from(value: HitPointsRegen) -> Self {
        let regen_type = value.regen_type();
        let qualifier = match regen_type {
            RegenType::Night => Some(AT_NIGHT),
            RegenType::Blight => Some(ON_BLIGHT),
            RegenType::Always | RegenType::None => None,
        };
        let gain_text = value.display();
        let gain_muted = value.is_muted();
        Self {
            qualifier,
            gain_text,
            gain_muted,
        }
    }
}
