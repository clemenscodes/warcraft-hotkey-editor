/// Which drawer action a row triggers. It drives the row's live weight, its
/// `aria-*` flags, and which handler is wired to it — resolved by the builder
/// from the static content, never chosen in the body.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(super) enum BurgerAction {
    /// The primary Grid Layout call-to-action.
    Layout,
    Undo,
    Redo,
    Upload,
    Templates,
    SystemHotkeys,
    Preview,
    Resolve,
    Download,
    Help,
}
