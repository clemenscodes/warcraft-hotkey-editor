use warcraft_api::HitPointsRegen;
use warcraft_api::RegenType;

const AT_NIGHT: &str = "at night";
const ON_BLIGHT: &str = "on blight";

pub(super) struct HitPointsRegenPresentation {
    pub(super) qualifier: Option<&'static str>,
}

impl From<HitPointsRegen> for HitPointsRegenPresentation {
    fn from(value: HitPointsRegen) -> Self {
        let regen_type = value.regen_type();
        let qualifier = match regen_type {
            RegenType::Night => Some(AT_NIGHT),
            RegenType::Blight => Some(ON_BLIGHT),
            RegenType::Always | RegenType::None => None,
        };
        Self { qualifier }
    }
}
