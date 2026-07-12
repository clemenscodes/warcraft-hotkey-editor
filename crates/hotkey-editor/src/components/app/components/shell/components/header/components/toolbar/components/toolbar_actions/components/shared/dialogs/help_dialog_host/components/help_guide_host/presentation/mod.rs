use super::data::{HELP_CONTENT, HelpContent};
use super::model::HelpGuideHostModel;

/// The guide host's shaped data: the whole onboarding guide content the body lays out.
pub(super) struct HelpGuideHostPresentation {
    pub(super) content: HelpContent,
}

/// Sources the static guide content behind a single flat call, so the body only places it —
/// the one piece of work the body is not allowed to do.
pub(super) fn use_help_guide_host() -> HelpGuideHostPresentation {
    let content = HELP_CONTENT;
    HelpGuideHostPresentation { content }
}

impl ddd::Presentation for HelpGuideHostPresentation {
    type Model = HelpGuideHostModel;
}
