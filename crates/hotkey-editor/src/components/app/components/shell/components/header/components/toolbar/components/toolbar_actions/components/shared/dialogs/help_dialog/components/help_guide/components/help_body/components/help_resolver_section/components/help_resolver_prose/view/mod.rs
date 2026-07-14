#[derive(Clone, PartialEq)]
pub struct HelpResolverProseView {
    pub paragraphs: &'static [&'static str],
}

impl ddd::View for HelpResolverProseView {}
