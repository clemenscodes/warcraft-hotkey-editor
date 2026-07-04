use crate::classes;

// The button's box and container-query context: it owns the size per band and marks
// itself the query container, so the button fills it (`size-full`) and every `cqi`
// length inside the button — border, radius, icon, badge — scales against this box.
// The box is 36px on phones and tablets (touch-compact, beside the burger). From laptop
// up it is a viewport-proportional square — `1.55vw` lands on ~30px through the common
// desktop widths (matching the deprecated production sizing) and grows with the viewport;
// the tight toolbar gap keeps the ten-button row well inside its grid track.
const BASE: &[&str] = &["inline-flex", "shrink-0", "[container-type:inline-size]"];
const MOBILE: &[&str] = &["mobile:w-9", "mobile:h-9"];
const TABLET: &[&str] = &["tablet:w-9", "tablet:h-9"];
const LAPTOP: &[&str] = &[
    "laptop:w-[clamp(1.75rem,1.55vw,4rem)]",
    "laptop:h-[clamp(1.75rem,1.55vw,4rem)]",
];
const DESKTOP: &[&str] = &[
    "desktop:w-[clamp(1.75rem,1.55vw,4rem)]",
    "desktop:h-[clamp(1.75rem,1.55vw,4rem)]",
];
const QHD: &[&str] = &[
    "qhd:w-[clamp(1.75rem,1.55vw,4rem)]",
    "qhd:h-[clamp(1.75rem,1.55vw,4rem)]",
];
const UHD: &[&str] = &[
    "uhd:w-[clamp(1.75rem,1.55vw,4rem)]",
    "uhd:h-[clamp(1.75rem,1.55vw,4rem)]",
];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
