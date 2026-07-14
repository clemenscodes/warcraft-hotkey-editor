use super::view::HelpResolverProseView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HelpResolverProseModel {
    pub paragraphs: &'static [&'static str],
}

impl From<&HelpResolverProseView> for HelpResolverProseModel {
    fn from(view: &HelpResolverProseView) -> Self {
        let HelpResolverProseView { paragraphs } = view.clone();
        Self { paragraphs }
    }
}

impl ddd::Model for HelpResolverProseModel {
    type View = HelpResolverProseView;
}
