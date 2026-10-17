#[derive(Clone, PartialEq)]
pub struct HelpResolverProseView {
    pub paragraphs: &'static [&'static str],
}

impl ddd::Layered for HelpResolverProseView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for HelpResolverProseView {}
