use super::props::BreadcrumbProps;

/// The breadcrumb's derived aria state.
pub(super) struct BreadcrumbModel {
    pub(super) aria_current: &'static str,
}

impl From<&BreadcrumbProps> for BreadcrumbModel {
    fn from(props: &BreadcrumbProps) -> Self {
        let aria_current = if props.active { "page" } else { "false" };
        Self { aria_current }
    }
}
