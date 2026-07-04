use crate::{classes, styling::TailwindClass, tw};

// The layout button's box and container-query context. It owns the centered column at
// laptop width and up (hidden below, where the drawer offers the action instead). The
// header stretches it to the shared row height, and `aspect-[39/10]` turns that height
// into a definite width — so the box is sized entirely by the bar, with no fixed length
// of its own. It marks itself the query container off that definite width, so the button
// fills it (`size-full`) and every `cqi` length inside the button — padding, gap, border,
// radius, font, icon — resolves against this box and scales as one drawing with the bar.
const BASE: &[TailwindClass] = tw![
    "hidden",
    "items-center",
    "justify-center",
    "aspect-[39/10]",
    "[container-type:inline-size]",
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
