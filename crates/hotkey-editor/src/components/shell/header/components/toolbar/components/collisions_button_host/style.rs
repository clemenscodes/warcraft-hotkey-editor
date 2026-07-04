use crate::{classes, styling::TailwindClass, tw};

// The button's box and container-query context: it owns the size per band and marks
// itself the query container, so the button fills it (`size-full`) and every `cqi`
// length inside the button — border, radius, icon, badge — scales against this box.
// The box is 36px on phones and tablets (touch-compact, beside the burger). From laptop
// up it carries no size of its own: `h-full` fills the toolbar's shared row height and
// `aspect-square` turns that into its width, so it matches the layout button and the
// inline actions exactly and rescales with the bar.
const BASE: &[TailwindClass] = tw!["inline-flex", "shrink-0", "[container-type:inline-size]"];
const MOBILE: &[TailwindClass] = tw!["mobile:w-9", "mobile:h-9"];
const TABLET: &[TailwindClass] = tw!["tablet:w-9", "tablet:h-9"];
const LAPTOP: &[TailwindClass] = tw!["laptop:h-full", "laptop:aspect-square"];
const DESKTOP: &[TailwindClass] = tw!["desktop:h-full", "desktop:aspect-square"];
const QHD: &[TailwindClass] = tw!["qhd:h-full", "qhd:aspect-square"];
const UHD: &[TailwindClass] = tw!["uhd:h-full", "uhd:aspect-square"];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
