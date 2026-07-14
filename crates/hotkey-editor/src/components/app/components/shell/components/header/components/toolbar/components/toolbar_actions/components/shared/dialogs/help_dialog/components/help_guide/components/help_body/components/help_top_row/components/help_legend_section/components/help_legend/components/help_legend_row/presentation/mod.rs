use super::model::HelpLegendRowModel;

pub struct HelpLegendRowPresentation {
    pub(super) icon: &'static str,
    pub(super) label: &'static str,
    pub(super) description: &'static str,
}

impl From<&HelpLegendRowModel> for HelpLegendRowPresentation {
    fn from(model: &HelpLegendRowModel) -> Self {
        let entry = model.entry;
        let icon = entry.icon();
        let label = entry.label();
        let description = entry.description();
        Self {
            icon,
            label,
            description,
        }
    }
}

impl ddd::Presentation for HelpLegendRowPresentation {
    type Model = HelpLegendRowModel;
}
