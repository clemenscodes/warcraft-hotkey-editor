use super::props::ResolveBreadcrumbProps;

/// The breadcrumb's derived aria state.
pub(super) struct ResolveBreadcrumbModel {
    pub(super) aria_current: &'static str,
}

impl From<&ResolveBreadcrumbProps> for ResolveBreadcrumbModel {
    fn from(props: &ResolveBreadcrumbProps) -> Self {
        let aria_current = if props.active { "page" } else { "false" };
        Self { aria_current }
    }
}
