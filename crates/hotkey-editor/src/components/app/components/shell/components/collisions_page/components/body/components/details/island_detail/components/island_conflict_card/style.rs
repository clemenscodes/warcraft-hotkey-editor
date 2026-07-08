use tw_macro::ClassList;

/// The conflict card shell: the `.conflict-card` class is coupled to the e2e suite, so
/// it is written literally (shared value across the three conflict cards).
pub(super) const CONFLICT_CARD: ClassList = ClassList::new("conflict-card grid min-w-0");
/// The card surface: bordered, tinted, centered — the collision conflict look.
pub(super) const PANEL: ClassList = ClassList::new(
    "flex flex-col py-6 bg-warcraft-bg-mid/45 border rounded-panel gap-6 px-4 items-center min-w-0 border-warcraft-blue-deep",
);

/// The abilities row flanking the separator; shared value with the pair row and multi
/// stack.
pub(super) const ABILITY_ROW: ClassList = ClassList::new(
    "grid grid-cols-[1fr_auto_1fr] items-start justify-items-center gap-3 w-full data-[multi=true]:grid-cols-none data-[multi=true]:flex data-[multi=true]:flex-wrap data-[multi=true]:justify-center",
);
