use tw_macro::{ClassList, tw};
classes! {
    base: tw![
        "flex",
        "items-baseline",
        "gap-2",
        "text-xl",
        "leading-title",
        "text-shadow-drop",
        "min-w-0",
    ],
    mobile: tw!["mobile:text-2xl", "mobile:leading-heading"],
}

/// The category label: the row's own gold treatment, worn directly rather than
/// selected through a shared variant.
pub(super) const LABEL: ClassList =
    ClassList::new("flex-[0_1_auto] min-w-0 text-inherit text-warcraft-gold/90");

/// The hit-points figure: green, semibold, enlarged — the vitality headline look.
pub(super) const VALUE: ClassList = ClassList::new(
    "flex-[1_1_auto] min-w-0 text-right text-warcraft-success font-semibold text-2xl [font-variant-numeric:tabular-nums] mobile:text-3xl",
);
