use tw_macro::ClassList;

/// The conflict card shell: the `.conflict-card` class is coupled to the e2e suite, so
/// it is written literally (shared value across the three conflict cards).
pub(super) const CONFLICT_CARD: ClassList = ClassList::new("conflict-card grid min-w-0");
/// The card surface: bordered, tinted, centered — the collision conflict look.
pub(super) const PANEL: ClassList = ClassList::new(
    "flex flex-col py-6 bg-warcraft-bg-mid/45 border rounded-panel gap-6 px-4 items-center min-w-0 border-warcraft-blue-deep",
);
