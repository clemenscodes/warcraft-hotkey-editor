use dioxus::prelude::*;

/// The instruction block's content: the intro line and the optional warning. The
/// filename chip is fixed, so it is not carried here.
#[derive(Props, Clone, PartialEq)]
pub struct InfoContentProps {
    pub intro: &'static str,
    pub warning: Option<&'static str>,
}
