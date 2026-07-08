use super::components::template_gallery::TemplateGalleryProps;
use dioxus::prelude::*;

/// The templates dialog's scroll region input: the gallery of template cards it holds.
#[derive(Props, Clone, PartialEq)]
pub struct TemplatesDialogBodyProps {
    pub gallery: TemplateGalleryProps,
}

impl From<&TemplatesDialogBodyProps> for TemplateGalleryProps {
    fn from(props: &TemplatesDialogBodyProps) -> Self {
        props.gallery.clone()
    }
}
