use crate::services::navigation::app_view::AppView;

/// The app shell's layout state. The collisions view is a single full-bleed page and
/// drops the inter-section gap; every other view keeps the standard section gaps.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum AppLayout {
    #[default]
    Standard,
    Collisions,
}

impl From<AppView> for AppLayout {
    fn from(view: AppView) -> Self {
        match view {
            AppView::Collisions { .. } => Self::Collisions,
            AppView::Editor | AppView::Resolve => Self::Standard,
        }
    }
}
