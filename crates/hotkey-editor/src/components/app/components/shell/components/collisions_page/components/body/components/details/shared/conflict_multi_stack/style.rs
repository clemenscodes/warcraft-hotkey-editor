use tw_macro::ClassList;

/// The abilities row (multi-carrier variant); shared value with the pair row and the
/// island conflict card.
pub(super) const ABILITY_ROW: ClassList = ClassList::new(
    "grid grid-cols-[1fr_auto_1fr] items-start justify-items-center gap-3 w-full data-[multi=true]:grid-cols-none data-[multi=true]:flex data-[multi=true]:flex-wrap data-[multi=true]:justify-center",
);
