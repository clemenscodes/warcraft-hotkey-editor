use super::components::active_breadcrumb::ActiveBreadcrumbProps;
use super::components::idle_breadcrumb::IdleBreadcrumbProps;
use super::props::BreadcrumbProps;

impl From<&BreadcrumbProps> for IdleBreadcrumbProps {
    fn from(props: &BreadcrumbProps) -> Self {
        let label = props.label.clone();
        let count = props.count;
        let onclick = props.onclick;
        Self {
            label,
            count,
            onclick,
        }
    }
}

impl From<&BreadcrumbProps> for ActiveBreadcrumbProps {
    fn from(props: &BreadcrumbProps) -> Self {
        let label = props.label.clone();
        let count = props.count;
        let onclick = props.onclick;
        Self {
            label,
            count,
            onclick,
        }
    }
}
