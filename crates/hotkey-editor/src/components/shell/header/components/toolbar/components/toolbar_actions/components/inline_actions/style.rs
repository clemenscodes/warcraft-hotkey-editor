use crate::{classes, styling::TailwindClass, tw};

// The inline file-button row is shown from laptop up (the burger replaces it below).
// Its height is the box every toolbar button fills, and `h-full` takes it straight from
// the toolbar's shared row height — so the whole row rescales with the bar and matches the
// collisions and layout buttons, with no fixed size of its own. The tight inter-button gap
// keeps the full ten-button row well inside its grid track.
const BASE: &[TailwindClass] = tw![
    "hidden",
    "flex-row",
    "items-center",
    "justify-end",
    "gap-[clamp(0.2rem,0.2vw,0.5rem)]",
    "min-w-0",
    "h-full",
];

const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw!["laptop:flex"];
const DESKTOP: &[TailwindClass] = tw!["desktop:flex"];
const QHD: &[TailwindClass] = tw!["qhd:flex"];
const UHD: &[TailwindClass] = tw!["uhd:flex"];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
