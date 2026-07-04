use super::components::toast_list_item::ToastListItemProps;
use super::props::ToastListProps;

/// The per-record item props for the list body to loop over. Cloning each record
/// and pairing it with the shared remove callback is exactly the work the body
/// may not do, so it lives here.
pub struct ToastListPresentation {
    items: Vec<ToastListItemProps>,
}

impl ToastListPresentation {
    pub fn items(self) -> Vec<ToastListItemProps> {
        self.items
    }
}

impl From<&ToastListProps> for ToastListPresentation {
    fn from(props: &ToastListProps) -> Self {
        let on_remove = props.on_remove;
        let items = props
            .toasts
            .iter()
            .map(|source| {
                let record = source.clone();
                ToastListItemProps { record, on_remove }
            })
            .collect();
        Self { items }
    }
}
