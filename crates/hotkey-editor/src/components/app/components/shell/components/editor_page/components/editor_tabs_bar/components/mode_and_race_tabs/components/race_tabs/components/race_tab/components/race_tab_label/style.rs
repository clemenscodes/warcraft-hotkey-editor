use tw_macro::internal::{join_into, joined_len};
use tw_macro::tw;
use tw_macro::{ClassList, TailwindClass};
use warcraft_api::Race;

classes! {
    base: tw![
        "relative",
        "z-2",
        "py-1.5",
        "px-2.5",
        "pb-2",
        "w-full",
        "text-white",
        "min-w-0",
    ],
    mobile: tw![
        "mobile:pt-1",
        "mobile:px-0.5",
        "mobile:pb-2",
        "mobile:text-xs",
        "mobile:tracking-snug",
    ],
    tablet: tw![
        "tablet:pt-1",
        "tablet:px-0.5",
        "tablet:pb-2",
        "tablet:text-xs",
        "tablet:tracking-snug",
    ],
}

// The label wears its OWN race's accent when its tab is active — a per-label
// data→token mapping keyed on the race, not a runtime state table. A plain `match`
// layering the race's active-accent overlay onto the shared base (via a small local
// join macro), never a `states!` table. The active trigger is the parent button's
// `group` `data-active`, read through the `group-data-[active]` variant.
macro_rules! race_tab_label_class {
    ($($utility:literal),+ $(,)?) => {{
        const OVERLAY: &[TailwindClass] = tw![$($utility),+];
        const LEN: usize = joined_len(CLASS_STR, &[OVERLAY]);
        const BYTES: [u8; LEN] = join_into::<LEN>(CLASS_STR, &[OVERLAY]);
        const JOINED: ClassList = ClassList::new(match ::core::str::from_utf8(&BYTES) {
            ::core::result::Result::Ok(class) => class,
            ::core::result::Result::Err(_) => ::core::panic!("non-utf8 race tab label class"),
        });
        JOINED
    }};
}

pub(super) fn class(race: Race) -> ClassList {
    match race {
        Race::Human => race_tab_label_class!["group-data-[active=true]:text-race-human"],
        Race::Orc => race_tab_label_class!["group-data-[active=true]:text-race-orc"],
        Race::Nightelf => race_tab_label_class!["group-data-[active=true]:text-race-nightelf"],
        Race::Undead => race_tab_label_class!["group-data-[active=true]:text-race-undead"],
        Race::Neutral => race_tab_label_class!["group-data-[active=true]:text-warcraft-gold"],
    }
}
