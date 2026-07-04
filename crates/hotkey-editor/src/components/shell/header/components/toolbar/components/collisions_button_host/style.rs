use crate::{classes, styling::TailwindClass, tw};

// The button's box and container-query context: it owns the size per band and marks
// itself the query container, so the button fills it (`size-full`) and every `cqi`
// length inside the button — border, radius, icon, badge — scales against this box.
// The box is 36px on phones and tablets (touch-compact, beside the burger). From laptop
// up it is a viewport-proportional square — `1.55vw` lands on ~30px through the common
// desktop widths and grows with the viewport; the tight toolbar gap keeps the ten-button
// row well inside its grid track.
const BASE: &[TailwindClass] = tw!["inline-flex", "shrink-0", "[container-type:inline-size]"];
const MOBILE: &[TailwindClass] = tw!["mobile:w-9", "mobile:h-9"];
const TABLET: &[TailwindClass] = tw!["tablet:w-9", "tablet:h-9"];
const LAPTOP: &[TailwindClass] = tw![
    "laptop:w-[clamp(1.75rem,1.55vw,4rem)]",
    "laptop:h-[clamp(1.75rem,1.55vw,4rem)]",
];
const DESKTOP: &[TailwindClass] = tw![
    "desktop:w-[clamp(1.75rem,1.55vw,4rem)]",
    "desktop:h-[clamp(1.75rem,1.55vw,4rem)]",
];
const QHD: &[TailwindClass] = tw![
    "qhd:w-[clamp(1.75rem,1.55vw,4rem)]",
    "qhd:h-[clamp(1.75rem,1.55vw,4rem)]",
];
const UHD: &[TailwindClass] = tw![
    "uhd:w-[clamp(1.75rem,1.55vw,4rem)]",
    "uhd:h-[clamp(1.75rem,1.55vw,4rem)]",
];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
