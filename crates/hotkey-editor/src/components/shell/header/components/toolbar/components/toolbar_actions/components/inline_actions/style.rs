use crate::classes;

// The inline file-button row is shown from laptop up (the burger replaces it below).
// Its height is the box every toolbar button fills and scales its `cqi` chrome against,
// so expressing it as a viewport-proportional clamp makes the whole row grow with the
// viewport while fitting the grid's side track beside the centered layout button —
// `1.55vw` matches the collisions button (~30px at common desktop widths), and the tight
// inter-button gap keeps the full ten-button row well inside the track.
const BASE: &[&str] = &[
    "hidden",
    "flex-row",
    "items-center",
    "justify-end",
    "gap-[clamp(0.2rem,0.2vw,0.5rem)]",
    "min-w-0",
    "h-[clamp(1.75rem,1.55vw,4rem)]",
];

const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &["laptop:flex"];
const DESKTOP: &[&str] = &["desktop:flex"];
const QHD: &[&str] = &["qhd:flex"];
const UHD: &[&str] = &["uhd:flex"];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
