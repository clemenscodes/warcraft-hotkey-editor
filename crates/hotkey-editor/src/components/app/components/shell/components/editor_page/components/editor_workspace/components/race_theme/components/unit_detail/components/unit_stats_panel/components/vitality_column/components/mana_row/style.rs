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

/// The category label: the row's own gold treatment.
pub(super) const LABEL: ClassList =
    ClassList::new("flex-[0_1_auto] min-w-0 text-inherit text-warcraft-gold/90");

/// The mana figure: the human-blue accent, semibold and enlarged, dimmed to faint when
/// the unit has no mana pool.
pub(super) const VALUE: ClassList = ClassList::new(
    "flex-[1_1_auto] min-w-0 text-right text-race-human font-semibold text-2xl [font-variant-numeric:tabular-nums] data-[zero=true]:text-warcraft-text-faint data-[zero=true]:font-normal mobile:text-3xl",
);
