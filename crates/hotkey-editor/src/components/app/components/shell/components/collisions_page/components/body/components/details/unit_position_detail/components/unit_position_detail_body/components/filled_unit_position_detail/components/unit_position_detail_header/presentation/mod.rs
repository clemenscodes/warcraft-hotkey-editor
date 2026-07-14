use super::model::UnitPositionDetailHeaderModel;
use warcraft_api::WarcraftObjectId;

/// The position-collision detail pane header's presentation: the unit button inputs and
/// its text meta column, shaped from the unit view. Built purely from the model — a
/// shaping leaf.
pub struct UnitPositionDetailHeaderPresentation {
    pub(super) name: String,
    pub(super) unit_id: WarcraftObjectId,
    pub(super) icon_url: Option<String>,
    pub(super) count: usize,
    pub(super) meta_name: String,
}

impl From<&UnitPositionDetailHeaderModel> for UnitPositionDetailHeaderPresentation {
    fn from(model: &UnitPositionDetailHeaderModel) -> Self {
        let unit = &model.unit;
        let name = unit.name().to_owned();
        let unit_id = unit.unit_id();
        let icon_url = unit.icon_url().map(str::to_owned);
        let count = model.count;
        let meta_name = name.clone();
        Self {
            name,
            unit_id,
            icon_url,
            count,
            meta_name,
        }
    }
}

impl ddd::Presentation for UnitPositionDetailHeaderPresentation {
    type Model = UnitPositionDetailHeaderModel;
}
