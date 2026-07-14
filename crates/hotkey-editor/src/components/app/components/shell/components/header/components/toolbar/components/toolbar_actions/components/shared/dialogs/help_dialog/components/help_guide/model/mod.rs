use super::view::HelpGuideView;

#[derive(Clone, PartialEq, Default)]
pub struct HelpGuideModel;

impl From<&HelpGuideView> for HelpGuideModel {
    fn from(_view: &HelpGuideView) -> Self {
        Self
    }
}

impl ddd::Model for HelpGuideModel {
    type View = HelpGuideView;
}
