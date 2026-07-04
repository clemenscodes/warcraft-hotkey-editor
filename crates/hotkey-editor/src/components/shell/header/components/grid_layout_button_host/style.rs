use crate::{classes, styling::TailwindClass, tw};

// The layout button's box and container-query context. It owns the centered column at
// laptop width and up (hidden below, where the drawer offers the action instead) and
// marks itself the query container, so the button fills it (`w-full`) and every `cqi`
// length inside the button — padding, gap, border, radius, font, icon — resolves against
// this box. Its width is viewport-proportional (a vw clamp), so the whole button scales
// as one drawing across laptop, desktop, and 4K.
const BASE: &[TailwindClass] = tw![
    "hidden",
    "items-center",
    "justify-center",
    "[container-type:inline-size]",
    "w-[clamp(7rem,7.2vw,22rem)]",
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
