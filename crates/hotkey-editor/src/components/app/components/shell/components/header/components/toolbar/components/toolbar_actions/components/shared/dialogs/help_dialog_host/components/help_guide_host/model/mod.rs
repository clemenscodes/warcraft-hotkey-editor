use super::view::HelpGuideHostView;

/// The guide host's ddd model. The component sources the static guide content in its
/// presentation builder, so this is the fieldless published contract the frame's body region
/// names as its `Render::Model`.
#[derive(Clone, PartialEq, Default)]
pub struct HelpGuideHostModel;

impl From<&HelpGuideHostView> for HelpGuideHostModel {
    fn from(_view: &HelpGuideHostView) -> Self {
        Self
    }
}

impl ddd::Model for HelpGuideHostModel {
    type View = HelpGuideHostView;
}
