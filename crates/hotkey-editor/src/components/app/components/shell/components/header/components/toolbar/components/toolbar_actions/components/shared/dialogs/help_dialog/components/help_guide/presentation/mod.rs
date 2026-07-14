use super::data::{HELP_CONTENT, HelpContent};
use super::model::HelpGuideModel;

pub(super) struct HelpGuidePresentation {
    pub(super) content: HelpContent,
}

pub(super) fn use_help_guide() -> HelpGuidePresentation {
    let content = HELP_CONTENT;
    HelpGuidePresentation { content }
}

impl ddd::Presentation for HelpGuidePresentation {
    type Model = HelpGuideModel;
}
