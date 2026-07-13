use super::view::HelpGuideView;

/// The guide host's ddd model. The component sources the static guide content in its
/// presentation builder, so this is the fieldless published contract the frame's body region
/// names as its `Render::Model`.
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
