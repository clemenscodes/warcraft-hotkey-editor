use crate::components::app::components::shell::components::shared::icons::{ICON_MOON, ICON_SKULL};
use warcraft_api::RegenType;

/// The icon a conditional hit point regeneration is marked with, and what it
/// means. The figure carries the icon; the note below the table spells it out.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct RegenMark {
    icon: &'static str,
    meaning: &'static str,
}

const NIGHT_MARK: RegenMark = RegenMark {
    icon: ICON_MOON,
    meaning: "Regenerates only at night",
};
const BLIGHT_MARK: RegenMark = RegenMark {
    icon: ICON_SKULL,
    meaning: "Regenerates only on blight",
};

impl RegenMark {
    /// The mark a regeneration of `regen_type` carries, if it is conditional.
    pub(crate) fn for_regen_type(regen_type: RegenType) -> Option<Self> {
        match regen_type {
            RegenType::Night => Some(NIGHT_MARK),
            RegenType::Blight => Some(BLIGHT_MARK),
            RegenType::Always | RegenType::None => None,
        }
    }

    pub(crate) fn icon(&self) -> &'static str {
        self.icon
    }

    pub(crate) fn meaning(&self) -> &'static str {
        self.meaning
    }
}
