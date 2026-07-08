use super::props::GoldHeadingProps;
use super::style;
use tw_macro::ClassList;

/// The heading's presentation: the variant-selected class plus the text. The
/// weight-specific class is chosen here, never in the body.
pub struct GoldHeadingPresentation {
    pub(super) class: ClassList,
    pub(super) title: String,
}

impl From<&GoldHeadingProps> for GoldHeadingPresentation {
    fn from(props: &GoldHeadingProps) -> Self {
        let class = style::class(props.variant);
        let title = props.title.clone();
        Self { class, title }
    }
}
