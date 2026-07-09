use tw_macro::ClassList;

/// The conflict card shell: the `.conflict-card` class is coupled to the e2e suite, so
/// it is written literally (shared value across the three conflict cards).
pub(super) const CONFLICT_CARD: ClassList = ClassList::new("conflict-card grid min-w-0");
