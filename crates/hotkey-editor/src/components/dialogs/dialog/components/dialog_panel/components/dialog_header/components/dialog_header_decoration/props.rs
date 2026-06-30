use dioxus::prelude::*;

/// A header flourish. The trailing one is mirrored, so the variant is a single
/// `flipped` flag that selects the modifier class.
#[derive(Props, Clone, PartialEq)]
pub struct DialogHeaderDecorationProps {
    #[props(default)]
    pub flipped: bool,
}

/// The resolved class list, built from the flag so the body only places it.
pub(super) struct DialogHeaderDecorationPresentation {
    pub(super) class: String,
}

impl From<&DialogHeaderDecorationProps> for DialogHeaderDecorationPresentation {
    fn from(props: &DialogHeaderDecorationProps) -> Self {
        let mut class = String::from("dialog-header-decoration");
        if props.flipped {
            class.push_str(" dialog-header-decoration-flipped");
        }
        Self { class }
    }
}
