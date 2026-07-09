use tw_macro::internal::{join_into, joined_len};
use tw_macro::tw;
use tw_macro::{ClassList, TailwindClass};
use warcraft_api::Race;

// The race-theme container is a `display:contents` grouping wrapper whose sole job is
// to publish the active race's colour as the `--race-color` custom property for every
// descendant (the unit list and the unit detail panel). CSS custom properties inherit
// through a `contents` element, so the box disappearing from layout does not stop the
// var reaching the cards below. Each arm mirrors the exact `--color-race-*` token the
// race tabs wear for that race (including the `-strong` variants for Orc and Neutral),
// so the theme is identical whether you read it off a tab or a card. This is a plain
// `match`, never a `states!` table: the wrapper is not a stateful element with N
// mutually-exclusive looks, it is one look parameterised by the active race.

macro_rules! race_theme_class {
    ($($utility:literal),+ $(,)?) => {{
        const OVERLAY: &[TailwindClass] = tw![$($utility),+];
        const LEN: usize = joined_len("", &[OVERLAY]);
        const BYTES: [u8; LEN] = join_into::<LEN>("", &[OVERLAY]);
        const CLASS: ClassList = ClassList::new(match ::core::str::from_utf8(&BYTES) {
            ::core::result::Result::Ok(class) => class,
            ::core::result::Result::Err(_) => ::core::panic!("non-utf8 race theme class"),
        });
        CLASS
    }};
}

pub(super) fn theme(race: Race) -> ClassList {
    match race {
        Race::Human => race_theme_class![
            "contents",
            "[--race-color:var(--color-race-human)]",
            "[--race-accent:var(--color-race-human)]"
        ],
        Race::Orc => race_theme_class![
            "contents",
            "[--race-color:var(--color-race-orc-strong)]",
            "[--race-accent:var(--color-race-orc)]"
        ],
        Race::Undead => race_theme_class![
            "contents",
            "[--race-color:var(--color-race-undead)]",
            "[--race-accent:var(--color-race-undead)]"
        ],
        Race::Nightelf => race_theme_class![
            "contents",
            "[--race-color:var(--color-race-nightelf)]",
            "[--race-accent:var(--color-race-nightelf)]"
        ],
        Race::Neutral => race_theme_class![
            "contents",
            "[--race-color:var(--color-race-neutral-strong)]",
            "[--race-accent:var(--color-warcraft-gold)]"
        ],
    }
}
