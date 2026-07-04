use crate::{classes, styling::TailwindClass, tw};

// The button's box and container-query context: it owns the size per band and marks
// itself the query container, so the button fills it (`size-full`) and every `cqi`
// length inside the button — border, radius, icon, badge — scales against this box.
// The laptop-and-up default (in BASE) carries no size of its own: `h-full` fills the
// toolbar's shared row height and `aspect-square` turns that into its width, so it matches
// the layout button and the inline actions exactly and rescales with the bar. The two
// touch bands override it with a 36px square (touch-compact, beside the burger).
const BASE: &[TailwindClass] = tw![
    "inline-flex",
    "shrink-0",
    "[container-type:inline-size]",
    "h-full",
    "aspect-square"
];
const MOBILE: &[TailwindClass] = tw!["mobile:w-9", "mobile:h-9"];
const TABLET: &[TailwindClass] = tw!["tablet:w-9", "tablet:h-9"];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
