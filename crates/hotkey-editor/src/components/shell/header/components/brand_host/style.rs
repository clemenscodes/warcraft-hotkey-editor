use crate::{classes, styling::TailwindClass, tw};

// The brand's box, and the query context every `cqi` length inside the brand resolves
// against. Its width is definite per band, never `flex-auto` (which grabs all free space
// and blows the wordmark up on tablet). On phones and tablets it is a fraction of the
// viewport; on laptop and up it is a capped `cqi` box measured off the header bar (the
// header is the container there), so the brand scales off the bar rather than the raw
// viewport and sits left-aligned in its grid track. Either way the whole brand scales as
// one drawing off this box — one SVG, no truncation.
const BASE: &[TailwindClass] = tw!["[container-type:inline-size]", "min-w-0"];
const MOBILE: &[TailwindClass] = tw!["mobile:w-[72vw]"];
const TABLET: &[TailwindClass] = tw!["tablet:w-[55vw]"];
const LAPTOP: &[TailwindClass] = tw!["laptop:w-[clamp(12rem,26cqi,56rem)]"];
const DESKTOP: &[TailwindClass] = tw!["desktop:w-[clamp(12rem,26cqi,56rem)]"];
const QHD: &[TailwindClass] = tw!["qhd:w-[clamp(12rem,26cqi,56rem)]"];
const UHD: &[TailwindClass] = tw!["uhd:w-[clamp(12rem,26cqi,56rem)]"];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
