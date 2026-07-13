/// The published `View` contract mirroring [`HelpResolverProseModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HelpResolverProseView {
    pub paragraphs: &'static [&'static str],
}

impl ddd::View for HelpResolverProseView {}
